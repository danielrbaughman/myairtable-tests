// TC1 — Multi-page pagination: list more records than Airtable's 100-record default page in a
// single query (no max_records) and assert the client's offset loop reassembles every page.
// Exercises the untested page-reassembly path in the generated client. Parity target for the
// other 8 suites.

use myairtable_tests::*;
use std::collections::HashSet;

fn setup() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::new(&api_key, &base_id)
}

// A primary-key prefix unique to this run (distinct per scenario via `label`).
fn primary_key(label: &str) -> String {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let rand = ts.wrapping_mul(2654435761) % 900000 + 100000;
    format!("Rust Pagination {label} {ts}-{rand}")
}

// 105 > the 100-record default page size, so a no-max_records list spans two pages.
const COUNT: usize = 105;

// Best-effort cleanup: delete the created ids; on failure, sweep by FIND prefix.
async fn try_delete_many(at: &Airtable, ids: &[String], suite: &str) {
    if !ids.is_empty() && at.primary.dict.delete_many(ids).await.is_ok() {
        return;
    }
    let params = AirtableQuery::new().formula(format!("FIND(\"{suite}\", {{Primary Key}})"));
    if let Ok(stray) = at.primary.dict.get_many(&params).await {
        if !stray.is_empty() {
            let stray_ids: Vec<String> = stray.iter().map(|r| r.id.clone()).collect();
            let _ = at.primary.dict.delete_many(&stray_ids).await;
        }
    }
}

// =============================================================================
// ORM / typed accessor
// =============================================================================

#[tokio::test]
async fn orm_get_spanning_multiple_pages_returns_every_record() {
    let at = setup();
    let suite = primary_key("Orm");

    let models: Vec<PrimaryModel> = (1..=COUNT)
        .map(|i| PrimaryModel {
            primary_key: Some(format!("{suite} {i}")),
            ..Default::default()
        })
        .collect();

    let created = at.primary.create_many(&models, false).await.unwrap();
    let created_ids: Vec<String> = created
        .iter()
        .map(|m| m.id.as_deref().unwrap().to_string())
        .collect();
    assert_eq!(created.len(), COUNT);

    // No max_records: the offset loop must walk both pages and return all 105.
    let params = AirtableQuery::new().formula(format!("FIND(\"{suite}\", {{Primary Key}})"));
    let result: Result<Vec<PrimaryModel>, _> = at.primary.get_many(&params).await;
    let results = match result {
        Ok(r) => r,
        Err(e) => {
            try_delete_many(&at, &created_ids, &suite).await;
            panic!("get_many failed: {e:?}");
        }
    };

    let result_ids: HashSet<String> = results
        .iter()
        .map(|m| m.id.as_deref().unwrap().to_string())
        .collect();
    let expected_ids: HashSet<String> = created_ids.iter().cloned().collect();

    try_delete_many(&at, &created_ids, &suite).await;

    assert_eq!(results.len(), COUNT);
    assert_eq!(result_ids, expected_ids);
}

// =============================================================================
// TC5 — explicit page size. 25 records over page_size 10 spans 3 pages (10+10+5).
// =============================================================================

const PAGE_SIZE_COUNT: usize = 25;

#[tokio::test]
async fn explicit_page_size_returns_all_records_across_pages() {
    let at = setup();
    let suite = primary_key("PageSize");

    let models: Vec<PrimaryModel> = (1..=PAGE_SIZE_COUNT)
        .map(|i| PrimaryModel {
            primary_key: Some(format!("{suite} {i}")),
            ..Default::default()
        })
        .collect();

    let created = at.primary.create_many(&models, false).await.unwrap();
    let created_ids: Vec<String> = created
        .iter()
        .map(|m| m.id.as_deref().unwrap().to_string())
        .collect();

    // page_size 10, NO max_records: the offset loop must walk all 3 pages and return all 25.
    let params = AirtableQuery::new()
        .formula(format!("FIND(\"{suite}\", {{Primary Key}})"))
        .page_size(10);
    let result: Result<Vec<PrimaryModel>, _> = at.primary.get_many(&params).await;
    let results = match result {
        Ok(r) => r,
        Err(e) => {
            try_delete_many(&at, &created_ids, &suite).await;
            panic!("get_many failed: {e:?}");
        }
    };

    try_delete_many(&at, &created_ids, &suite).await;

    assert_eq!(results.len(), PAGE_SIZE_COUNT);
}

#[tokio::test]
async fn page_size_with_max_records_caps_the_total() {
    let at = setup();
    let suite = primary_key("PageCap");

    let models: Vec<PrimaryModel> = (1..=PAGE_SIZE_COUNT)
        .map(|i| PrimaryModel {
            primary_key: Some(format!("{suite} {i}")),
            ..Default::default()
        })
        .collect();

    let created = at.primary.create_many(&models, false).await.unwrap();
    let created_ids: Vec<String> = created
        .iter()
        .map(|m| m.id.as_deref().unwrap().to_string())
        .collect();

    // page_size 10 + max_records 15: max_records caps the total mid-stream (not a page multiple).
    let params = AirtableQuery::new()
        .formula(format!("FIND(\"{suite}\", {{Primary Key}})"))
        .page_size(10)
        .max_records(15);
    let result: Result<Vec<PrimaryModel>, _> = at.primary.get_many(&params).await;
    let results = match result {
        Ok(r) => r,
        Err(e) => {
            try_delete_many(&at, &created_ids, &suite).await;
            panic!("get_many failed: {e:?}");
        }
    };

    try_delete_many(&at, &created_ids, &suite).await;

    assert_eq!(results.len(), 15);
}

// =============================================================================
// Dict / untyped accessor
// =============================================================================

#[tokio::test]
async fn dict_get_spanning_multiple_pages_returns_every_record() {
    let at = setup();
    let suite = primary_key("Dict");

    let creates: Vec<Fields> = (1..=COUNT)
        .map(|i| {
            let mut f = Fields::new();
            f.set(PrimaryFields::PRIMARY_KEY_ID, format!("{suite} {i}"));
            f
        })
        .collect();

    let created = at.primary.dict.create_many(&creates, true).await.unwrap();
    let created_ids: Vec<String> = created.iter().map(|r| r.id.clone()).collect();
    assert_eq!(created.len(), COUNT);

    // No max_records: the offset loop must walk both pages and return all 105.
    let params = AirtableQuery::new().formula(format!("FIND(\"{suite}\", {{Primary Key}})"));
    let result = at.primary.dict.get_many(&params).await;
    let results = match result {
        Ok(r) => r,
        Err(e) => {
            try_delete_many(&at, &created_ids, &suite).await;
            panic!("get_many failed: {e:?}");
        }
    };

    let result_ids: HashSet<String> = results.iter().map(|r| r.id.clone()).collect();
    let expected_ids: HashSet<String> = created_ids.iter().cloned().collect();

    try_delete_many(&at, &created_ids, &suite).await;

    assert_eq!(results.len(), COUNT);
    assert_eq!(result_ids, expected_ids);
}
