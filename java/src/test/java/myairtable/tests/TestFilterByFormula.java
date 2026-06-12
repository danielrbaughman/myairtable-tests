package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.fasterxml.jackson.databind.node.ArrayNode;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Objects;
import java.util.Set;
import java.util.stream.Collectors;
import myairtable.Airtable;
import myairtable.AirtableDateParser;
import myairtable.AirtableJson;
import myairtable.AirtableQuery;
import myairtable.DictTable;
import myairtable.Fields;
import myairtable.Formulas;
import myairtable.FormulasFilters;
import myairtable.FormulasModel;
import myairtable.PrimaryFields;
import myairtable.PrimaryFilters;
import myairtable.PrimaryModel;
import myairtable.PrimaryView;
import myairtable.SecondaryModel;
import myairtable.SortDirection;
import org.junit.jupiter.api.Test;

/**
 * Filter-by-formula integration tests. Parity with Kotlin TestFilterByFormula / Swift
 * TestFilterByFormula / Rust test_filter_by_formula.
 */
class TestFilterByFormula {
  private final Airtable airtable = TestSetup.makeAirtable();

  // region Shared helpers (parity with rust scope_to / primary helpers)

  /** Scope a formula to specific record IDs so suite runs don't interfere. */
  private static String scopeTo(List<String> ids) {
    List<String> parts = ids.stream().map(id -> "RECORD_ID()='" + id + "'").toList();
    return parts.size() == 1 ? parts.get(0) : Formulas.or(parts);
  }

  /** Dict-path fields container with just the primary key set. */
  private static Fields primaryFields(String name) {
    Fields fields = new Fields(new HashMap<>(), PrimaryFields.nameToId);
    fields.setString(PrimaryFields.primaryKeyId, name);
    return fields;
  }

  private static List<String> names(List<DictTable.Record> records) {
    return records.stream()
        .map(r -> r.fields().getString(PrimaryFields.primaryKeyId))
        .filter(Objects::nonNull)
        .sorted()
        .toList();
  }

  /** Count of Primary dict records matching {@code formula} within {@code scope}. */
  private int count(String scope, String formula) {
    return airtable
        .primary()
        .dict()
        .get(new AirtableQuery().withFormula(Formulas.and(scope, formula)))
        .size();
  }

  // endregion

  // region Text field filter

  @Test
  void textFieldEquals() {
    String suite = TestSetup.primaryKey("Filter", "TextEq");
    PrimaryModel created =
        airtable
            .primary()
            .create(
                PrimaryModel.builder()
                    .primaryKey(suite)
                    .singleLineText("UniqueFilterValue")
                    .build());
    String recordId = created.getId();
    assertNotNull(recordId, "Missing id");

    try {
      PrimaryFilters f = PrimaryModel.f;
      String filterFormula = f.singleLineText.eq("UniqueFilterValue");
      List<PrimaryModel> results =
          airtable.primary().get(new AirtableQuery().withFormula(filterFormula));

      assertTrue(results.stream().anyMatch(r -> recordId.equals(r.getId())));

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

  // region Record ID filter

  @Test
  void recordIdFilter() {
    String suite = TestSetup.primaryKey("Filter", "RecordID");
    PrimaryModel created =
        airtable.primary().create(PrimaryModel.builder().primaryKey(suite).build());
    String recordId = created.getId();
    assertNotNull(recordId, "Missing id");

    try {
      PrimaryFilters f = PrimaryModel.f;
      String filterFormula = f.id.eq(recordId);
      List<PrimaryModel> results =
          airtable.primary().get(new AirtableQuery().withFormula(filterFormula));

      assertEquals(1, results.size());
      assertEquals(recordId, results.get(0).getId());

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

  // region Number field filter

  @Test
  void numberFieldFilter() {
    String suite = TestSetup.primaryKey("Filter", "Number");
    PrimaryModel c1 =
        airtable
            .primary()
            .create(PrimaryModel.builder().numberInt(10.0).primaryKey(suite + " 1").build());
    PrimaryModel c2 =
        airtable
            .primary()
            .create(PrimaryModel.builder().numberInt(20.0).primaryKey(suite + " 2").build());
    String id1 = c1.getId();
    String id2 = c2.getId();
    assertNotNull(id1, "Missing ids");
    assertNotNull(id2, "Missing ids");

    try {
      PrimaryFilters f = PrimaryModel.f;
      // Filter for numberInt > 15 — should match only c2.
      String filterFormula = f.numberInt.greaterThan(15);
      List<PrimaryModel> results =
          airtable.primary().get(new AirtableQuery().withFormula(filterFormula));
      assertTrue(results.stream().anyMatch(r -> id2.equals(r.getId())));
      assertFalse(results.stream().anyMatch(r -> id1.equals(r.getId())));

      airtable.primary().deleteAll(List.of(id1, id2));
    } catch (Throwable e) {
      try {
        airtable.primary().deleteAll(List.of(id1, id2));
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion

  // region Combinators

  @Test
  void andCombinator() {
    String suite = TestSetup.primaryKey("Filter", "AND");
    PrimaryModel created =
        airtable
            .primary()
            .create(
                PrimaryModel.builder().checkbox(true).numberInt(42.0).primaryKey(suite).build());
    String recordId = created.getId();
    assertNotNull(recordId, "Missing id");

    try {
      PrimaryFilters f = PrimaryModel.f;
      String filterFormula = Formulas.and(f.checkbox.isTrue(), f.numberInt.eq(42));
      List<PrimaryModel> results =
          airtable.primary().get(new AirtableQuery().withFormula(filterFormula));
      assertTrue(results.stream().anyMatch(r -> recordId.equals(r.getId())));

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

  // region Filter by view

  @Test
  void filterByView() {
    List<Fields> creates = new ArrayList<>();
    for (int i = 0; i < 5; i++) {
      creates.add(primaryFields("Filter Test " + i));
    }
    for (int i = 0; i < 5; i++) {
      creates.add(primaryFields("Don't Include Test " + i));
    }
    List<DictTable.Record> created = airtable.primary().dict().create(creates);
    List<String> ids = created.stream().map(DictTable.Record::id).toList();

    try {
      // Live coverage for the AirtableView overload (myairtable-yvb3).
      List<DictTable.Record> results =
          airtable.primary().dict().get(new AirtableQuery().withView(PrimaryView.FILTER_BY_VIEW));
      assertTrue(results.size() >= 5);
      for (DictTable.Record record : results) {
        String name = record.fields().getString(PrimaryFields.primaryKeyId);
        if (name == null) {
          name = "";
        }
        assertTrue(name.startsWith("Filter Test"), "Unexpected record: " + name);
      }
      airtable.primary().dict().delete(ids);
    } catch (Throwable e) {
      try {
        airtable.primary().dict().delete(ids);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion

  // region Record ID inList

  @Test
  void recordIdInList() {
    String suite = TestSetup.primaryKey("Filter", "InList");
    List<Fields> creates = new ArrayList<>();
    for (int i = 0; i < 3; i++) {
      creates.add(primaryFields(suite + " " + i));
    }
    List<DictTable.Record> created = airtable.primary().dict().create(creates);
    List<String> ids = created.stream().map(DictTable.Record::id).toList();

    try {
      PrimaryFilters f = PrimaryModel.f;

      // inList multiple
      List<DictTable.Record> results =
          airtable
              .primary()
              .dict()
              .get(new AirtableQuery().withFormula(f.id.inList(List.of(ids.get(0), ids.get(1)))));
      assertEquals(2, results.size());

      // inList single
      results =
          airtable
              .primary()
              .dict()
              .get(new AirtableQuery().withFormula(f.id.inList(List.of(ids.get(0)))));
      assertEquals(1, results.size());

      // inList empty
      results =
          airtable.primary().dict().get(new AirtableQuery().withFormula(f.id.inList(List.of())));
      assertTrue(results.isEmpty());

      airtable.primary().dict().delete(ids);
    } catch (Throwable e) {
      try {
        airtable.primary().dict().delete(ids);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion

  // region Text field predicates

  @Test
  void textFieldPredicates() {
    List<DictTable.Record> created =
        airtable
            .primary()
            .dict()
            .create(
                List.of(
                    primaryFields("FbText Alpha One"),
                    primaryFields("FbText Alpha Two"),
                    primaryFields("FbText Beta One")));
    List<String> ids = created.stream().map(DictTable.Record::id).toList();
    String scope = scopeTo(ids);
    PrimaryFilters f = PrimaryModel.f;

    try {
      assertEquals(1, count(scope, f.primaryKey.eq("FbText Alpha One")));
      assertEquals(2, count(scope, f.primaryKey.neq("FbText Alpha One")));
      assertEquals(2, count(scope, f.primaryKey.contains("Alpha", false, true)));
      assertEquals(
          3, count(scope, f.primaryKey.containsAny(List.of("Alpha", "Beta"), false, true)));
      assertEquals(1, count(scope, f.primaryKey.containsAll(List.of("Alpha", "One"), false, true)));
      assertEquals(1, count(scope, f.primaryKey.notContains("Alpha", false, true)));
      assertEquals(2, count(scope, f.primaryKey.startsWith("FbText Alpha", false, true)));
      assertEquals(1, count(scope, f.primaryKey.notStartsWith("FbText Alpha", false, true)));

      airtable.primary().dict().delete(ids);
    } catch (Throwable e) {
      try {
        airtable.primary().dict().delete(ids);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion

  // region Number field predicates

  @Test
  void numberFieldPredicates() {
    String suite = TestSetup.primaryKey("Filter", "NumPreds");
    List<Long> values = List.of(10L, 20L, 30L);
    List<Fields> creates = new ArrayList<>();
    for (int i = 0; i < values.size(); i++) {
      Fields fields = primaryFields(suite + " " + i);
      fields.setLong(PrimaryFields.numberIntId, values.get(i));
      creates.add(fields);
    }
    List<DictTable.Record> created = airtable.primary().dict().create(creates);
    List<String> ids = created.stream().map(DictTable.Record::id).toList();
    String scope = scopeTo(ids);
    PrimaryFilters f = PrimaryModel.f;

    try {
      assertEquals(1, count(scope, f.numberInt.eq(20)));
      assertEquals(2, count(scope, f.numberInt.neq(20)));
      assertEquals(2, count(scope, f.numberInt.greaterThan(10)));
      assertEquals(2, count(scope, f.numberInt.lessThan(30)));
      assertEquals(2, count(scope, f.numberInt.greaterThanOrEquals(20)));
      assertEquals(2, count(scope, f.numberInt.lessThanOrEquals(20)));
      assertEquals(3, count(scope, f.numberInt.between(10, 30)));
      assertEquals(1, count(scope, f.numberInt.between(10, 30, false)));

      airtable.primary().dict().delete(ids);
    } catch (Throwable e) {
      try {
        airtable.primary().dict().delete(ids);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion

  // region Boolean field

  @Test
  void booleanFieldFilter() {
    String suite = TestSetup.primaryKey("Filter", "Bool");
    Fields f1 = primaryFields(suite + " A");
    f1.setBoolean(PrimaryFields.checkboxId, true);
    Fields f2 = primaryFields(suite + " B");
    f2.setBoolean(PrimaryFields.checkboxId, false);
    List<DictTable.Record> created = airtable.primary().dict().create(List.of(f1, f2));
    List<String> ids = created.stream().map(DictTable.Record::id).toList();
    String scope = scopeTo(ids);
    PrimaryFilters f = PrimaryModel.f;

    try {
      assertEquals(1, count(scope, f.checkbox.eq(true)));
      assertTrue(count(scope, f.checkbox.eq(false)) >= 1);
      assertEquals(1, count(scope, f.checkbox.isTrue()));
      assertTrue(count(scope, f.checkbox.isFalse()) >= 1);

      airtable.primary().dict().delete(ids);
    } catch (Throwable e) {
      try {
        airtable.primary().dict().delete(ids);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion

  // region Attachments field

  @Test
  void attachmentsFieldFilter() {
    String suite = TestSetup.primaryKey("Filter", "Attach");
    Fields f1 = primaryFields(suite + " A");
    ArrayNode attachments = AirtableJson.MAPPER.createArrayNode();
    attachments
        .addObject()
        .put(
            "url",
            "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png");
    f1.set(PrimaryFields.attachmentId, attachments);
    Fields f2 = primaryFields(suite + " B");
    List<DictTable.Record> created = airtable.primary().dict().create(List.of(f1, f2));
    List<String> ids = created.stream().map(DictTable.Record::id).toList();
    String scope = scopeTo(ids);
    PrimaryFilters f = PrimaryModel.f;

    try {
      List<DictTable.Record> notEmpty =
          airtable
              .primary()
              .dict()
              .get(new AirtableQuery().withFormula(Formulas.and(scope, f.attachment.notEmpty())));
      assertFalse(notEmpty.isEmpty());

      List<DictTable.Record> empty =
          airtable
              .primary()
              .dict()
              .get(new AirtableQuery().withFormula(Formulas.and(scope, f.attachment.empty())));
      assertFalse(empty.isEmpty());

      airtable.primary().dict().delete(ids);
    } catch (Throwable e) {
      try {
        airtable.primary().dict().delete(ids);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion

  // region Date field predicates

  @Test
  void dateFieldPredicates() {
    String suite = TestSetup.primaryKey("Filter", "Date");
    Fields f1 = primaryFields(suite + " A");
    f1.setString(PrimaryFields.dateId, "2024-01-15");
    Fields f2 = primaryFields(suite + " B");
    f2.setString(PrimaryFields.dateId, "2024-06-15");
    Fields f3 = primaryFields(suite + " C"); // no date
    List<DictTable.Record> created = airtable.primary().dict().create(List.of(f1, f2, f3));
    List<String> ids = created.stream().map(DictTable.Record::id).toList();
    String scope = scopeTo(ids);
    PrimaryFilters f = PrimaryModel.f;

    try {
      assertEquals(1, count(scope, f.date.on("2024-01-15")));
      assertTrue(count(scope, f.date.notOn("2024-01-15")) >= 1);
      assertTrue(count(scope, f.date.onOrAfter("2024-06-15")) >= 1);
      assertTrue(count(scope, f.date.onOrBefore("2024-01-15")) >= 1);
      assertTrue(count(scope, f.date.after("2024-01-15")) >= 1);
      assertTrue(count(scope, f.date.before("2024-06-15")) >= 1);
      assertEquals(2, count(scope, f.date.between("2024-01-15", "2024-06-15")));
      assertEquals(2, count(scope, f.date.between("2024-01-01", "2024-12-31", false)));
      assertTrue(count(scope, f.date.notEmpty()) >= 1);
      assertTrue(count(scope, f.date.empty()) >= 1);

      airtable.primary().dict().delete(ids);
    } catch (Throwable e) {
      try {
        airtable.primary().dict().delete(ids);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion

  // region Date field chained (time-ago)

  @Test
  void dateFieldChained() {
    String suite = TestSetup.primaryKey("Filter", "DateChain");
    Fields f1 = primaryFields(suite + " A");
    f1.setString(PrimaryFields.dateId, "2024-01-15");
    Fields f2 = primaryFields(suite + " B");
    f2.setString(PrimaryFields.dateId, "2024-06-15");
    List<DictTable.Record> created = airtable.primary().dict().create(List.of(f1, f2));
    List<String> ids = created.stream().map(DictTable.Record::id).toList();
    String scope = scopeTo(ids);
    PrimaryFilters f = PrimaryModel.f;

    try {
      // before().daysAgo(1) — both dates are in the past.
      List<DictTable.Record> past =
          airtable
              .primary()
              .dict()
              .get(
                  new AirtableQuery().withFormula(Formulas.and(scope, f.date.before().daysAgo(1))));
      assertFalse(past.isEmpty());

      // after().yearsAgo(100) — both dates are within the last 100 years.
      List<DictTable.Record> recent =
          airtable
              .primary()
              .dict()
              .get(
                  new AirtableQuery()
                      .withFormula(Formulas.and(scope, f.date.after().yearsAgo(100))));
      assertFalse(recent.isEmpty());

      airtable.primary().dict().delete(ids);
    } catch (Throwable e) {
      try {
        airtable.primary().dict().delete(ids);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion

  // region Complex formulas (AND / OR / NOT / XOR)

  @Test
  void complexFormulas() {
    String suite = TestSetup.primaryKey("Filter", "Complex");
    Fields f1 = primaryFields(suite + " A");
    f1.setLong(PrimaryFields.numberIntId, 10L);
    f1.setBoolean(PrimaryFields.checkboxId, true);
    Fields f2 = primaryFields(suite + " B");
    f2.setLong(PrimaryFields.numberIntId, 20L);
    f2.setBoolean(PrimaryFields.checkboxId, false);
    Fields f3 = primaryFields(suite + " C");
    f3.setLong(PrimaryFields.numberIntId, 30L);
    f3.setBoolean(PrimaryFields.checkboxId, true);
    List<DictTable.Record> created = airtable.primary().dict().create(List.of(f1, f2, f3));
    List<String> ids = created.stream().map(DictTable.Record::id).toList();
    String scope = scopeTo(ids);
    PrimaryFilters f = PrimaryModel.f;

    try {
      // AND(number > 10, checkbox) -> only C
      assertEquals(1, count(scope, Formulas.and(f.numberInt.greaterThan(10), f.checkbox.isTrue())));
      // AND(contains, gte) -> B and C
      assertEquals(
          2,
          count(
              scope,
              Formulas.and(
                  f.primaryKey.contains(suite, false, true), f.numberInt.greaterThanOrEquals(20))));
      // OR(number = 10, number = 30) -> A and C
      assertEquals(2, count(scope, Formulas.or(f.numberInt.eq(10), f.numberInt.eq(30))));
      // OR(checkbox, number = 20) -> all three
      assertEquals(3, count(scope, Formulas.or(f.checkbox.isTrue(), f.numberInt.eq(20))));
      // XOR(checkbox, number >= 30) -> only A
      assertEquals(
          1, count(scope, Formulas.xor(f.checkbox.isTrue(), f.numberInt.greaterThanOrEquals(30))));
      // NOT(primaryKey = "A") -> B and C
      assertEquals(2, count(scope, Formulas.not(f.primaryKey.eq(suite + " A"))));
      // AND(OR(number=10, number=30), checkbox) -> A and C
      assertEquals(
          2,
          count(
              scope,
              Formulas.and(
                  Formulas.or(f.numberInt.eq(10), f.numberInt.eq(30)), f.checkbox.isTrue())));
      // OR(AND(number > 10, checkbox), primaryKey = "A") -> A and C
      assertEquals(
          2,
          count(
              scope,
              Formulas.or(
                  Formulas.and(f.numberInt.greaterThan(10), f.checkbox.isTrue()),
                  f.primaryKey.eq(suite + " A"))));
      // NOT(AND(checkbox, number > 20)) -> A and B
      assertEquals(
          2,
          count(
              scope, Formulas.not(Formulas.and(f.checkbox.isTrue(), f.numberInt.greaterThan(20)))));
      // AND(NOT(checkbox), gte(20)) -> only B
      assertEquals(
          1,
          count(
              scope,
              Formulas.and(
                  Formulas.not(f.checkbox.isTrue()), f.numberInt.greaterThanOrEquals(20))));

      airtable.primary().dict().delete(ids);
    } catch (Throwable e) {
      try {
        airtable.primary().dict().delete(ids);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion

  // region Max records

  @Test
  void maxRecords() {
    String suite = TestSetup.primaryKey("Filter", "MaxRec");
    List<Fields> creates = new ArrayList<>();
    for (int i = 1; i <= 5; i++) {
      creates.add(primaryFields(suite + " " + i));
    }
    List<DictTable.Record> created = airtable.primary().dict().create(creates);
    List<String> ids = created.stream().map(DictTable.Record::id).toList();

    try {
      List<DictTable.Record> results =
          airtable
              .primary()
              .dict()
              .get(new AirtableQuery().withFormula(scopeTo(ids)).withMaxRecords(3));
      assertEquals(3, results.size());

      airtable.primary().dict().delete(ids);
    } catch (Throwable e) {
      try {
        airtable.primary().dict().delete(ids);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion

  // region Sort

  @Test
  void sortRecords() {
    String suite = TestSetup.primaryKey("Filter", "Sort");
    Fields f1 = primaryFields(suite + " C");
    f1.setLong(PrimaryFields.numberIntId, 30L);
    Fields f2 = primaryFields(suite + " A");
    f2.setLong(PrimaryFields.numberIntId, 10L);
    Fields f3 = primaryFields(suite + " B");
    f3.setLong(PrimaryFields.numberIntId, 20L);
    List<DictTable.Record> created = airtable.primary().dict().create(List.of(f1, f2, f3));
    List<String> ids = created.stream().map(DictTable.Record::id).toList();
    String scope = scopeTo(ids);

    try {
      List<DictTable.Record> asc =
          airtable
              .primary()
              .dict()
              .get(new AirtableQuery().withFormula(scope).withSort(PrimaryFields.numberIntId));
      assertEquals(
          List.of(10L, 20L, 30L),
          asc.stream()
              .map(r -> r.fields().getLong(PrimaryFields.numberIntId))
              .filter(Objects::nonNull)
              .toList());

      List<DictTable.Record> desc =
          airtable
              .primary()
              .dict()
              .get(
                  new AirtableQuery()
                      .withFormula(scope)
                      .withSort(PrimaryFields.numberIntId, SortDirection.DESC));
      assertEquals(
          List.of(30L, 20L, 10L),
          desc.stream()
              .map(r -> r.fields().getLong(PrimaryFields.numberIntId))
              .filter(Objects::nonNull)
              .toList());

      airtable.primary().dict().delete(ids);
    } catch (Throwable e) {
      try {
        airtable.primary().dict().delete(ids);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion

  // region Field selection

  @Test
  void fieldSelection() {
    String suite = TestSetup.primaryKey("Filter", "FieldSel");
    Fields fields = primaryFields(suite);
    fields.setString(PrimaryFields.singleLineTextId, "Hello");
    fields.setLong(PrimaryFields.numberIntId, 42L);
    DictTable.Record created = airtable.primary().dict().create(fields);

    try {
      List<DictTable.Record> results =
          airtable
              .primary()
              .dict()
              .get(
                  new AirtableQuery()
                      .withFormula("RECORD_ID()='" + created.id() + "'")
                      .withFields(List.of(PrimaryFields.primaryKeyId)));
      assertEquals(1, results.size());
      Fields f = results.get(0).fields();
      assertNotNull(f.get(PrimaryFields.primaryKeyId));
      assertNull(f.get(PrimaryFields.singleLineTextId));
      assertNull(f.get(PrimaryFields.numberIntId));

      airtable.primary().dict().delete(created.id());
    } catch (Throwable e) {
      try {
        airtable.primary().dict().delete(created.id());
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  // endregion

  // region Lookup field (ARRAYJOIN regression)

  @Test
  void lookupFieldFilter() {
    String suite = TestSetup.primaryKey("Filter", "Lookup");

    // Three Secondary records provide the looked-up text. Primary.lookup
    // pulls from Secondary.value through Primary.linkSingle.
    SecondaryModel secA =
        airtable
            .secondary()
            .create(
                SecondaryModel.builder().name(suite + " Sec A").value("Groundwork BioAg").build());
    SecondaryModel secB =
        airtable
            .secondary()
            .create(
                SecondaryModel.builder().name(suite + " Sec B").value("Groundwork Lab").build());
    SecondaryModel secC =
        airtable
            .secondary()
            .create(SecondaryModel.builder().name(suite + " Sec C").value("Other Vendor").build());
    String secAId = secA.getId();
    String secBId = secB.getId();
    String secCId = secC.getId();
    assertNotNull(secAId, "Missing secondary ids");
    assertNotNull(secBId, "Missing secondary ids");
    assertNotNull(secCId, "Missing secondary ids");
    List<String> secIds = List.of(secAId, secBId, secCId);

    PrimaryModel primA =
        airtable
            .primary()
            .create(
                PrimaryModel.builder()
                    .linkSingle(List.of(secAId))
                    .primaryKey(suite + " A")
                    .build());
    PrimaryModel primB =
        airtable
            .primary()
            .create(
                PrimaryModel.builder()
                    .linkSingle(List.of(secBId))
                    .primaryKey(suite + " B")
                    .build());
    PrimaryModel primC =
        airtable
            .primary()
            .create(
                PrimaryModel.builder()
                    .linkSingle(List.of(secCId))
                    .primaryKey(suite + " C")
                    .build());
    String primAId = primA.getId();
    String primBId = primB.getId();
    String primCId = primC.getId();
    assertNotNull(primAId, "Missing primary ids");
    assertNotNull(primBId, "Missing primary ids");
    assertNotNull(primCId, "Missing primary ids");
    List<String> primIds = List.of(primAId, primBId, primCId);
    String scope = scopeTo(primIds);
    PrimaryFilters f = PrimaryModel.f;

    try {
      // contains — the regression test for the silently-empty bug.
      assertEquals(
          List.of(suite + " A", suite + " B"),
          matchNames(scope, f.lookup.contains("groundwork", false, true)));
      // contains, no match
      assertTrue(
          matchNames(scope, f.lookup.contains("nonexistent-substring-xyz", false, true)).isEmpty());
      // startsWith
      assertEquals(
          List.of(suite + " A", suite + " B"),
          matchNames(scope, f.lookup.startsWith("Ground", false, true)));
      // endsWith
      assertEquals(
          List.of(suite + " A"), matchNames(scope, f.lookup.endsWith("BioAg", false, true)));
      // equals — Airtable already coerces arrays under `=`.
      assertEquals(List.of(suite + " A"), matchNames(scope, f.lookup.eq("Groundwork BioAg")));
      // notContains
      assertEquals(
          List.of(suite + " C"),
          matchNames(scope, f.lookup.notContains("Groundwork", false, true)));

      airtable.primary().deleteAll(primIds);
      airtable.secondary().deleteAll(secIds);
    } catch (Throwable e) {
      try {
        airtable.primary().deleteAll(primIds);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      try {
        airtable.secondary().deleteAll(secIds);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  /** Sorted primary-key names of dict records matching {@code formula} within {@code scope}. */
  private List<String> matchNames(String scope, String formula) {
    List<DictTable.Record> records =
        airtable
            .primary()
            .dict()
            .get(new AirtableQuery().withFormula(Formulas.and(scope, formula)));
    return names(records);
  }

  // endregion

  // region Date field vs date field

  @Test
  void dateFieldVsField() {
    String suite = TestSetup.primaryKey("Filter", "VsField");
    FormulasFilters f = FormulasModel.f;

    List<FormulasModel> created =
        airtable
            .formulas()
            .create(
                List.of(
                    mkFormulas(
                        suite,
                        "Equal",
                        "2024-06-15",
                        "2024-06-15T00:00:00.000Z",
                        "2024-06-15T00:00:00.000Z"),
                    mkFormulas(
                        suite,
                        "FirstBefore",
                        "2024-01-15",
                        "2024-06-15T00:00:00.000Z",
                        "2024-12-15T00:00:00.000Z"),
                    mkFormulas(
                        suite,
                        "FirstAfter",
                        "2024-12-15",
                        "2024-06-15T00:00:00.000Z",
                        "2024-01-15T00:00:00.000Z"),
                    mkFormulas(
                        suite,
                        "Between",
                        "2024-06-15",
                        "2024-01-15T00:00:00.000Z",
                        "2024-12-15T00:00:00.000Z")));
    List<String> ids = created.stream().map(FormulasModel::getId).filter(Objects::nonNull).toList();
    assertEquals(4, ids.size(), "Missing formulas ids");
    String scope = scopeTo(ids);

    try {
      assertEquals(Set.of("Equal"), formulasMatchNames(suite, scope, f.firstDate.on(f.secondDate)));
      assertEquals(
          Set.of("FirstBefore", "FirstAfter", "Between"),
          formulasMatchNames(suite, scope, f.firstDate.notOn(f.secondDate)));
      assertEquals(
          Set.of("FirstBefore"),
          formulasMatchNames(suite, scope, f.firstDate.before(f.secondDate)));
      assertEquals(
          Set.of("FirstAfter", "Between"),
          formulasMatchNames(suite, scope, f.firstDate.after(f.secondDate)));
      assertEquals(
          Set.of("Equal", "FirstBefore"),
          formulasMatchNames(suite, scope, f.firstDate.onOrBefore(f.secondDate)));
      assertEquals(
          Set.of("Equal", "FirstAfter", "Between"),
          formulasMatchNames(suite, scope, f.firstDate.onOrAfter(f.secondDate)));
      assertEquals(
          Set.of("Equal", "Between"),
          formulasMatchNames(suite, scope, f.firstDate.between(f.secondDate, f.thirdDate)));
      assertEquals(
          Set.of("Between"),
          formulasMatchNames(suite, scope, f.firstDate.between(f.secondDate, f.thirdDate, false)));
      assertEquals(
          Set.of("Equal", "FirstBefore"),
          formulasMatchNames(suite, scope, f.firstDate.between("2024-01-01", f.secondDate)));

      airtable.formulas().deleteAll(ids);
    } catch (Throwable e) {
      try {
        airtable.formulas().deleteAll(ids);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  private static FormulasModel mkFormulas(
      String suite, String name, String first, String second, String third) {
    return FormulasModel.builder()
        .firstDate(Objects.requireNonNull(AirtableDateParser.parse(first)))
        .primaryKey(suite + " " + name)
        .secondDate(Objects.requireNonNull(AirtableDateParser.parse(second)))
        .thirdDate(Objects.requireNonNull(AirtableDateParser.parse(third)))
        .build();
  }

  /** Suite-prefix-stripped primary keys of Formulas records matching the scoped formula. */
  private Set<String> formulasMatchNames(String suite, String scope, String formula) {
    List<FormulasModel> models =
        airtable.formulas().get(new AirtableQuery().withFormula(Formulas.and(scope, formula)));
    return models.stream()
        .map(FormulasModel::getPrimaryKey)
        .filter(Objects::nonNull)
        .map(name -> name.replace(suite + " ", ""))
        .collect(Collectors.toSet());
  }

  // endregion
}
