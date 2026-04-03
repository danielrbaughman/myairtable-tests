use myairtable_tests::*;

fn setup_cached() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::with_cache(&api_key, &base_id, 60)
}

fn setup_uncached() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::new(&api_key, &base_id)
}

// =============================================================================
// Cache hit — second get returns same data without API call
// =============================================================================

#[tokio::test]
async fn cache_hit_returns_same_data() {
    let at = setup_cached();

    let fields = CreatePrimaryModel {
        primary_key: Some("Cache Hit Test".to_string()),
        ..Default::default()
    };
    let created = at.primary.create_one(&fields).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    // First get populates cache
    let first = at.primary.get_one(&id).await.unwrap();
    // Second get should return cached data
    let second = at.primary.get_one(&id).await.unwrap();

    assert_eq!(first.primary_key, second.primary_key);
    assert_eq!(first.id, second.id);

    at.primary.delete_one(&id).await.unwrap();
}

// =============================================================================
// Cache disabled by default
// =============================================================================

#[tokio::test]
async fn cache_disabled_by_default() {
    let at = setup_uncached();

    let fields = CreatePrimaryModel {
        primary_key: Some("No Cache Test".to_string()),
        ..Default::default()
    };
    let created = at.primary.create_one(&fields).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    // Get should work but not cache (cache_seconds = 0)
    let result = at.primary.get_one(&id).await.unwrap();
    assert_eq!(result.primary_key.as_deref(), Some("No Cache Test"));

    at.primary.delete_one(&id).await.unwrap();
}

// =============================================================================
// Mutation invalidation
// =============================================================================

#[tokio::test]
async fn create_invalidates_cache() {
    let at = setup_cached();

    let fields = CreatePrimaryModel {
        primary_key: Some("Mutation Test 1".to_string()),
        ..Default::default()
    };
    let created = at.primary.create_one(&fields).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    // Populate cache
    at.primary.get_one(&id).await.unwrap();

    // Create should invalidate cache
    let fields2 = CreatePrimaryModel {
        primary_key: Some("Mutation Test 2".to_string()),
        ..Default::default()
    };
    let created2 = at.primary.create_one(&fields2).await.unwrap();
    let id2 = created2.id.as_deref().unwrap().to_string();

    // After invalidation, get should hit API (not cache)
    let result = at.primary.get_one(&id).await.unwrap();
    assert_eq!(result.primary_key.as_deref(), Some("Mutation Test 1"));

    at.primary.delete_one(&id).await.unwrap();
    at.primary.delete_one(&id2).await.unwrap();
}

#[tokio::test]
async fn update_invalidates_cache() {
    let at = setup_cached();

    let fields = CreatePrimaryModel {
        primary_key: Some("Update Invalidation".to_string()),
        ..Default::default()
    };
    let created = at.primary.create_one(&fields).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    // Populate cache
    at.primary.get_one(&id).await.unwrap();

    // Update invalidates cache
    let update = CreatePrimaryModel {
        primary_key: Some("Updated Value".to_string()),
        ..Default::default()
    };
    at.primary.update_one(&id, &update).await.unwrap();

    // Next get should see updated value (cache was invalidated)
    let result = at.primary.get_one(&id).await.unwrap();
    assert_eq!(result.primary_key.as_deref(), Some("Updated Value"));

    at.primary.delete_one(&id).await.unwrap();
}

#[tokio::test]
async fn delete_invalidates_cache() {
    let at = setup_cached();

    let fields = CreatePrimaryModel {
        primary_key: Some("Delete Invalidation".to_string()),
        ..Default::default()
    };
    let created = at.primary.create_one(&fields).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    // Populate cache
    at.primary.get_one(&id).await.unwrap();

    // Delete invalidates cache
    at.primary.delete_one(&id).await.unwrap();

    // Next get should fail (record deleted, cache cleared)
    assert!(at.primary.get_one(&id).await.is_err());
}

// =============================================================================
// Manual invalidation
// =============================================================================

#[tokio::test]
async fn manual_invalidate_cache() {
    let at = setup_cached();

    let fields = CreatePrimaryModel {
        primary_key: Some("Manual Invalidation".to_string()),
        ..Default::default()
    };
    let created = at.primary.create_one(&fields).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    // Populate cache
    at.primary.get_one(&id).await.unwrap();

    // Manual invalidation
    at.primary.invalidate_cache();

    // Get still works (re-fetches from API)
    let result = at.primary.get_one(&id).await.unwrap();
    assert_eq!(result.id.as_deref(), Some(id.as_str()));

    at.primary.delete_one(&id).await.unwrap();
}

// =============================================================================
// Cache expiry
// =============================================================================

#[tokio::test]
async fn cache_expires_after_ttl() {
    let mut at = setup_uncached();
    at.primary.set_cache_seconds(1); // 1 second TTL

    let fields = CreatePrimaryModel {
        primary_key: Some("Expiry Test".to_string()),
        ..Default::default()
    };
    let created = at.primary.create_one(&fields).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    // Populate cache
    let first = at.primary.get_one(&id).await.unwrap();
    assert_eq!(first.primary_key.as_deref(), Some("Expiry Test"));

    // Wait for TTL to expire
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // After expiry, should re-fetch from API
    let after = at.primary.get_one(&id).await.unwrap();
    assert_eq!(after.primary_key.as_deref(), Some("Expiry Test"));

    at.primary.delete_one(&id).await.unwrap();
}

// =============================================================================
// Cache serves stale data
// =============================================================================

#[tokio::test]
async fn cache_serves_stale_data() {
    let at = setup_cached();

    let fields = CreatePrimaryModel {
        primary_key: Some("Stale Data Test".to_string()),
        ..Default::default()
    };
    let created = at.primary.create_one(&fields).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    // Populate cache
    let first = at.primary.get_one(&id).await.unwrap();
    assert_eq!(first.primary_key.as_deref(), Some("Stale Data Test"));

    // Update via dict layer (bypasses ORM cache invalidation)
    let mut update_fields = Fields::new();
    update_fields.set(PrimaryFields::PRIMARY_KEY_ID, "Updated Directly");
    at.primary
        .dict
        .update_one(&id, &update_fields, true)
        .await
        .unwrap();

    // ORM cache should still return stale data
    let stale = at.primary.get_one(&id).await.unwrap();
    assert_eq!(stale.primary_key.as_deref(), Some("Stale Data Test"));

    at.primary.delete_one(&id).await.unwrap();
}

// =============================================================================
// Different parameters produce different cache keys
// =============================================================================

#[tokio::test]
async fn different_queries_cached_independently() {
    let at = setup_cached();

    let fields1 = CreatePrimaryModel {
        primary_key: Some("Cache Key 1".to_string()),
        ..Default::default()
    };
    let fields2 = CreatePrimaryModel {
        primary_key: Some("Cache Key 2".to_string()),
        ..Default::default()
    };
    let c1 = at.primary.create_one(&fields1).await.unwrap();
    let c2 = at.primary.create_one(&fields2).await.unwrap();
    let id1 = c1.id.as_deref().unwrap().to_string();
    let id2 = c2.id.as_deref().unwrap().to_string();

    // Each get populates a separate cache entry
    let get1 = at.primary.get_one(&id1).await.unwrap();
    let get2 = at.primary.get_one(&id2).await.unwrap();

    // Re-get returns correct cached data (not mixed up)
    let reget1 = at.primary.get_one(&id1).await.unwrap();
    let reget2 = at.primary.get_one(&id2).await.unwrap();
    assert_eq!(reget1.primary_key, get1.primary_key);
    assert_eq!(reget2.primary_key, get2.primary_key);

    at.primary.delete_one(&id1).await.unwrap();
    at.primary.delete_one(&id2).await.unwrap();
}

// =============================================================================
// Formula query caching
// =============================================================================

#[tokio::test]
async fn formula_query_cached() {
    let at = setup_cached();

    let fields = CreatePrimaryModel {
        primary_key: Some("Formula Cache Test".to_string()),
        ..Default::default()
    };
    let created = at.primary.create_one(&fields).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    // Query with formula
    let query = AirtableQuery::new().formula(PrimaryModel::F.primary_key.equals(
        "Formula Cache Test",
        true,
        false,
    ));
    let first = at.primary.get_many(&query).await.unwrap();
    assert!(!first.is_empty());

    // Second query with same formula returns cached data
    let second = at.primary.get_many(&query).await.unwrap();
    assert_eq!(first.len(), second.len());

    at.primary.delete_one(&id).await.unwrap();
}
