use myairtable_tests::*;
use serde_json::json;

/// Helper to wrap fields in an Airtable record envelope.
fn record_json(fields: serde_json::Value) -> serde_json::Value {
    json!({
        "id": "recTEST123456789",
        "fields": fields,
        "createdTime": "2025-01-15T10:00:00.000Z"
    })
}

// =============================================================================
// Deserialization (field IDs as keys)
// =============================================================================

#[test]
fn deserialize_simple_fields_by_id() {
    let json = record_json(json!({
        "fldol5Q4wmQJQvPRy": "Hello World",
        "fld0BL2lFo9fqcKv3": "Single line",
        "fldHCJoYBiFVsNvP4": "test@example.com",
        "fldjQIaAZVegb1FUa": true,
        "fldOfPKGmnRPv94QH": 42,
        "fldmU0X2l4RWd21dd": 3.14,
        "fldLTyf6ljS0rhur8": 3600,
        "fldC6LfNVvVIxKyQH": "2025-01-15",
    }));

    let record: Record = serde_json::from_value(json).unwrap();
    assert_eq!(record.id, "recTEST123456789");

    let f = &record.fields;
    assert_eq!(f.get(PrimaryFields::PRIMARY_KEY_ID).unwrap(), "Hello World");
    assert_eq!(
        f.get(PrimaryFields::SINGLE_LINE_TEXT_ID).unwrap(),
        "Single line"
    );
    assert_eq!(f.get(PrimaryFields::EMAIL_ID).unwrap(), "test@example.com");
    assert_eq!(f.get(PrimaryFields::CHECKBOX_ID).unwrap(), true);
    assert_eq!(f.get(PrimaryFields::NUMBER_INT_ID).unwrap(), 42);
    assert_eq!(f.get(PrimaryFields::NUMBER_FLOAT_ID).unwrap(), 3.14);
    assert_eq!(f.get(PrimaryFields::DURATION_ID).unwrap(), 3600);
    assert_eq!(f.get(PrimaryFields::DATE_ID).unwrap(), "2025-01-15");
}

// =============================================================================
// Deserialization (field names as keys)
// =============================================================================

#[test]
fn deserialize_simple_fields_by_name() {
    let json = record_json(json!({
        "Primary Key": "Hello World",
        "Single Line Text": "Single line",
        "Checkbox": true,
        "Number (int)": 42,
    }));

    let record: Record = serde_json::from_value(json).unwrap();
    let f = &record.fields;

    assert_eq!(f.get(PrimaryFields::PRIMARY_KEY).unwrap(), "Hello World");
    assert_eq!(
        f.get(PrimaryFields::SINGLE_LINE_TEXT).unwrap(),
        "Single line"
    );
    assert_eq!(f.get(PrimaryFields::CHECKBOX).unwrap(), true);
    assert_eq!(f.get(PrimaryFields::NUMBER_INT).unwrap(), 42);
}

// =============================================================================
// Missing fields
// =============================================================================

#[test]
fn missing_fields_are_none() {
    let json = record_json(json!({}));
    let record: Record = serde_json::from_value(json).unwrap();
    let f = &record.fields;

    assert!(f.get(PrimaryFields::PRIMARY_KEY).is_none());
    assert!(f.get(PrimaryFields::PRIMARY_KEY_ID).is_none());
    assert!(f.get(PrimaryFields::CHECKBOX).is_none());
    assert!(f.is_empty());
}

// =============================================================================
// Serialization
// =============================================================================

#[test]
fn serialize_fields() {
    let mut fields = Fields::new();
    fields.set(PrimaryFields::PRIMARY_KEY_ID, "Test");
    fields.set(PrimaryFields::CHECKBOX_ID, true);
    fields.set(PrimaryFields::NUMBER_INT_ID, 42);

    let json = serde_json::to_value(&fields).unwrap();
    let obj = json.as_object().unwrap();

    assert_eq!(obj.get("fldol5Q4wmQJQvPRy").unwrap(), "Test");
    assert_eq!(obj.get("fldjQIaAZVegb1FUa").unwrap(), true);
    assert_eq!(obj.get("fldOfPKGmnRPv94QH").unwrap(), 42);
    assert_eq!(obj.len(), 3);
}

#[test]
fn serialize_fields_by_name() {
    let mut fields = Fields::new();
    fields.set(PrimaryFields::PRIMARY_KEY, "Test");
    fields.set(PrimaryFields::CHECKBOX, true);

    let json = serde_json::to_value(&fields).unwrap();
    let obj = json.as_object().unwrap();

    assert_eq!(obj.get("Primary Key").unwrap(), "Test");
    assert_eq!(obj.get("Checkbox").unwrap(), true);
}

#[test]
fn round_trip_preserves_data() {
    let mut original = Fields::new();
    original.set(PrimaryFields::PRIMARY_KEY_ID, "Round Trip");
    original.set(PrimaryFields::NUMBER_INT_ID, 99);
    original.set(PrimaryFields::CHECKBOX_ID, false);

    let serialized = serde_json::to_string(&original).unwrap();
    let deserialized: Fields = serde_json::from_str(&serialized).unwrap();

    assert_eq!(
        deserialized.get(PrimaryFields::PRIMARY_KEY_ID).unwrap(),
        "Round Trip"
    );
    assert_eq!(deserialized.get(PrimaryFields::NUMBER_INT_ID).unwrap(), 99);
    assert_eq!(deserialized.get(PrimaryFields::CHECKBOX_ID).unwrap(), false);
}

// =============================================================================
// Paginated response
// =============================================================================

#[test]
fn deserialize_paginated_response() {
    let json = json!({
        "records": [
            record_json(json!({"fldol5Q4wmQJQvPRy": "Record 1"})),
            record_json(json!({"fldol5Q4wmQJQvPRy": "Record 2"})),
        ],
        "offset": "itr12345/rec67890"
    });

    let page: PaginatedResponse = serde_json::from_value(json).unwrap();
    assert_eq!(page.records.len(), 2);
    assert_eq!(
        page.records[0]
            .fields
            .get(PrimaryFields::PRIMARY_KEY_ID)
            .unwrap(),
        "Record 1"
    );
    assert_eq!(
        page.records[1]
            .fields
            .get(PrimaryFields::PRIMARY_KEY_ID)
            .unwrap(),
        "Record 2"
    );
    assert_eq!(page.offset.as_deref(), Some("itr12345/rec67890"));
}

#[test]
fn deserialize_paginated_response_no_offset() {
    let json = json!({
        "records": [
            record_json(json!({"fldol5Q4wmQJQvPRy": "Only Record"})),
        ]
    });

    let page: PaginatedResponse = serde_json::from_value(json).unwrap();
    assert_eq!(page.records.len(), 1);
    assert!(page.offset.is_none());
}

// =============================================================================
// Create field constants
// =============================================================================

#[test]
fn create_fields_has_writable_constants() {
    // Writable fields should exist
    assert_eq!(CreatePrimaryFields::PRIMARY_KEY, "Primary Key");
    assert_eq!(CreatePrimaryFields::PRIMARY_KEY_ID, "fldol5Q4wmQJQvPRy");
    assert_eq!(CreatePrimaryFields::CHECKBOX, "Checkbox");
    assert_eq!(CreatePrimaryFields::NUMBER_INT, "Number (int)");
}

#[test]
fn field_constants_match_between_all_and_create() {
    // Same field should have the same constant value in both
    assert_eq!(PrimaryFields::PRIMARY_KEY, CreatePrimaryFields::PRIMARY_KEY);
    assert_eq!(
        PrimaryFields::PRIMARY_KEY_ID,
        CreatePrimaryFields::PRIMARY_KEY_ID
    );
    assert_eq!(PrimaryFields::CHECKBOX, CreatePrimaryFields::CHECKBOX);
}
