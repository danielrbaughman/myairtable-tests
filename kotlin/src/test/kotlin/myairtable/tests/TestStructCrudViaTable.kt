package myairtable.tests

import kotlinx.coroutines.runBlocking
import myairtable.Fields
import myairtable.PrimaryFields
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * K3.7 — Dict-table CRUD parity with rust/tests/test_struct_crud_via_table.rs
 * and swift TestStructCrudViaTable. JUnit runs methods within a class
 * sequentially; cross-class parallelism is off (build.gradle.kts).
 */
class TestStructCrudViaTable {
    private val airtable = TestSetup.makeAirtable()

    // region Primary key only

    @Test
    fun primaryKeyOnlyCrudRoundTrip() =
        runBlocking {
            val primaryKey = TestSetup.primaryKey("StructCrud", "PKOnly")
            val fields = Fields(nameToId = PrimaryFields.nameToId)
            fields.setString(PrimaryFields.primaryKeyId, primaryKey)

            // Create
            val created = airtable.primary.dict.create(fields)
            assertTrue(created.id.isNotEmpty())
            assertEquals(primaryKey, created.fields.getString(PrimaryFields.primaryKeyId))

            val recordId = created.id
            try {
                // Read
                val fetched = airtable.primary.dict.get(recordId)
                assertEquals(recordId, fetched.id)
                assertEquals(primaryKey, fetched.fields.getString(PrimaryFields.primaryKeyId))

                // Update
                val update = Fields(nameToId = PrimaryFields.nameToId)
                val updatedKey = "$primaryKey Updated"
                update.setString(PrimaryFields.primaryKeyId, updatedKey)
                val updated = airtable.primary.dict.update(recordId, update)
                assertEquals(updatedKey, updated.fields.getString(PrimaryFields.primaryKeyId))

                // Delete + verify gone
                airtable.primary.dict.delete(recordId)
                val deleted = runCatching { airtable.primary.dict.get(recordId) }.isFailure
                assertTrue(deleted, "record should be gone after delete")
            } catch (e: Throwable) {
                // Best-effort cleanup on any intermediate failure.
                runCatching { airtable.primary.dict.delete(recordId) }
                throw e
            }
        }

    // endregion

    // region Dual ID/name access

    @Test
    fun dictTableSupportsDualIdNameFieldAccess() =
        runBlocking {
            val primaryKey = TestSetup.primaryKey("StructCrud", "DualAccess")
            val fields = Fields(nameToId = PrimaryFields.nameToId)
            // Set by ID…
            fields.setString(PrimaryFields.primaryKeyId, primaryKey)

            val created = airtable.primary.dict.create(fields)
            val recordId = created.id
            try {
                // …read back by name.
                assertEquals(primaryKey, created.fields.getString(PrimaryFields.primaryKeyName))
                // Also readable by ID.
                assertEquals(primaryKey, created.fields.getString(PrimaryFields.primaryKeyId))
                airtable.primary.dict.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.dict.delete(recordId) }
                throw e
            }
        }

    // endregion

    // region All simple properties

    @Test
    fun allSimplePropertiesRoundTrip() =
        runBlocking {
            val primaryKey = TestSetup.primaryKey("StructCrud", "AllProps")
            val fields = Fields(nameToId = PrimaryFields.nameToId)
            fields.setString(PrimaryFields.primaryKeyId, primaryKey)
            fields.setString(PrimaryFields.singleLineTextId, "Hello World")
            fields.setString(PrimaryFields.longTextId, "Long text content")
            fields.setString(PrimaryFields.longTextWithRichTextId, "Rich text content")
            fields.setString(PrimaryFields.emailId, "test@example.com")
            fields.setString(PrimaryFields.urlId, "https://example.com")
            fields.setString(PrimaryFields.phoneNumberId, "555-1234")
            fields.setBoolean(PrimaryFields.checkboxId, true)
            fields.setLong(PrimaryFields.numberIntId, 42L)
            fields.setDouble(PrimaryFields.numberFloatId, 3.14)
            fields.setLong(PrimaryFields.currencyIntId, 10L)
            fields.setDouble(PrimaryFields.currencyFloatId, 9.99)
            fields.setDouble(PrimaryFields.percentIntId, 0.5)
            fields.setDouble(PrimaryFields.percentFloatId, 0.333)
            fields.setLong(PrimaryFields.durationId, 3600L)
            fields.setLong(PrimaryFields.ratingId, 3L)
            fields.setString(PrimaryFields.dateId, "2025-01-15")
            fields.setString(PrimaryFields.dateWithTimeId, "2025-01-15T10:00:00.000Z")
            fields.setString(PrimaryFields.singleSelectId, "Choice 1")
            fields.setStrings(PrimaryFields.multipleSelectId, listOf("Option 1", "Option 2"))

            val created = airtable.primary.dict.create(fields)
            val recordId = created.id
            try {
                val f = created.fields
                assertEquals(primaryKey, f.getString(PrimaryFields.primaryKeyId))
                assertEquals("Hello World", f.getString(PrimaryFields.singleLineTextId))
                assertEquals("test@example.com", f.getString(PrimaryFields.emailId))
                assertEquals(true, f.getBoolean(PrimaryFields.checkboxId))
                assertEquals(42L, f.getLong(PrimaryFields.numberIntId))
                assertEquals(3.14, f.getDouble(PrimaryFields.numberFloatId))
                assertEquals(3L, f.getLong(PrimaryFields.ratingId))
                assertEquals("Choice 1", f.getString(PrimaryFields.singleSelectId))
                assertEquals(2, f.getArray(PrimaryFields.multipleSelectId)?.size)

                airtable.primary.dict.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.dict.delete(recordId) }
                throw e
            }
        }

    // endregion
}
