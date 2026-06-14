package tests

// G4.7 — Model-side fluent CRUD (Save/Fetch/Delete) parity with the other suites.

import (
	"context"
	"testing"

	airtable "myairtabletests/output"
)

func TestOrmCrudViaModel(t *testing.T) {
	at := newAirtable(t)
	ctx := context.Background()

	t.Run("SaveInsertThenUpdate", func(t *testing.T) {
		pk := primaryKey("OrmModel", "Save")
		m := at.Primary.Attach(&airtable.PrimaryModel{PrimaryKey: airtable.String(pk)})

		// Insert via Save.
		if err := m.Save(ctx); err != nil {
			t.Fatalf("save insert: %v", err)
		}
		defer at.Primary.Delete(ctx, m.ID()) //nolint:errcheck
		if m.ID() == "" {
			t.Fatal("id not set after insert")
		}

		// Mutate + Save (update via dirty diff).
		m.Email = airtable.String("model@example.com")
		if err := m.Save(ctx); err != nil {
			t.Fatalf("save update: %v", err)
		}
		if m.Email == nil || *m.Email != "model@example.com" {
			t.Fatalf("email after update: %v", m.Email)
		}
	})

	t.Run("Fetch", func(t *testing.T) {
		pk := primaryKey("OrmModel", "Fetch")
		created, err := at.Primary.Create(ctx, &airtable.PrimaryModel{PrimaryKey: airtable.String(pk)})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		defer at.Primary.Delete(ctx, created.ID()) //nolint:errcheck

		// Mutate server-side via a second handle, then Fetch should refresh.
		other, _ := at.Primary.Get(ctx, created.ID())
		other.Email = airtable.String("fetched@example.com")
		if _, err := at.Primary.Update(ctx, other); err != nil {
			t.Fatalf("update other: %v", err)
		}

		if err := created.Fetch(ctx); err != nil {
			t.Fatalf("fetch: %v", err)
		}
		if created.Email == nil || *created.Email != "fetched@example.com" {
			t.Fatalf("fetch did not refresh: %v", created.Email)
		}
	})

	t.Run("Delete", func(t *testing.T) {
		pk := primaryKey("OrmModel", "Delete")
		created, err := at.Primary.Create(ctx, &airtable.PrimaryModel{PrimaryKey: airtable.String(pk)})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		if err := created.Delete(ctx); err != nil {
			t.Fatalf("delete: %v", err)
		}
		if created.ID() != "" {
			t.Fatal("id should be cleared after delete")
		}
	})

	t.Run("UnsavedModelError", func(t *testing.T) {
		// A model with no attached client cannot be saved.
		m := &airtable.PrimaryModel{PrimaryKey: airtable.String("unsaved")}
		if err := m.Save(ctx); err == nil {
			t.Fatal("expected error saving model with no client")
		}
	})
}
