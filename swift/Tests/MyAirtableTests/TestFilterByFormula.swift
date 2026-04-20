// F7.13 — Filter-by-formula integration test. Parity with
// rust/tests/test_filter_by_formula.rs.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Filter by formula", .serialized)
struct TestFilterByFormula {
    private let airtable: Airtable

    init() {
        self.airtable = TestSetup.makeAirtable()
    }

    // MARK: - Text field filter

    @Test("Filter by text field equals")
    func textFieldEquals() async throws {
        let suite = TestSetup.primaryKey(for: "Filter", "TextEq")
        let created = try await airtable.primary.create(
            PrimaryModel(primaryKey: suite, singleLineText: "UniqueFilterValue")
        )
        guard let recordId = created.id else {
            Issue.record("Missing id")
            return
        }

        do {
            let f = PrimaryModel.f
            let filterFormula = f.singleLineText.equals("UniqueFilterValue")
            let results = try await airtable.primary.get(AirtableQuery().formula(filterFormula))

            #expect(results.contains { $0.id == recordId })

            try await airtable.primary.delete(recordId)
        } catch {
            try? await airtable.primary.delete(recordId)
            throw error
        }
    }

    // MARK: - Record ID filter

    @Test("Filter by record ID")
    func recordIdFilter() async throws {
        let suite = TestSetup.primaryKey(for: "Filter", "RecordID")
        let created = try await airtable.primary.create(
            PrimaryModel(primaryKey: suite)
        )
        guard let recordId = created.id else {
            Issue.record("Missing id")
            return
        }

        do {
            let f = PrimaryModel.f
            let filterFormula = f.id.equals(recordId)
            let results = try await airtable.primary.get(AirtableQuery().formula(filterFormula))

            #expect(results.count == 1)
            #expect(results.first?.id == recordId)

            try await airtable.primary.delete(recordId)
        } catch {
            try? await airtable.primary.delete(recordId)
            throw error
        }
    }

    // MARK: - Number field filter

    @Test("Filter by number field comparison")
    func numberFieldFilter() async throws {
        let suite = TestSetup.primaryKey(for: "Filter", "Number")
        let c1 = try await airtable.primary.create(
            PrimaryModel(numberInt: 10, primaryKey: "\(suite) 1")
        )
        let c2 = try await airtable.primary.create(
            PrimaryModel(numberInt: 20, primaryKey: "\(suite) 2")
        )
        guard let id1 = c1.id, let id2 = c2.id else {
            Issue.record("Missing ids")
            return
        }

        do {
            let f = PrimaryModel.f
            // Filter for numberInt > 15 — should match only c2.
            let filterFormula = f.numberInt.greaterThan(15)
            let results = try await airtable.primary.get(
                AirtableQuery().formula(filterFormula)
            )
            #expect(results.contains { $0.id == id2 })
            #expect(!results.contains { $0.id == id1 })

            try await airtable.primary.delete([id1, id2])
        } catch {
            try? await airtable.primary.delete([id1, id2])
            throw error
        }
    }

    // MARK: - Combinators

    @Test("AND combinator filters correctly")
    func andCombinator() async throws {
        let suite = TestSetup.primaryKey(for: "Filter", "AND")
        let created = try await airtable.primary.create(
            PrimaryModel(
                checkbox: true,
                numberInt: 42,
                primaryKey: suite
            )
        )
        guard let recordId = created.id else {
            Issue.record("Missing id")
            return
        }

        do {
            let f = PrimaryModel.f
            let filterFormula = Formulas.and(
                f.checkbox.isTrue(),
                f.numberInt.equals(42)
            )
            let results = try await airtable.primary.get(
                AirtableQuery().formula(filterFormula)
            )
            #expect(results.contains { $0.id == recordId })

            try await airtable.primary.delete(recordId)
        } catch {
            try? await airtable.primary.delete(recordId)
            throw error
        }
    }
}
