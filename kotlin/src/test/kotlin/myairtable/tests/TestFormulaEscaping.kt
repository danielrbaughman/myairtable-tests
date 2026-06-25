package myairtable.tests

import kotlinx.coroutines.runBlocking
import myairtable.AirtableQuery
import myairtable.Formulas
import myairtable.PrimaryModel
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * TC2 — Formula-value escaping. Filter predicates build formulas by interpolating user
 * values into quoted string literals; a value containing a quote, backslash, or other
 * meta-character must be escaped or the formula breaks (or silently mis-matches). These
 * tests round-trip special-character values through storage AND through the generated
 * `eq()` / `contains()` filter DSL against the live base, proving the escaping is correct.
 * Parity with C# TestFormulaEscaping.
 */
class TestFormulaEscaping {
    private val airtable = TestSetup.makeAirtable()

    // Values that break naive string interpolation into an Airtable formula literal.
    // Newline is covered separately (storage only) — Airtable formula string literals
    // can't carry a raw newline, so it's a storage concern, not a filter one.
    private val specialValues =
        listOf(
            "O'Brien single quote",
            "say \"hi\" double quote",
            "back\\slash path",
            "trailing backslash\\",
            "mixed a\"b'c\\d",
            "unicode café ☕ 日本語 🎉",
        )

    /** Scope a formula to records of this suite run via the primary key. */
    private fun scope(suite: String): String = """FIND("$suite", {Primary Key})"""

    @Test
    fun eqFilterMatchesValueWithSpecialChars() =
        runBlocking {
            val f = PrimaryModel.f
            for (special in specialValues) {
                val suite = TestSetup.primaryKey("Escaping", "Eq")
                val created = airtable.primary.create(PrimaryModel(primaryKey = suite, singleLineText = special))
                val recordId = created.id!!
                try {
                    // 1. Storage round-trips the exact value.
                    assertEquals(special, created.singleLineText, "storage round-trip for: $special")

                    // 2. The eq() filter, scoped to this record, matches via correctly-escaped interpolation.
                    val filter = Formulas.and(scope(suite), f.singleLineText.eq(special))
                    val results = airtable.primary.get(AirtableQuery(formula = filter))
                    assertTrue(results.any { it.id == recordId }, "eq() did not match value: $special (formula: $filter)")
                    assertTrue(
                        results.all { it.singleLineText == special },
                        "eq() matched a mismatched value for: $special (formula: $filter)",
                    )

                    airtable.primary.delete(recordId)
                } catch (e: Throwable) {
                    runCatching { airtable.primary.delete(recordId) }
                    throw e
                }
            }
        }

    @Test
    fun containsFilterMatchesValueWithSpecialChars() =
        runBlocking {
            val f = PrimaryModel.f
            for (special in specialValues) {
                val suite = TestSetup.primaryKey("Escaping", "Contains")
                // Embed the special token inside a longer value so contains (FIND>0) is a real substring test.
                val stored = "prefix $special suffix"
                val created = airtable.primary.create(PrimaryModel(primaryKey = suite, singleLineText = stored))
                val recordId = created.id!!
                try {
                    val filter = Formulas.and(scope(suite), f.singleLineText.contains(special))
                    val results = airtable.primary.get(AirtableQuery(formula = filter))
                    assertTrue(
                        results.any { it.id == recordId },
                        "contains() did not match value: $special (formula: $filter)",
                    )

                    airtable.primary.delete(recordId)
                } catch (e: Throwable) {
                    runCatching { airtable.primary.delete(recordId) }
                    throw e
                }
            }
        }

    @Test
    fun specialCharactersInPrimaryKeyRoundTripAndFilter() =
        runBlocking {
            // The primary key itself carries special chars, and we match on it with eq().
            val suite = TestSetup.primaryKey("Escaping", "PK")
            val pk = "$suite O'Brien \"quote\" back\\slash"
            val created = airtable.primary.create(PrimaryModel(primaryKey = pk))
            val recordId = created.id!!
            try {
                assertEquals(pk, created.primaryKey, "storage round-trip for primary key: $pk")

                val f = PrimaryModel.f
                val filter = f.primaryKey.eq(pk)
                val results = airtable.primary.get(AirtableQuery(formula = filter))
                assertTrue(results.any { it.id == recordId }, "eq() did not match primary key: $pk (formula: $filter)")
                assertEquals(1, results.size, "eq() matched extra records for primary key: $pk (formula: $filter)")

                airtable.primary.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }

    @Test
    fun newlineAndTabValuesRoundTripThroughStorage() =
        runBlocking {
            // Newlines/tabs are a storage concern (long text), not expressible in a formula literal.
            // Verify they survive create -> fetch unchanged.
            val suite = TestSetup.primaryKey("Escaping", "Newline")
            val value = "line1\nline2\twith tab\r\nwindows"
            val created = airtable.primary.create(PrimaryModel(primaryKey = suite, longText = value))
            val recordId = created.id!!
            try {
                val fetched = airtable.primary.get(recordId)
                assertEquals(value, fetched.longText, "newline/tab round-trip through storage")

                airtable.primary.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }
}
