package myairtable.tests

import kotlinx.coroutines.runBlocking
import myairtable.PrimaryModel
import myairtable.PrimaryMultipleSelectOption
import myairtable.PrimarySingleSelectOption
import myairtable.S
import myairtable.cleanValues
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull

/**
 * TC6 — Primary "Formula (Complex)"/"Formula (Nested)" evaluated at runtime. The base runtime
 * suite only covers the Formulas table; the Primary complex formula concatenates ~35 fields
 * through IF(field, field, "None"), a richer transpile path never checked against the API before.
 *
 * We compare the transpiled evaluateFormulaComplex() to the API line-by-line for the
 * DETERMINISTIC, offline-reproducible field types (text, checkbox, single/multi select, numbers,
 * currency, email, url, phone). The formula also references server-computed fields (Created/Last
 * Modified Time + By, Auto Number, Button, Formula(ID)/(Simple)) and link/lookup/rollup — Airtable
 * renders those from data the offline runtime doesn't hold, so those lines are NOT expected to
 * match offline. See myairtable-5b0n.
 *
 * This suite specifically locks in the multi-select array-join fix (myairtable-bb7f): a multi-value
 * field coerces to "Option 1, Option 2", not just the first element.
 *
 * Parity with C# TestPrimaryFormulaRuntime.
 */
class TestPrimaryFormulaRuntime {
    private val airtable = TestSetup.makeAirtable()

    private fun newRecord(suite: String): PrimaryModel =
        PrimaryModel(
            primaryKey = suite,
            singleLineText = "hello",
            longText = "long text",
            email = "a@b.co",
            url = "https://x.co",
            phoneNumber = "555-1212",
            checkbox = true,
            numberInt = 42.0,
            numberFloat = 3.5,
            currencyInt = 10.0,
            currencyFloat = 9.99,
            singleSelect = PrimarySingleSelectOption.CHOICE_1,
            multipleSelect = listOf(PrimaryMultipleSelectOption.OPTION_1, PrimaryMultipleSelectOption.OPTION_2),
        )

    @Test
    fun complexFormulaRendersDeterministicFieldsLikeApi() =
        runBlocking {
            val suite = TestSetup.primaryKey("PrimaryFormula", "Complex")
            val created = airtable.primary.create(newRecord(suite))
            val recordId = created.id!!
            try {
                val fetched = airtable.primary.get(recordId)
                val api = fetched.formulaComplex.cleanValues.firstOrNull() ?: ""
                val runtime = S(fetched.evaluateFormulaComplex())

                for (label in deterministicLabels) {
                    assertEquals(line(api, label), line(runtime, label), "field line mismatch for '$label'")
                }

                // The multi-select join is the headline fix: both sides render all options, comma-joined.
                assertEquals("Multiple Select: Option 1, Option 2", line(runtime, "Multiple Select"))

                airtable.primary.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }

    @Test
    fun nestedFormulaEvaluatesWithoutThrowing() =
        runBlocking {
            // Formula (Nested) = Formula(ID) & Formula(Simple) & Formula(Complex) — it chains three
            // COMPUTED formula fields. Offline the runtime can't reproduce computed-field values, so
            // the content isn't asserted; this confirms the transpiled nested-formula method is
            // generated and evaluates without error. See myairtable-5b0n.
            val suite = TestSetup.primaryKey("PrimaryFormula", "Nested")
            val created = airtable.primary.create(newRecord(suite))
            val recordId = created.id!!
            try {
                val fetched = airtable.primary.get(recordId)
                val runtime = S(fetched.evaluateFormulaNested()) // must not throw
                assertNotNull(runtime)

                airtable.primary.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }

    companion object {
        // Field labels whose rendering the offline runtime can reproduce exactly.
        private val deterministicLabels =
            listOf(
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
            )

        /** Extract the "Label: value" line for [label] from a formula result. */
        private fun line(
            formula: String,
            label: String,
        ): String {
            for (l in formula.split('\n')) {
                if (l.startsWith("$label: ")) return l
            }
            return "<missing: $label>"
        }
    }
}
