use myairtable_tests::airtable_runtime as F;
use myairtable_tests::*;
use serde_json::json;

fn setup() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::new(&api_key, &base_id)
}

/// Shared test fixture: create a record with all fields populated, fetch it back with
/// API-computed formula values, then run all assertions and clean up.
#[tokio::test]
async fn runtime_formulas_match_api() {
    let at = setup();

    // Create a single record with all input fields (matching Python/TypeScript test data)
    let fields = Fields(
        vec![
            (FormulasFields::PRIMARY_KEY_ID, json!("RustRuntimeTest")),
            (FormulasFields::FIRST_NUMBER_ID, json!(10)),
            (FormulasFields::SECOND_NUMBER_ID, json!(20)),
            (FormulasFields::THIRD_NUMBER_ID, json!(30)),
            (FormulasFields::FIRST_TEXT_ID, json!("Hello")),
            (FormulasFields::SECOND_TEXT_ID, json!("World")),
            (FormulasFields::THIRD_TEXT_ID, json!("!")),
            (
                FormulasFields::FIRST_DATE_ID,
                json!("2024-01-01T00:00:00.000Z"),
            ),
            (
                FormulasFields::SECOND_DATE_ID,
                json!("2024-02-01T00:00:00.000Z"),
            ),
            (
                FormulasFields::THIRD_DATE_ID,
                json!("2024-03-01T00:00:00.000Z"),
            ),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect(),
    );

    let record = at.formulas.dict.create_one(&fields, true).await.unwrap();
    let record_id = record.id.clone();

    // Re-fetch to get formula values computed by Airtable
    let fetched = at.formulas.dict.get_one(&record_id, true).await.unwrap();

    // Deserialize, converting any error objects ({"error": "..."}) to null
    let mut fields_json = serde_json::to_value(&fetched.fields).unwrap();
    if let Some(obj) = fields_json.as_object_mut() {
        for key in obj.keys().cloned().collect::<Vec<_>>() {
            if obj[&key].is_object() {
                obj.insert(key, serde_json::Value::Null);
            }
        }
    }
    let model: FormulasModel = serde_json::from_value(fields_json).unwrap();

    // --- Math formula: exact match ---
    let api_math = model.math_formula.clone();
    let runtime_math = F::S(&model.evaluate_math_formula());
    assert_eq!(api_math, Some(runtime_math), "Math formula mismatch");

    // --- Text formula: exact match ---
    let api_text = model.text_formula.clone();
    let runtime_text = F::S(&model.evaluate_text_formula());
    assert_eq!(api_text, Some(runtime_text), "Text formula mismatch");

    // --- Date formula: partial match (TODAY/TONOW/FROMNOW are time-dependent) ---
    let api_date = model.date_formula.clone().unwrap_or_default();
    let runtime_date = F::S(&model.evaluate_date_formula());

    // Check all deterministic parts
    assert!(runtime_date.contains("YEAR: 2024"), "got: {}", runtime_date);
    assert!(runtime_date.contains("MONTH: 1"), "got: {}", runtime_date);
    assert!(runtime_date.contains("DAY: 1"), "got: {}", runtime_date);
    assert!(runtime_date.contains("HOUR: 0"), "got: {}", runtime_date);
    assert!(runtime_date.contains("MINUTE: 0"), "got: {}", runtime_date);
    assert!(runtime_date.contains("SECOND: 0"), "got: {}", runtime_date);
    assert!(
        runtime_date.contains("WORKDAY_DIFF: "),
        "got: {}",
        runtime_date
    );
    assert!(
        runtime_date.contains("DATEADD+2d: 2024-01-03"),
        "got: {}",
        runtime_date
    );
    assert!(
        runtime_date.contains("IS_BEFORE: Yes"),
        "got: {}",
        runtime_date
    );
    assert!(
        runtime_date.contains("IS_SAME day: No"),
        "got: {}",
        runtime_date
    );
    assert!(
        runtime_date.contains("IS_AFTER: Yes"),
        "got: {}",
        runtime_date
    );
    assert!(runtime_date.contains("DATESTR: "), "got: {}", runtime_date);
    assert!(runtime_date.contains("TIMESTR: "), "got: {}", runtime_date);

    // Compare the deterministic prefix of API vs runtime (before TODAY: which varies)
    let api_prefix = api_date.split(", TODAY:").next().unwrap_or("");
    let runtime_prefix = runtime_date.split(", TODAY:").next().unwrap_or("");
    assert_eq!(
        api_prefix, runtime_prefix,
        "Date formula mismatch (pre-TODAY portion)"
    );

    // Cleanup
    at.formulas.dict.delete_one(&record_id).await.unwrap();
}
