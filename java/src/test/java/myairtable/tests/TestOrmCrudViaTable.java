package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import myairtable.Airtable;
import myairtable.AirtableException;
import myairtable.AirtableQuery;
import myairtable.OrmTable;
import myairtable.PrimaryFields;
import myairtable.PrimaryModel;
import myairtable.PrimaryMultipleSelectOption;
import myairtable.PrimarySingleSelectOption;
import org.junit.jupiter.api.Test;

/**
 * J4.7 — Typed ORM CRUD via table methods. Parity with Kotlin TestOrmCrudViaTable / Swift
 * TestOrmCrudViaTable / Rust test_orm_crud_via_table.
 */
class TestOrmCrudViaTable {
  private final Airtable airtable = TestSetup.makeAirtable();

  @Test
  void primaryKeyOnlyCrudRoundTrip() {
    String primaryKey = TestSetup.primaryKey("OrmTable", "PKOnly");
    PrimaryModel created =
        airtable.primary().create(PrimaryModel.builder().primaryKey(primaryKey).build());
    assertNotNull(created.getId());
    assertEquals(primaryKey, created.getPrimaryKey());

    String recordId = created.getId();
    try {
      PrimaryModel fetched = airtable.primary().get(recordId);
      assertEquals(recordId, fetched.getId());
      assertEquals(primaryKey, fetched.getPrimaryKey());
      assertTrue(fetched.dirtyFields().isEmpty(), "fresh decode has no dirty fields");

      fetched.setPrimaryKey(primaryKey + " Updated");
      assertFalse(fetched.dirtyFields().isEmpty());
      PrimaryModel updated = airtable.primary().update(fetched);
      assertEquals(primaryKey + " Updated", updated.getPrimaryKey());

      airtable.primary().delete(recordId);
      assertThrows(
          AirtableException.class,
          () -> airtable.primary().get(recordId),
          "record should be gone after delete");
    } catch (Throwable e) {
      try {
        airtable.primary().delete(recordId);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  @Test
  void allSimplePropertiesRoundTrip() {
    String primaryKey = TestSetup.primaryKey("OrmTable", "AllProps");
    PrimaryModel model =
        PrimaryModel.builder()
            .primaryKey(primaryKey)
            .singleLineText("Hello World")
            .longText("Long text content")
            .email("test@example.com")
            .url("https://example.com")
            .phoneNumber("555-1234")
            .checkbox(true)
            .numberInt(42.0)
            .numberFloat(3.14)
            .currencyInt(10.0)
            .currencyFloat(9.99)
            .percentInt(0.5)
            .percentFloat(0.333)
            .duration(Duration.ofSeconds(3600))
            .build();
    PrimaryModel created = airtable.primary().create(model);
    String recordId = created.getId();
    try {
      assertEquals(primaryKey, created.getPrimaryKey());
      assertEquals("Hello World", created.getSingleLineText());
      assertEquals("test@example.com", created.getEmail());
      assertEquals(true, created.getCheckbox());
      assertEquals(42.0, created.getNumberInt());
      assertEquals(3.14, created.getNumberFloat());
      assertEquals(9.99, created.getCurrencyFloat());
      assertEquals("https://example.com", created.getUrl());
      assertEquals("555-1234", created.getPhoneNumber());
      assertEquals(
          Duration.ofSeconds(3600), created.getDuration(), "duration wire unit is seconds");
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

  @Test
  void selectEnumsAndRatingRoundTrip() {
    String primaryKey = TestSetup.primaryKey("OrmTable", "Selects");
    PrimaryModel model =
        PrimaryModel.builder()
            .primaryKey(primaryKey)
            .singleSelect(PrimarySingleSelectOption.CHOICE_1)
            .multipleSelect(
                List.of(PrimaryMultipleSelectOption.OPTION_1, PrimaryMultipleSelectOption.OPTION_2))
            .rating(3L)
            .build();
    PrimaryModel created = airtable.primary().create(model);
    String recordId = created.getId();
    try {
      assertEquals(PrimarySingleSelectOption.CHOICE_1, created.getSingleSelect());
      assertEquals(
          List.of(PrimaryMultipleSelectOption.OPTION_1, PrimaryMultipleSelectOption.OPTION_2),
          created.getMultipleSelect());
      assertEquals(3L, created.getRating());

      PrimaryModel fetched = airtable.primary().get(recordId);
      assertEquals(PrimarySingleSelectOption.CHOICE_1, fetched.getSingleSelect());

      fetched.setSingleSelect(PrimarySingleSelectOption.CHOICE_2);
      fetched.setMultipleSelect(List.of(PrimaryMultipleSelectOption.OPTION_3));
      fetched.setRating(5L);
      PrimaryModel updated = airtable.primary().update(fetched);
      assertEquals(PrimarySingleSelectOption.CHOICE_2, updated.getSingleSelect());
      assertEquals(List.of(PrimaryMultipleSelectOption.OPTION_3), updated.getMultipleSelect());
      assertEquals(5L, updated.getRating());

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

  @Test
  void getQueryReturnsFilteredRecordSet() {
    String suite = TestSetup.primaryKey("OrmTable", "List");
    List<PrimaryModel> models = new ArrayList<>();
    for (int i = 1; i <= 3; i++) {
      models.add(PrimaryModel.builder().primaryKey(suite + " " + i).build());
    }
    List<PrimaryModel> created = airtable.primary().create(models);
    List<String> createdIds = created.stream().map(PrimaryModel::getId).toList();
    try {
      assertEquals(3, created.size());
      List<PrimaryModel> results =
          airtable
              .primary()
              .get(new AirtableQuery().withFormula("FIND(\"" + suite + "\", {Primary Key})"));
      assertEquals(3, results.size());
      assertEquals(
          java.util.Set.copyOf(createdIds),
          java.util.Set.copyOf(results.stream().map(PrimaryModel::getId).toList()));
      airtable.primary().deleteAll(createdIds);
    } catch (Throwable e) {
      try {
        airtable.primary().deleteAll(createdIds);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  @Test
  void fieldSelectionAndMaxRecordsApply() {
    String suite = TestSetup.primaryKey("OrmTable", "Select");
    List<PrimaryModel> models = new ArrayList<>();
    for (int i = 1; i <= 3; i++) {
      models.add(
          PrimaryModel.builder().primaryKey(suite + " " + i).singleLineText("text " + i).build());
    }
    List<PrimaryModel> created = airtable.primary().create(models);
    List<String> createdIds = created.stream().map(PrimaryModel::getId).toList();
    try {
      List<PrimaryModel> results =
          airtable
              .primary()
              .get(
                  new AirtableQuery()
                      .withFormula("FIND(\"" + suite + "\", {Primary Key})")
                      .withFields(List.of(PrimaryFields.primaryKeyId))
                      .withMaxRecords(2));
      assertEquals(2, results.size(), "maxRecords caps the result set");
      for (PrimaryModel model : results) {
        assertNotNull(model.getPrimaryKey());
        assertNull(model.getSingleLineText(), "unselected fields decode as null");
      }
      airtable.primary().deleteAll(createdIds);
    } catch (Throwable e) {
      try {
        airtable.primary().deleteAll(createdIds);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  @Test
  void invalidRecordIdThrows() {
    assertThrows(AirtableException.class, () -> airtable.primary().get("recDOESNOTEXIST123"));
  }

  @Test
  void batchCreateUpdateDeleteChunksAboveTheBatchLimit() {
    String suite = TestSetup.primaryKey("OrmTable", "Batch");
    int count = 111;
    List<PrimaryModel> models = new ArrayList<>();
    for (int i = 1; i <= count; i++) {
      models.add(PrimaryModel.builder().primaryKey(suite + " " + i).build());
    }
    // create() chunks into 10-record batches; a mid-batch failure throws WITHOUT returning the
    // ids of records already created in earlier batches, so the by-id cleanup below can't see
    // them. Pull the create inside the try and fall back to a prefix sweep on failure (the suite
    // prefix is unique per run), so partial creates are still cleaned up.
    List<String> createdIds = List.of();
    try {
      List<PrimaryModel> created = airtable.primary().create(models);
      createdIds = created.stream().map(PrimaryModel::getId).toList();

      assertEquals(count, created.size());
      for (int i = 0; i < created.size(); i++) {
        assertEquals(suite + " " + (i + 1), created.get(i).getPrimaryKey());
      }

      for (PrimaryModel model : created) {
        model.setPrimaryKey(model.getPrimaryKey() + " Updated");
      }
      List<PrimaryModel> updated = airtable.primary().update(created);
      assertEquals(count, updated.size());
      for (int i = 0; i < updated.size(); i++) {
        assertEquals(suite + " " + (i + 1) + " Updated", updated.get(i).getPrimaryKey());
      }

      airtable.primary().deleteAll(createdIds);
      String firstId = createdIds.get(0);
      assertThrows(AirtableException.class, () -> airtable.primary().get(firstId));
    } catch (Throwable e) {
      // First delete the ids we know about, then sweep by prefix to catch any partial-batch
      // creates whose ids never came back (the original primary key carries the unique suite
      // prefix; the " Updated" rename only happens after a fully successful create).
      try {
        if (!createdIds.isEmpty()) {
          airtable.primary().deleteAll(createdIds);
        }
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      try {
        List<String> straySweep =
            airtable
                .primary()
                .get(new AirtableQuery().withFormula("FIND(\"" + suite + "\", {Primary Key})"))
                .stream()
                .map(PrimaryModel::getId)
                .toList();
        if (!straySweep.isEmpty()) {
          airtable.primary().deleteAll(straySweep);
        }
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  @Test
  void upsertInsertsWhenNoMatchThenUpdatesOnMatch() {
    String primaryKey = TestSetup.primaryKey("OrmTable", "Upsert");
    // Insert path
    OrmTable.UpsertResult<PrimaryModel> first =
        airtable
            .primary()
            .upsert(
                PrimaryModel.builder().primaryKey(primaryKey).build(),
                List.of(PrimaryFields.primaryKeyId));
    String recordId = first.model().getId();
    try {
      assertTrue(first.wasCreated(), "no match yet — upsert inserts");
      assertEquals(primaryKey, first.model().getPrimaryKey());

      // Update path: same primary key, new field value.
      OrmTable.UpsertResult<PrimaryModel> second =
          airtable
              .primary()
              .upsert(
                  PrimaryModel.builder().primaryKey(primaryKey).singleLineText("merged").build(),
                  List.of(PrimaryFields.primaryKeyId));
      assertFalse(second.wasCreated(), "matched on primary key — upsert updates");
      assertEquals(recordId, second.model().getId());
      assertEquals("merged", second.model().getSingleLineText());

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
}
