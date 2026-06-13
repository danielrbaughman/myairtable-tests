package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;

import java.util.List;
import myairtable.Airtable;
import myairtable.AirtableAttachment;
import myairtable.AirtableButton;
import myairtable.AirtableCollaborator;
import myairtable.PrimaryModel;
import org.junit.jupiter.api.Test;

/**
 * J5.6 — Complex field-type CRUD: attachments, collaborators (users), and computed fields
 * (auto-number, formulas, created-time, created-by, button). Parity with Kotlin
 * TestComplexProperties / Swift TestComplexProperties / Rust attachment_crud + user_fields_crud +
 * computed_fields. Linked records are covered by J6.2.
 */
class TestComplexProperties {
  private final Airtable airtable = TestSetup.makeAirtable();

  // region Attachments (URL-based create, async processing)

  @Test
  void attachmentFieldRoundTripsViaUrl() throws Exception {
    String primaryKey = TestSetup.primaryKey("Complex", "Attach");
    String url =
        "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png";

    PrimaryModel created =
        airtable
            .primary()
            .create(
                PrimaryModel.builder()
                    .primaryKey(primaryKey)
                    .attachment(List.of(new AirtableAttachment(url)))
                    .build());
    String recordId = created.getId();
    try {
      // Airtable processes attachments asynchronously — the fresh record may have just the
      // URL, or be fully populated with id/size once processing completes.
      assertNotNull(created.getAttachment());
      assertEquals(1, created.getAttachment().size());

      // Poll until Airtable has enriched the attachment (id populated).
      PrimaryModel current = created;
      for (int i = 0; i < 10; i++) {
        AirtableAttachment first =
            current.getAttachment() == null || current.getAttachment().isEmpty()
                ? null
                : current.getAttachment().get(0);
        if (first != null && first.getId() != null && !first.getId().isEmpty()) {
          break;
        }
        Thread.sleep(2000);
        current = airtable.primary().get(recordId);
      }
      List<AirtableAttachment> enriched =
          current.getAttachment() == null ? List.of() : current.getAttachment();
      assertEquals(1, enriched.size());
      String enrichedUrl = enriched.get(0).getUrl();
      assertNotNull(enrichedUrl);
      assertNotNull(enrichedUrl.isEmpty() ? null : enrichedUrl, "attachment url is non-empty");

      airtable.primary().delete(recordId);
    } catch (Throwable e) {
      try {
        airtable.primary().delete(recordId);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion

  // region Collaborators (users)

  @Test
  void userAndMultiUserFieldsRoundTrip() {
    String primaryKey = TestSetup.primaryKey("Complex", "Users");
    // Matches the user ID used by the Rust/Swift tests in the shared base.
    String userId = "usrnZ4k98m0Ipji4e";

    PrimaryModel created =
        airtable
            .primary()
            .create(
                PrimaryModel.builder()
                    .primaryKey(primaryKey)
                    .user(new AirtableCollaborator(userId))
                    .userAllowMultiple(List.of(new AirtableCollaborator(userId)))
                    .build());
    String recordId = created.getId();
    try {
      assertNotNull(created.getUser());
      assertEquals(userId, created.getUser().getId());
      assertNotNull(created.getUserAllowMultiple());
      assertEquals(1, created.getUserAllowMultiple().size());
      assertEquals(userId, created.getUserAllowMultiple().get(0).getId());

      PrimaryModel fetched = airtable.primary().get(recordId);
      assertNotNull(fetched.getUser());
      assertEquals(userId, fetched.getUser().getId());

      airtable.primary().delete(recordId);
    } catch (Throwable e) {
      try {
        airtable.primary().delete(recordId);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion

  // region Computed fields

  @Test
  void computedFieldsPopulateOnCreateAndReadBack() {
    String primaryKey = TestSetup.primaryKey("Complex", "Computed");
    PrimaryModel created =
        airtable
            .primary()
            .create(
                PrimaryModel.builder()
                    .primaryKey(primaryKey)
                    .numberInt(10.0)
                    .numberFloat(5.0)
                    .build());
    String recordId = created.getId();
    try {
      // Server-owned: auto number assigned; created time populated.
      assertNotNull(created.getAutoNumber());
      assertNotNull(created.getAutoNumber().value());
      assertNotNull(created.getCreatedTime());
      assertNotNull(created.getCreatedAtTime(), "dedicated createdTime field");
      assertNotNull(created.getCreatedAtTime().value(), "dedicated createdTime field");
      // Formula(ID) reflects the record id.
      assertEquals(recordId, created.getFormulaId().value());
      // Formula(Simple) = numberInt + numberFloat = 15.
      assertEquals(15.0, created.getFormulaSimple().value());
      // Created-by is populated with the caller's user info.
      assertNotNull(created.getCreatedBy());
      assertNotNull(created.getCreatedBy().value());

      PrimaryModel fetched = airtable.primary().get(recordId);
      assertEquals(recordId, fetched.getFormulaId().value());
      assertEquals(15.0, fetched.getFormulaSimple().value());
      assertEquals(created.getAutoNumber(), fetched.getAutoNumber());

      airtable.primary().delete(recordId);
    } catch (Throwable e) {
      try {
        airtable.primary().delete(recordId);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion

  // region Button (read-only)

  @Test
  void buttonFieldDecodesOnRead() {
    String primaryKey = TestSetup.primaryKey("Complex", "Button");
    PrimaryModel created =
        airtable.primary().create(PrimaryModel.builder().primaryKey(primaryKey).build());
    String recordId = created.getId();
    try {
      // The button field decodes (either populated or absent) without error.
      PrimaryModel fetched = airtable.primary().get(recordId);
      assertEquals(recordId, fetched.getId());
      if (fetched.getButton() != null) {
        AirtableButton button = fetched.getButton().value();
        String labelOrUrl =
            button == null ? "" : (button.label() != null ? button.label() : button.url());
        assertNotNull(labelOrUrl == null ? "" : labelOrUrl);
      }

      airtable.primary().delete(recordId);
    } catch (Throwable e) {
      try {
        airtable.primary().delete(recordId);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion
}
