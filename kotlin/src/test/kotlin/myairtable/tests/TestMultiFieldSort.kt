package myairtable.tests

import kotlinx.coroutines.runBlocking
import myairtable.AirtableQuery
import myairtable.DictTable
import myairtable.Fields
import myairtable.Formulas
import myairtable.PrimaryFields
import myairtable.PrimaryModel
import myairtable.SortDirection
import kotlin.test.Test
import kotlin.test.assertEquals

/**
 * TC11 — Multi-field sort + sort combined with a filter. The base filter suite only covers a
 * single-field sort. This verifies a two-field sort (primary key with ties broken by a secondary
 * key) and sorting within a filtered scope. Parity with C# TestMultiFieldSort.
 */
class TestMultiFieldSort {
    private val airtable = TestSetup.makeAirtable()

    /** Dict-path fields container for a single row. */
    private fun row(
        suite: String,
        number: Long,
        text: String,
    ): Fields {
        val fields = Fields(nameToId = PrimaryFields.nameToId)
        fields.setString(PrimaryFields.primaryKeyId, "$suite $text")
        fields.setLong(PrimaryFields.numberIntId, number)
        fields.setString(PrimaryFields.singleLineTextId, text)
        return fields
    }

    /** Scope a formula to specific record IDs so suite runs don't interfere. */
    private fun scopeTo(ids: List<String>): String = Formulas.or(ids.map { "RECORD_ID()='$it'" })

    private fun texts(records: List<DictTable.Record>): List<String> = records.mapNotNull { it.fields.getString(PrimaryFields.singleLineTextId) }

    @Test
    fun twoFieldSortBreaksTiesOnSecondKey(): Unit =
        runBlocking {
            val suite = TestSetup.primaryKey("Sort", "TwoField")
            // NumberInt ties at 10 (rows "b" and "a"); the secondary SingleLineText sort orders them.
            val created =
                airtable.primary.dict.create(
                    listOf(
                        row(suite, 10, "b"),
                        row(suite, 10, "a"),
                        row(suite, 20, "c"),
                    ),
                )
            val ids = created.map { it.id }
            try {
                val results =
                    airtable.primary.dict.get(
                        AirtableQuery(formula = scopeTo(ids))
                            .withSort(PrimaryFields.numberIntId, SortDirection.ASC)
                            .withSort(PrimaryFields.singleLineTextId, SortDirection.ASC),
                    )
                // (10,a), (10,b), (20,c) — tie on 10 broken by text asc.
                assertEquals(listOf("a", "b", "c"), texts(results))
            } finally {
                tryDeleteMany(ids)
            }
        }

    @Test
    fun secondaryDescendingReversesTiedGroup(): Unit =
        runBlocking {
            val suite = TestSetup.primaryKey("Sort", "MixedDir")
            val created =
                airtable.primary.dict.create(
                    listOf(
                        row(suite, 10, "a"),
                        row(suite, 10, "b"),
                        row(suite, 20, "c"),
                    ),
                )
            val ids = created.map { it.id }
            try {
                val results =
                    airtable.primary.dict.get(
                        AirtableQuery(formula = scopeTo(ids))
                            .withSort(PrimaryFields.numberIntId, SortDirection.ASC)
                            .withSort(PrimaryFields.singleLineTextId, SortDirection.DESC),
                    )
                // NumberInt asc, then text DESC within the 10-tie: (10,b), (10,a), (20,c).
                assertEquals(listOf("b", "a", "c"), texts(results))
            } finally {
                tryDeleteMany(ids)
            }
        }

    @Test
    fun sortCombinedWithAFilter(): Unit =
        runBlocking {
            val suite = TestSetup.primaryKey("Sort", "WithFilter")
            val created =
                airtable.primary.dict.create(
                    listOf(
                        row(suite, 30, "x"),
                        row(suite, 10, "y"),
                        row(suite, 20, "z"),
                        row(suite, 5, "low"), // filtered out by NumberInt > 5
                    ),
                )
            val ids = created.map { it.id }
            try {
                val f = PrimaryModel.f
                val filter = Formulas.and(scopeTo(ids), f.numberInt.greaterThan(5))
                val results =
                    airtable.primary.dict.get(
                        AirtableQuery(formula = filter)
                            .withSort(PrimaryFields.numberIntId, SortDirection.ASC),
                    )
                // Filtered to NumberInt > 5, sorted asc: 10(y), 20(z), 30(x).
                assertEquals(listOf("y", "z", "x"), texts(results))
            } finally {
                tryDeleteMany(ids)
            }
        }

    private suspend fun tryDeleteMany(ids: List<String>) {
        if (ids.isEmpty()) return
        runCatching { airtable.primary.dict.delete(ids) }
    }
}
