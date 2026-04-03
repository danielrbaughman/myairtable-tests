use myairtable_tests::*;

fn setup() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::new(&api_key, &base_id)
}

async fn create_primary(at: &Airtable, fields: &Fields) -> Record {
    at.primary.dict.create_one(fields, true).await.unwrap()
}

fn primary(name: &str) -> Fields {
    let mut f = Fields::new();
    f.set(PrimaryFields::PRIMARY_KEY_ID, name);
    f
}

fn scope_to(ids: &[&str]) -> String {
    if ids.len() == 1 {
        return format!("RECORD_ID()='{}'", ids[0]);
    }
    let parts: Vec<String> = ids.iter().map(|id| format!("RECORD_ID()='{id}'")).collect();
    format!("OR({})", parts.join(","))
}

// =============================================================================
// Filter by view
// =============================================================================

#[tokio::test]
async fn filter_by_view() {
    let at = setup();

    let mut all_records = Vec::new();
    for i in 0..5 {
        all_records.push(primary(&format!("Filter Test {i}")));
    }
    for i in 0..5 {
        all_records.push(primary(&format!("Don't Include Test {i}")));
    }
    let created = at
        .primary
        .dict
        .create_many(&all_records, true)
        .await
        .unwrap();
    let ids: Vec<String> = created.iter().map(|r| r.id.clone()).collect();

    let params = AirtableQuery::new().view(PrimaryView::FilterByView);
    let results = at.primary.dict.get_many(&params).await.unwrap();

    assert!(results.len() >= 5);
    for record in &results {
        let name = record
            .fields
            .get(PrimaryFields::PRIMARY_KEY_ID)
            .unwrap()
            .as_str()
            .unwrap();
        assert!(name.starts_with("Filter Test"), "Unexpected record: {name}");
    }

    at.primary.dict.delete_many(&ids).await.unwrap();
}

// =============================================================================
// Filter by ID formula
// =============================================================================

#[tokio::test]
async fn filter_by_id_equals() {
    let at = setup();
    let f = PrimaryModel::F;

    let r1 = create_primary(&at, &primary("FbId A")).await;
    let r2 = create_primary(&at, &primary("FbId B")).await;
    let r3 = create_primary(&at, &primary("FbId C")).await;

    // equals
    let params = AirtableQuery::new().formula(f.id.equals(&r1.id));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, r1.id);

    // in_list multiple
    let params = AirtableQuery::new().formula(f.id.in_list(&[&r1.id, &r2.id]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // in_list single
    let params = AirtableQuery::new().formula(f.id.in_list(&[&r1.id]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 1);

    // in_list empty
    let params = AirtableQuery::new().formula(f.id.in_list(&[]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 0);

    at.primary
        .delete_many(&[r1.id, r2.id, r3.id])
        .await
        .unwrap();
}

// =============================================================================
// Filter by text field formula
// =============================================================================

#[tokio::test]
async fn filter_by_text_field() {
    let at = setup();
    let f = PrimaryModel::F;

    let r1 = create_primary(&at, &primary("FbText Alpha One")).await;
    let r2 = create_primary(&at, &primary("FbText Alpha Two")).await;
    let r3 = create_primary(&at, &primary("FbText Beta One")).await;
    let scope = scope_to(&[&r1.id, &r2.id, &r3.id]);

    // equals
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &f.primary_key.equals("FbText Alpha One", true, false),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 1);

    // not_equals
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &f.primary_key.not_equals("FbText Alpha One", true, false),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // contains
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &f.primary_key.contains("Alpha", false, true),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // contains_any
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &f.primary_key.contains_any(&["Alpha", "Beta"], false, true),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 3);

    // contains_all
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &f.primary_key.contains_all(&["Alpha", "One"], false, true),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 1);

    // not_contains
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &f.primary_key.not_contains("Alpha", false, true),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert!(results.len() >= 1);
    for r in &results {
        let name = r
            .fields
            .get(PrimaryFields::PRIMARY_KEY_ID)
            .unwrap()
            .as_str()
            .unwrap();
        assert!(!name.contains("Alpha"));
    }

    // starts_with
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &f.primary_key.starts_with("FbText Alpha", false, true),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // not_starts_with
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &f.primary_key.not_starts_with("FbText Alpha", false, true),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert!(results.len() >= 1);

    at.primary
        .delete_many(&[r1.id, r2.id, r3.id])
        .await
        .unwrap();
}

// =============================================================================
// Filter by number field formula
// =============================================================================

#[tokio::test]
async fn filter_by_number_field() {
    let at = setup();
    let f = PrimaryModel::F;

    let mut f1 = primary("FbNum A");
    f1.set(PrimaryFields::NUMBER_INT_ID, 10);
    let mut f2 = primary("FbNum B");
    f2.set(PrimaryFields::NUMBER_INT_ID, 20);
    let mut f3 = primary("FbNum C");
    f3.set(PrimaryFields::NUMBER_INT_ID, 30);

    let r1 = create_primary(&at, &f1).await;
    let r2 = create_primary(&at, &f2).await;
    let r3 = create_primary(&at, &f3).await;
    let scope = scope_to(&[&r1.id, &r2.id, &r3.id]);

    // equals
    let params = AirtableQuery::new().formula(formula::AND(&[&scope, &f.number_int.equals(20)]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 1);

    // not_equals
    let params =
        AirtableQuery::new().formula(formula::AND(&[&scope, &f.number_int.not_equals(20)]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // greater_than
    let params =
        AirtableQuery::new().formula(formula::AND(&[&scope, &f.number_int.greater_than(10)]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // less_than
    let params = AirtableQuery::new().formula(formula::AND(&[&scope, &f.number_int.less_than(30)]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // greater_than_or_equals
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &f.number_int.greater_than_or_equals(20),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // less_than_or_equals
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &f.number_int.less_than_or_equals(20),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // between inclusive
    let params =
        AirtableQuery::new().formula(formula::AND(&[&scope, &f.number_int.between(10, 30, true)]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 3);

    // between exclusive
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &f.number_int.between(10, 30, false),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 1);

    at.primary
        .delete_many(&[r1.id, r2.id, r3.id])
        .await
        .unwrap();
}

// =============================================================================
// Filter by boolean field formula
// =============================================================================

#[tokio::test]
async fn filter_by_boolean_field() {
    let at = setup();
    let f = PrimaryModel::F;

    let mut f1 = primary("FbBool A");
    f1.set(PrimaryFields::CHECKBOX_ID, true);
    let mut f2 = primary("FbBool B");
    f2.set(PrimaryFields::CHECKBOX_ID, false);

    let r1 = create_primary(&at, &f1).await;
    let r2 = create_primary(&at, &f2).await;
    let scope = scope_to(&[&r1.id, &r2.id]);

    // equals(true)
    let params = AirtableQuery::new().formula(formula::AND(&[&scope, &f.checkbox.equals(true)]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 1);

    // equals(false)
    let params = AirtableQuery::new().formula(formula::AND(&[&scope, &f.checkbox.equals(false)]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert!(results.len() >= 1);

    // is_true
    let params = AirtableQuery::new().formula(formula::AND(&[&scope, &f.checkbox.is_true()]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 1);

    // is_false
    let params = AirtableQuery::new().formula(formula::AND(&[&scope, &f.checkbox.is_false()]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert!(results.len() >= 1);

    at.primary.dict.delete_many(&[r1.id, r2.id]).await.unwrap();
}

// =============================================================================
// Filter by attachments field formula
// =============================================================================

#[tokio::test]
async fn filter_by_attachments_field() {
    let at = setup();
    let f = PrimaryModel::F;

    let mut f1 = primary("FbAttach A");
    f1.set(PrimaryFields::ATTACHMENT_ID, serde_json::json!([
        {"url": "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"}
    ]));
    let f2 = primary("FbAttach B");

    let r1 = create_primary(&at, &f1).await;
    let r2 = create_primary(&at, &f2).await;
    let scope = scope_to(&[&r1.id, &r2.id]);

    // not_empty
    let params = AirtableQuery::new().formula(formula::AND(&[&scope, &f.attachment.not_empty()]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert!(results.len() >= 1);

    // empty
    let params = AirtableQuery::new().formula(formula::AND(&[&scope, &f.attachment.empty()]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert!(results.len() >= 1);

    at.primary.dict.delete_many(&[r1.id, r2.id]).await.unwrap();
}

// =============================================================================
// Filter by date field formula
// =============================================================================

#[tokio::test]
async fn filter_by_date_field() {
    let at = setup();
    let f = PrimaryModel::F;

    let mut f1 = primary("FbDate A");
    f1.set(PrimaryFields::DATE_ID, "2024-01-15");
    let mut f2 = primary("FbDate B");
    f2.set(PrimaryFields::DATE_ID, "2024-06-15");
    let f3 = primary("FbDate C"); // no date

    let r1 = create_primary(&at, &f1).await;
    let r2 = create_primary(&at, &f2).await;
    let r3 = create_primary(&at, &f3).await;
    let scope = scope_to(&[&r1.id, &r2.id, &r3.id]);

    // on
    let params = AirtableQuery::new().formula(formula::AND(&[&scope, &f.date.on("2024-01-15")]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 1);

    // not_on
    let params =
        AirtableQuery::new().formula(formula::AND(&[&scope, &f.date.not_on("2024-01-15")]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert!(results.len() >= 1);

    // on_or_after
    let params =
        AirtableQuery::new().formula(formula::AND(&[&scope, &f.date.on_or_after("2024-06-15")]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert!(results.len() >= 1);

    // on_or_before
    let params =
        AirtableQuery::new().formula(formula::AND(&[&scope, &f.date.on_or_before("2024-01-15")]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert!(results.len() >= 1);

    // after
    let params = AirtableQuery::new().formula(formula::AND(&[&scope, &f.date.after("2024-01-15")]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert!(results.len() >= 1);

    // before
    let params =
        AirtableQuery::new().formula(formula::AND(&[&scope, &f.date.before("2024-06-15")]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert!(results.len() >= 1);

    // between inclusive
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &f.date.between("2024-01-15", "2024-06-15", true),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // between exclusive
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &f.date.between("2024-01-01", "2024-12-31", false),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // not_empty
    let params = AirtableQuery::new().formula(formula::AND(&[&scope, &f.date.not_empty()]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert!(results.len() >= 1);

    // empty
    let params = AirtableQuery::new().formula(formula::AND(&[&scope, &f.date.empty()]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert!(results.len() >= 1);

    at.primary
        .delete_many(&[r1.id, r2.id, r3.id])
        .await
        .unwrap();
}

// =============================================================================
// Filter by date field chained formula (time-ago)
// =============================================================================

#[tokio::test]
async fn filter_by_date_field_chained() {
    let at = setup();
    let f = PrimaryModel::F;

    let mut f1 = primary("FbDateChain A");
    f1.set(PrimaryFields::DATE_ID, "2024-01-15");
    let mut f2 = primary("FbDateChain B");
    f2.set(PrimaryFields::DATE_ID, "2024-06-15");

    let r1 = create_primary(&at, &f1).await;
    let r2 = create_primary(&at, &f2).await;
    let scope = scope_to(&[&r1.id, &r2.id]);

    // before().days_ago(1) — both dates are in the past
    let params =
        AirtableQuery::new().formula(formula::AND(&[&scope, &f.date.before_chain().days_ago(1)]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert!(results.len() >= 1);

    // after().years_ago(100) — both dates are within last 100 years
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &f.date.after_chain().years_ago(100),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert!(results.len() >= 1);

    at.primary.dict.delete_many(&[r1.id, r2.id]).await.unwrap();
}

// =============================================================================
// Complex formulas (AND, OR, NOT, XOR)
// =============================================================================

#[tokio::test]
async fn filter_by_complex_formulas() {
    let at = setup();
    let f = PrimaryModel::F;

    let mut f1 = primary("FbComplex A");
    f1.set(PrimaryFields::NUMBER_INT_ID, 10);
    f1.set(PrimaryFields::CHECKBOX_ID, true);
    let mut f2 = primary("FbComplex B");
    f2.set(PrimaryFields::NUMBER_INT_ID, 20);
    f2.set(PrimaryFields::CHECKBOX_ID, false);
    let mut f3 = primary("FbComplex C");
    f3.set(PrimaryFields::NUMBER_INT_ID, 30);
    f3.set(PrimaryFields::CHECKBOX_ID, true);

    let r1 = create_primary(&at, &f1).await;
    let r2 = create_primary(&at, &f2).await;
    let r3 = create_primary(&at, &f3).await;
    let scope = scope_to(&[&r1.id, &r2.id, &r3.id]);

    // AND(number > 10, checkbox = true) -> only C
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &f.number_int.greater_than(10),
        &f.checkbox.is_true(),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 1);

    // AND(contains, gte) -> B and C
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &f.primary_key.contains("Complex", false, true),
        &f.number_int.greater_than_or_equals(20),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // OR(number = 10, number = 30) -> A and C
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &formula::OR(&[&f.number_int.equals(10), &f.number_int.equals(30)]),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // OR(checkbox = true, number = 20) -> all three
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &formula::OR(&[&f.checkbox.is_true(), &f.number_int.equals(20)]),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 3);

    // XOR(checkbox = true, number >= 30) -> only A (true XOR false)
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &formula::XOR(&[
            &f.checkbox.is_true(),
            &f.number_int.greater_than_or_equals(30),
        ]),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 1);

    // NOT(primary_key = "FbComplex A") -> B and C
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &formula::NOT(&f.primary_key.equals("FbComplex A", true, false)),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // AND(OR(number=10, number=30), checkbox=true) -> A and C
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &formula::OR(&[&f.number_int.equals(10), &f.number_int.equals(30)]),
        &f.checkbox.is_true(),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // OR(AND(number > 10, checkbox), primary_key = "FbComplex A") -> A and C
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &formula::OR(&[
            &formula::AND(&[&f.number_int.greater_than(10), &f.checkbox.is_true()]),
            &f.primary_key.equals("FbComplex A", true, false),
        ]),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // NOT(AND(checkbox, number > 20)) -> A and B
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &formula::NOT(&formula::AND(&[
            &f.checkbox.is_true(),
            &f.number_int.greater_than(20),
        ])),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 2);

    // AND(NOT(checkbox), gte(20)) -> only B
    let params = AirtableQuery::new().formula(formula::AND(&[
        &scope,
        &formula::NOT(&f.checkbox.is_true()),
        &f.number_int.greater_than_or_equals(20),
    ]));
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 1);

    at.primary
        .delete_many(&[r1.id, r2.id, r3.id])
        .await
        .unwrap();
}

// =============================================================================
// Max records
// =============================================================================

#[tokio::test]
async fn max_records() {
    let at = setup();

    let records: Vec<Fields> = (1..=5)
        .map(|i| primary(&format!("MaxRecTest {i}")))
        .collect();
    let created = at.primary.dict.create_many(&records, true).await.unwrap();
    let ids: Vec<String> = created.iter().map(|r| r.id.clone()).collect();
    let id_refs: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();

    let params = AirtableQuery::new()
        .formula(scope_to(&id_refs))
        .max_records(3);
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 3);

    at.primary.dict.delete_many(&ids).await.unwrap();
}

// =============================================================================
// Sort
// =============================================================================

#[tokio::test]
async fn sort_records() {
    let at = setup();

    let mut f1 = primary("SortTest C");
    f1.set(PrimaryFields::NUMBER_INT_ID, 30);
    let mut f2 = primary("SortTest A");
    f2.set(PrimaryFields::NUMBER_INT_ID, 10);
    let mut f3 = primary("SortTest B");
    f3.set(PrimaryFields::NUMBER_INT_ID, 20);

    let created = at
        .primary
        .dict
        .create_many(&[f1, f2, f3], true)
        .await
        .unwrap();
    let ids: Vec<String> = created.iter().map(|r| r.id.clone()).collect();
    let id_refs: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();

    // Ascending
    let params = AirtableQuery::new()
        .formula(scope_to(&id_refs))
        .sort(PrimaryFields::NUMBER_INT, SortDirection::Asc);
    let results = at.primary.dict.get_many(&params).await.unwrap();
    let nums: Vec<i64> = results
        .iter()
        .map(|r| {
            r.fields
                .get(PrimaryFields::NUMBER_INT_ID)
                .unwrap()
                .as_i64()
                .unwrap()
        })
        .collect();
    assert_eq!(nums, vec![10, 20, 30]);

    // Descending
    let params = AirtableQuery::new()
        .formula(scope_to(&id_refs))
        .sort(PrimaryFields::NUMBER_INT, SortDirection::Desc);
    let results = at.primary.dict.get_many(&params).await.unwrap();
    let nums: Vec<i64> = results
        .iter()
        .map(|r| {
            r.fields
                .get(PrimaryFields::NUMBER_INT_ID)
                .unwrap()
                .as_i64()
                .unwrap()
        })
        .collect();
    assert_eq!(nums, vec![30, 20, 10]);

    at.primary.dict.delete_many(&ids).await.unwrap();
}

// =============================================================================
// Field selection
// =============================================================================

#[tokio::test]
async fn field_selection() {
    let at = setup();

    let mut f = primary("FieldSelectTest");
    f.set(PrimaryFields::SINGLE_LINE_TEXT_ID, "Hello");
    f.set(PrimaryFields::NUMBER_INT_ID, 42);

    let created = create_primary(&at, &f).await;

    let params = AirtableQuery::new()
        .formula(format!("RECORD_ID()='{}'", created.id))
        .fields(vec![PrimaryFields::PRIMARY_KEY_ID.to_string()]);
    let results = at.primary.dict.get_many(&params).await.unwrap();
    assert_eq!(results.len(), 1);

    let fields = &results[0].fields;
    assert!(fields.get(PrimaryFields::PRIMARY_KEY_ID).is_some());
    assert!(fields.get(PrimaryFields::SINGLE_LINE_TEXT_ID).is_none());
    assert!(fields.get(PrimaryFields::NUMBER_INT_ID).is_none());

    at.primary.dict.delete_one(&created.id).await.unwrap();
}
