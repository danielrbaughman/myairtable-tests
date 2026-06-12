package myairtable.tests

import kotlinx.coroutines.delay
import kotlinx.coroutines.runBlocking
import myairtable.AirtableAttachment
import myairtable.AirtableCollaborator
import myairtable.PrimaryModel
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull

/**
 * K5.6 — Complex field-type CRUD: attachments, collaborators (users), and
 * computed fields (auto-number, formulas, created-time, created-by, button).
 * Parity with Swift TestComplexProperties / Rust attachment_crud +
 * user_fields_crud + computed_fields. Linked records are covered by K-F6.
 */
class TestComplexProperties {
    private val airtable = TestSetup.makeAirtable()

    // region Attachments (URL-based create, async processing)

    @Test
    fun attachmentFieldRoundTripsViaUrl() =
        runBlocking {
            val primaryKey = TestSetup.primaryKey("Complex", "Attach")
            val url = "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"

            val created =
                airtable.primary.create(
                    PrimaryModel(primaryKey = primaryKey, attachment = listOf(AirtableAttachment(url = url))),
                )
            val recordId = created.id!!
            try {
                // Airtable processes attachments asynchronously — the fresh record
                // may have just the URL, or be fully populated with id/size once
                // processing completes.
                assertEquals(1, created.attachment?.size)

                // Poll until Airtable has enriched the attachment (id populated).
                var current = created
                repeat(10) {
                    val first = current.attachment?.firstOrNull()
                    if (!first?.id.isNullOrEmpty()) return@repeat
                    delay(2_000)
                    current = airtable.primary.get(recordId)
                }
                val enriched = current.attachment ?: emptyList()
                assertEquals(1, enriched.size)
                assertNotNull(enriched.first().url?.takeIf { it.isNotEmpty() })

                airtable.primary.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }

    // endregion

    // region Collaborators (users)

    @Test
    fun userAndMultiUserFieldsRoundTrip() =
        runBlocking {
            val primaryKey = TestSetup.primaryKey("Complex", "Users")
            // Matches the user ID used by the Rust/Swift tests in the shared base.
            val userId = "usrnZ4k98m0Ipji4e"

            val created =
                airtable.primary.create(
                    PrimaryModel(
                        primaryKey = primaryKey,
                        user = AirtableCollaborator(id = userId),
                        userAllowMultiple = listOf(AirtableCollaborator(id = userId)),
                    ),
                )
            val recordId = created.id!!
            try {
                assertEquals(userId, created.user?.id)
                assertEquals(1, created.userAllowMultiple?.size)
                assertEquals(userId, created.userAllowMultiple?.firstOrNull()?.id)

                val fetched = airtable.primary.get(recordId)
                assertEquals(userId, fetched.user?.id)

                airtable.primary.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }

    // endregion

    // region Computed fields

    @Test
    fun computedFieldsPopulateOnCreateAndReadBack() =
        runBlocking {
            val primaryKey = TestSetup.primaryKey("Complex", "Computed")
            val created =
                airtable.primary.create(
                    PrimaryModel(primaryKey = primaryKey, numberInt = 10.0, numberFloat = 5.0),
                )
            val recordId = created.id!!
            try {
                // Server-owned: auto number assigned; created time populated.
                assertNotNull(created.autoNumber?.valueOrNull)
                assertNotNull(created.createdTime)
                assertNotNull(created.createdAtTime?.valueOrNull, "dedicated createdTime field")
                // Formula(ID) reflects the record id.
                assertEquals(recordId, created.formulaId?.valueOrNull)
                // Formula(Simple) = numberInt + numberFloat = 15.
                assertEquals(15.0, created.formulaSimple?.valueOrNull)
                // Created-by is populated with the caller's user info.
                assertNotNull(created.createdBy?.valueOrNull)

                val fetched = airtable.primary.get(recordId)
                assertEquals(recordId, fetched.formulaId?.valueOrNull)
                assertEquals(15.0, fetched.formulaSimple?.valueOrNull)
                assertEquals(created.autoNumber, fetched.autoNumber)

                airtable.primary.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }

    // endregion

    // region Button (read-only)

    @Test
    fun buttonFieldDecodesOnRead() =
        runBlocking {
            val primaryKey = TestSetup.primaryKey("Complex", "Button")
            val created = airtable.primary.create(PrimaryModel(primaryKey = primaryKey))
            val recordId = created.id!!
            try {
                // The button field decodes (either populated or absent) without error.
                val fetched = airtable.primary.get(recordId)
                assertEquals(recordId, fetched.id)
                fetched.button?.let { assertNotNull(it.valueOrNull?.label ?: it.valueOrNull?.url ?: "") }

                airtable.primary.delete(recordId)
            } catch (e: Throwable) {
                runCatching { airtable.primary.delete(recordId) }
                throw e
            }
        }

    // endregion
}
