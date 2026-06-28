package myairtable.tests

import kotlinx.coroutines.runBlocking
import myairtable.AirtableAttachment
import myairtable.AirtableCollaborator
import myairtable.PrimaryModel
import myairtable.PrimaryMultipleSelectOption
import myairtable.PrimarySingleSelectOption
import java.time.Instant
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * K-F7 / TC7 — Field-type round-trip completeness via re-fetch. Several field types were only
 * asserted on the create response (or only offline-decoded), never written and read back through
 * the live API, and clearing/removing multi-value fields was untested. Each case here creates,
 * optionally updates, re-fetches, and asserts the server-side value. Parity with C#
 * TestFieldRoundTrip.
 */
class TestFieldRoundTrip {
    private val airtable = TestSetup.makeAirtable()

    private val userId = "usrnZ4k98m0Ipji4e" // shared-base user, as in TestComplexProperties

    @Test
    fun dateWithTimeWritesAndReadsBack() =
        runBlocking {
            val suite = TestSetup.primaryKey("FieldRT", "DateTime")
            val dt = Instant.parse("2024-03-15T14:30:00Z")
            val created =
                airtable.primary.create(
                    PrimaryModel(primaryKey = suite, dateWithTime = dt),
                )
            val recordId = created.id!!
            try {
                val fetched = airtable.primary.get(recordId)
                assertEquals(dt, fetched.dateWithTime)
            } finally {
                tryDelete(recordId)
            }
        }

    @Test
    fun richTextAndPercentCurrencyReadBack() =
        runBlocking {
            val suite = TestSetup.primaryKey("FieldRT", "Rich")
            val created =
                airtable.primary.create(
                    PrimaryModel(
                        primaryKey = suite,
                        longTextWithRichText = "**bold** and _italic_ text",
                        percentInt = 0.5,
                        percentFloat = 0.333,
                        currencyInt = 100.0,
                        currencyFloat = 19.99,
                    ),
                )
            val recordId = created.id!!
            try {
                val fetched = airtable.primary.get(recordId)
                assertEquals("**bold** and _italic_ text", fetched.longTextWithRichText)
                assertEquals(0.5, fetched.percentInt)
                assertEquals(0.333, fetched.percentFloat)
                assertEquals(100.0, fetched.currencyInt)
                assertEquals(19.99, fetched.currencyFloat)
            } finally {
                tryDelete(recordId)
            }
        }

    @Test
    fun clearingSingleAndMultiSelectReadsBackEmpty() =
        runBlocking {
            val suite = TestSetup.primaryKey("FieldRT", "ClearSelect")
            val created =
                airtable.primary.create(
                    PrimaryModel(
                        primaryKey = suite,
                        singleSelect = PrimarySingleSelectOption.CHOICE_1,
                        multipleSelect =
                            listOf(
                                PrimaryMultipleSelectOption.OPTION_1,
                                PrimaryMultipleSelectOption.OPTION_2,
                            ),
                    ),
                )
            val recordId = created.id!!
            try {
                assertEquals(PrimarySingleSelectOption.CHOICE_1, created.singleSelect)

                created.singleSelect = null
                created.multipleSelect = emptyList()
                airtable.primary.update(created)

                val fetched = airtable.primary.get(recordId)
                assertNull(fetched.singleSelect)
                assertTrue(fetched.multipleSelect.isNullOrEmpty())
            } finally {
                tryDelete(recordId)
            }
        }

    @Test
    fun removingACollaboratorReadsBackNull() =
        runBlocking {
            val suite = TestSetup.primaryKey("FieldRT", "RemoveUser")
            val created =
                airtable.primary.create(
                    PrimaryModel(primaryKey = suite, user = AirtableCollaborator(id = userId)),
                )
            val recordId = created.id!!
            try {
                assertEquals(userId, created.user?.id)

                created.user = null
                airtable.primary.update(created)

                val fetched = airtable.primary.get(recordId)
                assertNull(fetched.user)
            } finally {
                tryDelete(recordId)
            }
        }

    @Test
    fun attachmentReplaceAndRemoveReadBack() =
        runBlocking {
            val urlA = "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"
            val urlB = "https://www.w3.org/Icons/w3c_home.png"
            val suite = TestSetup.primaryKey("FieldRT", "Attach")
            val created =
                airtable.primary.create(
                    PrimaryModel(
                        primaryKey = suite,
                        attachment = listOf(AirtableAttachment(url = urlA)),
                    ),
                )
            val recordId = created.id!!
            try {
                // Replace the attachment with a different one.
                created.attachment = listOf(AirtableAttachment(url = urlB))
                airtable.primary.update(created)
                val replaced = airtable.primary.get(recordId)
                assertEquals(1, replaced.attachment?.size)

                // Remove all attachments.
                replaced.attachment = emptyList()
                airtable.primary.update(replaced)
                val cleared = airtable.primary.get(recordId)
                assertTrue(cleared.attachment.isNullOrEmpty())
            } finally {
                tryDelete(recordId)
            }
        }

    private suspend fun tryDelete(recordId: String?) {
        if (recordId.isNullOrEmpty()) return
        runCatching { airtable.primary.delete(recordId) }
    }
}
