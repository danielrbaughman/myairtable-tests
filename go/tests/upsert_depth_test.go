package tests

// TC10 — Upsert depth. The base CRUD suite (orm_crud_via_table_test.go) only
// covers single-record, single-merge-field upsert. This adds a multi-field
// merge key (a match must agree on ALL merge fields) and the multiple-match
// error path (a merge key matching more than one record is rejected by
// Airtable). Mirrors the C# TestUpsertDepth suite.
//
// Go's Upsert returns only (model, error) — there is no WasCreated flag — so
// insert-vs-update is inferred by comparing the returned record ID against the
// seed ID, exactly as the Upsert subtest in orm_crud_via_table_test.go does.

import (
	"context"
	"testing"

	airtable "myairtabletests/output"
)

func TestUpsertDepth(t *testing.T) {
	at := newAirtable(t)
	ctx := context.Background()

	t.Run("UpsertMatchesOnMultipleMergeFields", func(t *testing.T) {
		suite := primaryKey("Upsert", "MultiKey")
		var ids []string
		defer func() { tryDeleteMany(ctx, at, ids) }()

		// Seed a record identified by the (PrimaryKey, SingleLineText) pair.
		seed, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey:     airtable.String(suite),
			SingleLineText: airtable.String("anchor"),
		})
		if err != nil {
			t.Fatalf("seed create: %v", err)
		}
		ids = append(ids, seed.ID())

		mergeOn := []string{airtable.PrimaryPrimaryKeyFieldID, airtable.PrimarySingleLineTextFieldID}

		// Same pair -> UPDATE the seed (matched on both fields).
		update, err := at.Primary.Upsert(ctx, &airtable.PrimaryModel{
			PrimaryKey:     airtable.String(suite),
			SingleLineText: airtable.String("anchor"),
			LongText:       airtable.String("updated"),
		}, mergeOn)
		if err != nil {
			t.Fatalf("upsert update: %v", err)
		}
		if update.ID() != seed.ID() {
			t.Fatalf("expected upsert to UPDATE seed %s, got new id %s", seed.ID(), update.ID())
		}
		if update.LongText == nil || *update.LongText != "updated" {
			t.Fatalf("expected LongText \"updated\", got %v", update.LongText)
		}

		// Same PrimaryKey but a DIFFERENT SingleLineText -> no match on the pair -> INSERT.
		insert, err := at.Primary.Upsert(ctx, &airtable.PrimaryModel{
			PrimaryKey:     airtable.String(suite),
			SingleLineText: airtable.String("different"),
		}, mergeOn)
		if err != nil {
			t.Fatalf("upsert insert: %v", err)
		}
		ids = append(ids, insert.ID())
		if insert.ID() == seed.ID() {
			t.Fatalf("expected upsert to INSERT a new record, but it matched seed %s", seed.ID())
		}
	})

	t.Run("UpsertWithMultipleMatchesIsAPIError", func(t *testing.T) {
		suite := primaryKey("Upsert", "MultiMatch")
		var ids []string
		defer func() { tryDeleteMany(ctx, at, ids) }()

		// Two records share the same SingleLineText value.
		a, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey:     airtable.String(suite + " A"),
			SingleLineText: airtable.String("dupe"),
		})
		if err != nil {
			t.Fatalf("create a: %v", err)
		}
		ids = append(ids, a.ID())
		b, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey:     airtable.String(suite + " B"),
			SingleLineText: airtable.String("dupe"),
		})
		if err != nil {
			t.Fatalf("create b: %v", err)
		}
		ids = append(ids, b.ID())

		// Upsert merging only on SingleLineText="dupe" matches BOTH -> Airtable rejects it.
		upserted, err := at.Primary.Upsert(ctx, &airtable.PrimaryModel{
			SingleLineText: airtable.String("dupe"),
			LongText:       airtable.String("x"),
		}, []string{airtable.PrimarySingleLineTextFieldID})
		if err == nil {
			ids = append(ids, upserted.ID()) // unexpected success; clean it up
			t.Fatal("expected an error upserting with a merge key matching multiple records")
		}
		api := asAPIError(t, err)
		t.Logf("multiple match -> *APIError{StatusCode:%d, Type:%q}: %v", api.StatusCode, api.Type, err)
	})
}

func tryDeleteMany(ctx context.Context, at *airtable.Airtable, ids []string) {
	for _, id := range ids {
		_ = at.Primary.DeleteOne(ctx, id)
	}
}
