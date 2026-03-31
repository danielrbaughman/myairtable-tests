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
// Deserialization
// =============================================================================

#[test]
fn deserialize_simple_fields() {
    let json = record_json(json!({
        "fldol5Q4wmQJQvPRy": "Hello World",          // primary_key
        "fld0BL2lFo9fqcKv3": "Single line",           // single_line_text
        "fldHCJoYBiFVsNvP4": "test@example.com",      // email
        "fldLYloz2oP4ymf3B": "https://example.com",   // url
        "fld38tnNpHmoks8C8": "555-1234",              // phone_number
        "fldjQIaAZVegb1FUa": true,                    // checkbox
        "fldOfPKGmnRPv94QH": 42,                      // number_int
        "fldmU0X2l4RWd21dd": 3.14,                    // number_float
        "fldBfo74z9hD78hP8": 10,                      // currency_int
        "fldyh8pzDXiy5abEr": 9.99,                    // currency_float
        "fldbAAyWboGulpb4s": 0.5,                     // percent_int
        "fldiGui9ll69N7WOj": 0.333,                   // percent_float
        "fldLTyf6ljS0rhur8": 3600,                    // duration
        "fldC6LfNVvVIxKyQH": "2025-01-15",            // date
        "fldizYmjpXABGDLTG": "2025-01-15T10:00:00.000Z", // date_with_time
    }));

    let record: Record<Primary> = serde_json::from_value(json).unwrap();
    let f = &record.fields;

    assert_eq!(record.id, "recTEST123456789");
    assert_eq!(f.primary_key.as_deref(), Some("Hello World"));
    assert_eq!(f.single_line_text.as_deref(), Some("Single line"));
    assert_eq!(f.email.as_deref(), Some("test@example.com"));
    assert_eq!(f.url.as_deref(), Some("https://example.com"));
    assert_eq!(f.phone_number.as_deref(), Some("555-1234"));
    assert_eq!(f.checkbox, Some(true));
    assert_eq!(f.number_int, Some(42));
    assert_eq!(f.number_float, Some(3.14));
    assert_eq!(f.currency_int, Some(10.0));
    assert_eq!(f.currency_float, Some(9.99));
    assert_eq!(f.percent_int, Some(0.5));
    assert_eq!(f.percent_float, Some(0.333));
    assert_eq!(f.duration, Some(3600));
    assert_eq!(f.date.as_deref(), Some("2025-01-15"));
    assert_eq!(
        f.date_with_time.as_deref(),
        Some("2025-01-15T10:00:00.000Z")
    );
}

#[test]
fn deserialize_select_fields() {
    let json = record_json(json!({
        "fldn0GFFtMFpCXUNU": "Choice 1",                  // single_select
        "fld6GTabFmu1xKPvZ": ["Option 1", "Option 2"],    // multiple_select
    }));

    let record: Record<Primary> = serde_json::from_value(json).unwrap();
    let f = &record.fields;

    assert_eq!(f.single_select, Some(PrimarySingleSelectOption::Choice1));
    assert_eq!(
        f.multiple_select,
        Some(vec![
            PrimaryMultipleSelectOption::Option1,
            PrimaryMultipleSelectOption::Option2,
        ])
    );
}

#[test]
fn deserialize_unknown_select_variant() {
    let json = record_json(json!({
        "fldn0GFFtMFpCXUNU": "Some New Choice",
    }));

    let record: Record<Primary> = serde_json::from_value(json).unwrap();
    assert_eq!(
        record.fields.single_select,
        Some(PrimarySingleSelectOption::Unknown)
    );
}

#[test]
fn deserialize_linked_records() {
    let json = record_json(json!({
        "fld7F5onkDo6mkmbN": "recSINGLE12345678",                       // link_single
        "fldFyFheQWczd8oux": ["recMULTI123456780", "recMULTI123456781"], // link_multiple
    }));

    let record: Record<Primary> = serde_json::from_value(json).unwrap();
    let f = &record.fields;

    assert_eq!(f.link_single.as_deref(), Some("recSINGLE12345678"));
    assert_eq!(
        f.link_multiple,
        Some(vec![
            "recMULTI123456780".to_string(),
            "recMULTI123456781".to_string(),
        ])
    );
}

#[test]
fn deserialize_collaborator() {
    let json = record_json(json!({
        "fldU6SbLp8CSkLcA4": {                          // user
            "id": "usrABC123",
            "email": "user@example.com",
            "name": "Test User"
        },
    }));

    let record: Record<Primary> = serde_json::from_value(json).unwrap();
    let user = record.fields.user.unwrap();
    assert_eq!(user.id, "usrABC123");
    assert_eq!(user.email, "user@example.com");
    assert_eq!(user.name, Some("Test User".to_string()));
}

#[test]
fn deserialize_attachment() {
    let json = record_json(json!({
        "fldhF2AEuSC1haCZd": [{                         // attachment
            "id": "attABC123",
            "url": "https://example.com/file.png",
            "filename": "file.png",
            "type": "image/png",
            "size": 12345,
            "width": 800,
            "height": 600
        }],
    }));

    let record: Record<Primary> = serde_json::from_value(json).unwrap();
    let attachments = record.fields.attachment.unwrap();
    assert_eq!(attachments.len(), 1);
    assert_eq!(attachments[0].id, "attABC123");
    assert_eq!(attachments[0].filename, "file.png");
    assert_eq!(attachments[0].mime_type, Some("image/png".to_string()));
    assert_eq!(attachments[0].size, Some(12345));
    assert_eq!(attachments[0].width, Some(800));
}

// =============================================================================
// Missing fields default to None
// =============================================================================

#[test]
fn missing_fields_are_none() {
    let json = record_json(json!({}));
    let record: Record<Primary> = serde_json::from_value(json).unwrap();
    let f = &record.fields;

    assert!(f.primary_key.is_none());
    assert!(f.single_line_text.is_none());
    assert!(f.checkbox.is_none());
    assert!(f.number_int.is_none());
    assert!(f.single_select.is_none());
    assert!(f.multiple_select.is_none());
    assert!(f.link_single.is_none());
    assert!(f.attachment.is_none());
    assert!(f.user.is_none());
}

// =============================================================================
// Serialization (round-trip)
// =============================================================================

#[test]
fn serialize_skips_none_fields() {
    let primary = Primary {
        primary_key: Some("Test".to_string()),
        checkbox: Some(true),
        ..Default::default()
    };

    let json = serde_json::to_value(&primary).unwrap();
    let obj = json.as_object().unwrap();

    // Only the two set fields should be present (using field IDs as keys)
    assert_eq!(obj.get("fldol5Q4wmQJQvPRy").unwrap(), "Test"); // primary_key
    assert_eq!(obj.get("fldjQIaAZVegb1FUa").unwrap(), true); // checkbox

    // None fields should not be serialized
    assert!(!obj.contains_key("fld0BL2lFo9fqcKv3")); // single_line_text
    assert!(!obj.contains_key("fldOfPKGmnRPv94QH")); // number_int
}

#[test]
fn serialize_select_uses_display_name() {
    let primary = Primary {
        single_select: Some(PrimarySingleSelectOption::Choice2),
        multiple_select: Some(vec![
            PrimaryMultipleSelectOption::Option1,
            PrimaryMultipleSelectOption::Option3,
        ]),
        ..Default::default()
    };

    let json = serde_json::to_value(&primary).unwrap();
    assert_eq!(json["fldn0GFFtMFpCXUNU"], "Choice 2"); // single_select
    assert_eq!(json["fld6GTabFmu1xKPvZ"], json!(["Option 1", "Option 3"])); // multiple_select
}

#[test]
fn round_trip_preserves_data() {
    let original = Primary {
        primary_key: Some("Round Trip".to_string()),
        number_int: Some(99),
        checkbox: Some(false),
        single_select: Some(PrimarySingleSelectOption::Choice3),
        ..Default::default()
    };

    let serialized = serde_json::to_string(&original).unwrap();
    let deserialized: Primary = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.primary_key, original.primary_key);
    assert_eq!(deserialized.number_int, original.number_int);
    assert_eq!(deserialized.checkbox, original.checkbox);
    assert_eq!(deserialized.single_select, original.single_select);
    assert!(deserialized.email.is_none());
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

    let page: PaginatedResponse<Primary> = serde_json::from_value(json).unwrap();
    assert_eq!(page.records.len(), 2);
    assert_eq!(
        page.records[0].fields.primary_key.as_deref(),
        Some("Record 1")
    );
    assert_eq!(
        page.records[1].fields.primary_key.as_deref(),
        Some("Record 2")
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

    let page: PaginatedResponse<Primary> = serde_json::from_value(json).unwrap();
    assert_eq!(page.records.len(), 1);
    assert!(page.offset.is_none());
}

// =============================================================================
// Create/Update types
// =============================================================================

#[test]
fn create_type_excludes_computed_fields() {
    // CreatePrimary should have writable fields only
    let create = CreatePrimary {
        primary_key: Some("Test".to_string()),
        single_line_text: Some("Hello".to_string()),
        checkbox: Some(true),
        number_int: Some(42),
        ..Default::default()
    };

    let json = serde_json::to_value(&create).unwrap();
    let obj = json.as_object().unwrap();

    // Writable fields are present
    assert_eq!(obj.get("fldol5Q4wmQJQvPRy").unwrap(), "Test");
    assert_eq!(obj.get("fld0BL2lFo9fqcKv3").unwrap(), "Hello");
    assert_eq!(obj.get("fldjQIaAZVegb1FUa").unwrap(), true);
    assert_eq!(obj.get("fldOfPKGmnRPv94QH").unwrap(), 42);

    // Computed field IDs should not exist on CreatePrimary
    assert!(!obj.contains_key("fldizvTkxgIn0mC3L")); // auto_number
    assert!(!obj.contains_key("fld2vnFc0Bl5IOFUQ")); // formula_complex
    assert!(!obj.contains_key("fld2YgW382Kt9xltA")); // created_time
    assert!(!obj.contains_key("fldMinKh4pa3YX86g")); // last_modified_time
}

#[test]
fn update_type_is_alias_for_create() {
    // UpdatePrimary should be usable identically to CreatePrimary
    let update: UpdatePrimary = UpdatePrimary {
        primary_key: Some("Updated".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_value(&update).unwrap();
    assert_eq!(json["fldol5Q4wmQJQvPRy"], "Updated");
}

#[test]
fn create_type_serializes_select_enums() {
    let create = CreatePrimary {
        single_select: Some(PrimarySingleSelectOption::Choice1),
        multiple_select: Some(vec![PrimaryMultipleSelectOption::Option2]),
        ..Default::default()
    };

    let json = serde_json::to_value(&create).unwrap();
    assert_eq!(json["fldn0GFFtMFpCXUNU"], "Choice 1");
    assert_eq!(json["fld6GTabFmu1xKPvZ"], json!(["Option 2"]));
}
