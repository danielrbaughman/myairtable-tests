//! TC9 — Cache thread-safety.
//!
//! The TTL cache is shared mutable state on each table (`Mutex<HashMap<...>>` in `OrmTable`),
//! reached through an `Arc<AirtableClient>`. The caching suite only exercises it single-threaded.
//! These tests fire concurrent reads and concurrent mutations at one cached client and assert no
//! corruption/panic and a consistent result.
//!
//! Sharing model: `Airtable` is `Send + Sync` (client is `Arc<AirtableClient>`, caches are
//! `Mutex<HashMap>`), and `get_one`/`invalidate_cache` both take `&self`. So a single cached client
//! can be wrapped in `Arc<Airtable>` and driven from many concurrent tokio tasks — mirroring the
//! C# `Task.WhenAll` parity target.
//!
//! Note on parity with the C# port: C# calls `at.Client.InvalidateCache(PrimaryTable.TableId)`
//! (per-table) and `at.InvalidateAllCaches()`. The Rust client exposes per-table invalidation as
//! `at.primary.invalidate_cache()`; there is no single `invalidate_all_caches()` on `Airtable`, so
//! the "invalidate all" equivalent here invalidates each table's cache explicitly.

use std::sync::Arc;

use myairtable_tests::*;

fn setup_cached() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::with_cache(&api_key, &base_id, 60)
}

/// Unique primary-key value for a test (mirrors C# `TestSetup.PrimaryKey`).
fn primary_key(group: &str, name: &str) -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{group} {name} {nanos}")
}

/// Invalidate every table's cache — the "invalidate all caches" equivalent.
fn invalidate_all_caches(at: &Airtable) {
    at.formulas.invalidate_cache();
    at.primary.invalidate_cache();
    at.secondary.invalidate_cache();
    at.tertiary.invalidate_cache();
}

async fn best_effort_delete(at: &Airtable, id: &str) {
    let _ = at.primary.delete_one(&id.to_string()).await;
}

// =============================================================================
// Scenario 1 — concurrent gets of the same record are consistent
// =============================================================================

#[tokio::test]
async fn concurrent_gets_of_the_same_record_are_consistent() {
    let at = Arc::new(setup_cached());

    let key = primary_key("CacheConc", "Reads");
    let fields = PrimaryModel {
        primary_key: Some(key.clone()),
        ..Default::default()
    };
    let created = at.primary.create_one(&fields, false).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    // 25 concurrent gets race to populate/read the shared cache.
    let mut handles = Vec::with_capacity(25);
    for _ in 0..25 {
        let at = Arc::clone(&at);
        let id = id.clone();
        handles.push(tokio::spawn(async move { at.primary.get_one(&id).await }));
    }

    let mut results = Vec::with_capacity(handles.len());
    for h in handles {
        // No task should panic (cache corruption would surface as a poisoned-mutex panic here).
        let res = h.await.expect("get_one task panicked");
        results.push(res.expect("get_one returned an error"));
    }

    for m in &results {
        assert_eq!(m.primary_key.as_deref(), Some(key.as_str()));
        assert_eq!(m.id.as_deref(), Some(id.as_str()));
    }

    best_effort_delete(&at, &id).await;
}

// =============================================================================
// Scenario 2 — concurrent reads + mutations do not corrupt the cache
// =============================================================================

#[tokio::test]
async fn concurrent_reads_and_mutations_do_not_corrupt_the_cache() {
    let at = Arc::new(setup_cached());

    let key = primary_key("CacheConc", "Mix");
    let fields = PrimaryModel {
        primary_key: Some(key.clone()),
        ..Default::default()
    };
    let created = at.primary.create_one(&fields, false).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    // Interleave reads (populate cache), per-table invalidations, and invalidate-all concurrently.
    let mut handles = Vec::new();
    for _ in 0..15 {
        // Concurrent read.
        {
            let at = Arc::clone(&at);
            let id = id.clone();
            handles.push(tokio::spawn(async move {
                let _ = at.primary.get_one(&id).await;
            }));
        }
        // Per-table invalidation (mirrors C# `Client.InvalidateCache(PrimaryTable.TableId)`).
        {
            let at = Arc::clone(&at);
            handles.push(tokio::spawn(async move {
                at.primary.invalidate_cache();
            }));
        }
        // Invalidate-all equivalent (mirrors C# `InvalidateAllCaches`).
        {
            let at = Arc::clone(&at);
            handles.push(tokio::spawn(async move {
                invalidate_all_caches(&at);
            }));
        }
    }

    for h in handles {
        h.await.expect("concurrent task panicked");
    }

    // The client is still usable and returns the correct value afterward.
    let fetched = at.primary.get_one(&id).await.unwrap();
    assert_eq!(fetched.primary_key.as_deref(), Some(key.as_str()));

    best_effort_delete(&at, &id).await;
}
