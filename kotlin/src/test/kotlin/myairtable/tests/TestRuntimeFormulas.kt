package myairtable.tests

import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonNull
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import myairtable.AirtableDateParser
import myairtable.AirtableJson
import myairtable.FormulasFields
import myairtable.FormulasModel
import myairtable.S
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

/**
 * Runtime formula parity: create a record with all input fields, fetch it
 * back with API-computed formula values, and verify the transpiled
 * evaluate*() methods reproduce them. Parity with Swift TestRuntimeFormulas /
 * Rust test_runtime_formulas `runtime_formulas_match_api`.
 */
class TestRuntimeFormulas {
    private val airtable = TestSetup.makeAirtable()

    @Test
    fun runtimeFormulasMatchApi() =
        runBlocking {
            val new =
                FormulasModel(
                    firstDate = AirtableDateParser.parse("2024-01-01T00:00:00.000Z")!!,
                    firstNumber = 10.0,
                    firstText = "Hello",
                    primaryKey = "KotlinRuntimeTest",
                    secondDate = AirtableDateParser.parse("2024-02-01T00:00:00.000Z")!!,
                    secondNumber = 20.0,
                    secondText = "World",
                    thirdDate = AirtableDateParser.parse("2024-03-01T00:00:00.000Z")!!,
                    thirdNumber = 30.0,
                    thirdText = "!",
                )
            val created = airtable.formulas.create(new)
            val recordId = created.id
            assertNotNull(recordId, "Missing id on created model")

            try {
                // Re-fetch to get formula values computed by Airtable.
                val model = airtable.formulas.get(recordId)

                // --- Math formula: exact match ---
                val runtimeMath = S(model.evaluateMathFormula())
                assertEquals(model.mathFormula?.valueOrNull, runtimeMath, "Math formula mismatch")

                // --- Text formula: exact match ---
                val runtimeText = S(model.evaluateTextFormula())
                assertEquals(model.textFormula?.valueOrNull, runtimeText, "Text formula mismatch")

                // --- Date formula: partial match (TODAY/TONOW/FROMNOW are time-dependent) ---
                val apiDate = model.dateFormula?.valueOrNull ?: ""
                val runtimeDate = S(model.evaluateDateFormula())

                // Check all deterministic parts.
                assertTrue(runtimeDate.contains("YEAR: 2024"), "got: $runtimeDate")
                assertTrue(runtimeDate.contains("MONTH: 1"), "got: $runtimeDate")
                assertTrue(runtimeDate.contains("DAY: 1"), "got: $runtimeDate")
                assertTrue(runtimeDate.contains("HOUR: 0"), "got: $runtimeDate")
                assertTrue(runtimeDate.contains("MINUTE: 0"), "got: $runtimeDate")
                assertTrue(runtimeDate.contains("SECOND: 0"), "got: $runtimeDate")
                assertTrue(runtimeDate.contains("WORKDAY_DIFF: "), "got: $runtimeDate")
                assertTrue(runtimeDate.contains("DATEADD+2d: 2024-01-03"), "got: $runtimeDate")
                assertTrue(runtimeDate.contains("IS_BEFORE: Yes"), "got: $runtimeDate")
                assertTrue(runtimeDate.contains("IS_SAME day: No"), "got: $runtimeDate")
                assertTrue(runtimeDate.contains("IS_AFTER: Yes"), "got: $runtimeDate")
                assertTrue(runtimeDate.contains("DATESTR: "), "got: $runtimeDate")
                assertTrue(runtimeDate.contains("TIMESTR: "), "got: $runtimeDate")

                // Compare the deterministic prefix of API vs runtime (before TODAY: which varies).
                val apiPrefix = apiDate.substringBefore(", TODAY:")
                val runtimePrefix = runtimeDate.substringBefore(", TODAY:")
                assertEquals(apiPrefix, runtimePrefix, "Date formula mismatch (pre-TODAY portion)")
            } catch (e: Throwable) {
                runCatching { airtable.formulas.delete(recordId) }
                throw e
            }

            airtable.formulas.delete(recordId)
        }

    @Test
    fun evaluateRunsOffline() {
        // Offline smoke check: a model decoded from a synthetic fields object
        // (with dates) still evaluates every generated method without throwing.
        val fields =
            buildJsonObject {
                put(FormulasFields.primaryKeyId, "RuntimeTest")
                put(FormulasFields.firstNumberId, 10)
                put(FormulasFields.secondNumberId, 20)
                put(FormulasFields.thirdNumberId, 30)
                put(FormulasFields.firstTextId, "Hello")
                put(FormulasFields.secondTextId, "World")
                put(FormulasFields.thirdTextId, "!")
                put(FormulasFields.firstDateId, "2024-01-01")
                put(FormulasFields.secondDateId, "2024-02-01")
                put(FormulasFields.thirdDateId, "2024-03-01")
            }
        val model = AirtableJson.instance.decodeFromJsonElement(FormulasModel.serializer(), fields)

        assertNotEquals(JsonNull, model.evaluateMathFormula())
        val text = S(model.evaluateTextFormula())
        assertTrue(text.contains("Hello"))
        assertTrue(text.contains("World"))
        val date = S(model.evaluateDateFormula())
        assertTrue(date.contains("YEAR: 2024"))
    }
}
