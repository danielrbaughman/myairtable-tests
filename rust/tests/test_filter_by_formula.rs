use myairtable_tests::*;

fn setup() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::new(&api_key, &base_id)
}

/// Create a record, return its ID.
async fn create_primary(at: &Airtable, fields: &Fields) -> Record {
    at.primary.create_one(fields, true).await.unwrap()
}

fn primary(name: &str) -> Fields {
    let mut f = Fields::new();
    f.set(PrimaryFields::PRIMARY_KEY_ID, name);
    f
}

/// Build an OR(RECORD_ID()='id1', RECORD_ID()='id2', ...) formula to scope to specific records.
fn scope_to(ids: &[&str]) -> String {
    if ids.len() == 1 {
        return format!("RECORD_ID()='{}'", ids[0]);
    }
    let parts: Vec<String> = ids.iter().map(|id| format!("RECORD_ID()='{id}'")).collect();
    format!("OR({})", parts.join(","))
}

// =============================================================================
// Filter by view
// =============================================================================

#[tokio::test]
async fn filter_by_view() {
    let at = setup();

    // Create 5 "Filter Test" records (should appear in "Filter by View" view)
    // and 5 "Don't Include" records (should not appear)
    let mut all_records = Vec::new();
    for i in 0..5 {
        all_records.push(primary(&format!("Filter Test {i}")));
    }
    for i in 0..5 {
        all_records.push(primary(&format!("Don't Include Test {i}")));
    }
    let created = at.primary.create_many(&all_records, true).await.unwrap();
    let ids: Vec<String> = created.iter().map(|r| r.id.clone()).collect();

    // Query with the "Filter by View" view
    let params = AirtableQuery::new().view(PrimaryView::FilterByView);
    let results = at.primary.get_many(&params).await.unwrap();

    // Should only contain "Filter Test" records
    assert_eq!(results.records.len(), 5);
    for record in &results.records {
        let name = record
            .fields
            .get(PrimaryFields::PRIMARY_KEY_ID)
            .unwrap()
            .as_str()
            .unwrap();
        assert!(name.starts_with("Filter Test"), "Unexpected record: {name}");
    }

    // Cleanup
    at.primary.delete_many(&ids).await.unwrap();
}

// =============================================================================
// Formula filter
// =============================================================================

#[tokio::test]
async fn filter_by_formula() {
    let at = setup();

    let mut f1 = primary("FilterTest A");
    f1.set(PrimaryFields::NUMBER_INT_ID, 10);
    let mut f2 = primary("FilterTest B");
    f2.set(PrimaryFields::NUMBER_INT_ID, 20);

    let r1 = create_primary(&at, &f1).await;
    let r2 = create_primary(&at, &f2).await;

    // Filter: number = 20, scoped to our records
    let params = AirtableQuery::new().formula(format!(
        "AND({},{{{}}}=20)",
        scope_to(&[&r1.id, &r2.id]),
        PrimaryFields::NUMBER_INT
    ));
    let results = at.primary.get_many(&params).await.unwrap();
    assert_eq!(results.records.len(), 1);
    assert_eq!(
        results.records[0]
            .fields
            .get(PrimaryFields::NUMBER_INT_ID)
            .unwrap(),
        20
    );

    at.primary.delete_many(&[r1.id, r2.id]).await.unwrap();
}

// =============================================================================
// Max records
// =============================================================================

#[tokio::test]
async fn max_records() {
    let at = setup();

    let records: Vec<Fields> = (1..=5)
        .map(|i| primary(&format!("MaxRecTest {i}")))
        .collect();
    let created = at.primary.create_many(&records, true).await.unwrap();
    let ids: Vec<String> = created.iter().map(|r| r.id.clone()).collect();
    let id_refs: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();

    let params = AirtableQuery::new()
        .formula(scope_to(&id_refs))
        .max_records(3);
    let results = at.primary.get_many(&params).await.unwrap();
    assert_eq!(results.records.len(), 3);

    at.primary.delete_many(&ids).await.unwrap();
}

// =============================================================================
// Sort
// =============================================================================

#[tokio::test]
async fn sort_records() {
    let at = setup();

    let mut f1 = primary("SortTest C");
    f1.set(PrimaryFields::NUMBER_INT_ID, 30);
    let mut f2 = primary("SortTest A");
    f2.set(PrimaryFields::NUMBER_INT_ID, 10);
    let mut f3 = primary("SortTest B");
    f3.set(PrimaryFields::NUMBER_INT_ID, 20);

    let created = at.primary.create_many(&[f1, f2, f3], true).await.unwrap();
    let ids: Vec<String> = created.iter().map(|r| r.id.clone()).collect();
    let id_refs: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();

    // Sort ascending
    let params = AirtableQuery::new()
        .formula(scope_to(&id_refs))
        .sort(PrimaryFields::NUMBER_INT, SortDirection::Asc);
    let results = at.primary.get_many(&params).await.unwrap();
    let nums: Vec<i64> = results
        .records
        .iter()
        .map(|r| {
            r.fields
                .get(PrimaryFields::NUMBER_INT_ID)
                .unwrap()
                .as_i64()
                .unwrap()
        })
        .collect();
    assert_eq!(nums, vec![10, 20, 30]);

    // Sort descending
    let params = AirtableQuery::new()
        .formula(scope_to(&id_refs))
        .sort(PrimaryFields::NUMBER_INT, SortDirection::Desc);
    let results = at.primary.get_many(&params).await.unwrap();
    let nums: Vec<i64> = results
        .records
        .iter()
        .map(|r| {
            r.fields
                .get(PrimaryFields::NUMBER_INT_ID)
                .unwrap()
                .as_i64()
                .unwrap()
        })
        .collect();
    assert_eq!(nums, vec![30, 20, 10]);

    at.primary.delete_many(&ids).await.unwrap();
}

// =============================================================================
// Field selection
// =============================================================================

#[tokio::test]
async fn field_selection() {
    let at = setup();

    let mut f = primary("FieldSelectTest");
    f.set(PrimaryFields::SINGLE_LINE_TEXT_ID, "Hello");
    f.set(PrimaryFields::NUMBER_INT_ID, 42);

    let created = create_primary(&at, &f).await;

    let params = AirtableQuery::new()
        .formula(format!("RECORD_ID()='{}'", created.id))
        .fields(vec![PrimaryFields::PRIMARY_KEY_ID.to_string()]);
    let results = at.primary.get_many(&params).await.unwrap();
    assert_eq!(results.records.len(), 1);

    let fields = &results.records[0].fields;
    assert!(fields.get(PrimaryFields::PRIMARY_KEY_ID).is_some());
    assert!(fields.get(PrimaryFields::SINGLE_LINE_TEXT_ID).is_none());
    assert!(fields.get(PrimaryFields::NUMBER_INT_ID).is_none());

    at.primary.delete_one(&created.id).await.unwrap();
}
