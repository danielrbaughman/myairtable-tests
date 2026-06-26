//! TC10 - Upsert depth (Rust).
//!
//! Mirrors the C# `TestUpsertDepth` / Python `test_upsert_depth.py` suites: a multi-field merge key
//! (a match must agree on ALL merge fields), the multiple-match error path (a merge key matching
//! more than one record is rejected with 422), and a batched `upsert_many`. Exercises the generated
//! `at.primary.upsert(..., Some(&[...]))` / `upsert_many` with explicit `fields_to_merge_on`
//! (Airtable's server-side performUpsert). Insert-vs-update is inferred from the returned record id
//! (the generated upsert updates the model in place / returns the upserted models, not a wasCreated
//! flag).

use myairtable_tests::*;
use std::time::{SystemTime, UNIX_EPOCH};

// Field IDs for the Primary table (analogue of the C# PrimaryFields.*Id merge-key constants).
const PRIMARY_KEY_ID: &str = "fldol5Q4wmQJQvPRy";
const SINGLE_LINE_TEXT_ID: &str = "fld0BL2lFo9fqcKv3";

fn setup() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::new(&api_key, &base_id)
}

fn suite(tag: &str) -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("RsUpsert {millis}-{} {tag}", std::process::id())
}

fn model(primary_key: &str, single_line_text: &str, long_text: Option<&str>) -> PrimaryModel {
    PrimaryModel {
        primary_key: Some(primary_key.to_string()),
        single_line_text: Some(single_line_text.to_string()),
        long_text: long_text.map(|s| s.to_string()),
        ..Default::default()
    }
}

#[tokio::test]
async fn upsert_matches_on_multiple_merge_fields() {
    let at = setup();
    let s = suite("MultiKey");
    let mut ids: Vec<String> = Vec::new();

    // Seed a record identified by the (Primary Key, Single Line Text) pair.
    let seed = at
        .primary
        .create_one(&model(&s, "anchor", None))
        .await
        .unwrap();
    let seed_id = seed.id.clone().unwrap();
    ids.push(seed_id.clone());

    let merge_on = [PRIMARY_KEY_ID, SINGLE_LINE_TEXT_ID];

    // Same pair -> UPDATE the seed (matched on BOTH fields). Inferred by the returned id.
    let mut updated = model(&s, "anchor", Some("updated"));
    at.primary
        .upsert(&mut updated, Some(&merge_on))
        .await
        .unwrap();
    assert_eq!(updated.id.as_deref(), Some(seed_id.as_str()));
    assert_eq!(updated.long_text.as_deref(), Some("updated"));

    // Same Primary Key but a DIFFERENT Single Line Text -> no match on the pair -> INSERT.
    let mut inserted = model(&s, "different", None);
    at.primary
        .upsert(&mut inserted, Some(&merge_on))
        .await
        .unwrap();
    let inserted_id = inserted.id.clone().unwrap();
    assert_ne!(inserted_id, seed_id);
    ids.push(inserted_id);

    for id in &ids {
        at.primary.delete_one(id).await.ok();
    }
}

#[tokio::test]
async fn upsert_with_multiple_matches_errors() {
    let at = setup();
    let s = suite("MultiMatch");

    // Two records share the same Single Line Text value.
    let created = at
        .primary
        .create_many(&[
            model(&format!("{s} A"), "dupe", None),
            model(&format!("{s} B"), "dupe", None),
        ])
        .await
        .unwrap();
    let ids: Vec<String> = created.iter().map(|r| r.id.clone().unwrap()).collect();

    // Upsert merging only on Single Line Text="dupe" matches BOTH -> Airtable rejects it (422).
    let mut dup = model("ignored", "dupe", Some("x"));
    let result = at
        .primary
        .upsert(&mut dup, Some(&[SINGLE_LINE_TEXT_ID]))
        .await;
    match result {
        Err(AirtableError::Api { status, .. }) => assert_eq!(status, 422),
        other => panic!("expected Api 422 error, got {other:?}"),
    }

    for id in &ids {
        at.primary.delete_one(id).await.ok();
    }
}

#[tokio::test]
async fn upsert_many_batches_inserts_and_updates() {
    let at = setup();
    let s = suite("Batch");
    let mut ids: Vec<String> = Vec::new();

    // Seed one record; the batch will UPDATE it and INSERT a second.
    let seed = at
        .primary
        .create_one(&model(&s, "keep", None))
        .await
        .unwrap();
    let seed_id = seed.id.clone().unwrap();
    ids.push(seed_id.clone());

    let merge_on = [PRIMARY_KEY_ID, SINGLE_LINE_TEXT_ID];
    let batch = [model(&s, "keep", Some("batched")), model(&s, "fresh", None)];
    let out = at.primary.upsert_many(&batch, &merge_on).await.unwrap();
    assert_eq!(out.len(), 2);

    // The "keep" row updated the seed; the "fresh" row is new.
    let updated = out
        .iter()
        .find(|r| r.single_line_text.as_deref() == Some("keep"))
        .unwrap();
    assert_eq!(updated.id.as_deref(), Some(seed_id.as_str()));
    assert_eq!(updated.long_text.as_deref(), Some("batched"));

    let fresh = out
        .iter()
        .find(|r| r.single_line_text.as_deref() == Some("fresh"))
        .unwrap();
    let fresh_id = fresh.id.clone().unwrap();
    assert_ne!(fresh_id, seed_id);
    ids.push(fresh_id);

    for id in &ids {
        at.primary.delete_one(id).await.ok();
    }
}
