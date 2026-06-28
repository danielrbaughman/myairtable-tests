package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;

import java.util.List;
import myairtable.Airtable;
import myairtable.AirtableRuntime;
import myairtable.PrimaryModel;
import myairtable.PrimaryMultipleSelectOption;
import myairtable.PrimarySingleSelectOption;
import myairtable.VecOrValue;
import org.junit.jupiter.api.Test;

/**
 * TC6 — Primary "Formula (Complex)"/"Formula (Nested)" evaluated at runtime. The base runtime suite
 * only covers the Formulas table; the Primary complex formula concatenates ~35 fields through
 * IF(field, field, "None"), a richer transpile path never checked against the API before.
 *
 * <p>We compare the transpiled evaluateFormulaComplex() to the API line-by-line for the
 * DETERMINISTIC, offline-reproducible field types (text, checkbox, single/multi select, numbers,
 * currency, email, url, phone). The formula also references server-computed fields (Created/Last
 * Modified Time + By, Auto Number, Button, Formula(ID)/(Simple)) and link/lookup/rollup — Airtable
 * renders those from data the offline runtime doesn't hold (collaborator names, linked-record
 * display values, special wrappers), so those lines are NOT expected to match offline. See
 * myairtable-5b0n.
 *
 * <p>This suite specifically locks in the multi-select array-join fix (myairtable-bb7f): a
 * multi-value field coerces to "Option 1, Option 2", not just the first element.
 */
class TestPrimaryFormulaRuntime {
  private final Airtable airtable = TestSetup.makeAirtable();

  // Field labels whose rendering the offline runtime can reproduce exactly.
  private static final String[] DETERMINISTIC_LABELS = {
    "Single Line Text",
    "Long Text",
    "Checkbox",
    "Multiple Select",
    "Single Select",
    "Number (int)",
    "Number (float)",
    "Currency (int)",
    "Currency (float)",
    "Email",
    "URL",
    "Phone Number",
  };

  private static PrimaryModel newRecord(String suite) {
    return PrimaryModel.builder()
        .primaryKey(suite)
        .singleLineText("hello")
        .longText("long text")
        .email("a@b.co")
        .url("https://x.co")
        .phoneNumber("555-1212")
        .checkbox(true)
        .numberInt(42.0)
        .numberFloat(3.5)
        .currencyInt(10.0)
        .currencyFloat(9.99)
        .singleSelect(PrimarySingleSelectOption.CHOICE_1)
        .multipleSelect(
            List.of(PrimaryMultipleSelectOption.OPTION_1, PrimaryMultipleSelectOption.OPTION_2))
        .build();
  }

  /** Extract the "Label: value" line for {@code label} from a formula result. */
  private static String line(String formula, String label) {
    for (String line : formula.split("\n")) {
      if (line.startsWith(label + ": ")) {
        return line;
      }
    }
    return "<missing: " + label + ">";
  }

  @Test
  void complexFormulaRendersDeterministicFieldsLikeApi() {
    String suite = TestSetup.primaryKey("PrimaryFormula", "Complex");
    PrimaryModel created = airtable.primary().create(newRecord(suite));
    String recordId = created.getId();
    assertNotNull(recordId, "Missing id on created model");
    try {
      PrimaryModel fetched = airtable.primary().get(recordId);
      List<String> apiValues = VecOrValue.cleanValues(fetched.getFormulaComplex());
      String api = apiValues.isEmpty() ? "" : apiValues.get(0);
      String runtime = AirtableRuntime.S(fetched.evaluateFormulaComplex());

      for (String label : DETERMINISTIC_LABELS) {
        assertEquals(line(api, label), line(runtime, label), "Field line mismatch: " + label);
      }

      // The multi-select join is the headline fix: both sides render all options, comma-joined.
      assertEquals("Multiple Select: Option 1, Option 2", line(runtime, "Multiple Select"));

      airtable.primary().delete(recordId);
    } catch (Throwable e) {
      tryDelete(recordId);
      throw e;
    }
  }

  @Test
  void nestedFormulaEvaluatesWithoutThrowing() {
    // Formula (Nested) = Formula(ID) & Formula(Simple) & Formula(Complex) — it chains three
    // COMPUTED formula fields. Offline the runtime can't reproduce computed-field values (they
    // decode as special/wrapped types it doesn't re-evaluate), so the content isn't asserted; this
    // confirms the transpiled nested-formula method is generated and evaluates without error. See
    // myairtable-5b0n.
    String suite = TestSetup.primaryKey("PrimaryFormula", "Nested");
    PrimaryModel created = airtable.primary().create(newRecord(suite));
    String recordId = created.getId();
    assertNotNull(recordId, "Missing id on created model");
    try {
      PrimaryModel fetched = airtable.primary().get(recordId);
      String runtime = AirtableRuntime.S(fetched.evaluateFormulaNested()); // must not throw
      assertNotNull(runtime);

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
