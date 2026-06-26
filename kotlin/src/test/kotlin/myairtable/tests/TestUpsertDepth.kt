package myairtable.tests

import kotlinx.coroutines.runBlocking
import myairtable.AirtableException
import myairtable.PrimaryFields
import myairtable.PrimaryModel
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertIs
import kotlin.test.assertNotEquals
import kotlin.test.assertTrue

/**
 * TC10 — Upsert depth. The base CRUD suite only covers single-record, single-merge-field upsert.
 * This adds a multi-field merge key (match must agree on ALL merge fields) and the multiple-match
 * error path (a merge key matching more than one record is rejected by Airtable). Note: the
 * generated client exposes only single-record upsert, so batch upsert isn't covered here.
 * Parity with C# TestUpsertDepth.
 */
class TestUpsertDepth {
    private val airtable = TestSetup.makeAirtable()

    @Test
    fun upsertMatchesOnMultipleMergeFields(): Unit =
        runBlocking {
            val suite = TestSetup.primaryKey("Upsert", "MultiKey")
            val ids = mutableListOf<String>()
            try {
                // Seed a record identified by the (PrimaryKey, SingleLineText) pair.
                val seed = airtable.primary.create(PrimaryModel(primaryKey = suite, singleLineText = "anchor"))
                ids.add(seed.id!!)

                val mergeOn = listOf(PrimaryFields.primaryKeyId, PrimaryFields.singleLineTextId)

                // Same pair -> UPDATE the seed (matched on both fields).
                val update =
                    airtable.primary.upsert(
                        PrimaryModel(primaryKey = suite, singleLineText = "anchor", longText = "updated"),
                        mergeOn,
                    )
                assertFalse(update.wasCreated)
                assertEquals(seed.id, update.model.id)
                assertEquals("updated", update.model.longText)

                // Same PrimaryKey but a DIFFERENT SingleLineText -> no match on the pair -> INSERT.
                val insert =
                    airtable.primary.upsert(
                        PrimaryModel(primaryKey = suite, singleLineText = "different"),
                        mergeOn,
                    )
                ids.add(insert.model.id!!)
                assertTrue(insert.wasCreated)
                assertNotEquals(seed.id, insert.model.id)
            } finally {
                tryDeleteMany(ids)
            }
        }

    @Test
    fun upsertWithMultipleMatchesThrows(): Unit =
        runBlocking {
            val suite = TestSetup.primaryKey("Upsert", "MultiMatch")
            val ids = mutableListOf<String>()
            try {
                // Two records share the same SingleLineText value.
                val a = airtable.primary.create(PrimaryModel(primaryKey = "$suite A", singleLineText = "dupe"))
                val b = airtable.primary.create(PrimaryModel(primaryKey = "$suite B", singleLineText = "dupe"))
                ids.add(a.id!!)
                ids.add(b.id!!)

                // Upsert merging only on SingleLineText="dupe" matches BOTH -> Airtable rejects it.
                val ex =
                    kotlin.test.assertFailsWith<AirtableException> {
                        airtable.primary.upsert(
                            PrimaryModel(singleLineText = "dupe", longText = "x"),
                            listOf(PrimaryFields.singleLineTextId),
                        )
                    }
                // Tighten beyond "some typed error" to the SPECIFIC rejection, matching
                // Rust (`assert_eq!(status, 422)`) and Python (`status_code == 422`).
                // Airtable answers this with HTTP 422 + the structured envelope
                // {"error": {"type": "INVALID_VALUE_FOR_COLUMN",
                //            "message": "Cannot update more than one record for fields to merge on"}}.
                // The Kotlin runtime decodes that envelope into AirtableException.Api(code, apiMessage)
                // and does not retain the HTTP status (only AirtableException.Http carries statusCode),
                // so we pin the error's specific 422 type code + message rather than the numeric status.
                val api = assertIs<AirtableException.Api>(ex)
                assertEquals("INVALID_VALUE_FOR_COLUMN", api.code, "multiple-match upsert must surface the specific 422 error type, not a generic one")
                assertTrue(
                    api.apiMessage.contains("more than one record", ignoreCase = true),
                    "expected the multiple-match rejection message, got: ${api.apiMessage}",
                )
            } finally {
                tryDeleteMany(ids)
            }
        }

    private suspend fun tryDeleteMany(ids: List<String>) {
        if (ids.isEmpty()) return
        try {
            airtable.primary.delete(ids)
        } catch (_: AirtableException) {
        }
    }
}
