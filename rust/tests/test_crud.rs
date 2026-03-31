use myairtable_tests::*;

fn setup() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::new(&api_key, &base_id)
}

// =============================================================================
// Single record CRUD
// =============================================================================

#[tokio::test]
async fn create_get_update_delete() {
    let at = setup();
    let table = at.primary();

    // Create
    let fields = CreatePrimary {
        primary_key: Some("Rust CRUD Test".to_string()),
        single_line_text: Some("Hello from Rust".to_string()),
        number_int: Some(42),
        checkbox: Some(true),
        ..Default::default()
    };
    let created = table.create(&fields).await.unwrap();
    assert!(!created.id.is_empty());
    assert_eq!(
        created.fields.primary_key.as_deref(),
        Some("Rust CRUD Test")
    );
    assert_eq!(
        created.fields.single_line_text.as_deref(),
        Some("Hello from Rust")
    );
    assert_eq!(created.fields.number_int, Some(42));
    assert_eq!(created.fields.checkbox, Some(true));

    let record_id = created.id.clone();

    // Get
    let fetched = table.get(&record_id).await.unwrap();
    assert_eq!(fetched.id, record_id);
    assert_eq!(
        fetched.fields.primary_key.as_deref(),
        Some("Rust CRUD Test")
    );

    // Computed fields should be populated on read
    assert!(fetched.fields.auto_number.is_some());
    assert!(fetched.fields.created_at_time.is_some());

    // Update
    let update_fields = CreatePrimary {
        single_line_text: Some("Updated from Rust".to_string()),
        number_int: Some(99),
        ..Default::default()
    };
    let updated = table.update(&record_id, &update_fields).await.unwrap();
    assert_eq!(
        updated.fields.single_line_text.as_deref(),
        Some("Updated from Rust")
    );
    assert_eq!(updated.fields.number_int, Some(99));
    // Unchanged field should still be there
    assert_eq!(
        updated.fields.primary_key.as_deref(),
        Some("Rust CRUD Test")
    );

    // Delete
    table.delete(&record_id).await.unwrap();

    // Verify deleted
    let result = table.get(&record_id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn create_with_select_fields() {
    let at = setup();
    let table = at.primary();

    let fields = CreatePrimary {
        primary_key: Some("Rust Select Test".to_string()),
        single_select: Some(PrimarySingleSelectOption::Choice2),
        multiple_select: Some(vec![
            PrimaryMultipleSelectOption::Option1,
            PrimaryMultipleSelectOption::Option3,
        ]),
        ..Default::default()
    };

    let created = table.create(&fields).await.unwrap();
    assert_eq!(
        created.fields.single_select,
        Some(PrimarySingleSelectOption::Choice2)
    );
    assert_eq!(
        created.fields.multiple_select,
        Some(vec![
            PrimaryMultipleSelectOption::Option1,
            PrimaryMultipleSelectOption::Option3,
        ])
    );

    // Cleanup
    table.delete(&created.id).await.unwrap();
}

// =============================================================================
// Batch operations
// =============================================================================

#[tokio::test]
async fn create_many_and_delete_many() {
    let at = setup();
    let table = at.primary();

    // Create 3 records at once
    let records: Vec<CreatePrimary> = (1..=3)
        .map(|i| CreatePrimary {
            primary_key: Some(format!("Rust Batch {i}")),
            number_int: Some(i),
            ..Default::default()
        })
        .collect();

    let created = table.create_many(&records).await.unwrap();
    assert_eq!(created.len(), 3);
    for (i, record) in created.iter().enumerate() {
        assert_eq!(
            record.fields.primary_key.as_deref(),
            Some(format!("Rust Batch {}", i + 1).as_str())
        );
    }

    // Delete all
    let ids: Vec<RecordId> = created.iter().map(|r| r.id.clone()).collect();
    table.delete_many(&ids).await.unwrap();

    // Verify first one is deleted
    let result = table.get(&ids[0]).await;
    assert!(result.is_err());
}

// =============================================================================
// List records
// =============================================================================

#[tokio::test]
async fn list_records() {
    let at = setup();
    let table = at.secondary();

    let page = table.list(None).await.unwrap();
    // The test base should have at least some records
    assert!(!page.records.is_empty());

    // Each record should have an ID
    for record in &page.records {
        assert!(!record.id.is_empty());
    }
}
