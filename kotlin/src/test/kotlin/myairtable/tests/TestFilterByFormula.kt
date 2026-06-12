package myairtable.tests

import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.buildJsonArray
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import myairtable.AirtableDateParser
import myairtable.AirtableQuery
import myairtable.DictTable
import myairtable.Fields
import myairtable.Formulas
import myairtable.FormulasModel
import myairtable.PrimaryFields
import myairtable.PrimaryModel
import myairtable.PrimaryView
import myairtable.SecondaryModel
import myairtable.SortDirection
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * Filter-by-formula integration tests. Parity with Swift
 * TestFilterByFormula / Rust test_filter_by_formula.
 */
class TestFilterByFormula {
    private val airtable = TestSetup.makeAirtable()

    // region Shared helpers (parity with rust scope_to / primary helpers)

    /** Scope a formula to specific record IDs so suite runs don't interfere. */
    private fun scopeTo(ids: List<String>): String {
        val parts = ids.map { "RECORD_ID()='$it'" }
        return if (parts.size == 1) parts[0] else Formulas.or(parts)
    }

    /** Dict-path fields container with just the primary key set. */
    private fun primaryFields(name: String): Fields {
        val fields = Fields(nameToId = PrimaryFields.nameToId)
        fields.setString(PrimaryFields.primaryKeyId, name)
        return fields
    }

    private fun names(records: List<DictTable.Record>): List<String> = records.mapNotNull { it.fields.getString(PrimaryFields.primaryKeyId) }.sorted()

    // endregion

    // region Text field filter

    @Test
    fun textFieldEquals() =
        runBlocking {
            val suite = TestSetup.primaryKey("Filter", "TextEq")
            val created = airtable.primary.create(PrimaryModel(primaryKey = suite, singleLineText = "UniqueFilterValue"))
            val recordId = created.id
            assertNotNull(recordId, "Missing id")

            try {
                val f = PrimaryModel.f
                val filterFormula = f.singleLineText.eq("UniqueFilterValue")
                val results = airtable.primary.get(AirtableQuery(formula = filterFormula))

                assertTrue(results.any { it.id == recordId })

                airtable.primary.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }

    // endregion

    // region Record ID filter

    @Test
    fun recordIdFilter() =
        runBlocking {
            val suite = TestSetup.primaryKey("Filter", "RecordID")
            val created = airtable.primary.create(PrimaryModel(primaryKey = suite))
            val recordId = created.id
            assertNotNull(recordId, "Missing id")

            try {
                val f = PrimaryModel.f
                val filterFormula = f.id.eq(recordId)
                val results = airtable.primary.get(AirtableQuery(formula = filterFormula))

                assertEquals(1, results.size)
                assertEquals(recordId, results.first().id)

                airtable.primary.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }

    // endregion

    // region Number field filter

    @Test
    fun numberFieldFilter() =
        runBlocking {
            val suite = TestSetup.primaryKey("Filter", "Number")
            val c1 = airtable.primary.create(PrimaryModel(numberInt = 10.0, primaryKey = "$suite 1"))
            val c2 = airtable.primary.create(PrimaryModel(numberInt = 20.0, primaryKey = "$suite 2"))
            val id1 = c1.id
            val id2 = c2.id
            assertNotNull(id1, "Missing ids")
            assertNotNull(id2, "Missing ids")

            try {
                val f = PrimaryModel.f
                // Filter for numberInt > 15 — should match only c2.
                val filterFormula = f.numberInt.greaterThan(15)
                val results = airtable.primary.get(AirtableQuery(formula = filterFormula))
                assertTrue(results.any { it.id == id2 })
                assertFalse(results.any { it.id == id1 })

                airtable.primary.delete(listOf(id1, id2))
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(listOf(id1, id2)) }
                throw e
            }
        }

    // endregion

    // region Combinators

    @Test
    fun andCombinator() =
        runBlocking {
            val suite = TestSetup.primaryKey("Filter", "AND")
            val created = airtable.primary.create(PrimaryModel(checkbox = true, numberInt = 42.0, primaryKey = suite))
            val recordId = created.id
            assertNotNull(recordId, "Missing id")

            try {
                val f = PrimaryModel.f
                val filterFormula =
                    Formulas.and(
                        f.checkbox.isTrue(),
                        f.numberInt.eq(42),
                    )
                val results = airtable.primary.get(AirtableQuery(formula = filterFormula))
                assertTrue(results.any { it.id == recordId })

                airtable.primary.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }

    // endregion

    // region Filter by view

    @Test
    fun filterByView() =
        runBlocking {
            val creates = mutableListOf<Fields>()
            for (i in 0 until 5) creates.add(primaryFields("Filter Test $i"))
            for (i in 0 until 5) creates.add(primaryFields("Don't Include Test $i"))
            val created = airtable.primary.dict.create(creates)
            val ids = created.map { it.id }

            try {
                val results = airtable.primary.dict.get(AirtableQuery(view = PrimaryView.FILTER_BY_VIEW.id))
                assertTrue(results.size >= 5)
                for (record in results) {
                    val name = record.fields.getString(PrimaryFields.primaryKeyId) ?: ""
                    assertTrue(name.startsWith("Filter Test"), "Unexpected record: $name")
                }
                airtable.primary.dict.delete(ids)
            } catch (e: Throwable) {
                runCatching { airtable.primary.dict.delete(ids) }
                throw e
            }
        }

    // endregion

    // region Record ID inList

    @Test
    fun recordIdInList() =
        runBlocking {
            val suite = TestSetup.primaryKey("Filter", "InList")
            val created = airtable.primary.dict.create((0 until 3).map { primaryFields("$suite $it") })
            val ids = created.map { it.id }

            try {
                val f = PrimaryModel.f

                // inList multiple
                var results = airtable.primary.dict.get(AirtableQuery(formula = f.id.inList(listOf(ids[0], ids[1]))))
                assertEquals(2, results.size)

                // inList single
                results = airtable.primary.dict.get(AirtableQuery(formula = f.id.inList(listOf(ids[0]))))
                assertEquals(1, results.size)

                // inList empty
                results = airtable.primary.dict.get(AirtableQuery(formula = f.id.inList(emptyList())))
                assertTrue(results.isEmpty())

                airtable.primary.dict.delete(ids)
            } catch (e: Throwable) {
                runCatching { airtable.primary.dict.delete(ids) }
                throw e
            }
        }

    // endregion

    // region Text field predicates

    @Test
    fun textFieldPredicates() =
        runBlocking {
            val created =
                airtable.primary.dict.create(
                    listOf(
                        primaryFields("FbText Alpha One"),
                        primaryFields("FbText Alpha Two"),
                        primaryFields("FbText Beta One"),
                    ),
                )
            val ids = created.map { it.id }
            val scope = scopeTo(ids)
            val f = PrimaryModel.f

            suspend fun count(formula: String): Int =
                airtable.primary.dict
                    .get(AirtableQuery(formula = Formulas.and(scope, formula)))
                    .size

            try {
                assertEquals(1, count(f.primaryKey.eq("FbText Alpha One")))
                assertEquals(2, count(f.primaryKey.neq("FbText Alpha One")))
                assertEquals(2, count(f.primaryKey.contains("Alpha", caseSensitive = false, trim = true)))
                assertEquals(3, count(f.primaryKey.containsAny(listOf("Alpha", "Beta"), caseSensitive = false, trim = true)))
                assertEquals(1, count(f.primaryKey.containsAll(listOf("Alpha", "One"), caseSensitive = false, trim = true)))
                assertEquals(1, count(f.primaryKey.notContains("Alpha", caseSensitive = false, trim = true)))
                assertEquals(2, count(f.primaryKey.startsWith("FbText Alpha", caseSensitive = false, trim = true)))
                assertEquals(1, count(f.primaryKey.notStartsWith("FbText Alpha", caseSensitive = false, trim = true)))

                airtable.primary.dict.delete(ids)
            } catch (e: Throwable) {
                runCatching { airtable.primary.dict.delete(ids) }
                throw e
            }
        }

    // endregion

    // region Number field predicates

    @Test
    fun numberFieldPredicates() =
        runBlocking {
            val suite = TestSetup.primaryKey("Filter", "NumPreds")
            val creates =
                listOf(10L, 20L, 30L).mapIndexed { i, n ->
                    val fields = primaryFields("$suite $i")
                    fields.setLong(PrimaryFields.numberIntId, n)
                    fields
                }
            val created = airtable.primary.dict.create(creates)
            val ids = created.map { it.id }
            val scope = scopeTo(ids)
            val f = PrimaryModel.f

            suspend fun count(formula: String): Int =
                airtable.primary.dict
                    .get(AirtableQuery(formula = Formulas.and(scope, formula)))
                    .size

            try {
                assertEquals(1, count(f.numberInt.eq(20)))
                assertEquals(2, count(f.numberInt.neq(20)))
                assertEquals(2, count(f.numberInt.greaterThan(10)))
                assertEquals(2, count(f.numberInt.lessThan(30)))
                assertEquals(2, count(f.numberInt.greaterThanOrEquals(20)))
                assertEquals(2, count(f.numberInt.lessThanOrEquals(20)))
                assertEquals(3, count(f.numberInt.between(10, 30)))
                assertEquals(1, count(f.numberInt.between(10, 30, inclusive = false)))

                airtable.primary.dict.delete(ids)
            } catch (e: Throwable) {
                runCatching { airtable.primary.dict.delete(ids) }
                throw e
            }
        }

    // endregion

    // region Boolean field

    @Test
    fun booleanFieldFilter() =
        runBlocking {
            val suite = TestSetup.primaryKey("Filter", "Bool")
            val f1 = primaryFields("$suite A")
            f1.setBoolean(PrimaryFields.checkboxId, true)
            val f2 = primaryFields("$suite B")
            f2.setBoolean(PrimaryFields.checkboxId, false)
            val created = airtable.primary.dict.create(listOf(f1, f2))
            val ids = created.map { it.id }
            val scope = scopeTo(ids)
            val f = PrimaryModel.f

            suspend fun count(formula: String): Int =
                airtable.primary.dict
                    .get(AirtableQuery(formula = Formulas.and(scope, formula)))
                    .size

            try {
                assertEquals(1, count(f.checkbox.eq(true)))
                assertTrue(count(f.checkbox.eq(false)) >= 1)
                assertEquals(1, count(f.checkbox.isTrue()))
                assertTrue(count(f.checkbox.isFalse()) >= 1)

                airtable.primary.dict.delete(ids)
            } catch (e: Throwable) {
                runCatching { airtable.primary.dict.delete(ids) }
                throw e
            }
        }

    // endregion

    // region Attachments field

    @Test
    fun attachmentsFieldFilter() =
        runBlocking {
            val suite = TestSetup.primaryKey("Filter", "Attach")
            val f1 = primaryFields("$suite A")
            f1[PrimaryFields.attachmentId] =
                buildJsonArray {
                    add(
                        buildJsonObject {
                            put("url", "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png")
                        },
                    )
                }
            val f2 = primaryFields("$suite B")
            val created = airtable.primary.dict.create(listOf(f1, f2))
            val ids = created.map { it.id }
            val scope = scopeTo(ids)
            val f = PrimaryModel.f

            try {
                val notEmpty = airtable.primary.dict.get(AirtableQuery(formula = Formulas.and(scope, f.attachment.notEmpty())))
                assertTrue(notEmpty.size >= 1)

                val empty = airtable.primary.dict.get(AirtableQuery(formula = Formulas.and(scope, f.attachment.empty())))
                assertTrue(empty.size >= 1)

                airtable.primary.dict.delete(ids)
            } catch (e: Throwable) {
                runCatching { airtable.primary.dict.delete(ids) }
                throw e
            }
        }

    // endregion

    // region Date field predicates

    @Test
    fun dateFieldPredicates() =
        runBlocking {
            val suite = TestSetup.primaryKey("Filter", "Date")
            val f1 = primaryFields("$suite A")
            f1.setString(PrimaryFields.dateId, "2024-01-15")
            val f2 = primaryFields("$suite B")
            f2.setString(PrimaryFields.dateId, "2024-06-15")
            val f3 = primaryFields("$suite C") // no date
            val created = airtable.primary.dict.create(listOf(f1, f2, f3))
            val ids = created.map { it.id }
            val scope = scopeTo(ids)
            val f = PrimaryModel.f

            suspend fun count(formula: String): Int =
                airtable.primary.dict
                    .get(AirtableQuery(formula = Formulas.and(scope, formula)))
                    .size

            try {
                assertEquals(1, count(f.date.on("2024-01-15")))
                assertTrue(count(f.date.notOn("2024-01-15")) >= 1)
                assertTrue(count(f.date.onOrAfter("2024-06-15")) >= 1)
                assertTrue(count(f.date.onOrBefore("2024-01-15")) >= 1)
                assertTrue(count(f.date.after("2024-01-15")) >= 1)
                assertTrue(count(f.date.before("2024-06-15")) >= 1)
                assertEquals(2, count(f.date.between("2024-01-15", "2024-06-15")))
                assertEquals(2, count(f.date.between("2024-01-01", "2024-12-31", inclusive = false)))
                assertTrue(count(f.date.notEmpty()) >= 1)
                assertTrue(count(f.date.empty()) >= 1)

                airtable.primary.dict.delete(ids)
            } catch (e: Throwable) {
                runCatching { airtable.primary.dict.delete(ids) }
                throw e
            }
        }

    // endregion

    // region Date field chained (time-ago)

    @Test
    fun dateFieldChained() =
        runBlocking {
            val suite = TestSetup.primaryKey("Filter", "DateChain")
            val f1 = primaryFields("$suite A")
            f1.setString(PrimaryFields.dateId, "2024-01-15")
            val f2 = primaryFields("$suite B")
            f2.setString(PrimaryFields.dateId, "2024-06-15")
            val created = airtable.primary.dict.create(listOf(f1, f2))
            val ids = created.map { it.id }
            val scope = scopeTo(ids)
            val f = PrimaryModel.f

            try {
                // before().daysAgo(1) — both dates are in the past.
                val past = airtable.primary.dict.get(AirtableQuery(formula = Formulas.and(scope, f.date.before().daysAgo(1))))
                assertTrue(past.size >= 1)

                // after().yearsAgo(100) — both dates are within the last 100 years.
                val recent = airtable.primary.dict.get(AirtableQuery(formula = Formulas.and(scope, f.date.after().yearsAgo(100))))
                assertTrue(recent.size >= 1)

                airtable.primary.dict.delete(ids)
            } catch (e: Throwable) {
                runCatching { airtable.primary.dict.delete(ids) }
                throw e
            }
        }

    // endregion

    // region Complex formulas (AND / OR / NOT / XOR)

    @Test
    fun complexFormulas() =
        runBlocking {
            val suite = TestSetup.primaryKey("Filter", "Complex")
            val f1 = primaryFields("$suite A")
            f1.setLong(PrimaryFields.numberIntId, 10)
            f1.setBoolean(PrimaryFields.checkboxId, true)
            val f2 = primaryFields("$suite B")
            f2.setLong(PrimaryFields.numberIntId, 20)
            f2.setBoolean(PrimaryFields.checkboxId, false)
            val f3 = primaryFields("$suite C")
            f3.setLong(PrimaryFields.numberIntId, 30)
            f3.setBoolean(PrimaryFields.checkboxId, true)
            val created = airtable.primary.dict.create(listOf(f1, f2, f3))
            val ids = created.map { it.id }
            val scope = scopeTo(ids)
            val f = PrimaryModel.f

            suspend fun count(formula: String): Int =
                airtable.primary.dict
                    .get(AirtableQuery(formula = Formulas.and(scope, formula)))
                    .size

            try {
                // AND(number > 10, checkbox) -> only C
                assertEquals(1, count(Formulas.and(f.numberInt.greaterThan(10), f.checkbox.isTrue())))
                // AND(contains, gte) -> B and C
                assertEquals(
                    2,
                    count(
                        Formulas.and(
                            f.primaryKey.contains(suite, caseSensitive = false, trim = true),
                            f.numberInt.greaterThanOrEquals(20),
                        ),
                    ),
                )
                // OR(number = 10, number = 30) -> A and C
                assertEquals(2, count(Formulas.or(f.numberInt.eq(10), f.numberInt.eq(30))))
                // OR(checkbox, number = 20) -> all three
                assertEquals(3, count(Formulas.or(f.checkbox.isTrue(), f.numberInt.eq(20))))
                // XOR(checkbox, number >= 30) -> only A
                assertEquals(1, count(Formulas.xor(f.checkbox.isTrue(), f.numberInt.greaterThanOrEquals(30))))
                // NOT(primaryKey = "A") -> B and C
                assertEquals(2, count(Formulas.not(f.primaryKey.eq("$suite A"))))
                // AND(OR(number=10, number=30), checkbox) -> A and C
                assertEquals(
                    2,
                    count(
                        Formulas.and(
                            Formulas.or(f.numberInt.eq(10), f.numberInt.eq(30)),
                            f.checkbox.isTrue(),
                        ),
                    ),
                )
                // OR(AND(number > 10, checkbox), primaryKey = "A") -> A and C
                assertEquals(
                    2,
                    count(
                        Formulas.or(
                            Formulas.and(f.numberInt.greaterThan(10), f.checkbox.isTrue()),
                            f.primaryKey.eq("$suite A"),
                        ),
                    ),
                )
                // NOT(AND(checkbox, number > 20)) -> A and B
                assertEquals(2, count(Formulas.not(Formulas.and(f.checkbox.isTrue(), f.numberInt.greaterThan(20)))))
                // AND(NOT(checkbox), gte(20)) -> only B
                assertEquals(1, count(Formulas.and(Formulas.not(f.checkbox.isTrue()), f.numberInt.greaterThanOrEquals(20))))

                airtable.primary.dict.delete(ids)
            } catch (e: Throwable) {
                runCatching { airtable.primary.dict.delete(ids) }
                throw e
            }
        }

    // endregion

    // region Max records

    @Test
    fun maxRecords() =
        runBlocking {
            val suite = TestSetup.primaryKey("Filter", "MaxRec")
            val created = airtable.primary.dict.create((1..5).map { primaryFields("$suite $it") })
            val ids = created.map { it.id }

            try {
                val results = airtable.primary.dict.get(AirtableQuery(formula = scopeTo(ids), maxRecords = 3))
                assertEquals(3, results.size)

                airtable.primary.dict.delete(ids)
            } catch (e: Throwable) {
                runCatching { airtable.primary.dict.delete(ids) }
                throw e
            }
        }

    // endregion

    // region Sort

    @Test
    fun sortRecords() =
        runBlocking {
            val suite = TestSetup.primaryKey("Filter", "Sort")
            val f1 = primaryFields("$suite C")
            f1.setLong(PrimaryFields.numberIntId, 30)
            val f2 = primaryFields("$suite A")
            f2.setLong(PrimaryFields.numberIntId, 10)
            val f3 = primaryFields("$suite B")
            f3.setLong(PrimaryFields.numberIntId, 20)
            val created = airtable.primary.dict.create(listOf(f1, f2, f3))
            val ids = created.map { it.id }
            val scope = scopeTo(ids)

            try {
                val asc = airtable.primary.dict.get(AirtableQuery(formula = scope).withSort(PrimaryFields.numberIntId))
                assertEquals(listOf(10L, 20L, 30L), asc.mapNotNull { it.fields.getLong(PrimaryFields.numberIntId) })

                val desc = airtable.primary.dict.get(AirtableQuery(formula = scope).withSort(PrimaryFields.numberIntId, SortDirection.DESC))
                assertEquals(listOf(30L, 20L, 10L), desc.mapNotNull { it.fields.getLong(PrimaryFields.numberIntId) })

                airtable.primary.dict.delete(ids)
            } catch (e: Throwable) {
                runCatching { airtable.primary.dict.delete(ids) }
                throw e
            }
        }

    // endregion

    // region Field selection

    @Test
    fun fieldSelection() =
        runBlocking {
            val suite = TestSetup.primaryKey("Filter", "FieldSel")
            val fields = primaryFields(suite)
            fields.setString(PrimaryFields.singleLineTextId, "Hello")
            fields.setLong(PrimaryFields.numberIntId, 42)
            val created = airtable.primary.dict.create(fields)

            try {
                val results =
                    airtable.primary.dict.get(
                        AirtableQuery(
                            formula = "RECORD_ID()='${created.id}'",
                            fields = listOf(PrimaryFields.primaryKeyId),
                        ),
                    )
                assertEquals(1, results.size)
                val f = results[0].fields
                assertNotNull(f[PrimaryFields.primaryKeyId])
                assertNull(f[PrimaryFields.singleLineTextId])
                assertNull(f[PrimaryFields.numberIntId])

                airtable.primary.dict.delete(created.id)
            } catch (e: Throwable) {
                runCatching { airtable.primary.dict.delete(created.id) }
                throw e
            }
        }

    // endregion

    // region Lookup field (ARRAYJOIN regression)

    @Test
    fun lookupFieldFilter() =
        runBlocking {
            val suite = TestSetup.primaryKey("Filter", "Lookup")

            // Three Secondary records provide the looked-up text. Primary.lookup
            // pulls from Secondary.value through Primary.linkSingle.
            val secA = airtable.secondary.create(SecondaryModel(name = "$suite Sec A", value = "Groundwork BioAg"))
            val secB = airtable.secondary.create(SecondaryModel(name = "$suite Sec B", value = "Groundwork Lab"))
            val secC = airtable.secondary.create(SecondaryModel(name = "$suite Sec C", value = "Other Vendor"))
            val secAId = secA.id
            val secBId = secB.id
            val secCId = secC.id
            assertNotNull(secAId, "Missing secondary ids")
            assertNotNull(secBId, "Missing secondary ids")
            assertNotNull(secCId, "Missing secondary ids")
            val secIds = listOf(secAId, secBId, secCId)

            val primA = airtable.primary.create(PrimaryModel(linkSingle = listOf(secAId), primaryKey = "$suite A"))
            val primB = airtable.primary.create(PrimaryModel(linkSingle = listOf(secBId), primaryKey = "$suite B"))
            val primC = airtable.primary.create(PrimaryModel(linkSingle = listOf(secCId), primaryKey = "$suite C"))
            val primAId = primA.id
            val primBId = primB.id
            val primCId = primC.id
            assertNotNull(primAId, "Missing primary ids")
            assertNotNull(primBId, "Missing primary ids")
            assertNotNull(primCId, "Missing primary ids")
            val primIds = listOf(primAId, primBId, primCId)
            val scope = scopeTo(primIds)
            val f = PrimaryModel.f

            suspend fun matchNames(formula: String): List<String> {
                val records = airtable.primary.dict.get(AirtableQuery(formula = Formulas.and(scope, formula)))
                return names(records)
            }

            try {
                // contains — the regression test for the silently-empty bug.
                assertEquals(
                    listOf("$suite A", "$suite B"),
                    matchNames(f.lookup.contains("groundwork", caseSensitive = false, trim = true)),
                )
                // contains, no match
                assertTrue(matchNames(f.lookup.contains("nonexistent-substring-xyz", caseSensitive = false, trim = true)).isEmpty())
                // startsWith
                assertEquals(
                    listOf("$suite A", "$suite B"),
                    matchNames(f.lookup.startsWith("Ground", caseSensitive = false, trim = true)),
                )
                // endsWith
                assertEquals(
                    listOf("$suite A"),
                    matchNames(f.lookup.endsWith("BioAg", caseSensitive = false, trim = true)),
                )
                // equals — Airtable already coerces arrays under `=`.
                assertEquals(listOf("$suite A"), matchNames(f.lookup.eq("Groundwork BioAg")))
                // notContains
                assertEquals(
                    listOf("$suite C"),
                    matchNames(f.lookup.notContains("Groundwork", caseSensitive = false, trim = true)),
                )

                airtable.primary.delete(primIds)
                airtable.secondary.delete(secIds)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(primIds) }
                runCatching { airtable.secondary.delete(secIds) }
                throw e
            }
        }

    // endregion

    // region Date field vs date field

    @Test
    fun dateFieldVsField() =
        runBlocking {
            val suite = TestSetup.primaryKey("Filter", "VsField")
            val f = FormulasModel.f

            fun mk(
                name: String,
                first: String,
                second: String,
                third: String,
            ): FormulasModel =
                FormulasModel(
                    firstDate = AirtableDateParser.parse(first)!!,
                    primaryKey = "$suite $name",
                    secondDate = AirtableDateParser.parse(second)!!,
                    thirdDate = AirtableDateParser.parse(third)!!,
                )

            val created =
                airtable.formulas.create(
                    listOf(
                        mk("Equal", "2024-06-15", "2024-06-15T00:00:00.000Z", "2024-06-15T00:00:00.000Z"),
                        mk("FirstBefore", "2024-01-15", "2024-06-15T00:00:00.000Z", "2024-12-15T00:00:00.000Z"),
                        mk("FirstAfter", "2024-12-15", "2024-06-15T00:00:00.000Z", "2024-01-15T00:00:00.000Z"),
                        mk("Between", "2024-06-15", "2024-01-15T00:00:00.000Z", "2024-12-15T00:00:00.000Z"),
                    ),
                )
            val ids = created.mapNotNull { it.id }
            assertEquals(4, ids.size, "Missing formulas ids")
            val scope = scopeTo(ids)

            suspend fun matchNames(formula: String): Set<String> {
                val models = airtable.formulas.get(AirtableQuery(formula = Formulas.and(scope, formula)))
                return models.mapNotNull { it.primaryKey?.replace("$suite ", "") }.toSet()
            }

            try {
                assertEquals(setOf("Equal"), matchNames(f.firstDate.on(f.secondDate)))
                assertEquals(setOf("FirstBefore", "FirstAfter", "Between"), matchNames(f.firstDate.notOn(f.secondDate)))
                assertEquals(setOf("FirstBefore"), matchNames(f.firstDate.before(f.secondDate)))
                assertEquals(setOf("FirstAfter", "Between"), matchNames(f.firstDate.after(f.secondDate)))
                assertEquals(setOf("Equal", "FirstBefore"), matchNames(f.firstDate.onOrBefore(f.secondDate)))
                assertEquals(setOf("Equal", "FirstAfter", "Between"), matchNames(f.firstDate.onOrAfter(f.secondDate)))
                assertEquals(setOf("Equal", "Between"), matchNames(f.firstDate.between(f.secondDate, f.thirdDate)))
                assertEquals(setOf("Between"), matchNames(f.firstDate.between(f.secondDate, f.thirdDate, inclusive = false)))
                assertEquals(setOf("Equal", "FirstBefore"), matchNames(f.firstDate.between("2024-01-01", f.secondDate)))

                airtable.formulas.delete(ids)
            } catch (e: Throwable) {
                runCatching { airtable.formulas.delete(ids) }
                throw e
            }
        }

    // endregion
}
