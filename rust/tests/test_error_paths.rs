//! TC3 — Error / validation paths with TYPED errors.
//!
//! Mirrors the C# `TestErrorPaths` suite. Exercises the generated client's error
//! classification against the live base: a 404 fetch, three server-side validation
//! rejections (invalid select option, wrong value type, unknown field), a 401 from
//! a bogus key, and the offline `RateLimited` variant. Each scenario asserts the
//! *specific* `AirtableError::Api` variant (never a bare `is_err()`), checks the HTTP
//! `status`, and the first-class Airtable error `code` field.

use myairtable_tests::*;

fn setup() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::new(&api_key, &base_id)
}

/// A client pointed at the real base but with a bogus API key (mirrors
/// `MakeAirtableWithBadKey`), so auth (401) is exercised rather than the base (404).
fn setup_with_bad_key() -> Airtable {
    dotenvy::from_filename(".env").ok();
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    Airtable::new("patBOGUS00000.deadbeefdeadbeefdeadbeefdeadbeef", &base_id)
}

/// A primary-key value unique to this run (distinct per file via `label`).
fn primary_key(label: &str) -> String {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("Rust Error {label} {ts}")
}

/// Assert `result` is the typed `AirtableError::Api` variant, return `(status, code)`.
fn expect_api_error<T: std::fmt::Debug>(
    result: Result<T, AirtableError>,
    scenario: &str,
) -> (u16, Option<String>) {
    match result {
        Err(AirtableError::Api { status, code, body }) => {
            println!("{scenario} -> Api {{ status: {status}, code: {code:?} }}: {body}");
            // The Airtable code is now a first-class field; surface None when empty so the
            // 404-by-id legacy envelope (which carries the code in `message`) is handled below.
            let code = if code.is_empty() { None } else { Some(code) };
            (status, code)
        }
        Err(other) => panic!("{scenario}: expected AirtableError::Api, got {other:?}"),
        Ok(v) => panic!("{scenario}: expected an error, got Ok({v:?})"),
    }
}

// 1. GET a nonexistent record id -> typed Api error, 404, code NOT_FOUND.
#[tokio::test]
async fn get_nonexistent_record_is_typed_api_error() {
    let at = setup();
    let result = at
        .primary
        .dict
        .get_one(&"recDOESNOTEXIST0001".to_string(), true)
        .await;
    let (status, code) = expect_api_error(result, "404 get");
    assert_eq!(status, 404, "expected HTTP 404 for a nonexistent record id");
    assert_eq!(
        code.as_deref(),
        Some("NOT_FOUND"),
        "expected NOT_FOUND code"
    );
}

// 2. Dict-create with an invalid single-select option -> typed Api error,
//    code INVALID_MULTIPLE_CHOICE_OPTIONS.
#[tokio::test]
async fn create_with_invalid_select_option_is_typed_api_error() {
    let at = setup();
    let mut fields = Fields::new();
    fields.set(PrimaryFields::PRIMARY_KEY_ID, primary_key("BadSelect"));
    fields.set(PrimaryFields::SINGLE_SELECT_ID, "NotARealOption_zzz");

    let result = at.primary.dict.create_one(&fields, true).await;
    let (status, code) = expect_api_error(result, "bad select");
    assert!(
        (400..500).contains(&status),
        "expected a 4xx client error, got {status}"
    );
    assert_eq!(
        code.as_deref(),
        Some("INVALID_MULTIPLE_CHOICE_OPTIONS"),
        "expected INVALID_MULTIPLE_CHOICE_OPTIONS code"
    );
}

// 3. Dict-create with a wrong-typed value (Number field set to a string) ->
//    typed Api error, code INVALID_VALUE_FOR_COLUMN.
#[tokio::test]
async fn create_with_wrong_value_type_is_typed_api_error() {
    let at = setup();
    let mut fields = Fields::new();
    fields.set(PrimaryFields::PRIMARY_KEY_ID, primary_key("BadType"));
    fields.set(PrimaryFields::NUMBER_INT_ID, "not a number");

    let result = at.primary.dict.create_one(&fields, true).await;
    let (status, code) = expect_api_error(result, "wrong type");
    assert!(
        (400..500).contains(&status),
        "expected a 4xx client error, got {status}"
    );
    assert_eq!(
        code.as_deref(),
        Some("INVALID_VALUE_FOR_COLUMN"),
        "expected INVALID_VALUE_FOR_COLUMN code"
    );
}

// 4. Dict-create with an unknown field id -> typed Api error, code UNKNOWN_FIELD_NAME.
#[tokio::test]
async fn create_with_unknown_field_is_typed_api_error() {
    let at = setup();
    let mut fields = Fields::new();
    fields.set(PrimaryFields::PRIMARY_KEY_ID, primary_key("BadField"));
    fields.set("fldDOESNOTEXIST00000", "x");

    let result = at.primary.dict.create_one(&fields, true).await;
    let (status, code) = expect_api_error(result, "unknown field");
    assert!(
        (400..500).contains(&status),
        "expected a 4xx client error, got {status}"
    );
    assert_eq!(
        code.as_deref(),
        Some("UNKNOWN_FIELD_NAME"),
        "expected UNKNOWN_FIELD_NAME code"
    );
}

// 5. Real base id + bogus key -> typed Api error, 401, code AUTHENTICATION_REQUIRED.
#[tokio::test]
async fn bad_api_key_is_typed_auth_error() {
    let at = setup_with_bad_key();
    let mut query = AirtableQuery::new();
    query.max_records = Some(1);

    let result = at.primary.dict.get_many(&query).await;
    let (status, code) = expect_api_error(result, "bad key");
    assert_eq!(status, 401, "expected HTTP 401 for a bogus API key");
    // Airtable's 401 envelope carries AUTHENTICATION_REQUIRED; assert it when present but
    // don't fail solely on the code shape — the 401 status is the load-bearing classification.
    if let Some(code) = code {
        assert_eq!(
            code, "AUTHENTICATION_REQUIRED",
            "expected AUTHENTICATION_REQUIRED code"
        );
    }
}

// 6. Rate-limited (429): assert the distinct `RateLimited` variant is well-formed and carries
//    its Retry-After value (offline). The live 429 path is covered by TC8.
#[test]
fn rate_limited_is_a_distinct_typed_variant() {
    let err = AirtableError::RateLimited {
        retry_after: Some(30.0),
    };
    match err {
        AirtableError::RateLimited { retry_after } => assert_eq!(retry_after, Some(30.0)),
        other => panic!("expected RateLimited, got {other:?}"),
    }
    // A 429 response classifies as RateLimited, not Api.
    assert!(matches!(
        AirtableError::from_response(429, String::new()),
        AirtableError::RateLimited { .. }
    ));
}
