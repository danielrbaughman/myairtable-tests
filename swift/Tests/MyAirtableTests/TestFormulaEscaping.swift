// TC2 — Formula-value escaping. Filter predicates build formulas by interpolating user
// values into quoted string literals; a value containing a quote, backslash, or other
// meta-character must be escaped or the formula breaks (or silently mis-matches). These
// tests round-trip special-character values through storage AND through the generated
// .equals()/.contains() filter DSL against the live base, proving the escaping is correct.
// Parity with csharp/tests/TestFormulaEscaping.cs.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Formula-value escaping", .serialized)
struct TestFormulaEscaping {
    private let airtable: Airtable

    init() {
        self.airtable = TestSetup.makeAirtable()
    }

    // Values that break naive string interpolation into an Airtable formula literal.
    // Newline is covered separately (storage only) — Airtable formula string literals can't
    // carry a raw newline, so it's a storage concern, not a filter one.
    static let specialValues: [String] = [
        "O'Brien single quote",
        "say \"hi\" double quote",
        "back\\slash path",
        "trailing backslash\\",
        "mixed a\"b'c\\d",
        "unicode café ☕ 日本語 🎉",
    ]

    // MARK: - Eq filter

    @Test("Eq filter matches value with special chars", arguments: specialValues)
    func eqFilterMatchesValueWithSpecialChars(_ special: String) async throws {
        let suite = TestSetup.primaryKey(for: "Escaping", "Eq")
        let created = try await airtable.primary.create(
            PrimaryModel(primaryKey: suite, singleLineText: special)
        )
        guard let recordId = created.id else {
            Issue.record("Missing id")
            return
        }

        do {
            // 1. Storage round-trips the exact value.
            #expect(created.singleLineText == special)

            // 2. The .equals() filter, scoped to this record, matches via correctly-escaped
            //    interpolation.
            let f = PrimaryModel.f
            let filter = Formulas.and(
                "FIND(\"\(suite)\", {Primary Key})",
                f.singleLineText.equals(special)
            )
            let results = try await airtable.primary.get(AirtableQuery().formula(filter))
            #expect(
                results.contains { $0.id == recordId },
                "value=\(special.debugDescription) formula=\(filter.debugDescription)"
            )
            for r in results {
                #expect(r.singleLineText == special)
            }

            try await airtable.primary.delete(recordId)
        } catch {
            try? await airtable.primary.delete(recordId)
            throw error
        }
    }

    // MARK: - Contains filter

    @Test("Contains filter matches value with special chars", arguments: specialValues)
    func containsFilterMatchesValueWithSpecialChars(_ special: String) async throws {
        let suite = TestSetup.primaryKey(for: "Escaping", "Contains")
        // Embed the special token inside a longer value so contains (FIND>0) is a real
        // substring test.
        let stored = "prefix " + special + " suffix"
        let created = try await airtable.primary.create(
            PrimaryModel(primaryKey: suite, singleLineText: stored)
        )
        guard let recordId = created.id else {
            Issue.record("Missing id")
            return
        }

        do {
            let f = PrimaryModel.f
            let filter = Formulas.and(
                "FIND(\"\(suite)\", {Primary Key})",
                f.singleLineText.contains(special)
            )
            let results = try await airtable.primary.get(AirtableQuery().formula(filter))
            #expect(
                results.contains { $0.id == recordId },
                "value=\(special.debugDescription) formula=\(filter.debugDescription)"
            )

            try await airtable.primary.delete(recordId)
        } catch {
            try? await airtable.primary.delete(recordId)
            throw error
        }
    }

    // MARK: - Special chars in primary key

    @Test("Special characters in primary key round-trip and filter")
    func specialCharactersInPrimaryKeyRoundTripAndFilter() async throws {
        // The primary key itself carries special chars, and we match on it with .equals().
        let suite = TestSetup.primaryKey(for: "Escaping", "PK")
        let pk = suite + " O'Brien \"quote\" back\\slash"
        let created = try await airtable.primary.create(PrimaryModel(primaryKey: pk))
        guard let recordId = created.id else {
            Issue.record("Missing id")
            return
        }

        do {
            #expect(created.primaryKey == pk)
            let f = PrimaryModel.f
            let filter = f.primaryKey.equals(pk)
            let results = try await airtable.primary.get(AirtableQuery().formula(filter))
            #expect(
                results.contains { $0.id == recordId },
                "value=\(pk.debugDescription) formula=\(filter.debugDescription)"
            )
            #expect(results.count == 1)

            try await airtable.primary.delete(recordId)
        } catch {
            try? await airtable.primary.delete(recordId)
            throw error
        }
    }

    // MARK: - Newline / tab storage round-trip

    @Test("Newline and tab values round-trip through storage")
    func newlineAndTabValuesRoundTripThroughStorage() async throws {
        // Newlines/tabs are a storage concern (long text), not expressible in a formula
        // literal. Verify they survive create -> fetch unchanged.
        let suite = TestSetup.primaryKey(for: "Escaping", "Newline")
        let value = "line1\nline2\twith tab\r\nwindows"
        let created = try await airtable.primary.create(
            PrimaryModel(longText: value, primaryKey: suite)
        )
        guard let recordId = created.id else {
            Issue.record("Missing id")
            return
        }

        do {
            let fetched = try await airtable.primary.get(recordId)
            #expect(fetched.longText == value)

            try await airtable.primary.delete(recordId)
        } catch {
            try? await airtable.primary.delete(recordId)
            throw error
        }
    }
}
