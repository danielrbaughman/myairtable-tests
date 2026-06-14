package tests

// G5.6 — Complex field-type CRUD parity with Java/Kotlin/Swift TestComplexProperties:
// attachments (URL-based create + async enrichment), computed fields (auto-number,
// formulas, created-time, created-by), the read-only button field, and a duration
// round-trip. Live; distinct primary keys; cleans up.

import (
	"context"
	"testing"
	"time"

	airtable "myairtabletests/output"
)

func TestComplexProperties(t *testing.T) {
	at := newAirtable(t)
	ctx := context.Background()

	t.Run("Attachment", func(t *testing.T) {
		pk := primaryKey("Complex", "Attach")
		// Same stable public image the Java parity test uses.
		url := "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"

		created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey: airtable.String(pk),
			Attachment: []airtable.AirtableAttachment{{URL: url}},
		})
		if err != nil {
			t.Fatalf("create with attachment: %v", err)
		}
		defer at.Primary.DeleteOne(ctx, created.ID()) //nolint:errcheck

		if len(created.Attachment) != 1 {
			t.Fatalf("created attachment count = %d, want 1", len(created.Attachment))
		}

		// Airtable processes attachments asynchronously: the fresh record may only
		// carry the URL until processing enriches it with an id/size/filename.
		// Poll generously (up to 15s) until the first attachment is enriched.
		current := created
		for i := 0; i < 15; i++ {
			if len(current.Attachment) > 0 {
				first := current.Attachment[0]
				if first.ID != "" || first.Size != nil {
					break
				}
			}
			time.Sleep(1 * time.Second)
			refetched, err := at.Primary.GetOne(ctx, created.ID())
			if err != nil {
				t.Fatalf("poll get: %v", err)
			}
			current = refetched
		}

		if len(current.Attachment) != 1 {
			t.Fatalf("enriched attachment count = %d, want 1", len(current.Attachment))
		}
		first := current.Attachment[0]
		if first.ID == "" && first.Size == nil {
			t.Fatalf("attachment was never enriched (no id/size): %+v", first)
		}
		if first.URL == "" {
			t.Fatal("enriched attachment has empty url")
		}
	})

	t.Run("ComputedFields", func(t *testing.T) {
		pk := primaryKey("Complex", "Computed")
		created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey:  airtable.String(pk),
			NumberInt:   airtable.Float64(10.0),
			NumberFloat: airtable.Float64(5.0),
		})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		defer at.Primary.DeleteOne(ctx, created.ID()) //nolint:errcheck

		// Auto Number assigned by the server.
		if created.AutoNumber == nil {
			t.Fatal("auto number nil")
		}
		if _, ok := created.AutoNumber.Value(); !ok {
			t.Fatalf("auto number has no plain value: %+v", created.AutoNumber)
		}
		// Created Time (envelope) and the dedicated computed Created-Time field.
		if created.CreatedTime() == nil {
			t.Fatal("envelope created time nil")
		}
		if created.CreatedAtTime == nil {
			t.Fatal("dedicated created-time field nil")
		}
		if _, ok := created.CreatedAtTime.Value(); !ok {
			t.Fatalf("created-time field has no value: %+v", created.CreatedAtTime)
		}
		// Formula(ID) reflects the record id.
		if created.FormulaId == nil {
			t.Fatal("formula id nil")
		}
		if v, ok := created.FormulaId.Value(); !ok || v != created.ID() {
			t.Fatalf("formula id = %q ok=%v, want %q", v, ok, created.ID())
		}
		// Formula(Simple) = numberInt + numberFloat = 15.
		if created.FormulaSimple == nil {
			t.Fatal("formula simple nil")
		}
		if v, ok := created.FormulaSimple.Value(); !ok || v != 15.0 {
			t.Fatalf("formula simple = %v ok=%v, want 15", v, ok)
		}
		// Created-by populated with the caller.
		if created.CreatedBy == nil {
			t.Fatal("created by nil")
		}
		if _, ok := created.CreatedBy.Value(); !ok {
			t.Fatalf("created by has no value: %+v", created.CreatedBy)
		}

		// Re-fetch and re-assert the stable computed values.
		fetched, err := at.Primary.GetOne(ctx, created.ID())
		if err != nil {
			t.Fatalf("get: %v", err)
		}
		if v, ok := fetched.FormulaId.Value(); !ok || v != created.ID() {
			t.Fatalf("fetched formula id = %q ok=%v", v, ok)
		}
		if v, ok := fetched.FormulaSimple.Value(); !ok || v != 15.0 {
			t.Fatalf("fetched formula simple = %v ok=%v", v, ok)
		}
		cv, _ := created.AutoNumber.Value()
		fv, fok := fetched.AutoNumber.Value()
		if !fok || fv != cv {
			t.Fatalf("auto number changed across fetch: created=%v fetched=%v", cv, fv)
		}
	})

	t.Run("Button", func(t *testing.T) {
		pk := primaryKey("Complex", "Button")
		created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{PrimaryKey: airtable.String(pk)})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		defer at.Primary.DeleteOne(ctx, created.ID()) //nolint:errcheck

		// The button field decodes (populated or absent) without error.
		fetched, err := at.Primary.GetOne(ctx, created.ID())
		if err != nil {
			t.Fatalf("get: %v", err)
		}
		if fetched.ID() != created.ID() {
			t.Fatalf("fetched id = %q, want %q", fetched.ID(), created.ID())
		}
		if fetched.Button != nil {
			// If present, it should decode either as a value or special/error,
			// none of which is an error condition for the test.
			if btn, ok := fetched.Button.Value(); ok {
				_ = btn.Label
				_ = btn.URL
			}
		}
	})

	t.Run("DurationRoundTrip", func(t *testing.T) {
		pk := primaryKey("Complex", "Duration")
		created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey: airtable.String(pk),
			Duration:   airtable.Duration(90 * time.Second),
		})
		if err != nil {
			t.Fatalf("create with duration: %v", err)
		}
		defer at.Primary.DeleteOne(ctx, created.ID()) //nolint:errcheck

		if created.Duration == nil {
			t.Fatal("created duration nil")
		}
		if got := created.Duration.Duration(); got != 90*time.Second {
			t.Fatalf("created duration = %v, want 90s", got)
		}

		fetched, err := at.Primary.GetOne(ctx, created.ID())
		if err != nil {
			t.Fatalf("get: %v", err)
		}
		if fetched.Duration == nil {
			t.Fatal("fetched duration nil")
		}
		if got := fetched.Duration.Duration(); got != 90*time.Second {
			t.Fatalf("fetched duration = %v, want 90s", got)
		}
	})
}
