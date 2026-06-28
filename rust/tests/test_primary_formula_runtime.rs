//! TC6 — Primary "Formula (Complex)"/"Formula (Nested)" evaluated at runtime.
//!
//! The base runtime suite only covers the Formulas table; the Primary complex formula
//! concatenates ~35 fields through `IF(field, field, "None")`, a richer transpile path
//! never checked against the API before.
//!
//! We compare the transpiled `evaluate_formula_complex()` to the API line-by-line for the
//! DETERMINISTIC, offline-reproducible field types (text, checkbox, single/multi select,
//! numbers, currency, email, url, phone). The formula also references server-computed
//! fields (Created/Last Modified Time + By, Auto Number, Button, Formula(ID)/(Simple)) and
//! link/lookup/rollup — Airtable renders those from data the offline runtime doesn't hold
//! (collaborator names, linked-record display values, special wrappers), so those lines are
//! NOT expected to match offline. See myairtable-5b0n.
//!
//! This suite specifically locks in the multi-select array-join fix (myairtable-bb7f): a
//! multi-value field coerces to "Option 1, Option 2", not just the first element.

use myairtable_tests::airtable_runtime as F;
use myairtable_tests::*;

fn setup() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::new(&api_key, &base_id)
}

/// Field labels whose rendering the offline runtime can reproduce exactly.
const DETERMINISTIC_LABELS: &[&str] = &[
    "Single Line Text",
    "Long Text",
    "Checkbox",
    "Multiple Select",
    "Single Select",
    "Number (int)",
    "Number (float)",
    "Currency (int)",
    "Currency (float)",
    "Email",
    "URL",
    "Phone Number",
];

/// Build a Primary record with deterministic, offline-reproducible field values.
/// Link/lookup/rollup/user/attachment are left empty.
fn new_record(suite: &str) -> PrimaryModel {
    PrimaryModel {
        primary_key: Some(suite.to_string()),
        single_line_text: Some("hello".to_string()),
        long_text: Some("long text".to_string()),
        email: Some("a@b.co".to_string()),
        url: Some("https://x.co".to_string()),
        phone_number: Some("555-1212".to_string()),
        checkbox: Some(true),
        number_int: Some(42.0),
        number_float: Some(3.5),
        currency_int: Some(10.0),
        currency_float: Some(9.99),
        single_select: Some(PrimarySingleSelectOption::Choice1),
        multiple_select: Some(vec![
            PrimaryMultipleSelectOption::Option1,
            PrimaryMultipleSelectOption::Option2,
        ]),
        ..Default::default()
    }
}

/// Extract the "Label: value" line for `label` from a formula result.
fn line(formula: &str, label: &str) -> String {
    let prefix = format!("{label}: ");
    for l in formula.split('\n') {
        if l.starts_with(&prefix) {
            return l.to_string();
        }
    }
    format!("<missing: {label}>")
}

/// Extract the API string from the `Formula (Complex)` field, which decodes as
/// `VecOrValue<MaybeSpecialOrError<String>>` (mirrors C# `VecOrValue.CleanValues(...).First()`).
fn clean_first(field: &Option<VecOrValue<MaybeSpecialOrError<String>>>) -> String {
    match field {
        Some(VecOrValue::Single(MaybeSpecialOrError::Value(s))) => s.clone(),
        Some(VecOrValue::Multiple(items)) => items
            .iter()
            .flatten()
            .find_map(|m| m.value().cloned())
            .unwrap_or_default(),
        _ => String::new(),
    }
}

async fn try_delete(at: &Airtable, record_id: &str) {
    if record_id.is_empty() {
        return;
    }
    let _ = at.primary.delete_one(&record_id.to_string()).await;
}

#[tokio::test]
async fn complex_formula_renders_deterministic_fields_like_api() {
    let at = setup();

    let created = at
        .primary
        .create_one(&new_record("PrimaryFormula:Complex"), false)
        .await
        .unwrap();
    let record_id = created.id.as_deref().unwrap().to_string();

    let result: Result<(), String> = async {
        let fetched = at
            .primary
            .get_one(&record_id)
            .await
            .map_err(|e| e.to_string())?;
        let api = clean_first(&fetched.formula_complex);
        let runtime = F::S(&fetched.evaluate_formula_complex());
        println!("--- API ---\n{api}\n--- RUNTIME ---\n{runtime}");

        for label in DETERMINISTIC_LABELS {
            assert_eq!(
                line(&api, label),
                line(&runtime, label),
                "Deterministic field divergence for label {label:?}"
            );
        }

        // The multi-select join is the headline fix: both sides render all options, comma-joined.
        assert_eq!(
            "Multiple Select: Option 1, Option 2",
            line(&runtime, "Multiple Select")
        );
        Ok(())
    }
    .await;

    try_delete(&at, &record_id).await;
    result.unwrap();
}

#[tokio::test]
async fn nested_formula_evaluates_without_panicking() {
    // Formula (Nested) = Formula(ID) & Formula(Simple) & Formula(Complex) — it chains three
    // COMPUTED formula fields. Offline the runtime can't reproduce computed-field values (they
    // decode as special/wrapped types it doesn't re-evaluate), so the content isn't asserted;
    // this confirms the transpiled nested-formula method is generated and evaluates without
    // error. See myairtable-5b0n.
    let at = setup();

    let created = at
        .primary
        .create_one(&new_record("PrimaryFormula:Nested"), false)
        .await
        .unwrap();
    let record_id = created.id.as_deref().unwrap().to_string();

    let result: Result<(), String> = async {
        let fetched = at
            .primary
            .get_one(&record_id)
            .await
            .map_err(|e| e.to_string())?;
        let runtime = F::S(&fetched.evaluate_formula_nested()); // must not panic
        let _ = runtime;
        Ok(())
    }
    .await;

    try_delete(&at, &record_id).await;
    result.unwrap();
}
