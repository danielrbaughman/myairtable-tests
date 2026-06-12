package myairtable.tests

import kotlinx.coroutines.runBlocking
import myairtable.AirtableException
import myairtable.PrimaryModel
import myairtable.delete
import myairtable.fetch
import myairtable.save
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertTrue

/**
 * K4.7 — Model-side CRUD (save/fetch/delete extension functions). Parity with
 * Swift TestOrmCrudViaModel / Rust test_orm_crud_via_model.
 */
class TestOrmCrudViaModel {
    private val airtable = TestSetup.makeAirtable()

    @Test
    fun primaryKeyOnlyCrudViaModelMethods() =
        runBlocking {
            val primaryKey = TestSetup.primaryKey("OrmModel", "PKOnly")
            val created = airtable.primary.create(PrimaryModel(primaryKey = primaryKey))
            val recordId = created.id!!
            try {
                // Mutate + save() — no table argument needed.
                created.primaryKey = "$primaryKey Updated"
                val saved = created.save()
                assertEquals("$primaryKey Updated", saved.primaryKey)
                assertEquals(recordId, saved.id)

                // fetch() returns a fresh server copy.
                val fetched = saved.fetch()
                assertEquals("$primaryKey Updated", fetched.primaryKey)

                // delete() via the model.
                fetched.delete()
                val gone = runCatching { airtable.primary.get(recordId) }.isFailure
                assertTrue(gone)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }

    @Test
    fun dirtyTrackingResetsAfterASuccessfulSave() =
        runBlocking {
            val primaryKey = TestSetup.primaryKey("OrmModel", "Dirty")
            val created = airtable.primary.create(PrimaryModel(primaryKey = primaryKey))
            val recordId = created.id!!
            try {
                assertTrue(created.dirtyFields().isEmpty(), "freshly created model is clean")
                created.singleLineText = "dirty now"
                assertEquals(1, created.dirtyFields().size)

                val saved = created.save()
                assertTrue(saved.dirtyFields().isEmpty(), "saved model is clean again")
                assertEquals("dirty now", saved.singleLineText)

                airtable.primary.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }

    @Test
    fun deleteOnAnUnsavedModelThrows(): Unit =
        runBlocking {
            val model = PrimaryModel(primaryKey = "never saved")
            // Unsaved AND detached — the detached check fires first.
            assertFailsWith<AirtableException.Api> { model.delete() }
        }

    @Test
    fun fetchOnAnUnsavedModelThrows(): Unit =
        runBlocking {
            val model = PrimaryModel(primaryKey = "never saved")
            assertFailsWith<AirtableException.Api> { model.fetch() }
        }
}
