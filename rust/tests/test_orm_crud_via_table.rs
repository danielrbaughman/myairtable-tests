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
    assert_eq!(model.number_int, Some(42));
    assert_eq!(model.number_float, Some(3.14));
    // id and created_time are not in the fields JSON
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
    let model = CreatePrimaryModel {
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
        number_int: Some(99),
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
    // id and created_time should not appear in serialized output
    let model = PrimaryModel {
        id: Some("recTEST123".to_string()),
        created_time: Some("2025-01-01T00:00:00.000Z".to_string()),
        primary_key: Some("Test".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_value(&model).unwrap();
    let obj = json.as_object().unwrap();
    assert!(!obj.contains_key("id"));
    assert!(!obj.contains_key("created_time"));
    assert!(obj.contains_key("fldol5Q4wmQJQvPRy"));
}

// =============================================================================
// CRUD via OrmTable (integration tests)
// =============================================================================

#[tokio::test]
async fn orm_create_get_update_delete() {
    let at = setup();

    // Create
    let fields = CreatePrimaryModel {
        primary_key: Some("ORM CRUD Test".to_string()),
        single_line_text: Some("Hello from ORM".to_string()),
        number_int: Some(42),
        checkbox: Some(true),
        ..Default::default()
    };

    let created = at.primary_orm.create_one(&fields).await.unwrap();
    let id = created.id.clone().unwrap();
    assert_eq!(created.primary_key.as_deref(), Some("ORM CRUD Test"));
    assert_eq!(created.single_line_text.as_deref(), Some("Hello from ORM"));
    assert_eq!(created.number_int, Some(42));
    assert_eq!(created.checkbox, Some(true));
    assert!(created.auto_number.is_some());
    assert!(created.created_at_time.is_some());

    // Get
    let fetched = at.primary_orm.get_one(&id).await.unwrap();
    assert_eq!(fetched.id.as_deref(), Some(id.as_str()));
    assert_eq!(fetched.primary_key.as_deref(), Some("ORM CRUD Test"));

    // Update
    let update = CreatePrimaryModel {
        single_line_text: Some("Updated via ORM".to_string()),
        number_int: Some(99),
        ..Default::default()
    };
    let updated = at.primary_orm.update_one(&id, &update).await.unwrap();
    assert_eq!(updated.single_line_text.as_deref(), Some("Updated via ORM"));
    assert_eq!(updated.number_int, Some(99));
    assert_eq!(updated.primary_key.as_deref(), Some("ORM CRUD Test"));

    // Delete
    at.primary_orm.delete_one(&id).await.unwrap();
    assert!(at.primary_orm.get_one(&id).await.is_err());
}

#[tokio::test]
async fn orm_batch_create_delete() {
    let at = setup();

    let records: Vec<CreatePrimaryModel> = (1..=3)
        .map(|i| CreatePrimaryModel {
            primary_key: Some(format!("ORM Batch {i}")),
            number_int: Some(i),
            ..Default::default()
        })
        .collect();

    let created = at.primary_orm.create_many(&records).await.unwrap();
    assert_eq!(created.len(), 3);
    assert_eq!(created[0].primary_key.as_deref(), Some("ORM Batch 1"));
    assert_eq!(created[1].number_int, Some(2));

    let ids: Vec<RecordId> = created.iter().map(|r| r.id.clone().unwrap()).collect();
    at.primary_orm.delete_many(&ids).await.unwrap();
    assert!(at.primary_orm.get_one(&ids[0]).await.is_err());
}

#[tokio::test]
async fn orm_list_records() {
    let at = setup();

    let (records, _offset) = at.secondary_orm.get_many(&ListParams::new()).await.unwrap();
    assert!(!records.is_empty());
    for record in &records {
        assert!(record.id.is_some());
    }
}
