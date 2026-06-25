package tests

// TC6 — Primary "Formula (Complex)"/"Formula (Nested)" evaluated at runtime.
// Mirrors the C# TestPrimaryFormulaRuntime suite. The base runtime suite only
// covers the Formulas table; the Primary complex formula concatenates ~35 fields
// through IF(field, field, "None"), a richer transpile path never checked
// against the API before.
//
// We compare the transpiled EvaluateFormulaComplex() to the API line-by-line for
// the DETERMINISTIC, offline-reproducible field types (text, checkbox,
// single/multi select, numbers, currency, email, url, phone). The formula also
// references server-computed fields (Created/Last Modified Time + By, Auto
// Number, Button, Formula(ID)/(Simple)) and link/lookup/rollup — Airtable renders
// those from data the offline runtime doesn't hold, so those lines are NOT
// expected to match offline. See myairtable-5b0n.
//
// This suite specifically locks in the multi-select array-join fix
// (myairtable-bb7f): a multi-value field coerces to "Option 1, Option 2", not
// just the first element.

import (
	"context"
	"strings"
	"testing"

	airtable "myairtabletests/output"
)

// deterministicLabels are the field labels whose rendering the offline runtime
// can reproduce exactly.
var deterministicLabels = []string{
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
}

// newPrimaryFormulaRecord builds a Primary record with deterministic field
// values, leaving link/lookup/rollup/user/attachment empty.
func newPrimaryFormulaRecord(suite string) *airtable.PrimaryModel {
	return &airtable.PrimaryModel{
		PrimaryKey:     airtable.String(suite),
		SingleLineText: airtable.String("hello"),
		LongText:       airtable.String("long text"),
		Email:          airtable.String("a@b.co"),
		Url:            airtable.String("https://x.co"),
		PhoneNumber:    airtable.String("555-1212"),
		Checkbox:       airtable.Bool(true),
		NumberInt:      airtable.Float64(42),
		NumberFloat:    airtable.Float64(3.5),
		CurrencyInt:    airtable.Float64(10),
		CurrencyFloat:  airtable.Float64(9.99),
		SingleSelect:   primarySingleSelectPtr(airtable.PrimarySingleSelectOptionChoice1),
		MultipleSelect: []airtable.PrimaryMultipleSelectOption{
			airtable.PrimaryMultipleSelectOptionOption1,
			airtable.PrimaryMultipleSelectOptionOption2,
		},
	}
}

func primarySingleSelectPtr(v airtable.PrimarySingleSelectOption) *airtable.PrimarySingleSelectOption {
	return &v
}

// formulaLine extracts the "Label: value" line for label from a formula result.
func formulaLine(formula, label string) string {
	for _, line := range strings.Split(formula, "\n") {
		if strings.HasPrefix(line, label+": ") {
			return line
		}
	}
	return "<missing: " + label + ">"
}

func TestPrimaryFormulaRuntime(t *testing.T) {
	at := newAirtable(t)
	ctx := context.Background()

	t.Run("ComplexFormulaRendersDeterministicFieldsLikeApi", func(t *testing.T) {
		suite := primaryKey("PrimaryFormula", "Complex")
		created, err := at.Primary.CreateOne(ctx, newPrimaryFormulaRecord(suite))
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		recordID := created.ID()
		if recordID == "" {
			t.Fatal("missing id on created model")
		}
		defer at.Primary.DeleteOne(ctx, recordID) //nolint:errcheck

		fetched, err := at.Primary.GetOne(ctx, recordID)
		if err != nil {
			t.Fatalf("get: %v", err)
		}

		api := ""
		if fetched.FormulaComplex != nil {
			if vals := airtable.CleanValues(*fetched.FormulaComplex); len(vals) > 0 {
				api = vals[0]
			}
		}
		runtime := airtable.S(fetched.EvaluateFormulaComplex())
		t.Logf("--- API ---\n%s\n--- RUNTIME ---\n%s", api, runtime)

		for _, label := range deterministicLabels {
			gotAPI := formulaLine(api, label)
			gotRT := formulaLine(runtime, label)
			if gotAPI != gotRT {
				t.Errorf("formula line mismatch for %q:\n  api=%q\n  rt =%q", label, gotAPI, gotRT)
			}
		}

		// The multi-select join is the headline fix: both sides render all
		// options, comma-joined.
		if got := formulaLine(runtime, "Multiple Select"); got != "Multiple Select: Option 1, Option 2" {
			t.Errorf("multi-select join: got %q, want %q", got, "Multiple Select: Option 1, Option 2")
		}
	})

	t.Run("NestedFormulaEvaluatesWithoutThrowing", func(t *testing.T) {
		// Formula (Nested) = Formula(ID) & Formula(Simple) & Formula(Complex) —
		// it chains three COMPUTED formula fields. Offline the runtime can't
		// reproduce computed-field values, so the content isn't asserted; this
		// confirms the transpiled nested-formula method is generated and
		// evaluates without error. See myairtable-5b0n.
		suite := primaryKey("PrimaryFormula", "Nested")
		created, err := at.Primary.CreateOne(ctx, newPrimaryFormulaRecord(suite))
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		recordID := created.ID()
		defer at.Primary.DeleteOne(ctx, recordID) //nolint:errcheck

		fetched, err := at.Primary.GetOne(ctx, recordID)
		if err != nil {
			t.Fatalf("get: %v", err)
		}

		// Must not panic.
		runtime := airtable.S(fetched.EvaluateFormulaNested())
		_ = runtime
	})
}
