package myairtable.tests

import kotlinx.coroutines.runBlocking
import myairtable.PrimaryModel
import myairtable.SecondaryModel
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * K6.2 — Linked records as raw record-ID lists (Swift/Rust final shape — no
 * wrapper type). Resolution happens by passing the IDs to the linked table's
 * `get`. Parity with Swift TestLinkedRecords.
 */
class TestLinkedRecords {
    private val airtable = TestSetup.makeAirtable()

    @Test
    fun linkedRecordFieldsRoundTripViaRecordIds() =
        runBlocking {
            val suite = TestSetup.primaryKey("Linked", "RoundTrip")
            val sec1 = airtable.secondary.create(SecondaryModel(name = "$suite S1"))
            val sec2 = airtable.secondary.create(SecondaryModel(name = "$suite S2"))
            val sec1Id = sec1.id!!
            val sec2Id = sec2.id!!

            val prim =
                airtable.primary.create(
                    PrimaryModel(
                        primaryKey = suite,
                        linkSingle = listOf(sec1Id),
                        linkMultiple = listOf(sec1Id, sec2Id),
                    ),
                )
            val primId = prim.id!!
            try {
                assertEquals(listOf(sec1Id), prim.linkSingle)
                assertEquals(listOf(sec1Id, sec2Id), prim.linkMultiple)

                val fetched = airtable.primary.get(primId)
                assertEquals(listOf(sec1Id), fetched.linkSingle)
                assertEquals(listOf(sec1Id, sec2Id), fetched.linkMultiple)

                fetched.linkSingle = listOf(sec2Id)
                fetched.linkMultiple = listOf(sec1Id)
                val updated = airtable.primary.update(fetched)
                assertEquals(listOf(sec2Id), updated.linkSingle)
                assertEquals(listOf(sec1Id), updated.linkMultiple)

                airtable.primary.delete(primId)
                airtable.secondary.delete(listOf(sec1Id, sec2Id))
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(primId) }
                runCatching { airtable.secondary.delete(listOf(sec1Id, sec2Id)) }
                throw e
            }
        }

    @Test
    fun linkedSingleRecordResolvesViaSecondaryGet() =
        runBlocking {
            val suite = TestSetup.primaryKey("Linked", "Single")
            val sec = airtable.secondary.create(SecondaryModel(name = "$suite Target", value = "sv"))
            val secId = sec.id!!

            val prim = airtable.primary.create(PrimaryModel(primaryKey = suite, linkSingle = listOf(secId)))
            val primId = prim.id!!
            try {
                val linkedId = prim.linkSingle?.firstOrNull()
                assertEquals(secId, linkedId)

                val linked = airtable.secondary.get(linkedId!!)
                assertEquals("$suite Target", linked.name)
                assertEquals("sv", linked.value)

                airtable.primary.delete(primId)
                airtable.secondary.delete(secId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(primId) }
                runCatching { airtable.secondary.delete(secId) }
                throw e
            }
        }

    @Test
    fun linkedMultiRecordsResolveViaSecondaryGet() =
        runBlocking {
            val suite = TestSetup.primaryKey("Linked", "Multi")
            val sec1 = airtable.secondary.create(SecondaryModel(name = "$suite T1"))
            val sec2 = airtable.secondary.create(SecondaryModel(name = "$suite T2"))
            val secIds = listOf(sec1.id!!, sec2.id!!)

            val prim = airtable.primary.create(PrimaryModel(primaryKey = suite, linkMultiple = secIds))
            val primId = prim.id!!
            try {
                val linked = airtable.secondary.get(prim.linkMultiple ?: emptyList())
                assertEquals(2, linked.size)
                val names = linked.mapNotNull { it.name }
                assertTrue("$suite T1" in names)
                assertTrue("$suite T2" in names)

                airtable.primary.delete(primId)
                airtable.secondary.delete(secIds)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(primId) }
                runCatching { airtable.secondary.delete(secIds) }
                throw e
            }
        }
}
