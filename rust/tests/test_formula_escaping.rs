// TC2 — Formula-value escaping. Filter predicates build formulas by interpolating user values into
// quoted string literals; a value containing a quote, backslash, or other meta-character must be
// escaped or the formula breaks (or silently mis-matches). These tests round-trip special-character
// values through storage AND through the generated `.equals()/.contains()` filter DSL against the
// live base, proving the escaping is correct. Parity target for the other 8 suites.

use myairtable_tests::*;

fn setup() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::new(&api_key, &base_id)
}

// A primary-key value unique to this run (distinct per scenario via `label`).
fn primary_key(suite: &str, label: &str) -> String {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let rand = ts.wrapping_mul(2654435761) % 900000 + 100000;
    format!("Rust {suite} {label} {ts}-{rand}")
}

// Values that break naive string interpolation into an Airtable formula literal.
// Newline is covered separately (storage only) — Airtable formula string literals can't carry a
// raw newline, so it's a storage concern, not a filter one.
fn special_values() -> Vec<&'static str> {
    vec![
        "O'Brien single quote",
        "say \"hi\" double quote",
        "back\\slash path",
        "trailing backslash\\",
        "mixed a\"b'c\\d",
        "unicode café ☕ 日本語 🎉",
    ]
}

// Best-effort cleanup: ignore failures (the live base tolerates leftover records).
async fn try_delete(at: &Airtable, id: &str) {
    let _ = at.primary.delete_one(&id.to_string()).await;
}

// =============================================================================
// Eq filter matches a value with special characters
// =============================================================================

#[tokio::test]
async fn eq_filter_matches_value_with_special_chars() {
    let at = setup();
    let f = PrimaryModel::F;

    for special in special_values() {
        let suite = primary_key("Escaping", "Eq");

        let created = at
            .primary
            .create_one(&PrimaryModel {
                primary_key: Some(suite.clone()),
                single_line_text: Some(special.to_string()),
                ..Default::default()
            })
            .await
            .unwrap();
        let record_id = created.id.as_deref().unwrap().to_string();

        // 1. Storage round-trips the exact value.
        assert_eq!(
            created.single_line_text.as_deref(),
            Some(special),
            "storage did not round-trip value: {special:?}"
        );

        // 2. The .equals() filter, scoped to this record, matches via correctly-escaped interpolation.
        let scope = format!("FIND(\"{suite}\", {{Primary Key}})");
        let filter = formula::AND(&[&scope, &f.single_line_text.equals(special, true, false)]);
        let params = AirtableQuery::new().formula(filter);
        let results = at.primary.get_many(&params).await.unwrap();

        assert!(
            results
                .iter()
                .any(|r| r.id.as_deref() == Some(record_id.as_str())),
            "Eq filter did not match record for value: {special:?}"
        );
        for r in &results {
            assert_eq!(
                r.single_line_text.as_deref(),
                Some(special),
                "Eq filter matched a record with wrong value for: {special:?}"
            );
        }

        try_delete(&at, &record_id).await;
    }
}

// =============================================================================
// Contains filter matches a value with special characters
// =============================================================================

#[tokio::test]
async fn contains_filter_matches_value_with_special_chars() {
    let at = setup();
    let f = PrimaryModel::F;

    for special in special_values() {
        let suite = primary_key("Escaping", "Contains");
        // Embed the special token inside a longer value so Contains (FIND>0) is a real substring test.
        let stored = format!("prefix {special} suffix");

        let created = at
            .primary
            .create_one(&PrimaryModel {
                primary_key: Some(suite.clone()),
                single_line_text: Some(stored.clone()),
                ..Default::default()
            })
            .await
            .unwrap();
        let record_id = created.id.as_deref().unwrap().to_string();

        let scope = format!("FIND(\"{suite}\", {{Primary Key}})");
        let filter = formula::AND(&[&scope, &f.single_line_text.contains(special, true, false)]);
        let params = AirtableQuery::new().formula(filter);
        let results = at.primary.get_many(&params).await.unwrap();

        assert!(
            results
                .iter()
                .any(|r| r.id.as_deref() == Some(record_id.as_str())),
            "Contains filter did not match record for value: {special:?}"
        );

        try_delete(&at, &record_id).await;
    }
}

// =============================================================================
// Special characters in the primary key itself, matched with .equals()
// =============================================================================

#[tokio::test]
async fn special_characters_in_primary_key_round_trip_and_filter() {
    let at = setup();
    let f = PrimaryModel::F;

    let suite = primary_key("Escaping", "PK");
    let pk = format!("{suite} O'Brien \"quote\" back\\slash");

    let created = at
        .primary
        .create_one(&PrimaryModel {
            primary_key: Some(pk.clone()),
            ..Default::default()
        })
        .await
        .unwrap();
    let record_id = created.id.as_deref().unwrap().to_string();

    assert_eq!(created.primary_key.as_deref(), Some(pk.as_str()));

    let params = AirtableQuery::new().formula(f.primary_key.equals(&pk, true, false));
    let results = at.primary.get_many(&params).await.unwrap();

    assert!(
        results
            .iter()
            .any(|r| r.id.as_deref() == Some(record_id.as_str())),
        "Eq filter did not match record with special chars in primary key"
    );
    assert_eq!(
        results.len(),
        1,
        "Eq on primary key should match exactly one record"
    );

    try_delete(&at, &record_id).await;
}

// =============================================================================
// Newline / tab values round-trip through storage (long text)
// =============================================================================

#[tokio::test]
async fn newline_and_tab_values_round_trip_through_storage() {
    let at = setup();

    // Newlines/tabs are a storage concern (long text), not expressible in a formula literal.
    // Verify they survive create -> fetch unchanged.
    let suite = primary_key("Escaping", "Newline");
    let value = "line1\nline2\twith tab\r\nwindows";

    let created = at
        .primary
        .create_one(&PrimaryModel {
            primary_key: Some(suite),
            long_text: Some(value.to_string()),
            ..Default::default()
        })
        .await
        .unwrap();
    let record_id = created.id.as_deref().unwrap().to_string();

    let fetched = at.primary.get_one(&record_id).await.unwrap();
    assert_eq!(
        fetched.long_text.as_deref(),
        Some(value),
        "newline/tab value did not round-trip through storage"
    );

    try_delete(&at, &record_id).await;
}
