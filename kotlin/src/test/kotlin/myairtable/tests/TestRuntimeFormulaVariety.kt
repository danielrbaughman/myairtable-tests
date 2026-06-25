package myairtable.tests

import kotlinx.coroutines.runBlocking
import myairtable.AirtableDateParser
import myairtable.FormulasModel
import myairtable.S
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

/**
 * TC4 — Runtime-formula input variety. The base TestRuntimeFormulas suite only evaluates the
 * kitchen-sink formulas with ONE fully-populated input set, so the IF(OR(...=BLANK()))
 * short-circuit and varied inputs are never exercised. Each case here creates a Formulas record
 * with a specific input set, fetches the API-computed value, and asserts the transpiled
 * evaluate*() reproduces it.
 *
 * Scope notes (kept deliberately portable across all targets):
 *  - Numbers are positive with LOG/SQRT arguments chosen to be exact (powers of 10 / perfect
 *    squares). The Math formula calls LOG()/SQRT()/EXP() — transcendental results differ by a ULP
 *    between platforms' math libs and Airtable (V8), so irrational results (e.g. LOG(5)) are NOT
 *    bit-identical and would make an exact string compare flaky. Negatives/zero are excluded too:
 *    LOG(negative) and MOD(_,0) error inside this formula.
 *  - Text covers empty-ish edges (whitespace), unicode, and reserved punctuation (exercising the
 *    fixed ENCODE_URL_COMPONENT). An all-blank text input is excluded: Airtable returns blank for
 *    the whole formula (REPLACE past end-of-string errors), while the transpiler is lenient.
 *
 * Parity with C# TestRuntimeFormulaVariety.
 */
class TestRuntimeFormulaVariety {
    private val airtable = TestSetup.makeAirtable()

    private fun base(label: String): FormulasModel =
        FormulasModel(
            primaryKey = "Kotlin Variety $label",
            firstDate = AirtableDateParser.parse("2024-01-01T00:00:00.000Z")!!,
            secondDate = AirtableDateParser.parse("2024-02-01T00:00:00.000Z")!!,
            thirdDate = AirtableDateParser.parse("2024-03-01T00:00:00.000Z")!!,
        )

    // First Number: power of 10 (LOG exact). Second Number: perfect square (SQRT exact).
    private val numberCases =
        listOf(
            NumberCase("hundreds", 100.0, 16.0, 8.0),
            NumberCase("ones", 1.0, 4.0, 2.0),
            NumberCase("tens", 10.0, 25.0, 3.0),
        )

    private data class NumberCase(
        val label: String,
        val a: Double,
        val b: Double,
        val c: Double,
    )

    @Test
    fun mathFormulaMatchesApiForVariedNumbers() =
        runBlocking {
            for (case in numberCases) {
                val model = base("Math ${case.label}")
                model.firstNumber = case.a
                model.secondNumber = case.b
                model.thirdNumber = case.c
                model.firstText = "x"
                model.secondText = "y"
                model.thirdText = "z"
                val created = airtable.formulas.create(model)
                val recordId = created.id
                assertNotNull(recordId, "Missing id on created model")
                try {
                    val fetched = airtable.formulas.get(recordId)
                    val runtime = S(fetched.evaluateMathFormula())
                    println("${case.label}: api='${fetched.mathFormula?.valueOrNull}' runtime='$runtime'")
                    assertEquals(fetched.mathFormula?.valueOrNull, runtime, "Math mismatch for ${case.label}")
                } finally {
                    tryDelete(recordId)
                }
            }
        }

    @Test
    fun mathFormulaBlankBranchWhenNumbersMissing() =
        runBlocking {
            // First/Second Number left null -> OR(BLANK, BLANK) is true -> the formula returns BLANK().
            // This is the IF-true short-circuit that the base suite never reaches.
            val model = base("Blank")
            model.firstText = "x"
            model.secondText = "y"
            model.thirdText = "z"
            val created = airtable.formulas.create(model)
            val recordId = created.id
            assertNotNull(recordId, "Missing id on created model")
            try {
                val fetched = airtable.formulas.get(recordId)
                val apiVal = fetched.mathFormula?.valueOrNull
                val runtime = S(fetched.evaluateMathFormula())
                println("blank: api='$apiVal' runtime='$runtime'")
                assertTrue(apiVal.isNullOrEmpty(), "API expected blank, got '$apiVal'")
                assertTrue(runtime.isEmpty(), "runtime expected blank, got '$runtime'")
            } finally {
                tryDelete(recordId)
            }
        }

    private val textCases =
        listOf(
            TextCase("unicode", "café", "naïve", "日本語🎉"),
            TextCase("whitespace", "  he llo  ", "a b", "c"),
            TextCase("punct", "a.e-i+o", "x/y", "z"), // exercises fixed ENCODE_URL_COMPONENT
        )

    private data class TextCase(
        val label: String,
        val a: String,
        val b: String,
        val c: String,
    )

    @Test
    fun textFormulaMatchesApiForVariedText() =
        runBlocking {
            for (case in textCases) {
                val model = base("Text ${case.label}")
                model.firstNumber = 10.0
                model.secondNumber = 20.0
                model.thirdNumber = 30.0
                model.firstText = case.a
                model.secondText = case.b
                model.thirdText = case.c
                val created = airtable.formulas.create(model)
                val recordId = created.id
                assertNotNull(recordId, "Missing id on created model")
                try {
                    val fetched = airtable.formulas.get(recordId)
                    val runtime = S(fetched.evaluateTextFormula())
                    println("${case.label}: api='${fetched.textFormula?.valueOrNull}' runtime='$runtime'")
                    assertEquals(fetched.textFormula?.valueOrNull, runtime, "Text mismatch for ${case.label}")
                } finally {
                    tryDelete(recordId)
                }
            }
        }

    private suspend fun tryDelete(recordId: String?) {
        if (recordId == null) return
        runCatching { airtable.formulas.delete(recordId) }
    }
}
