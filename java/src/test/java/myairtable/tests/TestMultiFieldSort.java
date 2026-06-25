package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.HashMap;
import java.util.List;
import java.util.Objects;
import myairtable.Airtable;
import myairtable.AirtableQuery;
import myairtable.DictTable;
import myairtable.Fields;
import myairtable.Formulas;
import myairtable.PrimaryFields;
import myairtable.PrimaryModel;
import myairtable.SortDirection;
import org.junit.jupiter.api.Test;

/**
 * TC11 — Multi-field sort + sort combined with a filter. The base filter suite only covers a
 * single-field sort. This verifies a two-field sort (primary key with ties broken by a secondary
 * key) and sorting within a filtered scope. Parity with C# TestMultiFieldSort.
 */
class TestMultiFieldSort {
  private final Airtable airtable = TestSetup.makeAirtable();

  /** Dict-path fields row with primary key, numberInt, and singleLineText set. */
  private static Fields row(String suite, long number, String text) {
    Fields fields = new Fields(new HashMap<>(), PrimaryFields.nameToId);
    fields.setString(PrimaryFields.primaryKeyId, suite + " " + text);
    fields.setLong(PrimaryFields.numberIntId, number);
    fields.setString(PrimaryFields.singleLineTextId, text);
    return fields;
  }

  /** Scope a formula to specific record IDs so suite runs don't interfere. */
  private static String scopeTo(List<String> ids) {
    List<String> parts = ids.stream().map(id -> "RECORD_ID()='" + id + "'").toList();
    return parts.size() == 1 ? parts.get(0) : Formulas.or(parts);
  }

  private static List<String> texts(List<DictTable.Record> records) {
    return records.stream()
        .map(r -> r.fields().getString(PrimaryFields.singleLineTextId))
        .filter(Objects::nonNull)
        .toList();
  }

  @Test
  void twoFieldSortBreaksTiesOnSecondKey() {
    String suite = TestSetup.primaryKey("Sort", "TwoField");
    // NumberInt ties at 10 (rows "b" and "a"); the secondary SingleLineText sort orders them.
    List<DictTable.Record> created =
        airtable
            .primary()
            .dict()
            .create(List.of(row(suite, 10, "b"), row(suite, 10, "a"), row(suite, 20, "c")));
    List<String> ids = created.stream().map(DictTable.Record::id).toList();

    try {
      List<DictTable.Record> results =
          airtable
              .primary()
              .dict()
              .get(
                  new AirtableQuery()
                      .withFormula(scopeTo(ids))
                      .withSort(PrimaryFields.numberIntId, SortDirection.ASC)
                      .withSort(PrimaryFields.singleLineTextId, SortDirection.ASC));
      // (10,a), (10,b), (20,c) — tie on 10 broken by text asc.
      assertEquals(List.of("a", "b", "c"), texts(results));

      tryDeleteMany(ids);
    } catch (Throwable e) {
      tryDeleteMany(ids);
      throw e;
    }
  }

  @Test
  void secondaryDescendingReversesTiedGroup() {
    String suite = TestSetup.primaryKey("Sort", "MixedDir");
    List<DictTable.Record> created =
        airtable
            .primary()
            .dict()
            .create(List.of(row(suite, 10, "a"), row(suite, 10, "b"), row(suite, 20, "c")));
    List<String> ids = created.stream().map(DictTable.Record::id).toList();

    try {
      List<DictTable.Record> results =
          airtable
              .primary()
              .dict()
              .get(
                  new AirtableQuery()
                      .withFormula(scopeTo(ids))
                      .withSort(PrimaryFields.numberIntId, SortDirection.ASC)
                      .withSort(PrimaryFields.singleLineTextId, SortDirection.DESC));
      // NumberInt asc, then text DESC within the 10-tie: (10,b), (10,a), (20,c).
      assertEquals(List.of("b", "a", "c"), texts(results));

      tryDeleteMany(ids);
    } catch (Throwable e) {
      tryDeleteMany(ids);
      throw e;
    }
  }

  @Test
  void sortCombinedWithAFilter() {
    String suite = TestSetup.primaryKey("Sort", "WithFilter");
    List<DictTable.Record> created =
        airtable
            .primary()
            .dict()
            .create(
                List.of(
                    row(suite, 30, "x"),
                    row(suite, 10, "y"),
                    row(suite, 20, "z"),
                    row(suite, 5, "low"))); // filtered out by NumberInt > 5
    List<String> ids = created.stream().map(DictTable.Record::id).toList();

    try {
      String filter = Formulas.and(scopeTo(ids), PrimaryModel.f.numberInt.greaterThan(5));
      List<DictTable.Record> results =
          airtable
              .primary()
              .dict()
              .get(
                  new AirtableQuery()
                      .withFormula(filter)
                      .withSort(PrimaryFields.numberIntId, SortDirection.ASC));
      // Filtered to NumberInt > 5, sorted asc: 10(y), 20(z), 30(x).
      assertEquals(List.of("y", "z", "x"), texts(results));

      tryDeleteMany(ids);
    } catch (Throwable e) {
      tryDeleteMany(ids);
      throw e;
    }
  }

  private void tryDeleteMany(List<String> ids) {
    if (ids.isEmpty()) {
      return;
    }
    try {
      airtable.primary().dict().delete(ids);
    } catch (RuntimeException ignored) {
      // best-effort cleanup
    }
  }
}
