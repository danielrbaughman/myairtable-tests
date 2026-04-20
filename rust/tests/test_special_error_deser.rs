use myairtable_tests::*;

// SpecialNumber: accept object form, reject positional-array form.

#[test]
fn special_number_accepts_object_form() {
    let v: SpecialNumber = serde_json::from_str(r#"{"specialValue":"NaN"}"#).unwrap();
    assert_eq!(v.special_value, "NaN");
}

#[test]
fn special_number_rejects_array_form() {
    let result: Result<SpecialNumber, _> = serde_json::from_str(r#"["NaN"]"#);
    assert!(
        result.is_err(),
        "array form must not deserialize into SpecialNumber (regression: lookup-array fields collapse otherwise)"
    );
}

// ErrorValue: same.

#[test]
fn error_value_accepts_object_form() {
    let v: ErrorValue = serde_json::from_str(r##"{"error":"#ERROR!"}"##).unwrap();
    assert_eq!(v.error, "#ERROR!");
}

#[test]
fn error_value_rejects_array_form() {
    let result: Result<ErrorValue, _> = serde_json::from_str(r##"["#ERROR!"]"##);
    assert!(result.is_err());
}

// MaybeSpecialOrError<String>: untagged enum — array form must not route to Special.

#[test]
fn maybe_special_or_error_string_parses_plain_string() {
    let v: MaybeSpecialOrError<String> = serde_json::from_str(r#""Stukenholtz""#).unwrap();
    assert!(matches!(v, MaybeSpecialOrError::Value(ref s) if s == "Stukenholtz"));
}

#[test]
fn maybe_special_or_error_string_rejects_lookup_array() {
    // Pre-fix: ["Stukenholtz"] accidentally succeeded as Special.
    let result: Result<MaybeSpecialOrError<String>, _> = serde_json::from_str(r#"["Stukenholtz"]"#);
    assert!(result.is_err());
}

// VecOrValue<MaybeSpecialOrError<String>>: the real lookup-field shape.

#[test]
fn vec_or_value_single_string() {
    let v: VecOrValue<MaybeSpecialOrError<String>> = serde_json::from_str(r#""hello""#).unwrap();
    match v {
        VecOrValue::Single(MaybeSpecialOrError::Value(s)) => assert_eq!(s, "hello"),
        other => panic!("expected Single(Value), got {other:?}"),
    }
}

#[test]
fn vec_or_value_multiple_string_array() {
    let v: VecOrValue<MaybeSpecialOrError<String>> =
        serde_json::from_str(r#"["Stukenholtz Laboratory Inc"]"#).unwrap();
    match v {
        VecOrValue::Multiple(items) => {
            assert_eq!(items.len(), 1);
            match items[0].as_ref().expect("non-null item") {
                MaybeSpecialOrError::Value(s) => assert_eq!(s, "Stukenholtz Laboratory Inc"),
                other => panic!("expected Value, got {other:?}"),
            }
        }
        other => panic!("expected Multiple, got {other:?}"),
    }
}

#[test]
fn vec_or_value_nullable_items_preserved() {
    let v: VecOrValue<MaybeSpecialOrError<String>> =
        serde_json::from_str(r#"["a", null, "b"]"#).unwrap();
    match v {
        VecOrValue::Multiple(items) => {
            assert_eq!(items.len(), 3);
            assert!(items[1].is_none());
        }
        other => panic!("expected Multiple, got {other:?}"),
    }
}

#[test]
fn vec_or_value_special_number_object() {
    let v: VecOrValue<MaybeSpecialOrError<f64>> =
        serde_json::from_str(r#"{"specialValue":"NaN"}"#).unwrap();
    match v {
        VecOrValue::Single(MaybeSpecialOrError::Special(s)) => assert_eq!(s.special_value, "NaN"),
        other => panic!("expected Single(Special), got {other:?}"),
    }
}

#[test]
fn vec_or_value_error_value_object() {
    let v: VecOrValue<MaybeSpecialOrError<f64>> =
        serde_json::from_str(r##"{"error":"#ERROR!"}"##).unwrap();
    match v {
        VecOrValue::Single(MaybeSpecialOrError::Error(e)) => assert_eq!(e.error, "#ERROR!"),
        other => panic!("expected Single(Error), got {other:?}"),
    }
}

#[test]
fn special_number_roundtrip_serialize_deserialize() {
    let original = SpecialNumber {
        special_value: "NaN".to_string(),
    };
    let json = serde_json::to_string(&original).unwrap();
    assert_eq!(json, r#"{"specialValue":"NaN"}"#);
    let parsed: SpecialNumber = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, original);
}
