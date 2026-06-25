package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.stream.Stream;
import myairtable.Airtable;
import myairtable.AirtableDateParser;
import myairtable.AirtableRuntime;
import myairtable.FormulasModel;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

/**
 * TC4 — Runtime-formula input variety. The base TestRuntimeFormulas suite only evaluates the
 * kitchen-sink formulas with ONE fully-populated input set, so the IF(OR(...=BLANK()))
 * short-circuit and varied inputs are never exercised. Each case here creates a Formulas record
 * with a specific input set, fetches the API-computed value, and asserts the transpiled evaluate*()
 * reproduces it.
 *
 * <p>Scope notes (kept deliberately portable across all 9 targets):
 *
 * <ul>
 *   <li>Numbers are positive with LOG/SQRT arguments chosen to be exact (powers of 10 / perfect
 *       squares). The Math formula calls LOG()/SQRT()/EXP() — transcendental results differ by a
 *       ULP between platforms' math libs and Airtable (V8), so irrational results (e.g. LOG(5)) are
 *       NOT bit-identical and would make an exact string compare flaky. Negatives/zero are excluded
 *       too: LOG(negative) and MOD(_,0) error inside this formula.
 *   <li>Text covers empty-ish edges (whitespace), unicode, and reserved punctuation (exercising the
 *       fixed ENCODE_URL_COMPONENT). An all-blank text input is excluded: Airtable returns blank
 *       for the whole formula (REPLACE past end-of-string errors), while the transpiler is lenient.
 * </ul>
 */
class TestRuntimeFormulaVariety {
  private final Airtable airtable = TestSetup.makeAirtable();

  private static FormulasModel.Builder base(String label) {
    return FormulasModel.builder()
        .primaryKey("Java Variety " + label)
        .firstDate(AirtableDateParser.parse("2024-01-01T00:00:00.000Z"))
        .secondDate(AirtableDateParser.parse("2024-02-01T00:00:00.000Z"))
        .thirdDate(AirtableDateParser.parse("2024-03-01T00:00:00.000Z"));
  }

  // First Number: power of 10 (LOG exact). Second Number: perfect square (SQRT exact).
  static Stream<Arguments> numberCases() {
    return Stream.of(
        Arguments.of("hundreds", 100.0, 16.0, 8.0),
        Arguments.of("ones", 1.0, 4.0, 2.0),
        Arguments.of("tens", 10.0, 25.0, 3.0));
  }

  @ParameterizedTest(name = "math {0}")
  @MethodSource("numberCases")
  void mathFormulaMatchesApiForVariedNumbers(String label, double a, double b, double c) {
    FormulasModel model =
        base("Math " + label)
            .firstNumber(a)
            .secondNumber(b)
            .thirdNumber(c)
            .firstText("x")
            .secondText("y")
            .thirdText("z")
            .build();
    FormulasModel created = airtable.formulas().create(model);
    String recordId = created.getId();
    try {
      FormulasModel fetched = airtable.formulas().get(recordId);
      String runtime = AirtableRuntime.S(fetched.evaluateMathFormula());
      System.out.println(
          label + ": api='" + fetched.getMathFormula().value() + "' runtime='" + runtime + "'");
      assertEquals(fetched.getMathFormula().value(), runtime, "Math formula mismatch");
    } finally {
      tryDelete(recordId);
    }
  }

  @Test
  void mathFormulaBlankBranchWhenNumbersMissing() {
    // First/Second Number left null -> OR(BLANK, BLANK) is true -> the formula returns BLANK().
    // This is the IF-true short-circuit that the base suite never reaches.
    FormulasModel model = base("Blank").firstText("x").secondText("y").thirdText("z").build();
    FormulasModel created = airtable.formulas().create(model);
    String recordId = created.getId();
    try {
      FormulasModel fetched = airtable.formulas().get(recordId);
      String apiVal = fetched.getMathFormula() != null ? fetched.getMathFormula().value() : null;
      String runtime = AirtableRuntime.S(fetched.evaluateMathFormula());
      System.out.println("blank: api='" + apiVal + "' runtime='" + runtime + "'");
      assertTrue(apiVal == null || apiVal.isEmpty(), "API expected blank, got '" + apiVal + "'");
      assertTrue(
          runtime == null || runtime.isEmpty(), "runtime expected blank, got '" + runtime + "'");
    } finally {
      tryDelete(recordId);
    }
  }

  static Stream<Arguments> textCases() {
    return Stream.of(
        Arguments.of("unicode", "café", "naïve", "日本語🎉"),
        Arguments.of("whitespace", "  he llo  ", "a b", "c"),
        // exercises fixed ENCODE_URL_COMPONENT
        Arguments.of("punct", "a.e-i+o", "x/y", "z"));
  }

  @ParameterizedTest(name = "text {0}")
  @MethodSource("textCases")
  void textFormulaMatchesApiForVariedText(String label, String a, String b, String c) {
    FormulasModel model =
        base("Text " + label)
            .firstNumber(10.0)
            .secondNumber(20.0)
            .thirdNumber(30.0)
            .firstText(a)
            .secondText(b)
            .thirdText(c)
            .build();
    FormulasModel created = airtable.formulas().create(model);
    String recordId = created.getId();
    try {
      FormulasModel fetched = airtable.formulas().get(recordId);
      String runtime = AirtableRuntime.S(fetched.evaluateTextFormula());
      System.out.println(
          label + ": api='" + fetched.getTextFormula().value() + "' runtime='" + runtime + "'");
      assertEquals(fetched.getTextFormula().value(), runtime, "Text formula mismatch");
    } finally {
      tryDelete(recordId);
    }
  }

  private void tryDelete(String recordId) {
    if (recordId == null || recordId.isEmpty()) {
      return;
    }
    try {
      airtable.formulas().delete(recordId);
    } catch (RuntimeException ignored) {
      // best-effort cleanup
    }
  }
}
