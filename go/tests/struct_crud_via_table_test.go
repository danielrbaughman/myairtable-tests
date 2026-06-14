package tests

// G3.7 — Dict-table (raw Fields) CRUD parity with rust test_struct_crud_via_table
// and Java/Kotlin/Swift TestStructCrudViaTable. Live test against the real base;
// each subtest uses a distinct primary key and cleans up the record it creates.

import (
	"context"
	"testing"

	airtable "myairtabletests/output"
)

func newAirtable(t *testing.T) *airtable.Airtable {
	t.Helper()
	requireCreds(t)
	return airtable.NewWithBase(apiKey(), baseID(), 0)
}

func TestStructCrudViaTable(t *testing.T) {
	at := newAirtable(t)
	ctx := context.Background()

	t.Run("PrimaryKeyOnlyCrudRoundTrip", func(t *testing.T) {
		pk := primaryKey("StructCrud", "PKOnly")
		fields := airtable.NewFields(nil, airtable.PrimaryNameToID)
		_ = fields.Set(airtable.PrimaryPrimaryKeyFieldID, pk)

		created, err := at.PrimaryDict.Create(ctx, fields)
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		recordID := created.ID
		defer at.PrimaryDict.Delete(ctx, recordID) //nolint:errcheck // best-effort cleanup

		if recordID == "" {
			t.Fatal("created record has empty id")
		}
		if got := getString(t, created, airtable.PrimaryPrimaryKeyFieldID); got != pk {
			t.Fatalf("created primary key: got %q want %q", got, pk)
		}

		// Read
		fetched, err := at.PrimaryDict.Get(ctx, recordID)
		if err != nil {
			t.Fatalf("get: %v", err)
		}
		if fetched.ID != recordID {
			t.Fatalf("fetched id: got %q want %q", fetched.ID, recordID)
		}
		if got := getString(t, fetched, airtable.PrimaryPrimaryKeyFieldID); got != pk {
			t.Fatalf("fetched primary key: got %q want %q", got, pk)
		}

		// Update
		update := airtable.NewFields(nil, airtable.PrimaryNameToID)
		updatedKey := pk + " Updated"
		_ = update.Set(airtable.PrimaryPrimaryKeyFieldID, updatedKey)
		updated, err := at.PrimaryDict.Update(ctx, recordID, update)
		if err != nil {
			t.Fatalf("update: %v", err)
		}
		if got := getString(t, updated, airtable.PrimaryPrimaryKeyFieldID); got != updatedKey {
			t.Fatalf("updated primary key: got %q want %q", got, updatedKey)
		}

		// Delete + verify gone
		if err := at.PrimaryDict.Delete(ctx, recordID); err != nil {
			t.Fatalf("delete: %v", err)
		}
		// Airtable returns 404 or 403 for a deleted record depending on token
		// scope; assert only that the record is gone (an error is returned),
		// matching the other-language suites.
		if _, err := at.PrimaryDict.Get(ctx, recordID); err == nil {
			t.Fatal("expected an error after delete, got nil")
		}
	})

	t.Run("DualIDNameFieldAccess", func(t *testing.T) {
		pk := primaryKey("StructCrud", "DualAccess")
		fields := airtable.NewFields(nil, airtable.PrimaryNameToID)
		// Set by ID…
		_ = fields.Set(airtable.PrimaryPrimaryKeyFieldID, pk)

		created, err := at.PrimaryDict.Create(ctx, fields)
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		defer at.PrimaryDict.Delete(ctx, created.ID) //nolint:errcheck

		// …read back by NAME.
		if got := getString(t, created, airtable.PrimaryPrimaryKeyFieldName); got != pk {
			t.Fatalf("read-by-name: got %q want %q", got, pk)
		}
		// …and by ID.
		if got := getString(t, created, airtable.PrimaryPrimaryKeyFieldID); got != pk {
			t.Fatalf("read-by-id: got %q want %q", got, pk)
		}
	})

	t.Run("AllSimpleProperties", func(t *testing.T) {
		pk := primaryKey("StructCrud", "Simple")
		fields := airtable.NewFields(nil, airtable.PrimaryNameToID)
		_ = fields.Set(airtable.PrimaryPrimaryKeyFieldID, pk)
		_ = fields.Set(airtable.PrimaryEmailFieldID, "go@example.com")
		_ = fields.Set(airtable.PrimaryNumberIntFieldID, 42.0)
		_ = fields.Set(airtable.PrimaryCheckboxFieldID, true)
		_ = fields.Set(airtable.PrimaryLongTextFieldID, "hello\nworld")

		created, err := at.PrimaryDict.Create(ctx, fields)
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		defer at.PrimaryDict.Delete(ctx, created.ID) //nolint:errcheck

		if got := getString(t, created, airtable.PrimaryEmailFieldID); got != "go@example.com" {
			t.Fatalf("email: got %q", got)
		}
		var num float64
		if err := created.Fields.Get(airtable.PrimaryNumberIntFieldID, &num); err != nil || num != 42.0 {
			t.Fatalf("number: got %v err %v", num, err)
		}
		var checked bool
		if err := created.Fields.Get(airtable.PrimaryCheckboxFieldID, &checked); err != nil || !checked {
			t.Fatalf("checkbox: got %v err %v", checked, err)
		}
	})
}

func getString(t *testing.T, rec *airtable.DictRecord, key string) string {
	t.Helper()
	var s string
	if err := rec.Fields.Get(key, &s); err != nil {
		t.Fatalf("get %s: %v", key, err)
	}
	return s
}
