use myairtable_tests::*;
use serde_json::json;

fn setup() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::new(&api_key, &base_id)
}

// =============================================================================
// Serialization (unit tests, no API)
// =============================================================================

#[test]
fn model_deserializes_from_fields_json() {
    let json = json!({
        "fldol5Q4wmQJQvPRy": "Hello",
        "fldjQIaAZVegb1FUa": true,
        "fldOfPKGmnRPv94QH": 42,
        "fldmU0X2l4RWd21dd": 3.14,
    });

    let model: PrimaryModel = serde_json::from_value(json).unwrap();
    assert_eq!(model.primary_key, Some("Hello".to_string()));
    assert_eq!(model.checkbox, Some(true));
    assert_eq!(model.number_int, Some(42.0));
    assert_eq!(model.number_float, Some(3.14));
    assert!(model.id.is_none());
    assert!(model.created_time.is_none());
}

#[test]
fn model_missing_fields_are_none() {
    let json = json!({});
    let model: PrimaryModel = serde_json::from_value(json).unwrap();
    assert!(model.primary_key.is_none());
    assert!(model.checkbox.is_none());
    assert!(model.number_int.is_none());
}

#[test]
fn model_serializes_skipping_none() {
    let model = PrimaryModel {
        primary_key: Some("Test".to_string()),
        checkbox: Some(true),
        ..Default::default()
    };

    let json = serde_json::to_value(&model).unwrap();
    let obj = json.as_object().unwrap();
    assert_eq!(obj.get("fldol5Q4wmQJQvPRy").unwrap(), "Test");
    assert_eq!(obj.get("fldjQIaAZVegb1FUa").unwrap(), true);
    assert!(!obj.contains_key("fldOfPKGmnRPv94QH"));
}

#[test]
fn model_round_trip() {
    let original = PrimaryModel {
        primary_key: Some("Round Trip".to_string()),
        number_int: Some(99.0),
        single_select: Some(PrimarySingleSelectOption::Choice1),
        ..Default::default()
    };

    let serialized = serde_json::to_string(&original).unwrap();
    let deserialized: PrimaryModel = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.primary_key, original.primary_key);
    assert_eq!(deserialized.number_int, original.number_int);
    assert_eq!(deserialized.single_select, original.single_select);
    assert!(deserialized.email.is_none());
}

#[test]
fn model_select_enums() {
    let model = PrimaryModel {
        single_select: Some(PrimarySingleSelectOption::Choice2),
        multiple_select: Some(vec![
            PrimaryMultipleSelectOption::Option1,
            PrimaryMultipleSelectOption::Option3,
        ]),
        ..Default::default()
    };

    let json = serde_json::to_value(&model).unwrap();
    assert_eq!(json[PrimaryFields::SINGLE_SELECT_ID], "Choice 2");
    assert_eq!(
        json[PrimaryFields::MULTIPLE_SELECT_ID],
        json!(["Option 1", "Option 3"])
    );
}

#[test]
fn model_id_not_serialized() {
    let mut model = PrimaryModel {
        primary_key: Some("Test".to_string()),
        ..Default::default()
    };
    model.id = Some("recTEST123".to_string());
    model.created_time = Some("2025-01-01T00:00:00.000Z".to_string());

    let json = serde_json::to_value(&model).unwrap();
    let obj = json.as_object().unwrap();
    assert!(!obj.contains_key("id"));
    assert!(!obj.contains_key("created_time"));
    assert!(obj.contains_key("fldol5Q4wmQJQvPRy"));
}

// =============================================================================
// Primary key only — basic CRUD
// =============================================================================

#[tokio::test]
async fn primary_key_only_crud() {
    let at = setup();

    // Create
    let fields = PrimaryModel {
        primary_key: Some("ORM Primary Key Only".to_string()),
        ..Default::default()
    };
    let created = at.primary.create_one(&fields, false).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();
    assert_eq!(created.primary_key.as_deref(), Some("ORM Primary Key Only"));

    // Read
    let fetched = at.primary.get_one(&id).await.unwrap();
    assert_eq!(fetched.id.as_deref(), Some(id.as_str()));
    assert_eq!(fetched.primary_key.as_deref(), Some("ORM Primary Key Only"));

    // Update
    let update = PrimaryModel {
        primary_key: Some("ORM Updated Primary Key".to_string()),
        ..Default::default()
    };
    let updated = at.primary.update_one(&id, &update, false).await.unwrap();
    assert_eq!(
        updated.primary_key.as_deref(),
        Some("ORM Updated Primary Key")
    );

    // Delete
    at.primary.delete_one(&id).await.unwrap();
    assert!(at.primary.get_one(&id).await.is_err());
}

// =============================================================================
// All simple properties
// =============================================================================

#[tokio::test]
async fn all_simple_properties_crud() {
    let at = setup();

    let fields = PrimaryModel {
        primary_key: Some("ORM All Props".to_string()),
        single_line_text: Some("Hello World".to_string()),
        long_text: Some("Long text content".to_string()),
        long_text_with_rich_text: Some("Rich text content".to_string()),
        email: Some("test@example.com".to_string()),
        url: Some("https://example.com".to_string()),
        phone_number: Some("555-1234".to_string()),
        checkbox: Some(true),
        number_int: Some(42.0),
        number_float: Some(3.14),
        currency_int: Some(10.0),
        currency_float: Some(9.99),
        percent_int: Some(0.5),
        percent_float: Some(0.333),
        duration: Some(3600),
        rating: Some(3),
        date: Some("2025-01-15".to_string()),
        date_with_time: Some("2025-01-15T10:00:00.000Z".to_string()),
        single_select: Some(PrimarySingleSelectOption::Choice1),
        multiple_select: Some(vec![
            PrimaryMultipleSelectOption::Option1,
            PrimaryMultipleSelectOption::Option2,
        ]),
        ..Default::default()
    };

    let created = at.primary.create_one(&fields, false).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    assert_eq!(created.primary_key.as_deref(), Some("ORM All Props"));
    assert_eq!(created.single_line_text.as_deref(), Some("Hello World"));
    assert_eq!(created.long_text.as_deref(), Some("Long text content"));
    assert_eq!(
        created.long_text_with_rich_text.as_deref(),
        Some("Rich text content")
    );
    assert_eq!(created.email.as_deref(), Some("test@example.com"));
    assert_eq!(created.url.as_deref(), Some("https://example.com"));
    assert_eq!(created.phone_number.as_deref(), Some("555-1234"));
    assert_eq!(created.checkbox, Some(true));
    assert_eq!(created.number_int, Some(42.0));
    assert_eq!(created.number_float, Some(3.14));
    assert_eq!(created.currency_int, Some(10.0));
    assert_eq!(created.currency_float, Some(9.99));
    assert_eq!(created.percent_int, Some(0.5));
    assert_eq!(created.percent_float, Some(0.333));
    assert_eq!(created.duration, Some(3600));
    assert_eq!(created.date.as_deref(), Some("2025-01-15"));
    assert_eq!(
        created.date_with_time.as_deref(),
        Some("2025-01-15T10:00:00.000Z")
    );
    assert_eq!(
        created.single_select,
        Some(PrimarySingleSelectOption::Choice1)
    );
    assert_eq!(
        created.multiple_select,
        Some(vec![
            PrimaryMultipleSelectOption::Option1,
            PrimaryMultipleSelectOption::Option2,
        ])
    );

    // Read
    let read = at.primary.get_one(&id).await.unwrap();
    assert_eq!(read.primary_key.as_deref(), Some("ORM All Props"));
    assert_eq!(read.single_line_text.as_deref(), Some("Hello World"));
    assert_eq!(read.checkbox, Some(true));
    assert_eq!(read.number_int, Some(42.0));
    assert_eq!(read.single_select, Some(PrimarySingleSelectOption::Choice1));

    // Update all fields
    let update = PrimaryModel {
        primary_key: Some("ORM Updated All Props".to_string()),
        single_line_text: Some("Updated Hello".to_string()),
        long_text: Some("Updated long text".to_string()),
        long_text_with_rich_text: Some("Updated rich text".to_string()),
        email: Some("updated@example.com".to_string()),
        url: Some("https://updated.com".to_string()),
        phone_number: Some("555-5678".to_string()),
        checkbox: Some(false),
        number_int: Some(100.0),
        number_float: Some(2.72),
        currency_int: Some(20.0),
        currency_float: Some(19.99),
        percent_int: Some(0.75),
        percent_float: Some(0.667),
        duration: Some(7200),
        rating: Some(5),
        date: Some("2025-06-15".to_string()),
        date_with_time: Some("2025-06-15T14:00:00.000Z".to_string()),
        single_select: Some(PrimarySingleSelectOption::Choice2),
        multiple_select: Some(vec![
            PrimaryMultipleSelectOption::Option2,
            PrimaryMultipleSelectOption::Option3,
        ]),
        ..Default::default()
    };

    let updated = at.primary.update_one(&id, &update, false).await.unwrap();
    assert_eq!(
        updated.primary_key.as_deref(),
        Some("ORM Updated All Props")
    );
    assert_eq!(updated.single_line_text.as_deref(), Some("Updated Hello"));
    assert_eq!(updated.number_int, Some(100.0));
    assert_eq!(
        updated.single_select,
        Some(PrimarySingleSelectOption::Choice2)
    );
    assert_eq!(
        updated.multiple_select,
        Some(vec![
            PrimaryMultipleSelectOption::Option2,
            PrimaryMultipleSelectOption::Option3,
        ])
    );

    // Delete
    at.primary.delete_one(&id).await.unwrap();
    assert!(at.primary.get_one(&id).await.is_err());
}

// =============================================================================
// Linked records
// =============================================================================

#[tokio::test]
async fn linked_records_crud() {
    let at = setup();

    // Setup secondary records
    let sec1 = at
        .secondary
        .create_one(
            &SecondaryModel {
                name: Some("ORM Link Target 1".to_string()),
                value: Some("val1".to_string()),
                ..Default::default()
            },
            false,
        )
        .await
        .unwrap();
    let sec2 = at
        .secondary
        .create_one(
            &SecondaryModel {
                name: Some("ORM Link Target 2".to_string()),
                value: Some("val2".to_string()),
                ..Default::default()
            },
            false,
        )
        .await
        .unwrap();

    let sec1_id = sec1.id.as_deref().unwrap().to_string();
    let sec2_id = sec2.id.as_deref().unwrap().to_string();

    // Create with links
    let fields = PrimaryModel {
        primary_key: Some("ORM Link Test".to_string()),
        link_single: Some(vec![sec1_id.clone()]),
        link_multiple: Some(vec![sec1_id.clone(), sec2_id.clone()]),
        ..Default::default()
    };
    let created = at.primary.create_one(&fields, false).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();
    assert_eq!(created.link_single, Some(vec![sec1_id.clone()]));
    assert_eq!(
        created.link_multiple,
        Some(vec![sec1_id.clone(), sec2_id.clone()])
    );

    // Read
    let read = at.primary.get_one(&id).await.unwrap();
    assert_eq!(read.link_single, Some(vec![sec1_id.clone()]));
    assert_eq!(
        read.link_multiple,
        Some(vec![sec1_id.clone(), sec2_id.clone()])
    );

    // Update: swap links
    let update = PrimaryModel {
        link_single: Some(vec![sec2_id.clone()]),
        link_multiple: Some(vec![sec1_id.clone()]),
        ..Default::default()
    };
    let updated = at.primary.update_one(&id, &update, false).await.unwrap();
    assert_eq!(updated.link_single, Some(vec![sec2_id.clone()]));
    assert_eq!(updated.link_multiple, Some(vec![sec1_id.clone()]));

    // Cleanup
    at.primary.delete_one(&id).await.unwrap();
    at.secondary.delete_one(&sec1_id).await.unwrap();
    at.secondary.delete_one(&sec2_id).await.unwrap();
}

// =============================================================================
// Attachments
// =============================================================================

#[tokio::test]
async fn attachment_crud() {
    let at = setup();

    let fields = PrimaryModel {
        primary_key: Some("ORM Attachment Test".to_string()),
        attachment: Some(vec![Attachment {
            url:
                "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"
                    .to_string(),
            ..Default::default()
        }]),
        ..Default::default()
    };
    let created = at.primary.create_one(&fields, false).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();
    let attachments = created.attachment.unwrap();
    assert_eq!(attachments.len(), 1);
    assert!(!attachments[0].url.is_empty());

    // Read with retry (Airtable processes attachments async)
    let mut read = at.primary.get_one(&id).await.unwrap();
    for _ in 0..10 {
        if read.attachment.is_some() {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        read = at.primary.get_one(&id).await.unwrap();
    }
    let attachments = read.attachment.unwrap();
    assert_eq!(attachments.len(), 1);
    assert!(!attachments[0].url.is_empty());

    // Cleanup
    at.primary.delete_one(&id).await.unwrap();
}

// =============================================================================
// User / Collaborator fields
// =============================================================================

#[tokio::test]
async fn user_fields_crud() {
    let at = setup();

    let fields = PrimaryModel {
        primary_key: Some("ORM User Test".to_string()),
        user: Some(serde_json::from_value(json!({"id": "usrnZ4k98m0Ipji4e", "email": "9vymqckyxq@privaterelay.appleid.com", "name": "Daniel Baughman"})).unwrap()),
        user_allow_multiple: Some(vec![
            serde_json::from_value(json!({"id": "usrnZ4k98m0Ipji4e", "email": "9vymqckyxq@privaterelay.appleid.com", "name": "Daniel Baughman"})).unwrap(),
        ]),
        ..Default::default()
    };
    let created = at.primary.create_one(&fields, false).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    assert_eq!(created.user.as_ref().unwrap().id, "usrnZ4k98m0Ipji4e");
    let users = created.user_allow_multiple.as_ref().unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].id, "usrnZ4k98m0Ipji4e");

    // Read
    let read = at.primary.get_one(&id).await.unwrap();
    assert_eq!(read.user.as_ref().unwrap().id, "usrnZ4k98m0Ipji4e");

    // Cleanup
    at.primary.delete_one(&id).await.unwrap();
}

// =============================================================================
// Computed fields
// =============================================================================

#[tokio::test]
async fn computed_fields() {
    let at = setup();

    let fields = PrimaryModel {
        primary_key: Some("ORM Computed Test".to_string()),
        number_int: Some(10.0),
        number_float: Some(5.0),
        ..Default::default()
    };
    let created = at.primary.create_one(&fields, false).await.unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    assert!(created.auto_number.is_some());
    assert!(created.created_time.is_some());
    assert_eq!(
        created
            .formula_id
            .as_ref()
            .and_then(|x| x.value())
            .map(String::as_str),
        Some(id.as_str())
    );
    assert_eq!(
        created
            .formula_simple
            .as_ref()
            .and_then(|x| x.value())
            .copied(),
        Some(15.0)
    );

    // Read
    let read = at.primary.get_one(&id).await.unwrap();
    assert_eq!(
        read.formula_id
            .as_ref()
            .and_then(|x| x.value())
            .map(String::as_str),
        Some(id.as_str())
    );
    assert_eq!(
        read.formula_simple
            .as_ref()
            .and_then(|x| x.value())
            .copied(),
        Some(15.0)
    );

    // Cleanup
    at.primary.delete_one(&id).await.unwrap();
}

// =============================================================================
// Batch operations
// =============================================================================

#[tokio::test]
async fn batch_create_update_delete() {
    let at = setup();
    let count = 25;

    // Create
    let records: Vec<PrimaryModel> = (1..=count)
        .map(|i| PrimaryModel {
            primary_key: Some(format!("ORM Batch {i}")),
            number_int: Some(i as f64),
            ..Default::default()
        })
        .collect();

    let created = at.primary.create_many(&records, false).await.unwrap();
    assert_eq!(created.len(), count as usize);
    for (i, record) in created.iter().enumerate() {
        assert_eq!(
            record.primary_key.as_deref(),
            Some(format!("ORM Batch {}", i + 1).as_str())
        );
    }

    // Update all
    let ids: Vec<RecordId> = created
        .iter()
        .map(|r| r.id.as_deref().unwrap().to_string())
        .collect();
    let update_fields: Vec<PrimaryModel> = (1..=count)
        .map(|i| PrimaryModel {
            primary_key: Some(format!("ORM Updated Batch {i}")),
            ..Default::default()
        })
        .collect();
    let updates: Vec<(&RecordId, &PrimaryModel)> = ids.iter().zip(update_fields.iter()).collect();

    let updated = at.primary.update_many(&updates, false).await.unwrap();
    assert_eq!(updated.len(), count as usize);
    for (i, record) in updated.iter().enumerate() {
        assert_eq!(
            record.primary_key.as_deref(),
            Some(format!("ORM Updated Batch {}", i + 1).as_str())
        );
    }

    // Delete all
    at.primary.delete_many(&ids).await.unwrap();
    assert!(at.primary.get_one(&ids[0]).await.is_err());
}

// =============================================================================
// List records
// =============================================================================

#[tokio::test]
async fn list_records() {
    let at = setup();

    let records = at.secondary.get_many(&AirtableQuery::new()).await.unwrap();
    assert!(!records.is_empty());
    for record in &records {
        assert!(record.id.is_some());
    }
}

// =============================================================================
// Invalid record IDs
// =============================================================================

#[tokio::test]
async fn invalid_record_id() {
    let at = setup();
    assert!(at
        .primary
        .get_one(&"rec_INVALID_ID".to_string())
        .await
        .is_err());
}

#[tokio::test]
async fn empty_record_id() {
    let at = setup();
    assert!(at.primary.get_one(&String::new()).await.is_err());
}

// =============================================================================
// Upsert
// =============================================================================

#[tokio::test]
async fn upsert_as_create() {
    let at = setup();

    let mut model = PrimaryModel {
        primary_key: Some("Upsert Create Test".to_string()),
        ..Default::default()
    };
    model.set_client(
        std::sync::Arc::new(AirtableClient::new(
            &std::env::var("AIRTABLE_API_KEY").unwrap(),
            &std::env::var("AIRTABLE_BASE_ID").unwrap(),
        )),
        "tblmb3iqgpNS1ysV2",
    );

    // Upsert without ID → creates
    at.primary.upsert(&mut model, None, false).await.unwrap();
    assert!(model.id.is_some());
    assert_eq!(model.primary_key.as_deref(), Some("Upsert Create Test"));

    let id = model.id.as_deref().unwrap().to_string();

    // Verify via get
    let read = at.primary.get_one(&id).await.unwrap();
    assert_eq!(read.primary_key.as_deref(), Some("Upsert Create Test"));

    // Cleanup
    at.primary.delete_one(&id).await.unwrap();
}

#[tokio::test]
async fn upsert_as_update() {
    let at = setup();

    // First create a record
    let fields = PrimaryModel {
        primary_key: Some("Upsert Update Test".to_string()),
        ..Default::default()
    };
    let mut model = at.primary.create_one(&fields, false).await.unwrap();
    let id = model.id.as_deref().unwrap().to_string();

    // Modify and upsert → updates
    model.primary_key = Some("Upsert Updated".to_string());
    at.primary.upsert(&mut model, None, false).await.unwrap();

    assert_eq!(model.id.as_deref(), Some(id.as_str()));
    assert_eq!(model.primary_key.as_deref(), Some("Upsert Updated"));

    // Verify persisted
    let read = at.primary.get_one(&id).await.unwrap();
    assert_eq!(read.primary_key.as_deref(), Some("Upsert Updated"));

    // Cleanup
    at.primary.delete_one(&id).await.unwrap();
}
