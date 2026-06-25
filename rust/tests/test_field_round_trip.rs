// TC7 — Field-type round-trip completeness via re-fetch. Several field types were only asserted on
// the create response (or only offline-decoded), never written and read back through the live API,
// and clearing/removing multi-value fields was untested. Each case here creates, optionally updates,
// re-fetches, and asserts the server-side value. Parity target for the other 8 suites.

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

// Shared-base user, as in TestComplexProperties / user_fields_crud.
const USER_ID: &str = "usrnZ4k98m0Ipji4e";
const USER_EMAIL: &str = "9vymqckyxq@privaterelay.appleid.com";

async fn try_delete(at: &Airtable, record_id: &str) {
    let _ = at.primary.delete_one(&record_id.to_string()).await;
}

// =============================================================================
// 1. DateWithTime writes and reads back
// =============================================================================

#[tokio::test]
async fn date_with_time_writes_and_reads_back() {
    let at = setup();
    let suite = primary_key("FieldRT", "DateTime");
    let dt = "2024-03-15T14:30:00.000Z";

    let created = at
        .primary
        .create_one(&PrimaryModel {
            primary_key: Some(suite),
            date_with_time: Some(dt.to_string()),
            ..Default::default()
        })
        .await
        .unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    let result = async {
        let fetched = at.primary.get_one(&id).await.unwrap();
        assert_eq!(fetched.date_with_time.as_deref(), Some(dt));
    }
    .await;

    try_delete(&at, &id).await;
    result
}

// =============================================================================
// 2. Rich text + percent/currency read back
// =============================================================================

#[tokio::test]
async fn rich_text_and_percent_currency_read_back() {
    let at = setup();
    let suite = primary_key("FieldRT", "Rich");

    let created = at
        .primary
        .create_one(&PrimaryModel {
            primary_key: Some(suite),
            long_text_with_rich_text: Some("**bold** and _italic_ text".to_string()),
            percent_int: Some(0.5),
            percent_float: Some(0.333),
            currency_int: Some(100.0),
            currency_float: Some(19.99),
            ..Default::default()
        })
        .await
        .unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    let result = async {
        let fetched = at.primary.get_one(&id).await.unwrap();
        assert_eq!(
            fetched.long_text_with_rich_text.as_deref(),
            Some("**bold** and _italic_ text")
        );
        assert_eq!(fetched.percent_int, Some(0.5));
        assert_eq!(fetched.percent_float, Some(0.333));
        assert_eq!(fetched.currency_int, Some(100.0));
        assert_eq!(fetched.currency_float, Some(19.99));
    }
    .await;

    try_delete(&at, &id).await;
    result
}

// =============================================================================
// 3. Clearing single + multi select reads back empty
// =============================================================================

#[tokio::test]
async fn clearing_single_and_multi_select_reads_back_empty() {
    let at = setup();
    let suite = primary_key("FieldRT", "ClearSelect");

    let mut created = at
        .primary
        .create_one(&PrimaryModel {
            primary_key: Some(suite),
            single_select: Some(PrimarySingleSelectOption::Choice1),
            multiple_select: Some(vec![
                PrimaryMultipleSelectOption::Option1,
                PrimaryMultipleSelectOption::Option2,
            ]),
            ..Default::default()
        })
        .await
        .unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    let result = async {
        assert_eq!(
            created.single_select,
            Some(PrimarySingleSelectOption::Choice1)
        );

        // Clear single-select -> None, multi-select -> empty. The dirty diff (snapshot
        // taken on create) turns these into explicit nulls/empties on the wire.
        created.single_select = None;
        created.multiple_select = Some(vec![]);
        at.primary.update_one(&id, &created).await.unwrap();

        let fetched = at.primary.get_one(&id).await.unwrap();
        assert!(fetched.single_select.is_none());
        assert!(fetched
            .multiple_select
            .as_ref()
            .map(|v| v.is_empty())
            .unwrap_or(true));
    }
    .await;

    try_delete(&at, &id).await;
    result
}

// =============================================================================
// 4. Removing a collaborator reads back null
// =============================================================================

#[tokio::test]
async fn removing_a_collaborator_reads_back_null() {
    let at = setup();
    let suite = primary_key("FieldRT", "RemoveUser");

    let mut created = at
        .primary
        .create_one(&PrimaryModel {
            primary_key: Some(suite),
            user: Some(Collaborator {
                id: USER_ID.to_string(),
                email: USER_EMAIL.to_string(),
                name: None,
            }),
            ..Default::default()
        })
        .await
        .unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    let result = async {
        assert_eq!(created.user.as_ref().unwrap().id, USER_ID);

        created.user = None;
        at.primary.update_one(&id, &created).await.unwrap();

        let fetched = at.primary.get_one(&id).await.unwrap();
        assert!(fetched.user.is_none());
    }
    .await;

    try_delete(&at, &id).await;
    result
}

// =============================================================================
// 5. Attachment replace and remove read back
// =============================================================================

#[tokio::test]
async fn attachment_replace_and_remove_read_back() {
    let at = setup();
    const URL_A: &str =
        "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png";
    const URL_B: &str = "https://www.w3.org/Icons/w3c_home.png";
    let suite = primary_key("FieldRT", "Attach");

    let mut created = at
        .primary
        .create_one(&PrimaryModel {
            primary_key: Some(suite),
            attachment: Some(vec![Attachment {
                url: URL_A.to_string(),
                ..Default::default()
            }]),
            ..Default::default()
        })
        .await
        .unwrap();
    let id = created.id.as_deref().unwrap().to_string();

    let result = async {
        // Replace the attachment with a different one.
        created.attachment = Some(vec![Attachment {
            url: URL_B.to_string(),
            ..Default::default()
        }]);
        let mut replaced = at.primary.update_one(&id, &created).await.unwrap();

        // Airtable processes attachments asynchronously; retry the re-fetch until present.
        let mut fetched = at.primary.get_one(&id).await.unwrap();
        for _ in 0..10 {
            if fetched.attachment.as_ref().map(|a| !a.is_empty()) == Some(true) {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            fetched = at.primary.get_one(&id).await.unwrap();
        }
        let attachments = fetched.attachment.as_ref().expect("attachment present");
        assert_eq!(attachments.len(), 1);

        // Remove all attachments. Use the freshly fetched model so the snapshot reflects
        // the server's processed attachment list, then clear it.
        replaced.attachment = Some(vec![]);
        at.primary.update_one(&id, &replaced).await.unwrap();
        let cleared = at.primary.get_one(&id).await.unwrap();
        assert!(cleared
            .attachment
            .as_ref()
            .map(|a| a.is_empty())
            .unwrap_or(true));
    }
    .await;

    try_delete(&at, &id).await;
    result
}
