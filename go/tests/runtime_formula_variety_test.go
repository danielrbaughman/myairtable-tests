package tests

// TC4 — Runtime-formula input variety. The base TestRuntimeFormulas suite only
// evaluates the kitchen-sink formulas with ONE fully-populated input set, so the
// IF(OR(...=BLANK())) short-circuit and varied inputs are never exercised. Each
// case here creates a Formulas record with a specific input set, fetches the
// API-computed value, and asserts the transpiled Evaluate*() reproduces it.
//
// Scope notes (kept deliberately portable across all 9 targets):
//   - Numbers are positive with LOG/SQRT arguments chosen to be exact (powers of
//     10 / perfect squares). The Math formula calls LOG()/SQRT()/EXP() —
//     transcendental results differ by a ULP between platforms and Airtable (V8),
//     so irrational results (e.g. LOG(5)) are NOT bit-identical and would make an
//     exact string compare flaky. Negatives/zero are excluded too: LOG(negative)
//     and MOD(_,0) error inside this formula.
//   - Text covers empty-ish edges (whitespace), unicode, and reserved punctuation
//     (exercising the fixed ENCODE_URL_COMPONENT). An all-blank text input is
//     excluded: Airtable returns blank for the whole formula (REPLACE past
//     end-of-string errors), while the transpiler is lenient.

import (
	"context"
	"testing"
	"time"

	airtable "myairtabletests/output"
)

func TestRuntimeFormulaVariety(t *testing.T) {
	at := newAirtable(t)
	ctx := context.Background()

	mustDate := func(s string) *airtable.AirtableTime {
		parsed, err := time.Parse(time.RFC3339, s)
		if err != nil {
			t.Fatalf("parse date %q: %v", s, err)
		}
		return airtable.Time(parsed)
	}

	// base returns a fresh Formulas model with the shared label/date scaffold.
	base := func(label string) *airtable.FormulasModel {
		return &airtable.FormulasModel{
			PrimaryKey: airtable.String(primaryKey("GoVariety", label)),
			FirstDate:  mustDate("2024-01-01T00:00:00.000Z"),
			SecondDate: mustDate("2024-02-01T00:00:00.000Z"),
			ThirdDate:  mustDate("2024-03-01T00:00:00.000Z"),
		}
	}

	tryDelete := func(recordID string) {
		if recordID == "" {
			return
		}
		_ = at.Formulas.DeleteOne(ctx, recordID)
	}

	// First Number: power of 10 (LOG exact). Second Number: perfect square (SQRT exact).
	numberCases := []struct {
		label                string
		first, second, third float64
	}{
		{"hundreds", 100, 16, 8},
		{"ones", 1, 4, 2},
		{"tens", 10, 25, 3},
	}

	for _, tc := range numberCases {
		t.Run("Math/"+tc.label, func(t *testing.T) {
			model := base("Math " + tc.label)
			model.FirstNumber = airtable.Float64(tc.first)
			model.SecondNumber = airtable.Float64(tc.second)
			model.ThirdNumber = airtable.Float64(tc.third)
			model.FirstText = airtable.String("x")
			model.SecondText = airtable.String("y")
			model.ThirdText = airtable.String("z")

			created, err := at.Formulas.CreateOne(ctx, model)
			if err != nil {
				t.Fatalf("create: %v", err)
			}
			recordID := created.ID()
			defer tryDelete(recordID)

			fetched, err := at.Formulas.GetOne(ctx, recordID)
			if err != nil {
				t.Fatalf("get: %v", err)
			}
			apiMath, _ := fetched.MathFormula.Value()
			runtimeMath := airtable.S(fetched.EvaluateMathFormula())
			t.Logf("%s: api=%q runtime=%q", tc.label, apiMath, runtimeMath)
			if apiMath != runtimeMath {
				t.Errorf("math formula mismatch:\n  api=%q\n  rt =%q", apiMath, runtimeMath)
			}
		})
	}

	t.Run("Math/blank", func(t *testing.T) {
		// First/Second Number left null -> OR(BLANK, BLANK) is true -> the formula
		// returns BLANK(). This is the IF-true short-circuit that the base suite
		// never reaches. Assert BOTH the API value AND the transpiled output blank.
		model := base("Blank")
		model.FirstText = airtable.String("x")
		model.SecondText = airtable.String("y")
		model.ThirdText = airtable.String("z")

		created, err := at.Formulas.CreateOne(ctx, model)
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		recordID := created.ID()
		defer tryDelete(recordID)

		fetched, err := at.Formulas.GetOne(ctx, recordID)
		if err != nil {
			t.Fatalf("get: %v", err)
		}
		apiVal := ""
		if fetched.MathFormula != nil {
			apiVal, _ = fetched.MathFormula.Value()
		}
		runtimeVal := airtable.S(fetched.EvaluateMathFormula())
		t.Logf("blank: api=%q runtime=%q", apiVal, runtimeVal)
		if apiVal != "" {
			t.Errorf("API expected blank, got %q", apiVal)
		}
		if runtimeVal != "" {
			t.Errorf("runtime expected blank, got %q", runtimeVal)
		}
	})

	textCases := []struct {
		label                string
		first, second, third string
	}{
		{"unicode", "café", "naïve", "日本語🎉"},
		{"whitespace", "  he llo  ", "a b", "c"},
		{"punct", "a.e-i+o", "x/y", "z"}, // exercises fixed ENCODE_URL_COMPONENT
	}

	for _, tc := range textCases {
		t.Run("Text/"+tc.label, func(t *testing.T) {
			model := base("Text " + tc.label)
			model.FirstNumber = airtable.Float64(10)
			model.SecondNumber = airtable.Float64(20)
			model.ThirdNumber = airtable.Float64(30)
			model.FirstText = airtable.String(tc.first)
			model.SecondText = airtable.String(tc.second)
			model.ThirdText = airtable.String(tc.third)

			created, err := at.Formulas.CreateOne(ctx, model)
			if err != nil {
				t.Fatalf("create: %v", err)
			}
			recordID := created.ID()
			defer tryDelete(recordID)

			fetched, err := at.Formulas.GetOne(ctx, recordID)
			if err != nil {
				t.Fatalf("get: %v", err)
			}
			apiText, _ := fetched.TextFormula.Value()
			runtimeText := airtable.S(fetched.EvaluateTextFormula())
			t.Logf("%s: api=%q runtime=%q", tc.label, apiText, runtimeText)
			if apiText != runtimeText {
				t.Errorf("text formula mismatch:\n  api=%q\n  rt =%q", apiText, runtimeText)
			}
		})
	}
}
