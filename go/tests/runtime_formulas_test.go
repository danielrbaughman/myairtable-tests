package tests

// G8.12 — Runtime formula parity: create a Formulas record with all input fields,
// fetch it back with API-computed formula values, and verify the transpiled
// Evaluate*() methods reproduce them. Parity with the Java/Kotlin/Swift
// TestRuntimeFormulas suites; time-dependent parts (TODAY/TONOW/FROMNOW) are
// skipped, exactly as those do.

import (
	"context"
	"strings"
	"testing"
	"time"

	airtable "myairtabletests/output"
)

func TestRuntimeFormulas(t *testing.T) {
	at := newAirtable(t)
	ctx := context.Background()

	mustDate := func(s string) *airtable.AirtableTime {
		parsed, err := time.Parse(time.RFC3339, s)
		if err != nil {
			t.Fatalf("parse date %q: %v", s, err)
		}
		return airtable.Time(parsed)
	}

	t.Run("EvaluateMatchesAPI", func(t *testing.T) {
		fresh := &airtable.FormulasModel{
			FirstDate:    mustDate("2024-01-01T00:00:00.000Z"),
			FirstNumber:  airtable.Float64(10),
			FirstText:    airtable.String("Hello"),
			PrimaryKey:   airtable.String(primaryKey("GoRuntime", "Formulas")),
			SecondDate:   mustDate("2024-02-01T00:00:00.000Z"),
			SecondNumber: airtable.Float64(20),
			SecondText:   airtable.String("World"),
			ThirdDate:    mustDate("2024-03-01T00:00:00.000Z"),
			ThirdNumber:  airtable.Float64(30),
			ThirdText:    airtable.String("!"),
		}
		created, err := at.Formulas.CreateOne(ctx, fresh)
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		recordID := created.ID()
		if recordID == "" {
			t.Fatal("missing id on created model")
		}
		defer at.Formulas.DeleteOne(ctx, recordID) //nolint:errcheck

		// Re-fetch to get formula values computed by Airtable.
		model, err := at.Formulas.GetOne(ctx, recordID)
		if err != nil {
			t.Fatalf("get: %v", err)
		}

		// --- Math formula: exact match ---
		apiMath, _ := model.MathFormula.Value()
		runtimeMath := airtable.S(model.EvaluateMathFormula())
		if apiMath != runtimeMath {
			t.Errorf("math formula mismatch:\n  api=%q\n  rt =%q", apiMath, runtimeMath)
		}

		// --- Text formula: exact match ---
		apiText, _ := model.TextFormula.Value()
		runtimeText := airtable.S(model.EvaluateTextFormula())
		if apiText != runtimeText {
			t.Errorf("text formula mismatch:\n  api=%q\n  rt =%q", apiText, runtimeText)
		}

		// --- Date formula: deterministic parts match (TODAY/TONOW/FROMNOW vary) ---
		apiDate, _ := model.DateFormula.Value()
		runtimeDate := airtable.S(model.EvaluateDateFormula())
		for _, want := range []string{
			"YEAR: 2024",
			"MONTH: 1",
			"DAY: 1",
			"HOUR: 0",
			"MINUTE: 0",
			"SECOND: 0",
			"WORKDAY_DIFF: ",
			"DATEADD+2d: 2024-01-03",
			"IS_BEFORE: Yes",
			"IS_SAME day: No",
			"IS_AFTER: Yes",
			"DATESTR: ",
			"TIMESTR: ",
		} {
			if !strings.Contains(runtimeDate, want) {
				t.Errorf("date formula missing %q; got: %s", want, runtimeDate)
			}
		}

		// Compare the deterministic prefix of API vs runtime (before TODAY:).
		apiPrefix := strings.SplitN(apiDate, ", TODAY:", 2)[0]
		runtimePrefix := strings.SplitN(runtimeDate, ", TODAY:", 2)[0]
		if apiPrefix != runtimePrefix {
			t.Errorf("date formula pre-TODAY mismatch:\n  api=%q\n  rt =%q", apiPrefix, runtimePrefix)
		}
	})

	t.Run("EvaluateRunsOffline", func(t *testing.T) {
		// Offline smoke check: a model built directly still evaluates every
		// generated method without panicking.
		model := &airtable.FormulasModel{
			FirstDate:    mustDate("2024-01-01T00:00:00.000Z"),
			FirstNumber:  airtable.Float64(10),
			FirstText:    airtable.String("Hello"),
			PrimaryKey:   airtable.String("RuntimeTest"),
			SecondDate:   mustDate("2024-02-01T00:00:00.000Z"),
			SecondNumber: airtable.Float64(20),
			SecondText:   airtable.String("World"),
			ThirdDate:    mustDate("2024-03-01T00:00:00.000Z"),
			ThirdNumber:  airtable.Float64(30),
			ThirdText:    airtable.String("!"),
		}
		if model.EvaluateMathFormula() == nil {
			t.Fatal("math formula evaluated to nil")
		}
		text := airtable.S(model.EvaluateTextFormula())
		if !strings.Contains(text, "Hello") || !strings.Contains(text, "World") {
			t.Errorf("text formula missing inputs: %s", text)
		}
		date := airtable.S(model.EvaluateDateFormula())
		if !strings.Contains(date, "YEAR: 2024") {
			t.Errorf("date formula missing YEAR: %s", date)
		}
	})
}
