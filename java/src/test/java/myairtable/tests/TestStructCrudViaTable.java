package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.List;
import myairtable.Airtable;
import myairtable.AirtableException;
import myairtable.Fields;
import myairtable.PrimaryFields;
import org.junit.jupiter.api.Test;

/**
 * J3.7 — Dict-table CRUD parity with rust/tests/test_struct_crud_via_table.rs, Swift/Kotlin
 * TestStructCrudViaTable. JUnit runs methods within a class sequentially; cross-class parallelism
 * is off (build.gradle.kts).
 */
class TestStructCrudViaTable {
  private final Airtable airtable = TestSetup.makeAirtable();

  // ---- Primary key only ----

  @Test
  void primaryKeyOnlyCrudRoundTrip() {
    String primaryKey = TestSetup.primaryKey("StructCrud", "PKOnly");
    Fields fields = new Fields(java.util.Map.of(), PrimaryFields.nameToId);
    fields.setString(PrimaryFields.primaryKeyId, primaryKey);

    // Create
    var created = airtable.primary().dict().create(fields);
    String recordId = created.id();
    try {
      // Post-create asserts were previously outside the try, so a failure leaked the record.
      assertFalse(created.id().isEmpty());
      assertEquals(primaryKey, created.fields().getString(PrimaryFields.primaryKeyId));

      // Read
      var fetched = airtable.primary().dict().get(recordId);
      assertEquals(recordId, fetched.id());
      assertEquals(primaryKey, fetched.fields().getString(PrimaryFields.primaryKeyId));

      // Update
      Fields update = new Fields(java.util.Map.of(), PrimaryFields.nameToId);
      String updatedKey = primaryKey + " Updated";
      update.setString(PrimaryFields.primaryKeyId, updatedKey);
      var updated = airtable.primary().dict().update(recordId, update);
      assertEquals(updatedKey, updated.fields().getString(PrimaryFields.primaryKeyId));

      // Delete + verify gone
      airtable.primary().dict().delete(recordId);
      assertThrows(
          AirtableException.class,
          () -> airtable.primary().dict().get(recordId),
          "record should be gone after delete");
    } catch (Throwable e) {
      // Best-effort cleanup on any intermediate failure.
      try {
        airtable.primary().dict().delete(recordId);
      } catch (RuntimeException ignored) {
        // already deleted or cleanup failed — surface the original error
      }
      throw e;
    }
  }

  // ---- Dual ID/name access ----

  @Test
  void dictTableSupportsDualIdNameFieldAccess() {
    String primaryKey = TestSetup.primaryKey("StructCrud", "DualAccess");
    Fields fields = new Fields(java.util.Map.of(), PrimaryFields.nameToId);
    // Set by ID…
    fields.setString(PrimaryFields.primaryKeyId, primaryKey);

    var created = airtable.primary().dict().create(fields);
    String recordId = created.id();
    try {
      // …read back by name.
      assertEquals(primaryKey, created.fields().getString(PrimaryFields.primaryKeyName));
      // Also readable by ID.
      assertEquals(primaryKey, created.fields().getString(PrimaryFields.primaryKeyId));
      airtable.primary().dict().delete(recordId);
    } catch (Throwable e) {
      try {
        airtable.primary().dict().delete(recordId);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // ---- All simple properties ----

  @Test
  void allSimplePropertiesRoundTrip() {
    String primaryKey = TestSetup.primaryKey("StructCrud", "AllProps");
    Fields fields = new Fields(java.util.Map.of(), PrimaryFields.nameToId);
    fields.setString(PrimaryFields.primaryKeyId, primaryKey);
    fields.setString(PrimaryFields.singleLineTextId, "Hello World");
    fields.setString(PrimaryFields.longTextId, "Long text content");
    fields.setString(PrimaryFields.longTextWithRichTextId, "Rich text content");
    fields.setString(PrimaryFields.emailId, "test@example.com");
    fields.setString(PrimaryFields.urlId, "https://example.com");
    fields.setString(PrimaryFields.phoneNumberId, "555-1234");
    fields.setBoolean(PrimaryFields.checkboxId, true);
    fields.setLong(PrimaryFields.numberIntId, 42L);
    fields.setDouble(PrimaryFields.numberFloatId, 3.14);
    fields.setLong(PrimaryFields.currencyIntId, 10L);
    fields.setDouble(PrimaryFields.currencyFloatId, 9.99);
    fields.setDouble(PrimaryFields.percentIntId, 0.5);
    fields.setDouble(PrimaryFields.percentFloatId, 0.333);
    fields.setLong(PrimaryFields.durationId, 3600L);
    fields.setLong(PrimaryFields.ratingId, 3L);
    fields.setString(PrimaryFields.dateId, "2025-01-15");
    fields.setString(PrimaryFields.dateWithTimeId, "2025-01-15T10:00:00.000Z");
    fields.setString(PrimaryFields.singleSelectId, "Choice 1");
    fields.setStrings(PrimaryFields.multipleSelectId, List.of("Option 1", "Option 2"));

    var created = airtable.primary().dict().create(fields);
    String recordId = created.id();
    try {
      Fields f = created.fields();
      assertEquals(primaryKey, f.getString(PrimaryFields.primaryKeyId));
      assertEquals("Hello World", f.getString(PrimaryFields.singleLineTextId));
      assertEquals("test@example.com", f.getString(PrimaryFields.emailId));
      assertEquals(true, f.getBoolean(PrimaryFields.checkboxId));
      assertEquals(42L, f.getLong(PrimaryFields.numberIntId));
      assertEquals(3.14, f.getDouble(PrimaryFields.numberFloatId));
      assertEquals(3L, f.getLong(PrimaryFields.ratingId));
      assertEquals("Choice 1", f.getString(PrimaryFields.singleSelectId));
      assertTrue(f.getArray(PrimaryFields.multipleSelectId).size() == 2);

      airtable.primary().dict().delete(recordId);
    } catch (Throwable e) {
      try {
        airtable.primary().dict().delete(recordId);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }
}
