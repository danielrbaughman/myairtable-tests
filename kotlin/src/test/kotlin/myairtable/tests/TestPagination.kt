package myairtable.tests

import kotlinx.coroutines.runBlocking
import myairtable.AirtableException
import myairtable.AirtableQuery
import myairtable.Fields
import myairtable.PrimaryFields
import myairtable.PrimaryModel
import kotlin.test.Test
import kotlin.test.assertEquals

/**
 * TC1 — Multi-page pagination: list more records than Airtable's 100-record default page
 * in a single query (no maxRecords) and assert the client's offset loop reassembles every
 * page. Exercises the untested page-reassembly path in the generated client. Parity with
 * the C# TestPagination and the other suites.
 */
class TestPagination {
    private val airtable = TestSetup.makeAirtable()

    // 105 > the 100-record default page size, so a no-maxRecords list spans two pages.
    private val count = 105

    @Test
    fun ormGetSpanningMultiplePagesReturnsEveryRecord() =
        runBlocking {
            val suite = TestSetup.primaryKey("Pagination", "Orm")
            val models = (1..count).map { PrimaryModel(primaryKey = "$suite $it") }
            var createdIds: List<String> = emptyList()
            try {
                val created = airtable.primary.create(models)
                createdIds = created.mapNotNull { it.id }
                assertEquals(count, created.size)

                // No maxRecords: the offset loop must walk both pages and return all 105.
                val results = airtable.primary.get(AirtableQuery(formula = """FIND("$suite", {Primary Key})"""))
                assertEquals(count, results.size)
                assertEquals(createdIds.toSet(), results.mapNotNull { it.id }.toSet())
            } finally {
                tryDeleteMany(createdIds, suite)
            }
        }

    @Test
    fun dictGetSpanningMultiplePagesReturnsEveryRecord() =
        runBlocking {
            val suite = TestSetup.primaryKey("Pagination", "Dict")
            val creates =
                (1..count).map {
                    val fields = Fields(nameToId = PrimaryFields.nameToId)
                    fields.setString(PrimaryFields.primaryKeyId, "$suite $it")
                    fields
                }
            var createdIds: List<String> = emptyList()
            try {
                val created = airtable.primary.dict.create(creates)
                createdIds = created.map { it.id }
                assertEquals(count, created.size)

                val results = airtable.primary.dict.get(AirtableQuery(formula = """FIND("$suite", {Primary Key})"""))
                assertEquals(count, results.size)
                assertEquals(createdIds.toSet(), results.map { it.id }.toSet())
            } finally {
                tryDeleteMany(createdIds, suite)
            }
        }

    // TC5 — explicit page size. 25 records over pageSize 10 spans 3 pages (10+10+5).
    private val pageSizeCount = 25

    @Test
    fun explicitPageSizeReturnsAllRecordsAcrossPages() =
        runBlocking {
            val suite = TestSetup.primaryKey("Pagination", "PageSize")
            val models = (1..pageSizeCount).map { PrimaryModel(primaryKey = "$suite $it") }
            var createdIds: List<String> = emptyList()
            try {
                val created = airtable.primary.create(models)
                createdIds = created.mapNotNull { it.id }

                // pageSize 10, NO maxRecords: the offset loop must walk all 3 pages and return all 25.
                val results =
                    airtable.primary.get(
                        AirtableQuery(formula = """FIND("$suite", {Primary Key})""", pageSize = 10),
                    )
                assertEquals(pageSizeCount, results.size)
            } finally {
                tryDeleteMany(createdIds, suite)
            }
        }

    @Test
    fun pageSizeWithMaxRecordsCapsTheTotal() =
        runBlocking {
            val suite = TestSetup.primaryKey("Pagination", "PageCap")
            val models = (1..pageSizeCount).map { PrimaryModel(primaryKey = "$suite $it") }
            var createdIds: List<String> = emptyList()
            try {
                val created = airtable.primary.create(models)
                createdIds = created.mapNotNull { it.id }

                // pageSize 10 + maxRecords 15: maxRecords caps the total mid-stream (not a page multiple).
                val results =
                    airtable.primary.get(
                        AirtableQuery(
                            formula = """FIND("$suite", {Primary Key})""",
                            pageSize = 10,
                            maxRecords = 15,
                        ),
                    )
                assertEquals(15, results.size)
            } finally {
                tryDeleteMany(createdIds, suite)
            }
        }

    // ---- cleanup ----

    private suspend fun tryDeleteMany(
        ids: List<String>,
        suite: String,
    ) {
        if (ids.isNotEmpty()) {
            try {
                airtable.primary.dict.delete(ids)
                return
            } catch (_: AirtableException) {
                // fall through to a prefix sweep
            }
        }
        try {
            val stray = airtable.primary.dict.get(AirtableQuery(formula = """FIND("$suite", {Primary Key})"""))
            if (stray.isNotEmpty()) {
                airtable.primary.dict.delete(stray.map { it.id })
            }
        } catch (_: AirtableException) {
            // best-effort cleanup
        }
    }
}
