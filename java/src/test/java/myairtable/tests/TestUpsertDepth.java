package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.ArrayList;
import java.util.List;
import myairtable.Airtable;
import myairtable.AirtableException;
import myairtable.OrmTable;
import myairtable.PrimaryFields;
import myairtable.PrimaryModel;
import org.junit.jupiter.api.Test;

/**
 * TC10 — Upsert depth. The base CRUD suite only covers single-record, single-merge-field upsert.
 * This adds a multi-field merge key (match must agree on ALL merge fields) and the multiple-match
 * error path (a merge key matching more than one record is rejected by Airtable). Note: the
 * generated client exposes only single-record upsert, so batch upsert isn't covered here. Parity
 * with C# TestUpsertDepth.
 */
class TestUpsertDepth {
  private final Airtable airtable = TestSetup.makeAirtable();

  @Test
  void upsertMatchesOnMultipleMergeFields() {
    String suite = TestSetup.primaryKey("Upsert", "MultiKey");
    List<String> ids = new ArrayList<>();
    try {
      // Seed a record identified by the (PrimaryKey, SingleLineText) pair.
      PrimaryModel seed =
          airtable
              .primary()
              .create(PrimaryModel.builder().primaryKey(suite).singleLineText("anchor").build());
      ids.add(seed.getId());

      List<String> mergeOn = List.of(PrimaryFields.primaryKeyId, PrimaryFields.singleLineTextId);

      // Same pair -> UPDATE the seed (matched on both fields).
      OrmTable.UpsertResult<PrimaryModel> update =
          airtable
              .primary()
              .upsert(
                  PrimaryModel.builder()
                      .primaryKey(suite)
                      .singleLineText("anchor")
                      .longText("updated")
                      .build(),
                  mergeOn);
      assertFalse(update.wasCreated());
      assertEquals(seed.getId(), update.model().getId());
      assertEquals("updated", update.model().getLongText());

      // Same PrimaryKey but a DIFFERENT SingleLineText -> no match on the pair -> INSERT.
      OrmTable.UpsertResult<PrimaryModel> insert =
          airtable
              .primary()
              .upsert(
                  PrimaryModel.builder().primaryKey(suite).singleLineText("different").build(),
                  mergeOn);
      ids.add(insert.model().getId());
      assertTrue(insert.wasCreated());
      assertNotEquals(seed.getId(), insert.model().getId());
    } finally {
      tryDeleteMany(ids);
    }
  }

  @Test
  void upsertWithMultipleMatchesThrows() {
    String suite = TestSetup.primaryKey("Upsert", "MultiMatch");
    List<String> ids = new ArrayList<>();
    try {
      // Two records share the same SingleLineText value.
      PrimaryModel a =
          airtable
              .primary()
              .create(
                  PrimaryModel.builder().primaryKey(suite + " A").singleLineText("dupe").build());
      PrimaryModel b =
          airtable
              .primary()
              .create(
                  PrimaryModel.builder().primaryKey(suite + " B").singleLineText("dupe").build());
      ids.add(a.getId());
      ids.add(b.getId());

      // Upsert merging only on SingleLineText="dupe" matches BOTH -> Airtable rejects it.
      assertThrows(
          AirtableException.Api.class,
          () ->
              airtable
                  .primary()
                  .upsert(
                      PrimaryModel.builder().singleLineText("dupe").longText("x").build(),
                      List.of(PrimaryFields.singleLineTextId)));
    } finally {
      tryDeleteMany(ids);
    }
  }

  private void tryDeleteMany(List<String> ids) {
    if (ids.isEmpty()) {
      return;
    }
    try {
      airtable.primary().deleteAll(ids);
    } catch (RuntimeException ignored) {
      // best-effort cleanup
    }
  }
}
