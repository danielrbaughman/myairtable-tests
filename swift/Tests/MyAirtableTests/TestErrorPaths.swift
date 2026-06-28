// TC3 — Error / validation paths with TYPED errors. Parity with
// csharp/tests/TestErrorPaths.cs.
//
// Exercises the generated `AirtableError` enum against the live base: a 404
// fetch, server-side validation rejections (invalid select option, wrong value
// type, unknown field), and a 401 from a bad key. Each asserts a SPECIFIC enum
// case + its structured payload (code / status), proving the client classifies
// HTTP failures rather than collapsing them to one opaque error.
//
// Invalid creates are server-rejected, so there is nothing to clean up. Any
// create that unexpectedly succeeds is deleted so the base stays clean.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Error / validation paths (typed errors)", .serialized)
struct TestErrorPaths {
    private let airtable: Airtable

    init() {
        self.airtable = TestSetup.makeAirtable()
    }

    /// A `Fields` carrying only the primary key, ready for dict-create.
    private func primaryFieldsWith(_ suite: String) -> Fields {
        var fields = Fields(nameToId: PrimaryFields.nameToId)
        fields.setString(PrimaryFields.primaryKeyId, suite)
        return fields
    }

    // MARK: - 1. 404: nonexistent record id -> .api(code: NOT_FOUND)

    @Test("get() of a nonexistent record id throws .api with a non-empty code")
    func getNonexistentRecordThrowsApiError() async throws {
        do {
            _ = try await airtable.primary.dict.get("recDOESNOTEXIST0001")
            Issue.record("Expected get() of a nonexistent record to throw")
        } catch let error as AirtableError {
            print("404 -> \(error)")
            guard case .api(let code, let message) = error else {
                Issue.record("Expected .api, got \(error)")
                return
            }
            // Airtable's 404-for-a-missing-record uses the LEGACY string error
            // shape (`{"error":"NOT_FOUND"}`), not the structured
            // `{"type":...,"message":...}` envelope. The runtime maps the legacy
            // string into `.api(code:"UNKNOWN", message:<raw>)`, so the
            // NOT_FOUND signal lands in the message. Assert a non-empty code
            // (parity with C#, which only checks Code is non-empty here) and
            // that the missing-record signal is present.
            #expect(!code.isEmpty)
            #expect(code == "UNKNOWN" || code == "NOT_FOUND")
            #expect(message.contains("NOT_FOUND") || code == "NOT_FOUND")
        }
    }

    // MARK: - 2. Invalid single-select option -> .api(INVALID_MULTIPLE_CHOICE_OPTIONS)

    @Test("dict-create with an invalid single-select option throws .api")
    func createWithInvalidSelectOptionThrowsApiError() async throws {
        // The typed enum can't express an invalid option, so go through the
        // dict accessor (parity with the C# comment).
        var fields = primaryFieldsWith(TestSetup.primaryKey(for: "Error", "BadSelect"))
        fields.setString(PrimaryFields.singleSelectId, "NotARealOption_zzz")
        do {
            let created = try await airtable.primary.dict.create(fields)
            Issue.record("Expected invalid select option to be rejected")
            try? await airtable.primary.dict.delete(created.id)
        } catch let error as AirtableError {
            print("bad select -> \(error)")
            guard case .api(let code, _) = error else {
                Issue.record("Expected .api, got \(error)")
                return
            }
            #expect(!code.isEmpty)
            #expect(code == "INVALID_MULTIPLE_CHOICE_OPTIONS")
        }
    }

    // MARK: - 3. Wrong value type -> .api(INVALID_VALUE_FOR_COLUMN)

    @Test("dict-create with a string in a Number field throws .api")
    func createWithWrongValueTypeThrowsApiError() async throws {
        var fields = primaryFieldsWith(TestSetup.primaryKey(for: "Error", "BadType"))
        // Number(int) field given a string value — server rejects the type.
        fields.set(PrimaryFields.numberIntId, .string("not a number"))
        do {
            let created = try await airtable.primary.dict.create(fields)
            Issue.record("Expected wrong value type to be rejected")
            try? await airtable.primary.dict.delete(created.id)
        } catch let error as AirtableError {
            print("wrong type -> \(error)")
            guard case .api(let code, _) = error else {
                Issue.record("Expected .api, got \(error)")
                return
            }
            #expect(!code.isEmpty)
            #expect(code == "INVALID_VALUE_FOR_COLUMN")
        }
    }

    // MARK: - 4. Unknown field id -> .api(UNKNOWN_FIELD_NAME)

    @Test("dict-create with an unknown field id throws .api")
    func createWithUnknownFieldThrowsApiError() async throws {
        var fields = Fields(nameToId: PrimaryFields.nameToId)
        fields.setString(PrimaryFields.primaryKeyId, TestSetup.primaryKey(for: "Error", "BadField"))
        // A well-formed-but-nonexistent field id.
        fields.set("fldDOESNOTEXIST00000", .string("x"))
        do {
            let created = try await airtable.primary.dict.create(fields)
            Issue.record("Expected unknown field id to be rejected")
            try? await airtable.primary.dict.delete(created.id)
        } catch let error as AirtableError {
            print("unknown field -> \(error)")
            guard case .api(let code, _) = error else {
                Issue.record("Expected .api, got \(error)")
                return
            }
            #expect(!code.isEmpty)
            #expect(code == "UNKNOWN_FIELD_NAME")
        }
    }

    // MARK: - 5. Bad API key -> typed 401 error

    @Test("real base + bogus API key throws a typed 401 error")
    func badApiKeyThrowsAuthError() async throws {
        // Real base + bogus key so auth (401) is checked, not the base (404).
        let bad = TestSetup.makeAirtableWithBadKey()
        do {
            _ = try await bad.primary.dict.get(AirtableQuery().maxRecords(1))
            Issue.record("Expected a bogus API key to be rejected")
        } catch let error as AirtableError {
            print("bad key -> \(error)")
            // Auth failures come back either as a structured 401 envelope
            // (.api with AUTHENTICATION_REQUIRED) or, if not parseable as an
            // envelope, a raw .http(401) — both are acceptable typed
            // classifications (never a non-error). Assert the 401 specifically.
            switch error {
            case .api(let code, _):
                #expect(code == "AUTHENTICATION_REQUIRED")
            case .http(let statusCode, _):
                #expect(statusCode == 401)
            default:
                Issue.record("Expected .api or .http(401), got \(error)")
            }
        }
    }

    // MARK: - 6. Rate-limited is a distinct, well-formed case (offline)

    @Test("rateLimited is a distinct error case carrying its Retry-After (offline)")
    func rateLimitedErrorCarriesRetryAfter() {
        // The 429 path is exercised live in TC8; here we assert the type is a
        // distinct, well-formed member of the enum carrying its Retry-After.
        let err = AirtableError.rateLimited(retryAfter: 30.0)
        guard case .rateLimited(let retryAfter) = err else {
            Issue.record("Expected .rateLimited, got \(err)")
            return
        }
        #expect(retryAfter == 30.0)
        // Distinct from the other cases.
        #expect(err != AirtableError.http(statusCode: 429, body: nil))
        #expect(err != AirtableError.api(code: "RATE_LIMIT", message: ""))
    }
}
