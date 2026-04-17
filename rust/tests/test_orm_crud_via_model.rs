use myairtable_tests::*;
use std::sync::Arc;

fn setup() -> Arc<AirtableClient> {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Arc::new(AirtableClient::new(&api_key, &base_id))
}

const PRIMARY_TABLE_ID: &str = "tblmb3iqgpNS1ysV2";
const SECONDARY_TABLE_ID: &str = "tblPPScS3XMuFkDYN";

// =============================================================================
// Primary key only — save/fetch/delete via model
// =============================================================================

#[tokio::test]
async fn primary_key_only_via_model() {
    let client = setup();

    // Create via save()
    let mut model = PrimaryModel {
        primary_key: Some("Model CRUD Test".to_string()),
        ..Default::default()
    };
    model.set_client(client.clone(), PRIMARY_TABLE_ID);
    assert!(model.is_new());

    model.save().await.unwrap();
    assert!(!model.is_new());
    assert!(model.id.is_some());
    assert_eq!(model.primary_key.as_deref(), Some("Model CRUD Test"));

    let id = model.id.as_deref().unwrap().to_string();

    // Read via from_id + fetch
    let mut fetched = PrimaryModel::from_id(client.clone(), PRIMARY_TABLE_ID, &id);
    assert_eq!(fetched.id.as_deref(), Some(id.as_str()));
    assert!(fetched.primary_key.is_none());
    fetched.fetch().await.unwrap();
    assert_eq!(fetched.primary_key.as_deref(), Some("Model CRUD Test"));

    // Update via mutate + save
    fetched.primary_key = Some("Model Updated Key".to_string());
    fetched.save().await.unwrap();
    assert_eq!(fetched.primary_key.as_deref(), Some("Model Updated Key"));

    // Delete via model
    fetched.delete().await.unwrap();

    // Verify deleted
    let mut check = PrimaryModel::from_id(client.clone(), PRIMARY_TABLE_ID, &id);
    assert!(check.fetch().await.is_err());
}

// =============================================================================
// All simple properties via model
// =============================================================================

#[tokio::test]
async fn all_simple_properties_via_model() {
    let client = setup();

    let mut model = PrimaryModel {
        primary_key: Some("Model All Props".to_string()),
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
        rating: Some(serde_json::json!(3)),
        date: Some("2025-01-15".to_string()),
        date_with_time: Some("2025-01-15T10:00:00.000Z".to_string()),
        single_select: Some(PrimarySingleSelectOption::Choice1),
        multiple_select: Some(vec![
            PrimaryMultipleSelectOption::Option1,
            PrimaryMultipleSelectOption::Option2,
        ]),
        ..Default::default()
    };
    model.set_client(client.clone(), PRIMARY_TABLE_ID);
    model.save().await.unwrap();

    let id = model.id.as_deref().unwrap().to_string();

    // Verify create
    assert_eq!(model.primary_key.as_deref(), Some("Model All Props"));
    assert_eq!(model.single_line_text.as_deref(), Some("Hello World"));
    assert_eq!(model.long_text.as_deref(), Some("Long text content"));
    assert_eq!(
        model.long_text_with_rich_text.as_deref(),
        Some("Rich text content")
    );
    assert_eq!(model.email.as_deref(), Some("test@example.com"));
    assert_eq!(model.url.as_deref(), Some("https://example.com"));
    assert_eq!(model.phone_number.as_deref(), Some("555-1234"));
    assert_eq!(model.checkbox, Some(true));
    assert_eq!(model.number_int, Some(42.0));
    assert_eq!(model.number_float, Some(3.14));
    assert_eq!(model.currency_int, Some(10.0));
    assert_eq!(model.currency_float, Some(9.99));
    assert_eq!(model.percent_int, Some(0.5));
    assert_eq!(model.percent_float, Some(0.333));
    assert_eq!(model.duration, Some(3600));
    assert_eq!(model.date.as_deref(), Some("2025-01-15"));
    assert_eq!(
        model.date_with_time.as_deref(),
        Some("2025-01-15T10:00:00.000Z")
    );
    assert_eq!(
        model.single_select,
        Some(PrimarySingleSelectOption::Choice1)
    );
    assert_eq!(
        model.multiple_select,
        Some(vec![
            PrimaryMultipleSelectOption::Option1,
            PrimaryMultipleSelectOption::Option2
        ])
    );

    // Fetch fresh
    let mut read = PrimaryModel::from_id(client.clone(), PRIMARY_TABLE_ID, &id);
    read.fetch().await.unwrap();
    assert_eq!(read.primary_key.as_deref(), Some("Model All Props"));
    assert_eq!(read.single_line_text.as_deref(), Some("Hello World"));
    assert_eq!(read.checkbox, Some(true));
    assert_eq!(read.number_int, Some(42.0));
    assert_eq!(read.date.as_deref(), Some("2025-01-15"));
    assert_eq!(read.single_select, Some(PrimarySingleSelectOption::Choice1));

    // Update all
    read.primary_key = Some("Model Updated Props".to_string());
    read.single_line_text = Some("Updated Hello".to_string());
    read.long_text = Some("Updated long text".to_string());
    read.long_text_with_rich_text = Some("Updated rich text".to_string());
    read.email = Some("updated@example.com".to_string());
    read.url = Some("https://updated.com".to_string());
    read.phone_number = Some("555-5678".to_string());
    read.checkbox = Some(false);
    read.number_int = Some(100.0);
    read.number_float = Some(2.72);
    read.currency_int = Some(20.0);
    read.currency_float = Some(19.99);
    read.percent_int = Some(0.75);
    read.percent_float = Some(0.667);
    read.duration = Some(7200);
    read.rating = Some(serde_json::json!(5));
    read.date = Some("2025-06-15".to_string());
    read.date_with_time = Some("2025-06-15T14:00:00.000Z".to_string());
    read.single_select = Some(PrimarySingleSelectOption::Choice2);
    read.multiple_select = Some(vec![
        PrimaryMultipleSelectOption::Option2,
        PrimaryMultipleSelectOption::Option3,
    ]);
    read.save().await.unwrap();

    assert_eq!(read.primary_key.as_deref(), Some("Model Updated Props"));
    assert_eq!(read.single_line_text.as_deref(), Some("Updated Hello"));
    assert_eq!(read.number_int, Some(100.0));
    assert_eq!(read.single_select, Some(PrimarySingleSelectOption::Choice2));
    assert_eq!(
        read.multiple_select,
        Some(vec![
            PrimaryMultipleSelectOption::Option2,
            PrimaryMultipleSelectOption::Option3
        ])
    );

    read.delete().await.unwrap();
}

// =============================================================================
// Linked records via model
// =============================================================================

#[tokio::test]
async fn linked_records_via_model() {
    let client = setup();

    // Create secondary records
    let mut sec1 = SecondaryModel {
        name: Some("Model Link Target 1".to_string()),
        value: Some("val1".to_string()),
        ..Default::default()
    };
    sec1.set_client(client.clone(), SECONDARY_TABLE_ID);
    sec1.save().await.unwrap();

    let mut sec2 = SecondaryModel {
        name: Some("Model Link Target 2".to_string()),
        value: Some("val2".to_string()),
        ..Default::default()
    };
    sec2.set_client(client.clone(), SECONDARY_TABLE_ID);
    sec2.save().await.unwrap();

    let sec1_id = sec1.id.as_deref().unwrap().to_string();
    let sec2_id = sec2.id.as_deref().unwrap().to_string();

    // Create primary with links
    let mut model = PrimaryModel {
        primary_key: Some("Model Link Test".to_string()),
        link_single: Some(vec![sec1_id.clone()]),
        link_multiple: Some(vec![sec1_id.clone(), sec2_id.clone()]),
        ..Default::default()
    };
    model.set_client(client.clone(), PRIMARY_TABLE_ID);
    model.save().await.unwrap();

    assert_eq!(model.link_single, Some(vec![sec1_id.clone()]));
    assert_eq!(
        model.link_multiple,
        Some(vec![sec1_id.clone(), sec2_id.clone()])
    );

    // Fetch and verify
    let id = model.id.as_deref().unwrap().to_string();
    let mut read = PrimaryModel::from_id(client.clone(), PRIMARY_TABLE_ID, &id);
    read.fetch().await.unwrap();
    assert_eq!(read.link_single, Some(vec![sec1_id.clone()]));
    assert_eq!(
        read.link_multiple,
        Some(vec![sec1_id.clone(), sec2_id.clone()])
    );

    // Update: swap links
    read.link_single = Some(vec![sec2_id.clone()]);
    read.link_multiple = Some(vec![sec1_id.clone()]);
    read.save().await.unwrap();
    assert_eq!(read.link_single, Some(vec![sec2_id.clone()]));
    assert_eq!(read.link_multiple, Some(vec![sec1_id.clone()]));

    // Cleanup
    model.delete().await.unwrap();
    sec1.delete().await.unwrap();
    sec2.delete().await.unwrap();
}

// =============================================================================
// Attachments via model
// =============================================================================

#[tokio::test]
async fn attachments_via_model() {
    let client = setup();

    let mut model = PrimaryModel {
        primary_key: Some("Model Attachment Test".to_string()),
        attachment: Some(vec![Attachment {
            url:
                "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"
                    .to_string(),
            ..Default::default()
        }]),
        ..Default::default()
    };
    model.set_client(client.clone(), PRIMARY_TABLE_ID);
    model.save().await.unwrap();

    assert!(model.attachment.is_some());
    assert_eq!(model.attachment.as_ref().unwrap().len(), 1);
    assert!(!model.attachment.as_ref().unwrap()[0].url.is_empty());

    // Read with retry (attachments processed async)
    let id = model.id.as_deref().unwrap().to_string();
    let mut read = PrimaryModel::from_id(client.clone(), PRIMARY_TABLE_ID, &id);
    for _ in 0..10 {
        read.fetch().await.unwrap();
        if read.attachment.is_some() {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
    assert!(read.attachment.is_some());
    assert_eq!(read.attachment.as_ref().unwrap().len(), 1);

    model.delete().await.unwrap();
}

// =============================================================================
// User / Collaborator via model
// =============================================================================

#[tokio::test]
async fn user_fields_via_model() {
    let client = setup();

    let mut model = PrimaryModel {
        primary_key: Some("Model User Test".to_string()),
        user: Some(Collaborator {
            id: "usrnZ4k98m0Ipji4e".to_string(),
            email: "9vymqckyxq@privaterelay.appleid.com".to_string(),
            name: Some("Daniel Baughman".to_string()),
        }),
        user_allow_multiple: Some(vec![Collaborator {
            id: "usrnZ4k98m0Ipji4e".to_string(),
            email: "9vymqckyxq@privaterelay.appleid.com".to_string(),
            name: Some("Daniel Baughman".to_string()),
        }]),
        ..Default::default()
    };
    model.set_client(client.clone(), PRIMARY_TABLE_ID);
    model.save().await.unwrap();

    assert_eq!(model.user.as_ref().unwrap().id, "usrnZ4k98m0Ipji4e");
    assert_eq!(model.user_allow_multiple.as_ref().unwrap().len(), 1);

    // Fetch and verify
    let id = model.id.as_deref().unwrap().to_string();
    let mut read = PrimaryModel::from_id(client.clone(), PRIMARY_TABLE_ID, &id);
    read.fetch().await.unwrap();
    assert_eq!(read.user.as_ref().unwrap().id, "usrnZ4k98m0Ipji4e");

    model.delete().await.unwrap();
}

// =============================================================================
// Computed fields populated after save
// =============================================================================

#[tokio::test]
async fn computed_fields_via_model() {
    let client = setup();

    let mut model = PrimaryModel {
        primary_key: Some("Model Computed Test".to_string()),
        number_int: Some(10.0),
        number_float: Some(5.0),
        ..Default::default()
    };
    model.set_client(client.clone(), PRIMARY_TABLE_ID);
    model.save().await.unwrap();

    let id = model.id.as_deref().unwrap().to_string();
    assert!(model.auto_number.is_some());
    assert!(model.created_time.is_some());
    assert_eq!(
        model
            .formula_id
            .as_ref()
            .and_then(|x| x.value())
            .map(String::as_str),
        Some(id.as_str())
    );
    assert_eq!(
        model
            .formula_simple
            .as_ref()
            .and_then(|x| x.value())
            .copied(),
        Some(15.0)
    );

    // Fetch and verify
    let mut read = PrimaryModel::from_id(client.clone(), PRIMARY_TABLE_ID, &id);
    read.fetch().await.unwrap();
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

    read.delete().await.unwrap();
}

// =============================================================================
// Dirty tracking — only changed fields sent on update
// =============================================================================

#[tokio::test]
async fn dirty_tracking() {
    let client = setup();

    let mut model = PrimaryModel {
        primary_key: Some("Dirty Track Test".to_string()),
        single_line_text: Some("Original".to_string()),
        number_int: Some(1.0),
        ..Default::default()
    };
    model.set_client(client.clone(), PRIMARY_TABLE_ID);
    model.save().await.unwrap();

    // Mutate one field — snapshot diff detects the change
    model.number_int = Some(999.0);
    model.save().await.unwrap();

    assert_eq!(model.number_int, Some(999.0));
    assert_eq!(model.primary_key.as_deref(), Some("Dirty Track Test"));
    assert_eq!(model.single_line_text.as_deref(), Some("Original"));

    model.delete().await.unwrap();
}

// =============================================================================
// Invalid operations
// =============================================================================

#[tokio::test]
async fn fetch_invalid_id() {
    let client = setup();
    let mut model = PrimaryModel::from_id(client.clone(), PRIMARY_TABLE_ID, "rec_INVALID");
    assert!(model.fetch().await.is_err());
}

#[tokio::test]
async fn fetch_empty_id() {
    let client = setup();
    let mut model = PrimaryModel::from_id(client.clone(), PRIMARY_TABLE_ID, "");
    assert!(model.fetch().await.is_err());
}
