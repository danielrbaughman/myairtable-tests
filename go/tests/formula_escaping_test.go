package tests

// TC2 — Formula-value escaping. Filter predicates build formulas by interpolating
// user values into quoted string literals; a value containing a quote, backslash,
// or other meta-character must be escaped or the formula breaks (or silently
// mis-matches). These tests round-trip special-character values through storage AND
// through the generated .Eq()/.Contains() filter DSL against the live base, proving
// the escaping is correct. Parity with C# TestFormulaEscaping and the other suites.

import (
	"context"
	"testing"

	airtable "myairtabletests/output"
)

// specialValues break naive string interpolation into an Airtable formula literal.
// Newline/tab is covered separately (storage only) — Airtable formula string
// literals can't carry a raw newline, so it's a storage concern, not a filter one.
var specialValues = []string{
	"O'Brien single quote",
	"say \"hi\" double quote",
	"back\\slash path",
	"trailing backslash\\",
	"mixed a\"b'c\\d",
	"unicode café ☕ 日本語 🎉",
}

// scopeFind builds FIND("<suite>", {Primary Key}) so a filter is restricted to the
// records this run created (mirrors the C# Formulas.And($"FIND(...)", ...) scope).
func scopeFind(suite string) airtable.Formula {
	return airtable.NewFormula("FIND(\"" + escapeForLiteral(suite) + "\", {Primary Key})")
}

// escapeForLiteral mirrors the client's double-quoted-literal escaping (backslash
// first, then quote). The suite token is machine-generated and carries no special
// chars, but escaping it keeps the scope robust regardless.
func escapeForLiteral(value string) string {
	out := make([]rune, 0, len(value))
	for _, r := range value {
		switch r {
		case '\\':
			out = append(out, '\\', '\\')
		case '"':
			out = append(out, '\\', '"')
		default:
			out = append(out, r)
		}
	}
	return string(out)
}

func tryDelete(ctx context.Context, at *airtable.Airtable, id string) {
	if id == "" {
		return
	}
	_ = at.Primary.DeleteOne(ctx, id) // best-effort cleanup
}

func TestFormulaEscaping(t *testing.T) {
	at := newAirtable(t)
	ctx := context.Background()
	f := airtable.PrimaryF

	t.Run("EqFilterMatchesValueWithSpecialChars", func(t *testing.T) {
		for _, special := range specialValues {
			special := special
			t.Run(special, func(t *testing.T) {
				suite := primaryKey("Escaping", "Eq")
				created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
					PrimaryKey:     airtable.String(suite),
					SingleLineText: airtable.String(special),
				})
				if err != nil {
					t.Fatalf("create: %v", err)
				}
				defer tryDelete(ctx, at, created.ID())

				// 1. Storage round-trips the exact value.
				if created.SingleLineText == nil || *created.SingleLineText != special {
					t.Fatalf("storage round-trip: got %q want %q", deref(created.SingleLineText), special)
				}

				// 2. The .Eq() filter, scoped to this record, matches via
				//    correctly-escaped interpolation. Case-sensitive, no trim ->
				//    exact equality.
				filter := airtable.And(scopeFind(suite), f.SingleLineText.Eq(special, true, false))
				results, err := at.Primary.GetMany(ctx, (&airtable.Query{}).WithFilterFormula(filter))
				if err != nil {
					t.Fatalf("list (filter=%q): %v", filter.String(), err)
				}
				if !containsID(results, created.ID()) {
					t.Fatalf("Eq did not match record %s for value %q; formula=%q matched %d records",
						created.ID(), special, filter.String(), len(results))
				}
				for _, r := range results {
					if r.SingleLineText == nil || *r.SingleLineText != special {
						t.Fatalf("matched record has wrong value: got %q want %q (formula=%q)",
							deref(r.SingleLineText), special, filter.String())
					}
				}
			})
		}
	})

	t.Run("ContainsFilterMatchesValueWithSpecialChars", func(t *testing.T) {
		for _, special := range specialValues {
			special := special
			t.Run(special, func(t *testing.T) {
				suite := primaryKey("Escaping", "Contains")
				// Embed the special token inside a longer value so Contains
				// (FIND>0) is a real substring test.
				stored := "prefix " + special + " suffix"
				created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
					PrimaryKey:     airtable.String(suite),
					SingleLineText: airtable.String(stored),
				})
				if err != nil {
					t.Fatalf("create: %v", err)
				}
				defer tryDelete(ctx, at, created.ID())

				filter := airtable.And(scopeFind(suite), f.SingleLineText.Contains(special, true, false))
				results, err := at.Primary.GetMany(ctx, (&airtable.Query{}).WithFilterFormula(filter))
				if err != nil {
					t.Fatalf("list (filter=%q): %v", filter.String(), err)
				}
				if !containsID(results, created.ID()) {
					t.Fatalf("Contains did not match record %s for value %q; formula=%q matched %d records",
						created.ID(), special, filter.String(), len(results))
				}
			})
		}
	})

	t.Run("SpecialCharactersInPrimaryKeyRoundTripAndFilter", func(t *testing.T) {
		// The primary key itself carries special chars, matched with .Eq().
		suite := primaryKey("Escaping", "PK")
		pk := suite + " O'Brien \"quote\" back\\slash"
		created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{PrimaryKey: airtable.String(pk)})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		defer tryDelete(ctx, at, created.ID())

		if created.PrimaryKey == nil || *created.PrimaryKey != pk {
			t.Fatalf("storage round-trip: got %q want %q", deref(created.PrimaryKey), pk)
		}

		filter := f.PrimaryKey.Eq(pk, true, false)
		results, err := at.Primary.GetMany(ctx, (&airtable.Query{}).WithFilterFormula(filter))
		if err != nil {
			t.Fatalf("list (filter=%q): %v", filter.String(), err)
		}
		if !containsID(results, created.ID()) {
			t.Fatalf("Eq did not match PK record %s; formula=%q matched %d records",
				created.ID(), filter.String(), len(results))
		}
		if len(results) != 1 {
			t.Fatalf("expected exactly the created record, got %d (formula=%q)", len(results), filter.String())
		}
	})

	t.Run("NewlineAndTabValuesRoundTripThroughStorage", func(t *testing.T) {
		// Newlines/tabs are a storage concern (long text), not expressible in a
		// formula literal. Verify they survive create -> fetch unchanged.
		suite := primaryKey("Escaping", "Newline")
		value := "line1\nline2\twith tab\r\nwindows"
		created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey: airtable.String(suite),
			LongText:   airtable.String(value),
		})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		defer tryDelete(ctx, at, created.ID())

		fetched, err := at.Primary.GetOne(ctx, created.ID())
		if err != nil {
			t.Fatalf("get: %v", err)
		}
		if fetched.LongText == nil || *fetched.LongText != value {
			t.Fatalf("newline/tab round-trip: got %q want %q", deref(fetched.LongText), value)
		}
	})
}

func deref(s *string) string {
	if s == nil {
		return "<nil>"
	}
	return *s
}
