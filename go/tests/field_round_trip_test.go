package tests

// TC7 — Field-type round-trip completeness via re-fetch. Several field types were
// only asserted on the create response (or only offline-decoded), never written and
// read back through the live API, and clearing/removing multi-value fields was
// untested. Each subtest creates, optionally updates, re-fetches, and asserts the
// server-side value. Parity target mirroring csharp TestFieldRoundTrip. Live;
// distinct primary keys; best-effort cleanup.

import (
	"context"
	"testing"
	"time"

	airtable "myairtabletests/output"
)

// userID is the shared-base collaborator, as in TestComplexProperties.
const userID = "usrnZ4k98m0Ipji4e"

func TestFieldRoundTrip(t *testing.T) {
	at := newAirtable(t)
	ctx := context.Background()

	t.Run("DateWithTimeWritesAndReadsBack", func(t *testing.T) {
		pk := primaryKey("FieldRT", "DateTime")
		dt := time.Date(2024, 3, 15, 14, 30, 0, 0, time.UTC)
		created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey:   airtable.String(pk),
			DateWithTime: airtable.Time(dt),
		})
		if err != nil {
			t.Fatalf("create with date: %v", err)
		}
		defer at.Primary.DeleteOne(ctx, created.ID()) //nolint:errcheck

		fetched, err := at.Primary.GetOne(ctx, created.ID())
		if err != nil {
			t.Fatalf("get: %v", err)
		}
		if fetched.DateWithTime == nil {
			t.Fatal("fetched date nil")
		}
		if !fetched.DateWithTime.Time.Equal(dt) {
			t.Fatalf("fetched date = %v, want %v", fetched.DateWithTime.Time, dt)
		}
	})

	t.Run("RichTextAndPercentCurrencyReadBack", func(t *testing.T) {
		pk := primaryKey("FieldRT", "Rich")
		created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey:           airtable.String(pk),
			LongTextWithRichText: airtable.String("**bold** and _italic_ text"),
			PercentInt:           airtable.Float64(0.5),
			PercentFloat:         airtable.Float64(0.333),
			CurrencyInt:          airtable.Float64(100.0),
			CurrencyFloat:        airtable.Float64(19.99),
		})
		if err != nil {
			t.Fatalf("create with rich text/percent/currency: %v", err)
		}
		defer at.Primary.DeleteOne(ctx, created.ID()) //nolint:errcheck

		fetched, err := at.Primary.GetOne(ctx, created.ID())
		if err != nil {
			t.Fatalf("get: %v", err)
		}
		if fetched.LongTextWithRichText == nil || *fetched.LongTextWithRichText != "**bold** and _italic_ text" {
			t.Fatalf("rich text: %v", fetched.LongTextWithRichText)
		}
		if fetched.PercentInt == nil || *fetched.PercentInt != 0.5 {
			t.Fatalf("percent int: %v", fetched.PercentInt)
		}
		if fetched.PercentFloat == nil || *fetched.PercentFloat != 0.333 {
			t.Fatalf("percent float: %v", fetched.PercentFloat)
		}
		if fetched.CurrencyInt == nil || *fetched.CurrencyInt != 100.0 {
			t.Fatalf("currency int: %v", fetched.CurrencyInt)
		}
		if fetched.CurrencyFloat == nil || *fetched.CurrencyFloat != 19.99 {
			t.Fatalf("currency float: %v", fetched.CurrencyFloat)
		}
	})

	t.Run("ClearingSingleAndMultiSelectReadsBackEmpty", func(t *testing.T) {
		pk := primaryKey("FieldRT", "ClearSelect")
		single := airtable.PrimarySingleSelectOptionChoice1
		created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey:   airtable.String(pk),
			SingleSelect: &single,
			MultipleSelect: []airtable.PrimaryMultipleSelectOption{
				airtable.PrimaryMultipleSelectOptionOption1,
				airtable.PrimaryMultipleSelectOptionOption2,
			},
		})
		if err != nil {
			t.Fatalf("create with selects: %v", err)
		}
		defer at.Primary.DeleteOne(ctx, created.ID()) //nolint:errcheck

		if created.SingleSelect == nil || *created.SingleSelect != airtable.PrimarySingleSelectOptionChoice1 {
			t.Fatalf("created single select: %v", created.SingleSelect)
		}

		created.SingleSelect = nil
		created.MultipleSelect = []airtable.PrimaryMultipleSelectOption{}
		if _, err := at.Primary.UpdateOne(ctx, created); err != nil {
			t.Fatalf("update (clear selects): %v", err)
		}

		fetched, err := at.Primary.GetOne(ctx, created.ID())
		if err != nil {
			t.Fatalf("get: %v", err)
		}
		if fetched.SingleSelect != nil {
			t.Fatalf("single select should be cleared, got %v", *fetched.SingleSelect)
		}
		if len(fetched.MultipleSelect) != 0 {
			t.Fatalf("multi select should be empty, got %v", fetched.MultipleSelect)
		}
	})

	t.Run("RemovingACollaboratorReadsBackNull", func(t *testing.T) {
		pk := primaryKey("FieldRT", "RemoveUser")
		created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey: airtable.String(pk),
			User:       &airtable.AirtableCollaborator{ID: userID},
		})
		if err != nil {
			t.Fatalf("create with collaborator: %v", err)
		}
		defer at.Primary.DeleteOne(ctx, created.ID()) //nolint:errcheck

		if created.User == nil || created.User.ID != userID {
			t.Fatalf("created user: %v", created.User)
		}

		created.User = nil
		if _, err := at.Primary.UpdateOne(ctx, created); err != nil {
			t.Fatalf("update (clear user): %v", err)
		}

		fetched, err := at.Primary.GetOne(ctx, created.ID())
		if err != nil {
			t.Fatalf("get: %v", err)
		}
		if fetched.User != nil {
			t.Fatalf("user should be cleared, got %v", fetched.User)
		}
	})

	t.Run("AttachmentReplaceAndRemoveReadBack", func(t *testing.T) {
		const urlA = "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"
		const urlB = "https://www.w3.org/Icons/w3c_home.png"
		pk := primaryKey("FieldRT", "Attach")
		created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey: airtable.String(pk),
			Attachment: []airtable.AirtableAttachment{{URL: urlA}},
		})
		if err != nil {
			t.Fatalf("create with attachment: %v", err)
		}
		defer at.Primary.DeleteOne(ctx, created.ID()) //nolint:errcheck

		// Replace the attachment with a different one.
		created.Attachment = []airtable.AirtableAttachment{{URL: urlB}}
		if _, err := at.Primary.UpdateOne(ctx, created); err != nil {
			t.Fatalf("update (replace attachment): %v", err)
		}
		replaced, err := at.Primary.GetOne(ctx, created.ID())
		if err != nil {
			t.Fatalf("get (replaced): %v", err)
		}
		if len(replaced.Attachment) != 1 {
			t.Fatalf("replaced attachment count = %d, want 1", len(replaced.Attachment))
		}

		// Remove all attachments.
		replaced.Attachment = []airtable.AirtableAttachment{}
		if _, err := at.Primary.UpdateOne(ctx, replaced); err != nil {
			t.Fatalf("update (clear attachment): %v", err)
		}
		cleared, err := at.Primary.GetOne(ctx, created.ID())
		if err != nil {
			t.Fatalf("get (cleared): %v", err)
		}
		if len(cleared.Attachment) != 0 {
			t.Fatalf("attachment should be empty, got %v", cleared.Attachment)
		}
	})
}
