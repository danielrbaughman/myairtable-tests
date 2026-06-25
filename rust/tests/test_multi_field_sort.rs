use myairtable_tests::*;

// =============================================================================
// TC11 — Multi-field sort
//
// The base filter suite only covers a single-field sort. This verifies a
// two-field sort (primary key with ties broken by a secondary key) and sorting
// within a filtered scope. Mirrors csharp/tests/TestMultiFieldSort.cs.
// =============================================================================

fn setup() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::new(&api_key, &base_id)
}

fn row(suite: &str, number: i64, text: &str) -> Fields {
    let mut f = Fields::new();
    f.set(PrimaryFields::PRIMARY_KEY_ID, format!("{suite} {text}"));
    f.set(PrimaryFields::NUMBER_INT_ID, number);
    f.set(PrimaryFields::SINGLE_LINE_TEXT_ID, text);
    f
}

fn scope_to(ids: &[&str]) -> String {
    if ids.len() == 1 {
        return format!("RECORD_ID()='{}'", ids[0]);
    }
    let parts: Vec<String> = ids.iter().map(|id| format!("RECORD_ID()='{id}'")).collect();
    format!("OR({})", parts.join(","))
}

fn texts(records: &[Record]) -> Vec<String> {
    records
        .iter()
        .map(|r| {
            r.fields
                .get(PrimaryFields::SINGLE_LINE_TEXT_ID)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
        })
        .collect()
}

async fn try_delete_many(at: &Airtable, ids: &[String]) {
    if ids.is_empty() {
        return;
    }
    let _ = at.primary.dict.delete_many(ids).await;
}

// =============================================================================
// Two-field sort: ties on the primary key broken by the secondary key.
// =============================================================================

#[tokio::test]
async fn two_field_sort_breaks_ties_on_second_key() {
    let at = setup();
    let suite = "Sort TwoField";

    // NumberInt ties at 10 (rows "b" and "a"); the secondary SingleLineText sort orders them.
    let created = at
        .primary
        .dict
        .create_many(
            &[
                row(suite, 10, "b"),
                row(suite, 10, "a"),
                row(suite, 20, "c"),
            ],
            true,
        )
        .await
        .unwrap();
    let ids: Vec<String> = created.iter().map(|r| r.id.clone()).collect();
    let id_refs: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();

    let params = AirtableQuery::new()
        .formula(scope_to(&id_refs))
        .sort(PrimaryFields::NUMBER_INT_ID, SortDirection::Asc)
        .sort(PrimaryFields::SINGLE_LINE_TEXT_ID, SortDirection::Asc);
    let results = at.primary.dict.get_many(&params).await.unwrap();

    // (10,a), (10,b), (20,c) — tie on 10 broken by text asc.
    assert_eq!(texts(&results), vec!["a", "b", "c"]);

    try_delete_many(&at, &ids).await;
}

// =============================================================================
// Secondary descending reverses the tied group.
// =============================================================================

#[tokio::test]
async fn secondary_descending_reverses_tied_group() {
    let at = setup();
    let suite = "Sort MixedDir";

    let created = at
        .primary
        .dict
        .create_many(
            &[
                row(suite, 10, "a"),
                row(suite, 10, "b"),
                row(suite, 20, "c"),
            ],
            true,
        )
        .await
        .unwrap();
    let ids: Vec<String> = created.iter().map(|r| r.id.clone()).collect();
    let id_refs: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();

    let params = AirtableQuery::new()
        .formula(scope_to(&id_refs))
        .sort(PrimaryFields::NUMBER_INT_ID, SortDirection::Asc)
        .sort(PrimaryFields::SINGLE_LINE_TEXT_ID, SortDirection::Desc);
    let results = at.primary.dict.get_many(&params).await.unwrap();

    // NumberInt asc, then text DESC within the 10-tie: (10,b), (10,a), (20,c).
    assert_eq!(texts(&results), vec!["b", "a", "c"]);

    try_delete_many(&at, &ids).await;
}

// =============================================================================
// Sort combined with a filter.
// =============================================================================

#[tokio::test]
async fn sort_combined_with_a_filter() {
    let at = setup();
    let f = PrimaryModel::F;
    let suite = "Sort WithFilter";

    let created = at
        .primary
        .dict
        .create_many(
            &[
                row(suite, 30, "x"),
                row(suite, 10, "y"),
                row(suite, 20, "z"),
                row(suite, 5, "low"), // filtered out by NumberInt > 5
            ],
            true,
        )
        .await
        .unwrap();
    let ids: Vec<String> = created.iter().map(|r| r.id.clone()).collect();
    let id_refs: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();

    let filter = formula::AND(&[&scope_to(&id_refs), &f.number_int.greater_than(5)]);
    let params = AirtableQuery::new()
        .formula(filter)
        .sort(PrimaryFields::NUMBER_INT_ID, SortDirection::Asc);
    let results = at.primary.dict.get_many(&params).await.unwrap();

    // Filtered to NumberInt > 5, sorted asc: 10(y), 20(z), 30(x).
    assert_eq!(texts(&results), vec!["y", "z", "x"]);

    try_delete_many(&at, &ids).await;
}
