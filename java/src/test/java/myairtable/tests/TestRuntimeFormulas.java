package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.fasterxml.jackson.databind.node.ObjectNode;
import myairtable.Airtable;
import myairtable.AirtableDateParser;
import myairtable.AirtableJson;
import myairtable.AirtableRuntime;
import myairtable.FormulasFields;
import myairtable.FormulasModel;
import org.junit.jupiter.api.Test;

/**
 * J8.11 — Runtime formula parity: create a record with all input fields, fetch it back with
 * API-computed formula values, and verify the transpiled evaluate*() methods reproduce them. Parity
 * with Kotlin/Swift TestRuntimeFormulas / Rust runtime_formulas_match_api.
 */
class TestRuntimeFormulas {
  private final Airtable airtable = TestSetup.makeAirtable();

  @Test
  void runtimeFormulasMatchApi() {
    FormulasModel fresh =
        FormulasModel.builder()
            .firstDate(AirtableDateParser.parse("2024-01-01T00:00:00.000Z"))
            .firstNumber(10.0)
            .firstText("Hello")
            .primaryKey("JavaRuntimeTest")
            .secondDate(AirtableDateParser.parse("2024-02-01T00:00:00.000Z"))
            .secondNumber(20.0)
            .secondText("World")
            .thirdDate(AirtableDateParser.parse("2024-03-01T00:00:00.000Z"))
            .thirdNumber(30.0)
            .thirdText("!")
            .build();
    FormulasModel created = airtable.formulas().create(fresh);
    String recordId = created.getId();
    assertNotNull(recordId, "Missing id on created model");

    try {
      // Re-fetch to get formula values computed by Airtable.
      FormulasModel model = airtable.formulas().get(recordId);

      // --- Math formula: exact match ---
      String runtimeMath = AirtableRuntime.S(model.evaluateMathFormula());
      assertEquals(model.getMathFormula().value(), runtimeMath, "Math formula mismatch");

      // --- Text formula: exact match ---
      String runtimeText = AirtableRuntime.S(model.evaluateTextFormula());
      assertEquals(model.getTextFormula().value(), runtimeText, "Text formula mismatch");

      // --- Date formula: partial match (TODAY/TONOW/FROMNOW are time-dependent) ---
      String apiDate = model.getDateFormula().value() != null ? model.getDateFormula().value() : "";
      String runtimeDate = AirtableRuntime.S(model.evaluateDateFormula());

      // Check all deterministic parts.
      assertTrue(runtimeDate.contains("YEAR: 2024"), "got: " + runtimeDate);
      assertTrue(runtimeDate.contains("MONTH: 1"), "got: " + runtimeDate);
      assertTrue(runtimeDate.contains("DAY: 1"), "got: " + runtimeDate);
      assertTrue(runtimeDate.contains("HOUR: 0"), "got: " + runtimeDate);
      assertTrue(runtimeDate.contains("MINUTE: 0"), "got: " + runtimeDate);
      assertTrue(runtimeDate.contains("SECOND: 0"), "got: " + runtimeDate);
      assertTrue(runtimeDate.contains("WORKDAY_DIFF: "), "got: " + runtimeDate);
      assertTrue(runtimeDate.contains("DATEADD+2d: 2024-01-03"), "got: " + runtimeDate);
      assertTrue(runtimeDate.contains("IS_BEFORE: Yes"), "got: " + runtimeDate);
      assertTrue(runtimeDate.contains("IS_SAME day: No"), "got: " + runtimeDate);
      assertTrue(runtimeDate.contains("IS_AFTER: Yes"), "got: " + runtimeDate);
      assertTrue(runtimeDate.contains("DATESTR: "), "got: " + runtimeDate);
      assertTrue(runtimeDate.contains("TIMESTR: "), "got: " + runtimeDate);

      // Compare the deterministic prefix of API vs runtime (before TODAY: which varies).
      String apiPrefix = apiDate.split(", TODAY:")[0];
      String runtimePrefix = runtimeDate.split(", TODAY:")[0];
      assertEquals(apiPrefix, runtimePrefix, "Date formula mismatch (pre-TODAY portion)");
    } catch (Throwable e) {
      try {
        airtable.formulas().delete(recordId);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }

    airtable.formulas().delete(recordId);
  }

  @Test
  void evaluateRunsOffline() throws Exception {
    // Offline smoke check: a model decoded from a synthetic fields object
    // (with dates) still evaluates every generated method without throwing.
    ObjectNode fields = AirtableJson.MAPPER.createObjectNode();
    fields.put(FormulasFields.primaryKeyId, "RuntimeTest");
    fields.put(FormulasFields.firstNumberId, 10);
    fields.put(FormulasFields.secondNumberId, 20);
    fields.put(FormulasFields.thirdNumberId, 30);
    fields.put(FormulasFields.firstTextId, "Hello");
    fields.put(FormulasFields.secondTextId, "World");
    fields.put(FormulasFields.thirdTextId, "!");
    fields.put(FormulasFields.firstDateId, "2024-01-01");
    fields.put(FormulasFields.secondDateId, "2024-02-01");
    fields.put(FormulasFields.thirdDateId, "2024-03-01");
    FormulasModel model = AirtableJson.MAPPER.treeToValue(fields, FormulasModel.class);

    assertFalse(model.evaluateMathFormula().isNull());
    String text = AirtableRuntime.S(model.evaluateTextFormula());
    assertTrue(text.contains("Hello"));
    assertTrue(text.contains("World"));
    String date = AirtableRuntime.S(model.evaluateDateFormula());
    assertTrue(date.contains("YEAR: 2024"), "got: " + date);
  }
}
