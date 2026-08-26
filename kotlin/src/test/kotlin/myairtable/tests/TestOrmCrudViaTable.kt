package myairtable.tests

import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonPrimitive
import myairtable.AirtableException
import myairtable.AirtableQuery
import myairtable.PrimaryFields
import myairtable.PrimaryModel
import myairtable.PrimaryMultipleSelectOption
import myairtable.PrimarySingleSelectOption
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertFalse
import kotlin.test.assertNotEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue
import kotlin.time.Duration.Companion.seconds

/**
 * K4.7 — Typed ORM CRUD via table methods. Parity with Swift
 * TestOrmCrudViaTable / Rust test_orm_crud_via_table.
 */
class TestOrmCrudViaTable {
    private val airtable = TestSetup.makeAirtable()

    @Test
    fun primaryKeyOnlyCrudRoundTrip() =
        runBlocking {
            val primaryKey = TestSetup.primaryKey("OrmTable", "PKOnly")
            val created = airtable.primary.create(PrimaryModel(primaryKey = primaryKey))
            assertNotNull(created.id)
            assertEquals(primaryKey, created.primaryKey)

            val recordId = created.id!!
            try {
                val fetched = airtable.primary.get(recordId)
                assertEquals(recordId, fetched.id)
                assertEquals(primaryKey, fetched.primaryKey)
                assertTrue(fetched.dirtyFields().isEmpty(), "fresh decode has no dirty fields")

                fetched.primaryKey = "$primaryKey Updated"
                assertFalse(fetched.dirtyFields().isEmpty())
                val updated = airtable.primary.update(fetched)
                assertEquals("$primaryKey Updated", updated.primaryKey)

                airtable.primary.delete(recordId)
                val deleted = runCatching { airtable.primary.get(recordId) }.isFailure
                assertTrue(deleted)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }

    @Test
    fun allSimplePropertiesRoundTrip() =
        runBlocking {
            val primaryKey = TestSetup.primaryKey("OrmTable", "AllProps")
            val model =
                PrimaryModel(
                    primaryKey = primaryKey,
                    singleLineText = "Hello World",
                    longText = "Long text content",
                    email = "test@example.com",
                    url = "https://example.com",
                    phoneNumber = "555-1234",
                    checkbox = true,
                    numberInt = 42.0,
                    numberFloat = 3.14,
                    currencyInt = 10.0,
                    currencyFloat = 9.99,
                    percentInt = 0.5,
                    percentFloat = 0.333,
                    duration = 3600.seconds,
                )
            val created = airtable.primary.create(model)
            val recordId = created.id!!
            try {
                assertEquals(primaryKey, created.primaryKey)
                assertEquals("Hello World", created.singleLineText)
                assertEquals("test@example.com", created.email)
                assertEquals(true, created.checkbox)
                assertEquals(42.0, created.numberInt)
                assertEquals(3.14, created.numberFloat)
                assertEquals(9.99, created.currencyFloat)
                assertEquals("https://example.com", created.url)
                assertEquals("555-1234", created.phoneNumber)
                assertEquals<kotlin.time.Duration?>(3600.seconds, created.duration, "duration wire unit is seconds")
                airtable.primary.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }

    @Test
    fun selectEnumsAndRatingRoundTrip() =
        runBlocking {
            val primaryKey = TestSetup.primaryKey("OrmTable", "Selects")
            val model =
                PrimaryModel(
                    primaryKey = primaryKey,
                    singleSelect = PrimarySingleSelectOption.CHOICE_1,
                    multipleSelect = listOf(PrimaryMultipleSelectOption.OPTION_1, PrimaryMultipleSelectOption.OPTION_2),
                    rating = 3L,
                )
            val created = airtable.primary.create(model)
            val recordId = created.id!!
            try {
                assertEquals(PrimarySingleSelectOption.CHOICE_1, created.singleSelect)
                assertEquals(listOf(PrimaryMultipleSelectOption.OPTION_1, PrimaryMultipleSelectOption.OPTION_2), created.multipleSelect)
                assertEquals(3L, created.rating)

                val fetched = airtable.primary.get(recordId)
                assertEquals(PrimarySingleSelectOption.CHOICE_1, fetched.singleSelect)

                fetched.singleSelect = PrimarySingleSelectOption.CHOICE_2
                fetched.multipleSelect = listOf(PrimaryMultipleSelectOption.OPTION_3)
                fetched.rating = 5L
                val updated = airtable.primary.update(fetched)
                assertEquals(PrimarySingleSelectOption.CHOICE_2, updated.singleSelect)
                assertEquals(listOf(PrimaryMultipleSelectOption.OPTION_3), updated.multipleSelect)
                assertEquals(5L, updated.rating)

                airtable.primary.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }

    @Test
    fun getQueryReturnsFilteredRecordSet() =
        runBlocking {
            val suite = TestSetup.primaryKey("OrmTable", "List")
            val models = (1..3).map { PrimaryModel(primaryKey = "$suite $it") }
            val created = airtable.primary.create(models)
            val createdIds = created.mapNotNull { it.id }
            try {
                assertEquals(3, created.size)
                val results = airtable.primary.get(AirtableQuery(formula = """FIND("$suite", {Primary Key})"""))
                assertEquals(3, results.size)
                assertEquals(createdIds.toSet(), results.mapNotNull { it.id }.toSet())
                airtable.primary.delete(createdIds)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(createdIds) }
                throw e
            }
        }

    @Test
    fun fieldSelectionAndMaxRecordsApply() =
        runBlocking {
            val suite = TestSetup.primaryKey("OrmTable", "Select")
            val models = (1..3).map { PrimaryModel(primaryKey = "$suite $it", singleLineText = "text $it") }
            val created = airtable.primary.create(models)
            val createdIds = created.mapNotNull { it.id }
            try {
                val results =
                    airtable.primary.get(
                        AirtableQuery(
                            formula = """FIND("$suite", {Primary Key})""",
                            fields = listOf(PrimaryFields.primaryKeyId),
                            maxRecords = 2,
                        ),
                    )
                assertEquals(2, results.size, "maxRecords caps the result set")
                for (model in results) {
                    assertNotNull(model.primaryKey)
                    kotlin.test.assertNull(model.singleLineText, "unselected fields decode as null")
                }
                airtable.primary.delete(createdIds)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(createdIds) }
                throw e
            }
        }

    @Test
    fun invalidRecordIdThrows(): Unit =
        runBlocking {
            assertFailsWith<AirtableException> {
                airtable.primary.get("recDOESNOTEXIST123")
            }
        }

    @Test
    fun batchCreateUpdateDeleteChunksAboveTheBatchLimit() =
        runBlocking {
            val suite = TestSetup.primaryKey("OrmTable", "Batch")
            val count = 111
            val models = (1..count).map { PrimaryModel(primaryKey = "$suite $it") }
            val created = airtable.primary.create(models)
            val createdIds = created.mapNotNull { it.id }
            try {
                assertEquals(count, created.size)
                created.forEachIndexed { i, model -> assertEquals("$suite ${i + 1}", model.primaryKey) }

                for (model in created) model.primaryKey = "${model.primaryKey} Updated"
                val updated = airtable.primary.update(created)
                assertEquals(count, updated.size)
                updated.forEachIndexed { i, model -> assertEquals("$suite ${i + 1} Updated", model.primaryKey) }

                airtable.primary.delete(createdIds)
                assertFailsWith<AirtableException> { airtable.primary.get(createdIds.first()) }
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(createdIds) }
                throw e
            }
            Unit
        }

    @Test
    fun upsertInsertsWhenNoMatchThenUpdatesOnMatch() =
        runBlocking {
            val primaryKey = TestSetup.primaryKey("OrmTable", "Upsert")
            // Insert path
            val first = airtable.primary.upsert(PrimaryModel(primaryKey = primaryKey), listOf(PrimaryFields.primaryKeyId))
            val recordId = first.model.id!!
            try {
                assertTrue(first.wasCreated, "no match yet — upsert inserts")
                assertEquals(primaryKey, first.model.primaryKey)

                // Update path: same primary key, new field value.
                val second =
                    airtable.primary.upsert(
                        PrimaryModel(primaryKey = primaryKey, singleLineText = "merged"),
                        listOf(PrimaryFields.primaryKeyId),
                    )
                assertFalse(second.wasCreated, "matched on primary key — upsert updates")
                assertEquals(recordId, second.model.id)
                assertEquals("merged", second.model.singleLineText)

                airtable.primary.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }

    // Duplicate — parity with the other language duplicate suites.
    @Test
    fun duplicateCopiesWritableFieldsAndRecomputesTheRest() =
        runBlocking {
            val primaryKey = TestSetup.primaryKey("Duplicate", "Source")
            val source =
                airtable.primary.create(
                    PrimaryModel(
                        primaryKey = primaryKey,
                        singleLineText = "copy me",
                        // Stays <= 10 and != 20 on purpose: filter-by-formula asserts exact counts
                        // for `numberInt = 20` and `AND(numberInt > 10, checkbox = true)`.
                        numberInt = 7.0,
                        rating = 3L,
                    ),
                )
            val sourceId = source.id!!
            val copy = airtable.primary.duplicate(source)
            val copyId = copy.id!!
            try {
                assertNotEquals(sourceId, copyId)
                assertEquals(primaryKey, copy.primaryKey)
                assertEquals("copy me", copy.singleLineText)
                assertEquals(7.0, copy.numberInt)
                assertEquals(3L, copy.rating)
                // Formula (ID) resolves to RECORD_ID(), so on a true copy it is the COPY's id --
                // proof the computed fields were recalculated rather than copied.
                assertEquals(copyId, copy.formulaId?.valueOrNull)
                assertNotEquals(source.autoNumber?.valueOrNull, copy.autoNumber?.valueOrNull)
            } finally {
                airtable.primary.delete(listOf(sourceId, copyId))
            }
        }

    @Test
    fun duplicateByIdsPreservesInputOrder() =
        runBlocking {
            val aKey = TestSetup.primaryKey("Duplicate", "OrderA")
            val bKey = TestSetup.primaryKey("Duplicate", "OrderB")
            val a = airtable.primary.create(PrimaryModel(primaryKey = aKey))
            val b = airtable.primary.create(PrimaryModel(primaryKey = bKey))
            val copies = airtable.primary.duplicate(listOf(b.id!!, a.id!!))
            try {
                assertEquals(2, copies.size)
                assertEquals(bKey, copies[0].primaryKey)
                assertEquals(aKey, copies[1].primaryKey)
            } finally {
                airtable.primary.delete(listOf(a.id!!, b.id!!) + copies.map { it.id!! })
            }
        }
}
