package tests

// G6.2 — Linked records as raw []string record IDs; resolution via the linked
// table's Get. Parity with the other-language linked-records suites.

import (
	"context"
	"testing"

	airtable "myairtabletests/output"
)

func TestLinkedRecords(t *testing.T) {
	at := newAirtable(t)
	ctx := context.Background()

	t.Run("LinkSingleAndResolve", func(t *testing.T) {
		// Create a Secondary record to link to.
		sec, err := at.Secondary.CreateOne(ctx, &airtable.SecondaryModel{Name: airtable.String(primaryKey("Linked", "Sec"))})
		if err != nil {
			t.Fatalf("create secondary: %v", err)
		}
		defer at.Secondary.DeleteOne(ctx, sec.ID()) //nolint:errcheck

		// Create a Primary linking to it.
		prim, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey: airtable.String(primaryKey("Linked", "Prim")),
			LinkSingle: []string{sec.ID()},
		})
		if err != nil {
			t.Fatalf("create primary: %v", err)
		}
		defer at.Primary.DeleteOne(ctx, prim.ID()) //nolint:errcheck

		if len(prim.LinkSingle) != 1 || prim.LinkSingle[0] != sec.ID() {
			t.Fatalf("link single: %v", prim.LinkSingle)
		}

		// Resolve the link via the linked table.
		resolved, err := at.Secondary.GetOne(ctx, prim.LinkSingle[0])
		if err != nil {
			t.Fatalf("resolve link: %v", err)
		}
		if resolved.ID() != sec.ID() {
			t.Fatalf("resolved id: %s want %s", resolved.ID(), sec.ID())
		}
	})

	t.Run("LinkMultiple", func(t *testing.T) {
		s1, err := at.Secondary.CreateOne(ctx, &airtable.SecondaryModel{Name: airtable.String(primaryKey("Linked", "M1"))})
		if err != nil {
			t.Fatalf("create s1: %v", err)
		}
		defer at.Secondary.DeleteOne(ctx, s1.ID()) //nolint:errcheck
		s2, err := at.Secondary.CreateOne(ctx, &airtable.SecondaryModel{Name: airtable.String(primaryKey("Linked", "M2"))})
		if err != nil {
			t.Fatalf("create s2: %v", err)
		}
		defer at.Secondary.DeleteOne(ctx, s2.ID()) //nolint:errcheck

		prim, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey:   airtable.String(primaryKey("Linked", "PrimM")),
			LinkMultiple: []string{s1.ID(), s2.ID()},
		})
		if err != nil {
			t.Fatalf("create primary: %v", err)
		}
		defer at.Primary.DeleteOne(ctx, prim.ID()) //nolint:errcheck

		if len(prim.LinkMultiple) != 2 {
			t.Fatalf("link multiple len: %v", prim.LinkMultiple)
		}
	})

	t.Run("EmptyLinkIsNil", func(t *testing.T) {
		prim, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{PrimaryKey: airtable.String(primaryKey("Linked", "Empty"))})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		defer at.Primary.DeleteOne(ctx, prim.ID()) //nolint:errcheck
		if prim.LinkSingle != nil {
			t.Fatalf("expected nil link, got %v", prim.LinkSingle)
		}
	})
}
