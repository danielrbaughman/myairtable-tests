package tests

// TC11 — Multi-field sort + sort combined with a filter. The base filter suite
// only covers a single-field sort. This verifies a two-field sort (primary key
// with ties broken by a secondary key) and sorting within a filtered scope.
// Parity with the C# TestMultiFieldSort suite.

import (
	"context"
	"testing"

	airtable "myairtabletests/output"
)

// mfsRow builds a Primary dict Fields container for a sort-test row.
func mfsRow(t *testing.T, suite string, number float64, text string) *airtable.Fields {
	t.Helper()
	f := airtable.NewFields(nil, airtable.PrimaryNameToID)
	if err := f.Set(airtable.PrimaryPrimaryKeyFieldID, suite+" "+text); err != nil {
		t.Fatalf("set primary key: %v", err)
	}
	if err := f.Set(airtable.PrimaryNumberIntFieldID, number); err != nil {
		t.Fatalf("set number: %v", err)
	}
	if err := f.Set(airtable.PrimarySingleLineTextFieldID, text); err != nil {
		t.Fatalf("set text: %v", err)
	}
	return f
}

// mfsTexts reads the SingleLineText field from each record in order.
func mfsTexts(t *testing.T, recs []*airtable.DictRecord) []string {
	t.Helper()
	out := make([]string, 0, len(recs))
	for _, r := range recs {
		var s string
		if err := r.Fields.Get(airtable.PrimarySingleLineTextFieldID, &s); err != nil {
			t.Fatalf("get text: %v", err)
		}
		out = append(out, s)
	}
	return out
}

func TestMultiFieldSort(t *testing.T) {
	at := newAirtable(t)
	ctx := context.Background()

	t.Run("TwoFieldSortBreaksTiesOnSecondKey", func(t *testing.T) {
		suite := primaryKey("Sort", "TwoField")
		// NumberInt ties at 10 (rows "b" and "a"); the secondary SingleLineText sort orders them.
		created, err := at.PrimaryDict.CreateMany(ctx, []*airtable.Fields{
			mfsRow(t, suite, 10, "b"),
			mfsRow(t, suite, 10, "a"),
			mfsRow(t, suite, 20, "c"),
		})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		ids := dictIDs(created)
		defer at.PrimaryDict.DeleteMany(ctx, ids) //nolint:errcheck

		results, err := at.PrimaryDict.GetMany(ctx, (&airtable.Query{}).
			WithFilterFormula(scopeTo(ids)).
			WithSort(airtable.PrimaryNumberIntFieldID, airtable.SortAsc).
			WithSort(airtable.PrimarySingleLineTextFieldID, airtable.SortAsc))
		if err != nil {
			t.Fatalf("list: %v", err)
		}
		// (10,a), (10,b), (20,c) — tie on 10 broken by text asc.
		if got := mfsTexts(t, results); !equalStrings(got, []string{"a", "b", "c"}) {
			t.Errorf("got %v, want [a b c]", got)
		}
	})

	t.Run("SecondaryDescendingReversesTiedGroup", func(t *testing.T) {
		suite := primaryKey("Sort", "MixedDir")
		created, err := at.PrimaryDict.CreateMany(ctx, []*airtable.Fields{
			mfsRow(t, suite, 10, "a"),
			mfsRow(t, suite, 10, "b"),
			mfsRow(t, suite, 20, "c"),
		})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		ids := dictIDs(created)
		defer at.PrimaryDict.DeleteMany(ctx, ids) //nolint:errcheck

		results, err := at.PrimaryDict.GetMany(ctx, (&airtable.Query{}).
			WithFilterFormula(scopeTo(ids)).
			WithSort(airtable.PrimaryNumberIntFieldID, airtable.SortAsc).
			WithSort(airtable.PrimarySingleLineTextFieldID, airtable.SortDesc))
		if err != nil {
			t.Fatalf("list: %v", err)
		}
		// NumberInt asc, then text DESC within the 10-tie: (10,b), (10,a), (20,c).
		if got := mfsTexts(t, results); !equalStrings(got, []string{"b", "a", "c"}) {
			t.Errorf("got %v, want [b a c]", got)
		}
	})

	t.Run("SortCombinedWithAFilter", func(t *testing.T) {
		suite := primaryKey("Sort", "WithFilter")
		created, err := at.PrimaryDict.CreateMany(ctx, []*airtable.Fields{
			mfsRow(t, suite, 30, "x"),
			mfsRow(t, suite, 10, "y"),
			mfsRow(t, suite, 20, "z"),
			mfsRow(t, suite, 5, "low"), // filtered out by NumberInt > 5
		})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		ids := dictIDs(created)
		defer at.PrimaryDict.DeleteMany(ctx, ids) //nolint:errcheck

		filter := airtable.And(scopeTo(ids), airtable.PrimaryF.NumberInt.GreaterThan(5))
		results, err := at.PrimaryDict.GetMany(ctx, (&airtable.Query{}).
			WithFilterFormula(filter).
			WithSort(airtable.PrimaryNumberIntFieldID, airtable.SortAsc))
		if err != nil {
			t.Fatalf("list: %v", err)
		}
		// Filtered to NumberInt > 5, sorted asc: 10(y), 20(z), 30(x).
		if got := mfsTexts(t, results); !equalStrings(got, []string{"y", "z", "x"}) {
			t.Errorf("got %v, want [y z x]", got)
		}
	})
}
