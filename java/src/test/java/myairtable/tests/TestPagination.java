package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.ArrayList;
import java.util.List;
import myairtable.Airtable;
import myairtable.AirtableException;
import myairtable.AirtableQuery;
import myairtable.DictTable;
import myairtable.Fields;
import myairtable.PrimaryFields;
import myairtable.PrimaryModel;
import org.junit.jupiter.api.Test;

/**
 * TC1 — Multi-page pagination: list more records than Airtable's 100-record default page in a
 * single query (no maxRecords) and assert the client's offset loop reassembles every page.
 * Exercises the page-reassembly path in the generated client. Parity with C# TestPagination / other
 * language suites.
 */
class TestPagination {
  private final Airtable airtable = TestSetup.makeAirtable();

  // 105 > the 100-record default page size, so a no-maxRecords list spans two pages.
  private static final int COUNT = 105;

  @Test
  void ormGetSpanningMultiplePagesReturnsEveryRecord() {
    String suite = TestSetup.primaryKey("Pagination", "Orm");
    List<PrimaryModel> models = new ArrayList<>();
    for (int i = 1; i <= COUNT; i++) {
      models.add(PrimaryModel.builder().primaryKey(suite + " " + i).build());
    }
    List<String> createdIds = List.of();
    try {
      List<PrimaryModel> created = airtable.primary().create(models);
      createdIds = created.stream().map(PrimaryModel::getId).toList();
      assertEquals(COUNT, created.size());

      // No maxRecords: the offset loop must walk both pages and return all 105.
      List<PrimaryModel> results =
          airtable
              .primary()
              .get(new AirtableQuery().withFormula("FIND(\"" + suite + "\", {Primary Key})"));
      assertEquals(COUNT, results.size());
      assertEquals(
          java.util.Set.copyOf(createdIds),
          java.util.Set.copyOf(results.stream().map(PrimaryModel::getId).toList()));
    } catch (Throwable e) {
      tryDeleteMany(createdIds, suite);
      throw e;
    }
    tryDeleteMany(createdIds, suite);
  }

  @Test
  void dictGetSpanningMultiplePagesReturnsEveryRecord() {
    String suite = TestSetup.primaryKey("Pagination", "Dict");
    List<Fields> creates = new ArrayList<>();
    for (int i = 1; i <= COUNT; i++) {
      Fields fields = new Fields(new java.util.HashMap<>(), PrimaryFields.nameToId);
      fields.setString(PrimaryFields.primaryKeyId, suite + " " + i);
      creates.add(fields);
    }
    List<String> createdIds = List.of();
    try {
      List<DictTable.Record> created = airtable.primary().dict().create(creates);
      createdIds = created.stream().map(DictTable.Record::id).toList();
      assertEquals(COUNT, created.size());

      List<DictTable.Record> results =
          airtable
              .primary()
              .dict()
              .get(new AirtableQuery().withFormula("FIND(\"" + suite + "\", {Primary Key})"));
      assertEquals(COUNT, results.size());
      assertEquals(
          java.util.Set.copyOf(createdIds),
          java.util.Set.copyOf(results.stream().map(DictTable.Record::id).toList()));
    } catch (Throwable e) {
      tryDeleteMany(createdIds, suite);
      throw e;
    }
    tryDeleteMany(createdIds, suite);
  }

  // ---- cleanup ----

  /**
   * Best-effort cleanup: delete the ids we know about, then sweep by the unique suite prefix to
   * catch any partial-batch creates whose ids never came back (create() chunks into 10-record
   * batches and a mid-batch failure throws without returning earlier ids).
   */
  private void tryDeleteMany(List<String> ids, String suite) {
    try {
      if (!ids.isEmpty()) {
        airtable.primary().dict().delete(ids);
      }
    } catch (RuntimeException ignored) {
      // best-effort cleanup
    }
    try {
      List<String> stray =
          airtable
              .primary()
              .dict()
              .get(new AirtableQuery().withFormula("FIND(\"" + suite + "\", {Primary Key})"))
              .stream()
              .map(DictTable.Record::id)
              .toList();
      if (!stray.isEmpty()) {
        airtable.primary().dict().delete(stray);
      }
    } catch (AirtableException ignored) {
      // best-effort cleanup
    }
  }
}
