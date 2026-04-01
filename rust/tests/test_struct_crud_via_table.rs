use myairtable_tests::*;
use serde_json::json;

fn setup() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::new(&api_key, &base_id)
}

// =============================================================================
// Primary key only — basic CRUD
// =============================================================================

#[tokio::test]
async fn primary_key_only_crud() {
    let at = setup();

    // Create
    let mut fields = Fields::new();
    fields.set(PrimaryFields::PRIMARY_KEY_ID, "Rust Primary Key Only");

    let created = at.primary.create_one(&fields, true).await.unwrap();
    assert!(!created.id.is_empty());
    assert_eq!(
        created.fields.get(PrimaryFields::PRIMARY_KEY_ID).unwrap(),
        "Rust Primary Key Only"
    );

    let record_id = created.id.clone();

    // Read
    let fetched = at.primary.get_one(&record_id, true).await.unwrap();
    assert_eq!(fetched.id, record_id);
    assert_eq!(
        fetched.fields.get(PrimaryFields::PRIMARY_KEY_ID).unwrap(),
        "Rust Primary Key Only"
    );

    // Update
    let mut update = Fields::new();
    update.set(PrimaryFields::PRIMARY_KEY_ID, "Rust Updated Primary Key");
    let updated = at
        .primary
        .update_one(&record_id, &update, true)
        .await
        .unwrap();
    assert_eq!(
        updated.fields.get(PrimaryFields::PRIMARY_KEY_ID).unwrap(),
        "Rust Updated Primary Key"
    );

    // Delete
    at.primary.delete_one(&record_id).await.unwrap();

    // Verify deleted
    let result = at.primary.get_one(&record_id, true).await;
    assert!(result.is_err());
}

// =============================================================================
// All simple properties
// =============================================================================

#[tokio::test]
async fn all_simple_properties_crud() {
    let at = setup();

    // Create with all simple field types
    let mut fields = Fields::new();
    fields.set(PrimaryFields::PRIMARY_KEY_ID, "Rust All Props");
    fields.set(PrimaryFields::SINGLE_LINE_TEXT_ID, "Hello World");
    fields.set(PrimaryFields::LONG_TEXT_ID, "Long text content");
    fields.set(
        PrimaryFields::LONG_TEXT_WITH_RICH_TEXT_ID,
        "Rich text content",
    );
    fields.set(PrimaryFields::EMAIL_ID, "test@example.com");
    fields.set(PrimaryFields::URL_ID, "https://example.com");
    fields.set(PrimaryFields::PHONE_NUMBER_ID, "555-1234");
    fields.set(PrimaryFields::CHECKBOX_ID, true);
    fields.set(PrimaryFields::NUMBER_INT_ID, 42);
    fields.set(PrimaryFields::NUMBER_FLOAT_ID, 3.14);
    fields.set(PrimaryFields::CURRENCY_INT_ID, 10);
    fields.set(PrimaryFields::CURRENCY_FLOAT_ID, 9.99);
    fields.set(PrimaryFields::PERCENT_INT_ID, 0.5);
    fields.set(PrimaryFields::PERCENT_FLOAT_ID, 0.333);
    fields.set(PrimaryFields::DURATION_ID, 3600);
    fields.set(PrimaryFields::RATING_ID, 3);
    fields.set(PrimaryFields::DATE_ID, "2025-01-15");
    fields.set(PrimaryFields::DATE_WITH_TIME_ID, "2025-01-15T10:00:00.000Z");
    fields.set(PrimaryFields::SINGLE_SELECT_ID, "Choice 1");
    fields.set(
        PrimaryFields::MULTIPLE_SELECT_ID,
        json!(["Option 1", "Option 2"]),
    );

    let created = at.primary.create_one(&fields, true).await.unwrap();
    let record_id = created.id.clone();
    let f = &created.fields;

    assert_eq!(
        f.get(PrimaryFields::PRIMARY_KEY_ID).unwrap(),
        "Rust All Props"
    );
    assert_eq!(
        f.get(PrimaryFields::SINGLE_LINE_TEXT_ID).unwrap(),
        "Hello World"
    );
    assert_eq!(
        f.get(PrimaryFields::LONG_TEXT_ID).unwrap(),
        "Long text content"
    );
    assert_eq!(
        f.get(PrimaryFields::LONG_TEXT_WITH_RICH_TEXT_ID).unwrap(),
        "Rich text content"
    );
    assert_eq!(f.get(PrimaryFields::EMAIL_ID).unwrap(), "test@example.com");
    assert_eq!(f.get(PrimaryFields::URL_ID).unwrap(), "https://example.com");
    assert_eq!(f.get(PrimaryFields::PHONE_NUMBER_ID).unwrap(), "555-1234");
    assert_eq!(f.get(PrimaryFields::CHECKBOX_ID).unwrap(), true);
    assert_eq!(f.get(PrimaryFields::NUMBER_INT_ID).unwrap(), 42);
    assert_eq!(f.get(PrimaryFields::NUMBER_FLOAT_ID).unwrap(), 3.14);
    assert_eq!(f.get(PrimaryFields::CURRENCY_INT_ID).unwrap(), 10);
    assert_eq!(f.get(PrimaryFields::CURRENCY_FLOAT_ID).unwrap(), 9.99);
    assert_eq!(f.get(PrimaryFields::PERCENT_INT_ID).unwrap(), 0.5);
    assert_eq!(f.get(PrimaryFields::PERCENT_FLOAT_ID).unwrap(), 0.333);
    assert_eq!(f.get(PrimaryFields::DURATION_ID).unwrap(), 3600);
    assert_eq!(f.get(PrimaryFields::RATING_ID).unwrap(), 3);
    assert_eq!(f.get(PrimaryFields::DATE_ID).unwrap(), "2025-01-15");
    assert_eq!(
        f.get(PrimaryFields::DATE_WITH_TIME_ID).unwrap(),
        "2025-01-15T10:00:00.000Z"
    );
    assert_eq!(f.get(PrimaryFields::SINGLE_SELECT_ID).unwrap(), "Choice 1");
    assert_eq!(
        f.get(PrimaryFields::MULTIPLE_SELECT_ID).unwrap(),
        &json!(["Option 1", "Option 2"])
    );

    // Read
    let read = at.primary.get_one(&record_id, true).await.unwrap();
    assert_eq!(
        read.fields.get(PrimaryFields::PRIMARY_KEY_ID).unwrap(),
        "Rust All Props"
    );
    assert_eq!(
        read.fields.get(PrimaryFields::SINGLE_LINE_TEXT_ID).unwrap(),
        "Hello World"
    );
    assert_eq!(read.fields.get(PrimaryFields::CHECKBOX_ID).unwrap(), true);
    assert_eq!(read.fields.get(PrimaryFields::NUMBER_INT_ID).unwrap(), 42);
    assert_eq!(
        read.fields.get(PrimaryFields::SINGLE_SELECT_ID).unwrap(),
        "Choice 1"
    );

    // Update all fields
    let mut update = Fields::new();
    update.set(PrimaryFields::PRIMARY_KEY_ID, "Rust Updated All Props");
    update.set(PrimaryFields::SINGLE_LINE_TEXT_ID, "Updated Hello");
    update.set(PrimaryFields::LONG_TEXT_ID, "Updated long text");
    update.set(
        PrimaryFields::LONG_TEXT_WITH_RICH_TEXT_ID,
        "Updated rich text",
    );
    update.set(PrimaryFields::EMAIL_ID, "updated@example.com");
    update.set(PrimaryFields::URL_ID, "https://updated.com");
    update.set(PrimaryFields::PHONE_NUMBER_ID, "555-5678");
    update.set(PrimaryFields::CHECKBOX_ID, false);
    update.set(PrimaryFields::NUMBER_INT_ID, 100);
    update.set(PrimaryFields::NUMBER_FLOAT_ID, 2.72);
    update.set(PrimaryFields::CURRENCY_INT_ID, 20);
    update.set(PrimaryFields::CURRENCY_FLOAT_ID, 19.99);
    update.set(PrimaryFields::PERCENT_INT_ID, 0.75);
    update.set(PrimaryFields::PERCENT_FLOAT_ID, 0.667);
    update.set(PrimaryFields::DURATION_ID, 7200);
    update.set(PrimaryFields::RATING_ID, 5);
    update.set(PrimaryFields::DATE_ID, "2025-06-15");
    update.set(PrimaryFields::DATE_WITH_TIME_ID, "2025-06-15T14:00:00.000Z");
    update.set(PrimaryFields::SINGLE_SELECT_ID, "Choice 2");
    update.set(
        PrimaryFields::MULTIPLE_SELECT_ID,
        json!(["Option 2", "Option 3"]),
    );

    let updated = at
        .primary
        .update_one(&record_id, &update, true)
        .await
        .unwrap();
    let f = &updated.fields;
    assert_eq!(
        f.get(PrimaryFields::PRIMARY_KEY_ID).unwrap(),
        "Rust Updated All Props"
    );
    assert_eq!(
        f.get(PrimaryFields::SINGLE_LINE_TEXT_ID).unwrap(),
        "Updated Hello"
    );
    assert_eq!(f.get(PrimaryFields::NUMBER_INT_ID).unwrap(), 100);
    assert_eq!(f.get(PrimaryFields::SINGLE_SELECT_ID).unwrap(), "Choice 2");
    assert_eq!(
        f.get(PrimaryFields::MULTIPLE_SELECT_ID).unwrap(),
        &json!(["Option 2", "Option 3"])
    );

    // Delete
    at.primary.delete_one(&record_id).await.unwrap();
    assert!(at.primary.get_one(&record_id, true).await.is_err());
}

// =============================================================================
// Linked records
// =============================================================================

#[tokio::test]
async fn linked_records_crud() {
    let at = setup();

    // Setup: create two secondary records
    let mut sec1_fields = Fields::new();
    sec1_fields.set(SecondaryFields::NAME_ID, "Rust Link Target 1");
    sec1_fields.set(SecondaryFields::VALUE_ID, "val1");
    let sec1 = at.secondary.create_one(&sec1_fields, true).await.unwrap();

    let mut sec2_fields = Fields::new();
    sec2_fields.set(SecondaryFields::NAME_ID, "Rust Link Target 2");
    sec2_fields.set(SecondaryFields::VALUE_ID, "val2");
    let sec2 = at.secondary.create_one(&sec2_fields, true).await.unwrap();

    // Create primary with links
    let mut fields = Fields::new();
    fields.set(PrimaryFields::PRIMARY_KEY_ID, "Rust Link Test");
    fields.set(PrimaryFields::LINK_SINGLE_ID, json!([&sec1.id]));
    fields.set(PrimaryFields::LINK_MULTIPLE_ID, json!([&sec1.id, &sec2.id]));

    let created = at.primary.create_one(&fields, true).await.unwrap();
    let record_id = created.id.clone();
    assert_eq!(
        created.fields.get(PrimaryFields::LINK_SINGLE_ID).unwrap(),
        &json!([&sec1.id])
    );
    assert_eq!(
        created.fields.get(PrimaryFields::LINK_MULTIPLE_ID).unwrap(),
        &json!([&sec1.id, &sec2.id])
    );

    // Read
    let read = at.primary.get_one(&record_id, true).await.unwrap();
    assert_eq!(
        read.fields.get(PrimaryFields::LINK_SINGLE_ID).unwrap(),
        &json!([&sec1.id])
    );
    assert_eq!(
        read.fields.get(PrimaryFields::LINK_MULTIPLE_ID).unwrap(),
        &json!([&sec1.id, &sec2.id])
    );

    // Update: swap links
    let mut update = Fields::new();
    update.set(PrimaryFields::LINK_SINGLE_ID, json!([&sec2.id]));
    update.set(PrimaryFields::LINK_MULTIPLE_ID, json!([&sec1.id]));
    let updated = at
        .primary
        .update_one(&record_id, &update, true)
        .await
        .unwrap();
    assert_eq!(
        updated.fields.get(PrimaryFields::LINK_SINGLE_ID).unwrap(),
        &json!([&sec2.id])
    );
    assert_eq!(
        updated.fields.get(PrimaryFields::LINK_MULTIPLE_ID).unwrap(),
        &json!([&sec1.id])
    );

    // Cleanup
    at.primary.delete_one(&record_id).await.unwrap();
    at.secondary.delete_one(&sec1.id).await.unwrap();
    at.secondary.delete_one(&sec2.id).await.unwrap();
}

// =============================================================================
// Attachments
// =============================================================================

#[tokio::test]
async fn attachment_crud() {
    let at = setup();

    let mut fields = Fields::new();
    fields.set(PrimaryFields::PRIMARY_KEY_ID, "Rust Attachment Test");
    fields.set(
        PrimaryFields::ATTACHMENT_ID,
        json!([{"url": "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"}]),
    );

    let created = at.primary.create_one(&fields, true).await.unwrap();
    let record_id = created.id.clone();
    let attachments = created
        .fields
        .get(PrimaryFields::ATTACHMENT_ID)
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(attachments.len(), 1);
    assert!(attachments[0]["url"]
        .as_str()
        .unwrap()
        .starts_with("https://"));

    // Read (with retry — Airtable processes attachments asynchronously)
    let mut read = at.primary.get_one(&record_id, true).await.unwrap();
    for _ in 0..10 {
        if read.fields.get(PrimaryFields::ATTACHMENT_ID).is_some() {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        read = at.primary.get_one(&record_id, true).await.unwrap();
    }
    let attachments = read
        .fields
        .get(PrimaryFields::ATTACHMENT_ID)
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(attachments.len(), 1);
    assert!(attachments[0]["url"]
        .as_str()
        .unwrap()
        .starts_with("https://"));

    // Cleanup
    at.primary.delete_one(&record_id).await.unwrap();
}

// =============================================================================
// User / Collaborator fields
// =============================================================================

#[tokio::test]
async fn user_fields_crud() {
    let at = setup();

    let mut fields = Fields::new();
    fields.set(PrimaryFields::PRIMARY_KEY_ID, "Rust User Test");
    fields.set(
        PrimaryFields::USER_ID,
        json!({"id": "usrnZ4k98m0Ipji4e", "email": "9vymqckyxq@privaterelay.appleid.com", "name": "Daniel Baughman"}),
    );
    fields.set(
        PrimaryFields::USER_ALLOW_MULTIPLE_ID,
        json!([{"id": "usrnZ4k98m0Ipji4e", "email": "9vymqckyxq@privaterelay.appleid.com", "name": "Daniel Baughman"}]),
    );

    let created = at.primary.create_one(&fields, true).await.unwrap();
    let record_id = created.id.clone();

    let user = &created.fields.get(PrimaryFields::USER_ID).unwrap();
    assert_eq!(user["id"], "usrnZ4k98m0Ipji4e");

    let users = created
        .fields
        .get(PrimaryFields::USER_ALLOW_MULTIPLE_ID)
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0]["id"], "usrnZ4k98m0Ipji4e");

    // Read
    let read = at.primary.get_one(&record_id, true).await.unwrap();
    assert_eq!(
        read.fields.get(PrimaryFields::USER_ID).unwrap()["id"],
        "usrnZ4k98m0Ipji4e"
    );

    // Cleanup
    at.primary.delete_one(&record_id).await.unwrap();
}

// =============================================================================
// Computed fields
// =============================================================================

#[tokio::test]
async fn computed_fields() {
    let at = setup();

    let mut fields = Fields::new();
    fields.set(PrimaryFields::PRIMARY_KEY_ID, "Rust Computed Test");
    fields.set(PrimaryFields::NUMBER_INT_ID, 10);
    fields.set(PrimaryFields::NUMBER_FLOAT_ID, 5);

    let created = at.primary.create_one(&fields, true).await.unwrap();
    let record_id = created.id.clone();
    let f = &created.fields;

    // Auto Number should be present
    assert!(f
        .get(PrimaryFields::AUTO_NUMBER_ID)
        .unwrap()
        .as_i64()
        .is_some());
    // Created Time should be present
    assert!(f
        .get(PrimaryFields::CREATED_AT_TIME_ID)
        .unwrap()
        .as_str()
        .is_some());
    // Formula (ID) should equal record ID
    assert_eq!(
        f.get(PrimaryFields::FORMULA_ID_ID).unwrap(),
        record_id.as_str()
    );
    // Formula (Simple) = Number (int) + Number (float) = 15
    assert_eq!(f.get(PrimaryFields::FORMULA_SIMPLE_ID).unwrap(), 15);

    // Read and verify
    let read = at.primary.get_one(&record_id, true).await.unwrap();
    assert_eq!(
        read.fields.get(PrimaryFields::FORMULA_ID_ID).unwrap(),
        record_id.as_str()
    );
    assert_eq!(
        read.fields.get(PrimaryFields::FORMULA_SIMPLE_ID).unwrap(),
        15
    );

    // Cleanup
    at.primary.delete_one(&record_id).await.unwrap();
}

// =============================================================================
// Batch operations (large)
// =============================================================================

#[tokio::test]
async fn batch_create_update_delete() {
    let at = setup();
    let count = 25; // Enough to test multi-batch (>10)

    // Create
    let records: Vec<Fields> = (1..=count)
        .map(|i| {
            let mut f = Fields::new();
            f.set(PrimaryFields::PRIMARY_KEY_ID, format!("Rust Batch {i}"));
            f.set(PrimaryFields::NUMBER_INT_ID, i as i64);
            f
        })
        .collect();

    let created = at.primary.create_many(&records, true).await.unwrap();
    assert_eq!(created.len(), count);
    for (i, record) in created.iter().enumerate() {
        assert_eq!(
            record.fields.get(PrimaryFields::PRIMARY_KEY_ID).unwrap(),
            format!("Rust Batch {}", i + 1).as_str()
        );
    }

    // Update all
    let ids: Vec<RecordId> = created.iter().map(|r| r.id.clone()).collect();
    let update_fields: Vec<Fields> = (1..=count)
        .map(|i| {
            let mut f = Fields::new();
            f.set(
                PrimaryFields::PRIMARY_KEY_ID,
                format!("Rust Updated Batch {i}"),
            );
            f
        })
        .collect();
    let updates: Vec<(&RecordId, &Fields)> = ids.iter().zip(update_fields.iter()).collect();

    let updated = at.primary.update_many(&updates, true).await.unwrap();
    assert_eq!(updated.len(), count);
    for (i, record) in updated.iter().enumerate() {
        assert_eq!(
            record.fields.get(PrimaryFields::PRIMARY_KEY_ID).unwrap(),
            format!("Rust Updated Batch {}", i + 1).as_str()
        );
    }

    // Delete all
    at.primary.delete_many(&ids).await.unwrap();

    // Verify first one is deleted
    let result = at.primary.get_one(&ids[0], true).await;
    assert!(result.is_err());
}

// =============================================================================
// Get many (list)
// =============================================================================

#[tokio::test]
async fn get_many_records() {
    let at = setup();

    let page = at.secondary.get_many(true, None).await.unwrap();
    assert!(!page.records.is_empty());

    for record in &page.records {
        assert!(!record.id.is_empty());
    }
}

// =============================================================================
// Invalid record IDs
// =============================================================================

#[tokio::test]
async fn invalid_record_id() {
    let at = setup();

    let result = at
        .primary
        .get_one(&"rec_INVALID_ID".to_string(), true)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn empty_record_id() {
    let at = setup();

    let result = at.primary.get_one(&String::new(), true).await;
    assert!(result.is_err());
}
