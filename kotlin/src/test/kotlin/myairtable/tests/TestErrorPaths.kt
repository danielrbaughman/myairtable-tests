package myairtable.tests

import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonPrimitive
import myairtable.AirtableException
import myairtable.AirtableQuery
import myairtable.DictTable
import myairtable.Fields
import myairtable.PrimaryFields
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertTrue
import kotlin.test.fail

/**
 * TC3 — Error and validation paths with TYPED exceptions (not a bare catch-all).
 *
 * Exercises the generated sealed [AirtableException] hierarchy against the live base: a 404 fetch,
 * server-side validation rejections (invalid select option, wrong value type, unknown field), and a
 * 401 from a bogus key. Each asserts a SPECIFIC subtype + its structured fields ([AirtableException.Api.code]),
 * proving the client classifies HTTP failures rather than collapsing them into one opaque error.
 * Parity with csharp/tests/TestErrorPaths.cs.
 */
class TestErrorPaths {
    private val airtable = TestSetup.makeAirtable()

    private fun primaryFieldsWith(suite: String): Fields {
        val fields = Fields(nameToId = PrimaryFields.nameToId)
        fields.setString(PrimaryFields.primaryKeyId, suite)
        return fields
    }

    @Test
    fun getNonexistentRecordThrowsApiNotFound(): Unit =
        runBlocking {
            var caught: AirtableException? = null
            try {
                airtable.primary.get("recDOESNOTEXIST0001")
            } catch (e: AirtableException) {
                caught = e
            }
            val ex = requireApi(caught)
            println("404 -> AirtableException.Api(${ex.code}): ${ex.apiMessage}")
            // Airtable's single-record 404 returns the legacy bare-string envelope (`{"error":"NOT_FOUND"}`),
            // which the runtime maps to code="UNKNOWN" with the signal in the message — unlike the structured
            // `{"error":{"type":...}}` shape on creates. Either placement is an acceptable NOT_FOUND classification.
            assertTrue(
                ex.code == "NOT_FOUND" || ex.apiMessage.contains("NOT_FOUND"),
                "404 fetch should classify as NOT_FOUND (code=${ex.code}, message=${ex.apiMessage})",
            )
        }

    @Test
    fun createWithInvalidSelectOptionThrowsApi(): Unit =
        runBlocking {
            // The typed enum can't express an invalid option, so go through the dict accessor.
            val fields = primaryFieldsWith(TestSetup.primaryKey("Error", "BadSelect"))
            fields.setString(PrimaryFields.singleSelectId, "NotARealOption_zzz")
            val ex = expectApiOnCreate(fields)
            println("bad select -> AirtableException.Api(${ex.code}): ${ex.apiMessage}")
            assertEquals("INVALID_MULTIPLE_CHOICE_OPTIONS", ex.code, "invalid single-select option")
        }

    @Test
    fun createWithWrongValueTypeThrowsApi(): Unit =
        runBlocking {
            val fields = primaryFieldsWith(TestSetup.primaryKey("Error", "BadType"))
            // Number(int) field set to a string value the server can't coerce.
            fields[PrimaryFields.numberIntId] = JsonPrimitive("not a number")
            val ex = expectApiOnCreate(fields)
            println("wrong type -> AirtableException.Api(${ex.code}): ${ex.apiMessage}")
            assertEquals("INVALID_VALUE_FOR_COLUMN", ex.code, "wrong-typed value for a Number column")
        }

    @Test
    fun createWithUnknownFieldThrowsApi(): Unit =
        runBlocking {
            val fields = Fields(nameToId = PrimaryFields.nameToId)
            fields.setString(PrimaryFields.primaryKeyId, TestSetup.primaryKey("Error", "BadField"))
            // A field ID that does not exist in the base.
            fields["fldDOESNOTEXIST00000"] = JsonPrimitive("x")
            val ex = expectApiOnCreate(fields)
            println("unknown field -> AirtableException.Api(${ex.code}): ${ex.apiMessage}")
            assertEquals("UNKNOWN_FIELD_NAME", ex.code, "unknown field id")
        }

    @Test
    fun badApiKeyThrowsAuthError(): Unit =
        runBlocking {
            // Real base + bogus key so auth (401) is checked, not the base (404).
            val bad = TestSetup.makeAirtableWithBadKey()
            // A 401 comes back as a structured envelope (Api: AUTHENTICATION_REQUIRED) or, if the body
            // isn't parseable, a raw Http(401) — both are typed classifications (never the sealed base alone).
            val ex =
                assertFailsWith<AirtableException> {
                    bad.primary.get(AirtableQuery(maxRecords = 1))
                }
            println("bad key -> ${ex::class.simpleName}: ${ex.message}")
            when (ex) {
                is AirtableException.Api -> assertEquals("AUTHENTICATION_REQUIRED", ex.code, "401 -> AUTHENTICATION_REQUIRED")
                is AirtableException.Http -> assertEquals(401, ex.statusCode, "401 -> raw Http(401)")
                else -> fail("expected Api or Http, got ${ex::class.simpleName}: ${ex.message}")
            }
        }

    @Test
    fun rateLimitedIsDistinctTypeCarryingRetryAfter() {
        // Kotlin exposes a distinct rate-limited member of the hierarchy; the live 429 path is
        // exercised in TC8. Here we assert (offline) it's a well-formed, distinct subtype carrying
        // its Retry-After value — i.e. not collapsed into Api/Http.
        val err = AirtableException.RateLimited(30.0)
        // Widen to the sealed base so the classification check is meaningful (not statically known).
        val base: AirtableException = err
        val distinct =
            when (base) {
                is AirtableException.RateLimited -> true
                is AirtableException.Api, is AirtableException.Http -> false
                else -> false
            }
        assertTrue(distinct, "RateLimited is a distinct member of the hierarchy, not collapsed into Api/Http")
        assertEquals(30.0, err.retryAfterSeconds)
    }

    // region Helpers

    /** Narrow a caught exception to [AirtableException.Api] specifically (not just any AirtableException). */
    private fun requireApi(caught: AirtableException?): AirtableException.Api {
        assertTrue(caught != null, "expected an AirtableException to be thrown, but none was")
        assertTrue(
            caught is AirtableException.Api,
            "expected AirtableException.Api, got ${caught::class.simpleName}: ${caught.message}",
        )
        return caught
    }

    /**
     * Perform a dict-create expected to be server-rejected and assert it surfaces as
     * [AirtableException.Api]. If the create unexpectedly succeeds, delete the stray record before
     * failing so the live base stays clean.
     */
    private suspend fun expectApiOnCreate(fields: Fields): AirtableException.Api {
        var caught: AirtableException? = null
        var created: DictTable.Record? = null
        try {
            created = airtable.primary.dict.create(fields)
        } catch (e: AirtableException) {
            caught = e
        }
        if (created != null) {
            runCatching { airtable.primary.dict.delete(created.id) }
            fail("expected the invalid create to be rejected, but it succeeded (id=${created.id})")
        }
        return requireApi(caught)
    }

    // endregion
}
