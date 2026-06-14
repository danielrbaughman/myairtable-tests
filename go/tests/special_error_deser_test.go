package tests

// G5.7 — Offline parity with rust test_special_error_deser / Java/Kotlin/Swift
// TestSpecialErrorDeser: pure json.Unmarshal of Airtable's {"specialValue": ...} /
// {"error": ...} computed-field shapes through the exported runtime types, plus a
// generated-model decode proving erroring/special computed fields and lookup
// arrays (with nulls) don't break deserialization. No live credentials.

import (
	"encoding/json"
	"testing"

	airtable "myairtabletests/output"
)

func TestSpecialErrorDeser(t *testing.T) {
	t.Run("SpecialNumberObjectForm", func(t *testing.T) {
		var sn airtable.SpecialNumber
		if err := json.Unmarshal([]byte(`{"specialValue":"NaN"}`), &sn); err != nil {
			t.Fatalf("decode special: %v", err)
		}
		if sn.SpecialValue != "NaN" {
			t.Fatalf("specialValue = %q, want NaN", sn.SpecialValue)
		}
		// Array form is not a valid SpecialNumber object.
		if err := json.Unmarshal([]byte(`["NaN"]`), &sn); err == nil {
			t.Fatal("expected error decoding array form into SpecialNumber")
		}
	})

	t.Run("ErrorValueObjectForm", func(t *testing.T) {
		var ev airtable.ErrorValue
		if err := json.Unmarshal([]byte(`{"error":"#ERROR!"}`), &ev); err != nil {
			t.Fatalf("decode error: %v", err)
		}
		if ev.Error != "#ERROR!" {
			t.Fatalf("error = %q, want #ERROR!", ev.Error)
		}
		if err := json.Unmarshal([]byte(`["#ERROR!"]`), &ev); err == nil {
			t.Fatal("expected error decoding array form into ErrorValue")
		}
	})

	t.Run("MaybeSpecialOrErrorPlainString", func(t *testing.T) {
		var m airtable.MaybeSpecialOrError[string]
		if err := json.Unmarshal([]byte(`"Stukenholtz"`), &m); err != nil {
			t.Fatalf("decode plain string: %v", err)
		}
		v, ok := m.Value()
		if !ok || v != "Stukenholtz" {
			t.Fatalf("value = %q ok=%v, want Stukenholtz true", v, ok)
		}
		if m.IsSpecial() || m.IsError() {
			t.Fatal("plain string should be neither special nor error")
		}
		// Pre-fix regression in other targets: ["Stukenholtz"] accidentally decoded.
		var m2 airtable.MaybeSpecialOrError[string]
		if err := json.Unmarshal([]byte(`["Stukenholtz"]`), &m2); err == nil {
			t.Fatal("expected error decoding array form into MaybeSpecialOrError[string]")
		}
	})

	t.Run("MaybeSpecialOrErrorFloatSpecial", func(t *testing.T) {
		for _, special := range []string{"NaN", "Infinity", "-Infinity"} {
			var m airtable.MaybeSpecialOrError[float64]
			raw := `{"specialValue":"` + special + `"}`
			if err := json.Unmarshal([]byte(raw), &m); err != nil {
				t.Fatalf("decode %s: %v", special, err)
			}
			if !m.IsSpecial() {
				t.Fatalf("%s: IsSpecial = false", special)
			}
			if _, ok := m.Value(); ok {
				t.Fatalf("%s: Value should be absent", special)
			}
			if m.Special() == nil || m.Special().SpecialValue != special {
				t.Fatalf("%s: special = %+v", special, m.Special())
			}
		}
	})

	t.Run("MaybeSpecialOrErrorFloatError", func(t *testing.T) {
		var m airtable.MaybeSpecialOrError[float64]
		if err := json.Unmarshal([]byte(`{"error":"#ERROR!"}`), &m); err != nil {
			t.Fatalf("decode error: %v", err)
		}
		if !m.IsError() {
			t.Fatal("IsError = false")
		}
		if _, ok := m.Value(); ok {
			t.Fatal("Value should be absent for error")
		}
		if m.ErrorVal() == nil || m.ErrorVal().Error != "#ERROR!" {
			t.Fatalf("errVal = %+v", m.ErrorVal())
		}
	})

	t.Run("MaybeSpecialOrErrorFloatPlain", func(t *testing.T) {
		var m airtable.MaybeSpecialOrError[float64]
		if err := json.Unmarshal([]byte(`15`), &m); err != nil {
			t.Fatalf("decode plain float: %v", err)
		}
		v, ok := m.Value()
		if !ok || v != 15.0 {
			t.Fatalf("value = %v ok=%v, want 15 true", v, ok)
		}
	})

	t.Run("VecOrValueSingle", func(t *testing.T) {
		var v airtable.VecOrValue[airtable.MaybeSpecialOrError[string]]
		if err := json.Unmarshal([]byte(`"hello"`), &v); err != nil {
			t.Fatalf("decode single: %v", err)
		}
		if v.IsMultiple {
			t.Fatal("expected single shape")
		}
		got, ok := v.Single.Value()
		if !ok || got != "hello" {
			t.Fatalf("single value = %q ok=%v", got, ok)
		}
		if vals := v.Values(); len(vals) != 1 {
			t.Fatalf("Values() len = %d, want 1", len(vals))
		}
	})

	t.Run("VecOrValueMultiple", func(t *testing.T) {
		var v airtable.VecOrValue[airtable.MaybeSpecialOrError[string]]
		if err := json.Unmarshal([]byte(`["Stukenholtz Laboratory Inc"]`), &v); err != nil {
			t.Fatalf("decode multiple: %v", err)
		}
		if !v.IsMultiple {
			t.Fatal("expected multiple shape")
		}
		vals := v.Values()
		if len(vals) != 1 {
			t.Fatalf("Values() len = %d, want 1", len(vals))
		}
		got, ok := vals[0].Value()
		if !ok || got != "Stukenholtz Laboratory Inc" {
			t.Fatalf("value = %q ok=%v", got, ok)
		}
	})

	t.Run("VecOrValueMultipleWithNulls", func(t *testing.T) {
		var v airtable.VecOrValue[airtable.MaybeSpecialOrError[string]]
		if err := json.Unmarshal([]byte(`["a", null, "b"]`), &v); err != nil {
			t.Fatalf("decode with nulls: %v", err)
		}
		if !v.IsMultiple {
			t.Fatal("expected multiple shape")
		}
		vals := v.Values()
		if len(vals) != 3 {
			t.Fatalf("Values() len = %d, want 3", len(vals))
		}
		// Element 1 was JSON null -> no value present.
		if _, ok := vals[1].Value(); ok {
			t.Fatal("null element should have no value")
		}
		// Non-null elements present.
		if got, ok := vals[0].Value(); !ok || got != "a" {
			t.Fatalf("vals[0] = %q ok=%v", got, ok)
		}
		if got, ok := vals[2].Value(); !ok || got != "b" {
			t.Fatalf("vals[2] = %q ok=%v", got, ok)
		}
	})

	t.Run("VecOrValueSingleSpecial", func(t *testing.T) {
		var v airtable.VecOrValue[airtable.MaybeSpecialOrError[float64]]
		if err := json.Unmarshal([]byte(`{"specialValue":"NaN"}`), &v); err != nil {
			t.Fatalf("decode special single: %v", err)
		}
		if v.IsMultiple {
			t.Fatal("expected single shape")
		}
		if !v.Single.IsSpecial() || v.Single.Special().SpecialValue != "NaN" {
			t.Fatalf("special = %+v", v.Single.Special())
		}
	})

	t.Run("VecOrValueSingleError", func(t *testing.T) {
		var v airtable.VecOrValue[airtable.MaybeSpecialOrError[float64]]
		if err := json.Unmarshal([]byte(`{"error":"#ERROR!"}`), &v); err != nil {
			t.Fatalf("decode error single: %v", err)
		}
		if v.IsMultiple {
			t.Fatal("expected single shape")
		}
		if !v.Single.IsError() || v.Single.ErrorVal().Error != "#ERROR!" {
			t.Fatalf("errVal = %+v", v.Single.ErrorVal())
		}
	})

	// Decode a generated model containing erroring/special computed fields and a
	// lookup array with nulls — the exact failure mode that broke other targets.
	t.Run("ModelDecodesErroringComputedFields", func(t *testing.T) {
		fieldsJSON := `{
			"` + airtable.PrimaryPrimaryKeyFieldID + `": "ErrDeser",
			"` + airtable.PrimaryFormulaSimpleFieldID + `": {"error": "#ERROR!"},
			"` + airtable.PrimaryFormulaIdFieldID + `": "recERRDESER123456",
			"` + airtable.PrimaryLookupFieldID + `": ["Lab A", null],
			"` + airtable.PrimaryRollupFieldID + `": {"specialValue": "NaN"}
		}`
		var model airtable.PrimaryModel
		if err := json.Unmarshal([]byte(fieldsJSON), &model); err != nil {
			t.Fatalf("decode model: %v", err)
		}

		if model.PrimaryKey == nil || *model.PrimaryKey != "ErrDeser" {
			t.Fatalf("primary key = %v", model.PrimaryKey)
		}
		// Formula(Simple) is an error -> no plain value.
		if model.FormulaSimple == nil || !model.FormulaSimple.IsError() {
			t.Fatalf("formulaSimple should be error: %+v", model.FormulaSimple)
		}
		if _, ok := model.FormulaSimple.Value(); ok {
			t.Fatal("formulaSimple error should have no value")
		}
		// Formula(ID) is a plain string.
		if model.FormulaId == nil {
			t.Fatal("formulaId nil")
		}
		if v, ok := model.FormulaId.Value(); !ok || v != "recERRDESER123456" {
			t.Fatalf("formulaId = %q ok=%v", v, ok)
		}
		// Lookup decodes as a 2-element array with a null in the middle.
		if model.Lookup == nil || !model.Lookup.IsMultiple {
			t.Fatalf("lookup should be multiple: %+v", model.Lookup)
		}
		lvals := model.Lookup.Values()
		if len(lvals) != 2 {
			t.Fatalf("lookup len = %d, want 2", len(lvals))
		}
		if v, ok := lvals[0].Value(); !ok || v != "Lab A" {
			t.Fatalf("lookup[0] = %q ok=%v", v, ok)
		}
		if _, ok := lvals[1].Value(); ok {
			t.Fatal("lookup[1] (null) should have no value")
		}
		// Rollup is a single special number.
		if model.Rollup == nil || model.Rollup.IsMultiple {
			t.Fatalf("rollup should be single: %+v", model.Rollup)
		}
		if !model.Rollup.Single.IsSpecial() || model.Rollup.Single.Special().SpecialValue != "NaN" {
			t.Fatalf("rollup special = %+v", model.Rollup.Single.Special())
		}
	})
}
