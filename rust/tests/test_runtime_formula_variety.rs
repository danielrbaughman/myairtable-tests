use myairtable_tests::airtable_runtime as F;
use myairtable_tests::*;
use serde_json::json;

fn setup() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::new(&api_key, &base_id)
}

/// TC4 — Runtime-formula input variety. The base test_runtime_formulas suite only evaluates the
/// kitchen-sink formulas with ONE fully-populated input set, so the IF(OR(...=BLANK())) short-circuit
/// and varied inputs are never exercised. Each case here creates a Formulas record with a specific
/// input set, fetches the API-computed value, and asserts the transpiled evaluate_*() reproduces it.
///
/// Scope notes (kept deliberately portable across all 9 targets):
///  - Numbers are positive with LOG/SQRT arguments chosen to be exact (powers of 10 / perfect
///    squares). The Math formula calls LOG()/SQRT()/EXP() — transcendental results differ by a ULP
///    between platforms' math libs and Airtable (V8), so irrational results (e.g. LOG(5)) are NOT
///    bit-identical and would make an exact string compare flaky. Negatives/zero are excluded too:
///    LOG(negative) and MOD(_,0) error inside this formula.
///  - Text covers empty-ish edges (whitespace), unicode, and reserved punctuation (exercising the
///    fixed ENCODE_URL_COMPONENT). An all-blank text input is excluded: Airtable returns blank for
///    the whole formula (REPLACE past end-of-string errors), while the transpiler is lenient.

/// Build the common base fields (primary key + the three dates from the C# Base()).
fn base_fields(label: &str) -> Vec<(&'static str, serde_json::Value)> {
    vec![
        (
            FormulasFields::PRIMARY_KEY_ID,
            json!(format!("Rust Variety {}", label)),
        ),
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
}

fn make_fields(pairs: Vec<(&'static str, serde_json::Value)>) -> Fields {
    Fields(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}

/// Fetch a record back, converting any error objects ({"error": "..."}) to null, and deserialize.
async fn fetch_model(at: &Airtable, record_id: &String) -> FormulasModel {
    let fetched = at.formulas.dict.get_one(record_id, true).await.unwrap();
    let mut fields_json = serde_json::to_value(&fetched.fields).unwrap();
    if let Some(obj) = fields_json.as_object_mut() {
        for key in obj.keys().cloned().collect::<Vec<_>>() {
            if obj[&key].is_object() {
                obj.insert(key, serde_json::Value::Null);
            }
        }
    }
    serde_json::from_value(fields_json).unwrap()
}

// First Number: power of 10 (LOG exact). Second Number: perfect square (SQRT exact).
const NUMBER_CASES: &[(&str, f64, f64, f64)] = &[
    ("hundreds", 100.0, 16.0, 8.0),
    ("ones", 1.0, 4.0, 2.0),
    ("tens", 10.0, 25.0, 3.0),
];

#[tokio::test]
async fn math_formula_matches_api_for_varied_numbers() {
    let at = setup();

    for (label, a, b, c) in NUMBER_CASES.iter().copied() {
        let mut pairs = base_fields(&format!("Math {}", label));
        pairs.push((FormulasFields::FIRST_NUMBER_ID, json!(a)));
        pairs.push((FormulasFields::SECOND_NUMBER_ID, json!(b)));
        pairs.push((FormulasFields::THIRD_NUMBER_ID, json!(c)));
        pairs.push((FormulasFields::FIRST_TEXT_ID, json!("x")));
        pairs.push((FormulasFields::SECOND_TEXT_ID, json!("y")));
        pairs.push((FormulasFields::THIRD_TEXT_ID, json!("z")));
        let fields = make_fields(pairs);

        let created = at.formulas.dict.create_one(&fields, true).await.unwrap();
        let record_id = created.id.clone();

        let model = fetch_model(&at, &record_id).await;
        let api_math = model.math_formula.clone().and_then(|x| x.into_value());
        let runtime_math = F::S(&model.evaluate_math_formula());
        println!(
            "{}: api='{}' runtime='{}'",
            label,
            api_math.clone().unwrap_or_default(),
            runtime_math
        );

        at.formulas.dict.delete_one(&record_id).await.unwrap();

        assert_eq!(
            api_math,
            Some(runtime_math),
            "Math formula mismatch for case '{}'",
            label
        );
    }
}

#[tokio::test]
async fn math_formula_blank_branch_when_numbers_missing() {
    // First/Second Number left unset -> OR(BLANK, BLANK) is true -> the formula returns BLANK().
    // This is the IF-true short-circuit that the base suite never reaches.
    let at = setup();

    let mut pairs = base_fields("Blank");
    pairs.push((FormulasFields::FIRST_TEXT_ID, json!("x")));
    pairs.push((FormulasFields::SECOND_TEXT_ID, json!("y")));
    pairs.push((FormulasFields::THIRD_TEXT_ID, json!("z")));
    let fields = make_fields(pairs);

    let created = at.formulas.dict.create_one(&fields, true).await.unwrap();
    let record_id = created.id.clone();

    let model = fetch_model(&at, &record_id).await;
    let api_val = model.math_formula.clone().and_then(|x| x.into_value());
    let runtime = F::S(&model.evaluate_math_formula());
    println!(
        "blank: api='{}' runtime='{}'",
        api_val.clone().unwrap_or_default(),
        runtime
    );

    at.formulas.dict.delete_one(&record_id).await.unwrap();

    assert!(
        api_val.as_deref().unwrap_or("").is_empty(),
        "API expected blank, got '{:?}'",
        api_val
    );
    assert!(
        runtime.is_empty(),
        "runtime expected blank, got '{}'",
        runtime
    );
}

const TEXT_CASES: &[(&str, &str, &str, &str)] = &[
    ("unicode", "café", "naïve", "日本語🎉"),
    ("whitespace", "  he llo  ", "a b", "c"),
    ("punct", "a.e-i+o", "x/y", "z"), // exercises fixed ENCODE_URL_COMPONENT
];

#[tokio::test]
async fn text_formula_matches_api_for_varied_text() {
    let at = setup();

    for (label, a, b, c) in TEXT_CASES.iter().copied() {
        let mut pairs = base_fields(&format!("Text {}", label));
        pairs.push((FormulasFields::FIRST_NUMBER_ID, json!(10.0)));
        pairs.push((FormulasFields::SECOND_NUMBER_ID, json!(20.0)));
        pairs.push((FormulasFields::THIRD_NUMBER_ID, json!(30.0)));
        pairs.push((FormulasFields::FIRST_TEXT_ID, json!(a)));
        pairs.push((FormulasFields::SECOND_TEXT_ID, json!(b)));
        pairs.push((FormulasFields::THIRD_TEXT_ID, json!(c)));
        let fields = make_fields(pairs);

        let created = at.formulas.dict.create_one(&fields, true).await.unwrap();
        let record_id = created.id.clone();

        let model = fetch_model(&at, &record_id).await;
        let api_text = model.text_formula.clone().and_then(|x| x.into_value());
        let runtime_text = F::S(&model.evaluate_text_formula());
        println!(
            "{}: api='{}' runtime='{}'",
            label,
            api_text.clone().unwrap_or_default(),
            runtime_text
        );

        at.formulas.dict.delete_one(&record_id).await.unwrap();

        assert_eq!(
            api_text,
            Some(runtime_text),
            "Text formula mismatch for case '{}'",
            label
        );
    }
}
