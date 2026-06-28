package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.time.Instant;
import java.util.List;
import myairtable.Airtable;
import myairtable.AirtableAttachment;
import myairtable.AirtableCollaborator;
import myairtable.PrimaryModel;
import myairtable.PrimaryMultipleSelectOption;
import myairtable.PrimarySingleSelectOption;
import org.junit.jupiter.api.Test;

/**
 * TC7 — Field-type round-trip completeness via re-fetch. Several field types were only asserted on
 * the create response (or only offline-decoded), never written and read back through the live API,
 * and clearing/removing multi-value fields was untested. Each case here creates, optionally
 * updates, re-fetches, and asserts the server-side value. Parity with C# TestFieldRoundTrip.
 */
class TestFieldRoundTrip {
  private final Airtable airtable = TestSetup.makeAirtable();

  // shared-base user, as in TestComplexProperties
  private static final String USER_ID = "usrnZ4k98m0Ipji4e";

  @Test
  void dateWithTimeWritesAndReadsBack() {
    String suite = TestSetup.primaryKey("FieldRT", "DateTime");
    Instant dt = Instant.parse("2024-03-15T14:30:00Z");
    PrimaryModel created =
        airtable
            .primary()
            .create(PrimaryModel.builder().primaryKey(suite).dateWithTime(dt).build());
    String recordId = created.getId();
    try {
      PrimaryModel fetched = airtable.primary().get(recordId);
      assertEquals(dt, fetched.getDateWithTime());
      airtable.primary().delete(recordId);
    } catch (Throwable e) {
      tryDelete(recordId);
      throw e;
    }
  }

  @Test
  void richTextAndPercentCurrencyReadBack() {
    String suite = TestSetup.primaryKey("FieldRT", "Rich");
    PrimaryModel created =
        airtable
            .primary()
            .create(
                PrimaryModel.builder()
                    .primaryKey(suite)
                    .longTextWithRichText("**bold** and _italic_ text")
                    .percentInt(0.5)
                    .percentFloat(0.333)
                    .currencyInt(100.0)
                    .currencyFloat(19.99)
                    .build());
    String recordId = created.getId();
    try {
      PrimaryModel fetched = airtable.primary().get(recordId);
      assertEquals("**bold** and _italic_ text", fetched.getLongTextWithRichText());
      assertEquals(0.5, fetched.getPercentInt());
      assertEquals(0.333, fetched.getPercentFloat());
      assertEquals(100.0, fetched.getCurrencyInt());
      assertEquals(19.99, fetched.getCurrencyFloat());
      airtable.primary().delete(recordId);
    } catch (Throwable e) {
      tryDelete(recordId);
      throw e;
    }
  }

  @Test
  void clearingSingleAndMultiSelectReadsBackEmpty() {
    String suite = TestSetup.primaryKey("FieldRT", "ClearSelect");
    PrimaryModel created =
        airtable
            .primary()
            .create(
                PrimaryModel.builder()
                    .primaryKey(suite)
                    .singleSelect(PrimarySingleSelectOption.CHOICE_1)
                    .multipleSelect(
                        List.of(
                            PrimaryMultipleSelectOption.OPTION_1,
                            PrimaryMultipleSelectOption.OPTION_2))
                    .build());
    String recordId = created.getId();
    try {
      assertEquals(PrimarySingleSelectOption.CHOICE_1, created.getSingleSelect());

      created.setSingleSelect(null);
      created.setMultipleSelect(List.of());
      airtable.primary().update(created);

      PrimaryModel fetched = airtable.primary().get(recordId);
      assertNull(fetched.getSingleSelect());
      assertTrue(
          fetched.getMultipleSelect() == null || fetched.getMultipleSelect().isEmpty(),
          "multiple select cleared");
      airtable.primary().delete(recordId);
    } catch (Throwable e) {
      tryDelete(recordId);
      throw e;
    }
  }

  @Test
  void removingACollaboratorReadsBackNull() {
    String suite = TestSetup.primaryKey("FieldRT", "RemoveUser");
    PrimaryModel created =
        airtable
            .primary()
            .create(
                PrimaryModel.builder()
                    .primaryKey(suite)
                    .user(new AirtableCollaborator(USER_ID))
                    .build());
    String recordId = created.getId();
    try {
      assertNotNull(created.getUser());
      assertEquals(USER_ID, created.getUser().getId());

      created.setUser(null);
      airtable.primary().update(created);

      PrimaryModel fetched = airtable.primary().get(recordId);
      assertNull(fetched.getUser());
      airtable.primary().delete(recordId);
    } catch (Throwable e) {
      tryDelete(recordId);
      throw e;
    }
  }

  @Test
  void attachmentReplaceAndRemoveReadBack() {
    String urlA =
        "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png";
    String urlB = "https://www.w3.org/Icons/w3c_home.png";
    String suite = TestSetup.primaryKey("FieldRT", "Attach");
    PrimaryModel created =
        airtable
            .primary()
            .create(
                PrimaryModel.builder()
                    .primaryKey(suite)
                    .attachment(List.of(new AirtableAttachment(urlA)))
                    .build());
    String recordId = created.getId();
    try {
      // Replace the attachment with a different one.
      created.setAttachment(List.of(new AirtableAttachment(urlB)));
      airtable.primary().update(created);
      PrimaryModel replaced = airtable.primary().get(recordId);
      assertNotNull(replaced.getAttachment());
      assertEquals(1, replaced.getAttachment().size());

      // Remove all attachments.
      replaced.setAttachment(List.of());
      airtable.primary().update(replaced);
      PrimaryModel cleared = airtable.primary().get(recordId);
      assertTrue(
          cleared.getAttachment() == null || cleared.getAttachment().isEmpty(),
          "attachments cleared");
      airtable.primary().delete(recordId);
    } catch (Throwable e) {
      tryDelete(recordId);
      throw e;
    }
  }

  private void tryDelete(String recordId) {
    if (recordId == null || recordId.isEmpty()) {
      return;
    }
    try {
      airtable.primary().delete(recordId);
    } catch (RuntimeException ignored) {
      // best-effort cleanup
    }
  }
}
