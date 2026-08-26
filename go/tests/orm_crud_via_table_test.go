package tests

// G4.7 — Typed ORM table CRUD parity with rust test_orm_crud_via_table and the
// Java/Kotlin/Swift suites. Live; distinct primary keys; cleans up.

import (
	"context"
	"testing"

	airtable "myairtabletests/output"
)

func TestOrmCrudViaTable(t *testing.T) {
	at := newAirtable(t)
	ctx := context.Background()

	t.Run("PrimaryKeyOnly", func(t *testing.T) {
		pk := primaryKey("OrmCrud", "PKOnly")
		created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{PrimaryKey: airtable.String(pk)})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		defer at.Primary.DeleteOne(ctx, created.ID()) //nolint:errcheck
		if created.ID() == "" {
			t.Fatal("created has empty id")
		}
		if created.PrimaryKey == nil || *created.PrimaryKey != pk {
			t.Fatalf("created pk: %v", created.PrimaryKey)
		}

		fetched, err := at.Primary.GetOne(ctx, created.ID())
		if err != nil {
			t.Fatalf("get: %v", err)
		}
		if fetched.PrimaryKey == nil || *fetched.PrimaryKey != pk {
			t.Fatalf("fetched pk: %v", fetched.PrimaryKey)
		}

		fetched.PrimaryKey = airtable.String(pk + " Updated")
		updated, err := at.Primary.UpdateOne(ctx, fetched)
		if err != nil {
			t.Fatalf("update: %v", err)
		}
		if updated.PrimaryKey == nil || *updated.PrimaryKey != pk+" Updated" {
			t.Fatalf("updated pk: %v", updated.PrimaryKey)
		}

		if err := at.Primary.DeleteOne(ctx, created.ID()); err != nil {
			t.Fatalf("delete: %v", err)
		}
		if _, err := at.Primary.GetOne(ctx, created.ID()); err == nil {
			t.Fatal("expected error after delete")
		}
	})

	t.Run("AllSimpleProperties", func(t *testing.T) {
		pk := primaryKey("OrmCrud", "Simple")
		m := &airtable.PrimaryModel{
			PrimaryKey: airtable.String(pk),
			Email:      airtable.String("orm@example.com"),
			NumberInt:  airtable.Float64(42.0),
			Checkbox:   airtable.Bool(true),
		}
		created, err := at.Primary.CreateOne(ctx, m)
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		defer at.Primary.DeleteOne(ctx, created.ID()) //nolint:errcheck
		if created.Email == nil || *created.Email != "orm@example.com" {
			t.Fatalf("email: %v", created.Email)
		}
		if created.NumberInt == nil || *created.NumberInt != 42.0 {
			t.Fatalf("number: %v", created.NumberInt)
		}
		if created.Checkbox == nil || !*created.Checkbox {
			t.Fatalf("checkbox: %v", created.Checkbox)
		}
		// Auto Number is computed — decoded as MaybeSpecialOrError.
		if created.AutoNumber != nil {
			if _, ok := created.AutoNumber.Value(); !ok {
				t.Logf("auto number not a plain value (special/error): %+v", created.AutoNumber)
			}
		}
	})

	t.Run("BatchCreateAndDelete", func(t *testing.T) {
		// 11 records crosses the chunk-of-10 boundary.
		models := make([]*airtable.PrimaryModel, 11)
		for i := range models {
			models[i] = &airtable.PrimaryModel{PrimaryKey: airtable.String(primaryKey("OrmCrud", "Batch"))}
		}
		created, err := at.Primary.CreateMany(ctx, models)
		if err != nil {
			t.Fatalf("batch create: %v", err)
		}
		ids := make([]string, 0, len(created))
		for _, c := range created {
			ids = append(ids, c.ID())
		}
		defer at.Primary.Dict().DeleteMany(ctx, ids) //nolint:errcheck
		if len(created) != 11 {
			t.Fatalf("expected 11 created, got %d", len(created))
		}
	})

	t.Run("FieldSelectionAndMaxRecords", func(t *testing.T) {
		pk := primaryKey("OrmCrud", "Query")
		created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{PrimaryKey: airtable.String(pk), Email: airtable.String("q@example.com")})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		defer at.Primary.DeleteOne(ctx, created.ID()) //nolint:errcheck

		q := (&airtable.Query{}).WithFilter("{Primary Key} = \"" + pk + "\"").WithFields(airtable.PrimaryPrimaryKeyFieldID).WithMaxRecords(1)
		rows, err := at.Primary.GetMany(ctx, q)
		if err != nil {
			t.Fatalf("list: %v", err)
		}
		if len(rows) != 1 {
			t.Fatalf("expected 1 row, got %d", len(rows))
		}
		if rows[0].PrimaryKey == nil || *rows[0].PrimaryKey != pk {
			t.Fatalf("row pk: %v", rows[0].PrimaryKey)
		}
		// Email not requested -> should be nil.
		if rows[0].Email != nil {
			t.Fatalf("email should be unselected, got %v", *rows[0].Email)
		}
	})

	t.Run("InvalidIDError", func(t *testing.T) {
		if _, err := at.Primary.GetOne(ctx, "recDoesNotExist0000"); err == nil {
			t.Fatal("expected error for invalid id")
		}
	})

	t.Run("Upsert", func(t *testing.T) {
		pk := primaryKey("OrmCrud", "Upsert")
		// Insert on no match.
		m := &airtable.PrimaryModel{PrimaryKey: airtable.String(pk), Email: airtable.String("v1@example.com")}
		ins, err := at.Primary.Upsert(ctx, m, []string{airtable.PrimaryPrimaryKeyFieldID})
		if err != nil {
			t.Fatalf("upsert insert: %v", err)
		}
		defer at.Primary.DeleteOne(ctx, ins.ID()) //nolint:errcheck

		// Update on match (same primary key).
		m2 := &airtable.PrimaryModel{PrimaryKey: airtable.String(pk), Email: airtable.String("v2@example.com")}
		upd, err := at.Primary.Upsert(ctx, m2, []string{airtable.PrimaryPrimaryKeyFieldID})
		if err != nil {
			t.Fatalf("upsert update: %v", err)
		}
		if upd.ID() != ins.ID() {
			t.Fatalf("upsert should match existing record: %s != %s", upd.ID(), ins.ID())
		}
		if upd.Email == nil || *upd.Email != "v2@example.com" {
			t.Fatalf("upsert email: %v", upd.Email)
		}
	})
}

// Duplicate — parity with the rust/python/typescript duplicate suites.
func TestDuplicateViaTable(t *testing.T) {
	at := newAirtable(t)
	ctx := context.Background()

	t.Run("CopiesWritableFieldsAndRecomputesTheRest", func(t *testing.T) {
		pk := primaryKey("Duplicate", "Source")
		source, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey:     airtable.String(pk),
			SingleLineText: airtable.String("copy me"),
			// Stays <= 10 and != 20 on purpose: filter-by-formula asserts exact counts for
			// `numberInt = 20` and `AND(numberInt > 10, checkbox = true)` on the shared base.
			NumberInt: airtable.Float64(7),
			Rating:    airtable.Int64(3),
			Checkbox:  airtable.Bool(true),
		})
		if err != nil {
			t.Fatalf("create source: %v", err)
		}
		sourceID := source.ID()

		copied, err := at.Primary.DuplicateOne(ctx, source)
		if err != nil {
			t.Fatalf("duplicate: %v", err)
		}
		defer func() { _ = at.Primary.DeleteOne(ctx, copied.ID()) }()
		defer func() { _ = at.Primary.DeleteOne(ctx, sourceID) }()

		if copied.ID() == sourceID {
			t.Fatal("duplicate returned the source record")
		}
		// CreateOne hydrates the model it is given, so duplicating through it would have
		// overwritten the source's ID with the copy's. DuplicateOne must not.
		if source.ID() != sourceID {
			t.Errorf("source model was mutated: %s -> %s", sourceID, source.ID())
		}
		if got := *copied.PrimaryKey; got != pk {
			t.Errorf("primary key: want %q, got %q", pk, got)
		}
		if got := *copied.SingleLineText; got != "copy me" {
			t.Errorf("single line text: got %q", got)
		}
		if got := *copied.Rating; got != 3 {
			t.Errorf("rating: got %d", got)
		}
		// Formula (ID) resolves to RECORD_ID(), so on a true copy it is the COPY's id --
		// proof the computed fields were recalculated rather than copied.
		if v, ok := copied.FormulaId.Value(); !ok || v != copied.ID() {
			t.Errorf("formula id: want %q, got %q (ok=%v)", copied.ID(), v, ok)
		}
		copiedNum, copiedOK := copied.AutoNumber.Value()
		sourceNum, sourceOK := source.AutoNumber.Value()
		if copiedOK && sourceOK && copiedNum == sourceNum {
			t.Error("auto number was copied instead of recomputed")
		}
	})

	t.Run("ByIDsPreservesInputOrder", func(t *testing.T) {
		aPK := primaryKey("Duplicate", "OrderA")
		bPK := primaryKey("Duplicate", "OrderB")
		a, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{PrimaryKey: airtable.String(aPK)})
		if err != nil {
			t.Fatalf("create a: %v", err)
		}
		b, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{PrimaryKey: airtable.String(bPK)})
		if err != nil {
			t.Fatalf("create b: %v", err)
		}
		defer func() { _ = at.Primary.DeleteOne(ctx, a.ID()) }()
		defer func() { _ = at.Primary.DeleteOne(ctx, b.ID()) }()

		// The batched read is a RECORD_ID() OR-list, which comes back in table order.
		copies, err := at.Primary.DuplicateManyByIDs(ctx, []string{b.ID(), a.ID()})
		if err != nil {
			t.Fatalf("duplicate many: %v", err)
		}
		for _, c := range copies {
			defer func(id string) { _ = at.Primary.DeleteOne(ctx, id) }(c.ID())
		}
		if len(copies) != 2 {
			t.Fatalf("want 2 copies, got %d", len(copies))
		}
		if *copies[0].PrimaryKey != bPK || *copies[1].PrimaryKey != aPK {
			t.Errorf("input order not preserved: %q, %q", *copies[0].PrimaryKey, *copies[1].PrimaryKey)
		}
	})

	t.Run("MissingSourceIsReported", func(t *testing.T) {
		_, err := at.Primary.DuplicateManyByIDs(ctx, []string{"recDoesNotExist9"})
		if err == nil {
			t.Fatal("want an error for a missing source id")
		}
	})
}
