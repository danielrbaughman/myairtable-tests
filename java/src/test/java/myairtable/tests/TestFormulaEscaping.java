package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.List;
import myairtable.Airtable;
import myairtable.AirtableQuery;
import myairtable.Formulas;
import myairtable.PrimaryFilters;
import myairtable.PrimaryModel;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

/**
 * TC2 — Formula-value escaping. Filter predicates build formulas by interpolating user values into
 * quoted string literals; a value containing a quote, backslash, or other meta-character must be
 * escaped or the formula breaks (or silently mis-matches). These tests round-trip special-character
 * values through storage AND through the generated {@code eq()/contains()} filter DSL against the
 * live base, proving the escaping is correct. Parity with C# TestFormulaEscaping.
 */
class TestFormulaEscaping {
  private final Airtable airtable = TestSetup.makeAirtable();

  // Values that break naive string interpolation into an Airtable formula literal.
  // Newline is covered separately (storage only) — Airtable formula string literals can't carry a
  // raw newline, so it's a storage concern, not a filter one.
  static final String SINGLE_QUOTE = "O'Brien single quote";
  static final String DOUBLE_QUOTE = "say \"hi\" double quote";
  static final String BACKSLASH = "back\\slash path";
  static final String TRAILING_BACKSLASH = "trailing backslash\\";
  static final String MIXED = "mixed a\"b'c\\d";
  static final String UNICODE = "unicode café ☕ 日本語 🎉";

  @ParameterizedTest
  @ValueSource(
      strings = {
        SINGLE_QUOTE,
        DOUBLE_QUOTE,
        BACKSLASH,
        TRAILING_BACKSLASH,
        MIXED,
        UNICODE,
      })
  void eqFilterMatchesValueWithSpecialChars(String special) {
    String suite = TestSetup.primaryKey("Escaping", "Eq");
    PrimaryModel created =
        airtable
            .primary()
            .create(PrimaryModel.builder().primaryKey(suite).singleLineText(special).build());
    String recordId = created.getId();
    assertNotNull(recordId, "Missing id");

    try {
      // 1. Storage round-trips the exact value.
      assertEquals(special, created.getSingleLineText());

      // 2. The eq() filter, scoped to this record, matches via correctly-escaped interpolation.
      PrimaryFilters f = PrimaryModel.f;
      String filter =
          Formulas.and("FIND(\"" + suite + "\", {Primary Key})", f.singleLineText.eq(special));
      List<PrimaryModel> results = airtable.primary().get(new AirtableQuery().withFormula(filter));

      assertTrue(
          results.stream().anyMatch(r -> recordId.equals(r.getId())),
          "eq filter did not match record for value: " + special);
      for (PrimaryModel r : results) {
        assertEquals(special, r.getSingleLineText());
      }

      airtable.primary().delete(recordId);
    } catch (Throwable e) {
      tryDelete(recordId);
      throw e;
    }
  }

  @ParameterizedTest
  @ValueSource(
      strings = {
        SINGLE_QUOTE,
        DOUBLE_QUOTE,
        BACKSLASH,
        TRAILING_BACKSLASH,
        MIXED,
        UNICODE,
      })
  void containsFilterMatchesValueWithSpecialChars(String special) {
    String suite = TestSetup.primaryKey("Escaping", "Contains");
    // Embed the special token inside a longer value so contains (FIND>0) is a real substring test.
    String stored = "prefix " + special + " suffix";
    PrimaryModel created =
        airtable
            .primary()
            .create(PrimaryModel.builder().primaryKey(suite).singleLineText(stored).build());
    String recordId = created.getId();
    assertNotNull(recordId, "Missing id");

    try {
      PrimaryFilters f = PrimaryModel.f;
      String filter =
          Formulas.and(
              "FIND(\"" + suite + "\", {Primary Key})", f.singleLineText.contains(special));
      List<PrimaryModel> results = airtable.primary().get(new AirtableQuery().withFormula(filter));

      assertTrue(
          results.stream().anyMatch(r -> recordId.equals(r.getId())),
          "contains filter did not match record for value: " + special);

      airtable.primary().delete(recordId);
    } catch (Throwable e) {
      tryDelete(recordId);
      throw e;
    }
  }

  @Test
  void specialCharactersInPrimaryKeyRoundTripAndFilter() {
    // The primary key itself carries special chars, and we match on it with eq().
    String suite = TestSetup.primaryKey("Escaping", "PK");
    String pk = suite + " O'Brien \"quote\" back\\slash";
    PrimaryModel created = airtable.primary().create(PrimaryModel.builder().primaryKey(pk).build());
    String recordId = created.getId();
    assertNotNull(recordId, "Missing id");

    try {
      assertEquals(pk, created.getPrimaryKey());
      PrimaryFilters f = PrimaryModel.f;
      List<PrimaryModel> results =
          airtable.primary().get(new AirtableQuery().withFormula(f.primaryKey.eq(pk)));

      assertTrue(
          results.stream().anyMatch(r -> recordId.equals(r.getId())),
          "eq filter did not match primary key: " + pk);
      assertEquals(1, results.size());

      airtable.primary().delete(recordId);
    } catch (Throwable e) {
      tryDelete(recordId);
      throw e;
    }
  }

  @Test
  void newlineAndTabValuesRoundTripThroughStorageAndFieldSelection() {
    // Newlines/tabs are a storage concern (long text), not expressible in a formula literal.
    // Verify they survive create -> fetch unchanged.
    String suite = TestSetup.primaryKey("Escaping", "Newline");
    String value = "line1\nline2\twith tab\r\nwindows";
    PrimaryModel created =
        airtable.primary().create(PrimaryModel.builder().primaryKey(suite).longText(value).build());
    String recordId = created.getId();
    assertNotNull(recordId, "Missing id");

    try {
      PrimaryModel fetched = airtable.primary().get(recordId);
      assertEquals(value, fetched.getLongText());

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
