package tests

// G4.6 — Offline (no-network) model serialization parity with Java TestSerializing /
// Kotlin TestSerializing / Swift TestSerializing. These cases exercise the EXPORTED
// generated model structs with stdlib encoding/json directly: the json tags are the
// Airtable field IDs, so a fields-object keyed by field ID round-trips through the
// model. No credentials or network are required — these must run, not skip.

import (
	"encoding/json"
	"testing"

	airtable "myairtabletests/output"
)

// decodePrimary unmarshals a field-ID-keyed JSON object into a PrimaryModel.
func decodePrimary(t *testing.T, body string) airtable.PrimaryModel {
	t.Helper()
	var m airtable.PrimaryModel
	if err := json.Unmarshal([]byte(body), &m); err != nil {
		t.Fatalf("unmarshal PrimaryModel: %v", err)
	}
	return m
}

func TestSerializing(t *testing.T) {
	t.Run("DecodeWritableFieldsKeyedByFieldID", func(t *testing.T) {
		body := `{
			"` + airtable.PrimaryPrimaryKeyFieldID + `": "PK Value",
			"` + airtable.PrimaryEmailFieldID + `": "a@b.c",
			"` + airtable.PrimaryNumberIntFieldID + `": 42,
			"` + airtable.PrimaryCheckboxFieldID + `": true
		}`
		m := decodePrimary(t, body)
		if m.PrimaryKey == nil || *m.PrimaryKey != "PK Value" {
			t.Fatalf("PrimaryKey = %v, want \"PK Value\"", m.PrimaryKey)
		}
		if m.Email == nil || *m.Email != "a@b.c" {
			t.Fatalf("Email = %v, want \"a@b.c\"", m.Email)
		}
		if m.NumberInt == nil || *m.NumberInt != 42 {
			t.Fatalf("NumberInt = %v, want 42", m.NumberInt)
		}
		if m.Checkbox == nil || *m.Checkbox != true {
			t.Fatalf("Checkbox = %v, want true", m.Checkbox)
		}
	})

	t.Run("AbsentFieldIsNilNotZeroValue", func(t *testing.T) {
		m := decodePrimary(t, `{"`+airtable.PrimaryPrimaryKeyFieldID+`": "only pk"}`)
		if m.PrimaryKey == nil || *m.PrimaryKey != "only pk" {
			t.Fatalf("PrimaryKey = %v, want \"only pk\"", m.PrimaryKey)
		}
		// Absent writable fields decode to nil pointers, distinguishing "unset"
		// from a zero value ("" / 0 / false).
		if m.Email != nil {
			t.Fatalf("Email = %v, want nil for absent field", m.Email)
		}
		if m.NumberInt != nil {
			t.Fatalf("NumberInt = %v, want nil for absent field", m.NumberInt)
		}
		if m.Checkbox != nil {
			t.Fatalf("Checkbox = %v, want nil for absent field", m.Checkbox)
		}
	})

	t.Run("ComputedFieldDecodesPlainValue", func(t *testing.T) {
		// Auto Number is a computed *MaybeSpecialOrError[int64]; a plain number
		// decodes to a present value.
		m := decodePrimary(t, `{"`+airtable.PrimaryAutoNumberFieldID+`": 7}`)
		if m.AutoNumber == nil {
			t.Fatal("AutoNumber = nil, want present wrapper")
		}
		v, ok := m.AutoNumber.Value()
		if !ok || v != 7 {
			t.Fatalf("AutoNumber.Value() = (%d, %v), want (7, true)", v, ok)
		}
		if m.AutoNumber.IsSpecial() || m.AutoNumber.IsError() {
			t.Fatal("plain value should be neither special nor error")
		}
	})

	t.Run("ComputedFieldDecodesSpecialValue", func(t *testing.T) {
		m := decodePrimary(t, `{"`+airtable.PrimaryAutoNumberFieldID+`": {"specialValue": "NaN"}}`)
		if m.AutoNumber == nil {
			t.Fatal("AutoNumber = nil, want present wrapper")
		}
		if !m.AutoNumber.IsSpecial() {
			t.Fatal("AutoNumber.IsSpecial() = false, want true")
		}
		if _, ok := m.AutoNumber.Value(); ok {
			t.Fatal("AutoNumber.Value() ok = true for a special number, want false")
		}
		if sn := m.AutoNumber.Special(); sn == nil || sn.SpecialValue != "NaN" {
			t.Fatalf("Special() = %v, want SpecialValue=NaN", sn)
		}
	})

	t.Run("ComputedFieldDecodesErrorValue", func(t *testing.T) {
		m := decodePrimary(t, `{"`+airtable.PrimaryAutoNumberFieldID+`": {"error": "#ERROR!"}}`)
		if m.AutoNumber == nil {
			t.Fatal("AutoNumber = nil, want present wrapper")
		}
		if !m.AutoNumber.IsError() {
			t.Fatal("AutoNumber.IsError() = false, want true")
		}
		if _, ok := m.AutoNumber.Value(); ok {
			t.Fatal("AutoNumber.Value() ok = true for an error, want false")
		}
		if ev := m.AutoNumber.ErrorVal(); ev == nil || ev.Error != "#ERROR!" {
			t.Fatalf("ErrorVal() = %v, want Error=#ERROR!", ev)
		}
	})

	t.Run("AirtableTimeDecodesDateOnlyAndRFC3339", func(t *testing.T) {
		m := decodePrimary(t, `{
			"`+airtable.PrimaryDateFieldID+`": "2024-01-15",
			"`+airtable.PrimaryDateWithTimeFieldID+`": "2024-01-15T10:30:00.000Z"
		}`)
		if m.Date == nil {
			t.Fatal("Date = nil, want decoded date-only value")
		}
		if got := m.Date.Time.UTC().Format("2006-01-02T15:04:05Z07:00"); got != "2024-01-15T00:00:00Z" {
			t.Fatalf("Date = %q, want 2024-01-15T00:00:00Z", got)
		}
		if m.DateWithTime == nil {
			t.Fatal("DateWithTime = nil, want decoded RFC3339 value")
		}
		if got := m.DateWithTime.Time.UTC().Format("2006-01-02T15:04:05Z07:00"); got != "2024-01-15T10:30:00Z" {
			t.Fatalf("DateWithTime = %q, want 2024-01-15T10:30:00Z", got)
		}
	})

	t.Run("AirtableDurationDecodesNumericSeconds", func(t *testing.T) {
		m := decodePrimary(t, `{"`+airtable.PrimaryDurationFieldID+`": 3661}`)
		if m.Duration == nil {
			t.Fatal("Duration = nil, want decoded numeric seconds")
		}
		if got := m.Duration.Duration().Seconds(); got != 3661 {
			t.Fatalf("Duration = %v seconds, want 3661", got)
		}
	})

	t.Run("SliceFieldDecodes", func(t *testing.T) {
		// Link (multiple) is a []string of linked record IDs.
		m := decodePrimary(t, `{"`+airtable.PrimaryLinkMultipleFieldID+`": ["recAAA", "recBBB"]}`)
		if len(m.LinkMultiple) != 2 || m.LinkMultiple[0] != "recAAA" || m.LinkMultiple[1] != "recBBB" {
			t.Fatalf("LinkMultiple = %v, want [recAAA recBBB]", m.LinkMultiple)
		}
	})

	t.Run("MarshalEmitsFieldIDKeysAndOmitsNil", func(t *testing.T) {
		m := airtable.PrimaryModel{
			PrimaryKey: airtable.String("x"),
			Email:      airtable.String("a@b.c"),
			NumberInt:  airtable.Float64(7),
		}
		encoded, err := json.Marshal(&m)
		if err != nil {
			t.Fatalf("marshal: %v", err)
		}
		// Re-decode into a generic map so assertions are key-order independent.
		var out map[string]json.RawMessage
		if err := json.Unmarshal(encoded, &out); err != nil {
			t.Fatalf("re-unmarshal: %v", err)
		}
		// Set writable pointer fields appear under their field-ID keys.
		if string(out[airtable.PrimaryPrimaryKeyFieldID]) != `"x"` {
			t.Fatalf("PrimaryKey key = %s, want \"x\"", out[airtable.PrimaryPrimaryKeyFieldID])
		}
		if string(out[airtable.PrimaryEmailFieldID]) != `"a@b.c"` {
			t.Fatalf("Email key = %s, want \"a@b.c\"", out[airtable.PrimaryEmailFieldID])
		}
		if string(out[airtable.PrimaryNumberIntFieldID]) != `7` {
			t.Fatalf("NumberInt key = %s, want 7", out[airtable.PrimaryNumberIntFieldID])
		}
		// Nil writable fields are omitted (omitempty on the pointer tag).
		if _, ok := out[airtable.PrimaryCheckboxFieldID]; ok {
			t.Fatal("Checkbox present in output, want omitted for nil pointer")
		}
		if _, ok := out[airtable.PrimaryDurationFieldID]; ok {
			t.Fatal("Duration present in output, want omitted for nil pointer")
		}
	})
}
