package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.fasterxml.jackson.databind.node.TextNode;
import myairtable.Airtable;
import myairtable.AirtableException;
import myairtable.AirtableQuery;
import myairtable.DictTable;
import myairtable.Fields;
import myairtable.PrimaryFields;
import org.junit.jupiter.api.Test;

/**
 * TC3 — Error and validation paths with TYPED exceptions (not a bare catch-all). Exercises the
 * generated, sealed {@link AirtableException} hierarchy against the live base: a 404 fetch,
 * server-side validation rejections (invalid select option, wrong value type, unknown field), and a
 * 401 from a bad key.
 *
 * <p>Each scenario asserts a SPECIFIC subtype — {@link AirtableException.Api} (structured {@code
 * {"error": {"type", "message"}}} envelope, carrying a {@code code}) or {@link
 * AirtableException.Http} (raw non-2xx) — and inspects the {@code code} where available, proving
 * the client classifies HTTP failures rather than collapsing them into one opaque error. Parity
 * with csharp/tests/TestErrorPaths.cs.
 */
class TestErrorPaths {

  private final Airtable airtable = TestSetup.makeAirtable();

  private static Fields primaryFieldsWith(String suite) {
    Fields fields = new Fields(java.util.Map.of(), PrimaryFields.nameToId);
    fields.setString(PrimaryFields.primaryKeyId, suite);
    return fields;
  }

  @Test
  void getNonexistentRecordThrowsApiError() {
    AirtableException ex =
        assertThrows(AirtableException.class, () -> airtable.primary().get("recDOESNOTEXIST0001"));
    System.out.println("404 -> " + ex.getClass().getSimpleName() + ": " + ex.getMessage());
    AirtableException.Api api = assertInstanceOf(AirtableException.Api.class, ex);
    assertFalse(api.code() == null || api.code().isEmpty());
    // Airtable's 404 for a get-by-id uses the LEGACY string envelope `{"error": "NOT_FOUND"}`
    // (not the `{"error": {"type", "message"}}` object form). The static runtime maps that string
    // form to Api(code="UNKNOWN", apiMessage="NOT_FOUND"), so the NOT_FOUND signal is carried in
    // the
    // message, not the code. This mirrors the C# parity test, which likewise only asserts a
    // non-empty
    // code here rather than a specific one.
    assertEquals("NOT_FOUND", api.apiMessage());
  }

  @Test
  void createWithInvalidSelectOptionThrowsApiError() {
    // The typed enum can't express an invalid option, so go through the dict accessor.
    Fields fields = primaryFieldsWith(TestSetup.primaryKey("Error", "BadSelect"));
    fields.setString(PrimaryFields.singleSelectId, "NotARealOption_zzz");
    AirtableException ex =
        assertThrows(
            AirtableException.class,
            () -> cleanupIfCreated(airtable.primary().dict().create(fields)));
    System.out.println("bad select -> " + ex.getClass().getSimpleName() + ": " + ex.getMessage());
    AirtableException.Api api = assertInstanceOf(AirtableException.Api.class, ex);
    assertFalse(api.code() == null || api.code().isEmpty());
    assertEquals("INVALID_MULTIPLE_CHOICE_OPTIONS", api.code());
  }

  @Test
  void createWithWrongValueTypeThrowsApiError() {
    Fields fields = primaryFieldsWith(TestSetup.primaryKey("Error", "BadType"));
    // A Number(int) field given a string value — server rejects with a typed error.
    fields.set(PrimaryFields.numberIntId, TextNode.valueOf("not a number"));
    AirtableException ex =
        assertThrows(
            AirtableException.class,
            () -> cleanupIfCreated(airtable.primary().dict().create(fields)));
    System.out.println("wrong type -> " + ex.getClass().getSimpleName() + ": " + ex.getMessage());
    AirtableException.Api api = assertInstanceOf(AirtableException.Api.class, ex);
    assertEquals("INVALID_VALUE_FOR_COLUMN", api.code());
  }

  @Test
  void createWithUnknownFieldThrowsApiError() {
    Fields fields = new Fields(java.util.Map.of(), PrimaryFields.nameToId);
    fields.setString(PrimaryFields.primaryKeyId, TestSetup.primaryKey("Error", "BadField"));
    fields.set("fldDOESNOTEXIST00000", TextNode.valueOf("x"));
    AirtableException ex =
        assertThrows(
            AirtableException.class,
            () -> cleanupIfCreated(airtable.primary().dict().create(fields)));
    System.out.println(
        "unknown field -> " + ex.getClass().getSimpleName() + ": " + ex.getMessage());
    AirtableException.Api api = assertInstanceOf(AirtableException.Api.class, ex);
    assertEquals("UNKNOWN_FIELD_NAME", api.code());
  }

  @Test
  void badApiKeyThrowsAuthError() {
    // Real base + bogus key so auth (401) is checked, not the base (404).
    Airtable bad = TestSetup.makeAirtableWithBadKey();
    AirtableException ex =
        assertThrows(
            AirtableException.class,
            () -> bad.primary().get(new AirtableQuery().withMaxRecords(1)));
    System.out.println("bad key -> " + ex.getClass().getSimpleName() + ": " + ex.getMessage());
    // Auth failures come back as a structured 401 envelope (Api) or, if not parseable, a raw Http —
    // both are acceptable typed classifications (never the sealed base type alone).
    assertTrue(
        ex instanceof AirtableException.Api || ex instanceof AirtableException.Http,
        "expected Api/Http, got " + ex.getClass().getSimpleName());
    if (ex instanceof AirtableException.Api api) {
      assertEquals("AUTHENTICATION_REQUIRED", api.code());
    } else {
      assertEquals(401, ((AirtableException.Http) ex).statusCode());
    }
  }

  @Test
  void rateLimitedErrorIsDistinctWellFormedMember() {
    // The live 429 path is exercised in TC8; here we assert (offline) that the rate-limited error
    // is
    // a distinct, well-formed member of the sealed hierarchy carrying its Retry-After value.
    AirtableException.RateLimited err = new AirtableException.RateLimited(30.0);
    assertInstanceOf(AirtableException.class, err);
    assertEquals(30.0, err.retryAfterSeconds());
  }

  /**
   * Invalid creates are expected to be server-rejected; if one unexpectedly succeeds, delete the
   * record so the assertion failure that follows doesn't also leak a row into the shared base.
   */
  private void cleanupIfCreated(DictTable.Record created) {
    try {
      airtable.primary().dict().delete(created.id());
    } catch (RuntimeException ignored) {
      // best-effort cleanup
    }
  }
}
