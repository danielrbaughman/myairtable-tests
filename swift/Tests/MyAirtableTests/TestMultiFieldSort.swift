// TC11 — Multi-field sort + sort combined with a filter. The base filter suite
// only covers a single-field sort. This verifies a two-field sort (primary key
// with ties broken by a secondary key) and sorting within a filtered scope.
// Parity with csharp/tests/TestMultiFieldSort.cs.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Multi-field sort", .serialized)
struct TestMultiFieldSort {
    private let airtable: Airtable

    init() {
        self.airtable = TestSetup.makeAirtable()
    }

    // MARK: - Helpers

    /// Dict-path row with primary key, numberInt, and singleLineText set.
    private func row(_ suite: String, _ number: Int, _ text: String) -> Fields {
        var fields = Fields(nameToId: PrimaryFields.nameToId)
        fields.setString(PrimaryFields.primaryKeyId, "\(suite) \(text)")
        fields.setInt(PrimaryFields.numberIntId, number)
        fields.setString(PrimaryFields.singleLineTextId, text)
        return fields
    }

    /// Scope a formula to specific record IDs so suite runs don't interfere.
    private func scopeTo(_ ids: [String]) -> String {
        let parts = ids.map { "RECORD_ID()='\($0)'" }
        return parts.count == 1 ? parts[0] : Formulas.or(parts)
    }

    private func texts(of records: [DictTable.Record]) -> [String] {
        records.compactMap { $0.fields.getString(PrimaryFields.singleLineTextId) }
    }

    private func tryDeleteMany(_ ids: [String]) async {
        if ids.isEmpty { return }
        try? await airtable.primary.dict.delete(ids)
    }

    // MARK: - Two-field sort

    @Test("Two-field sort breaks ties on secondary key")
    func twoFieldSortBreaksTiesOnSecondKey() async throws {
        let suite = TestSetup.primaryKey(for: "Sort", "TwoField")
        // NumberInt ties at 10 (rows "b" and "a"); the secondary SingleLineText sort orders them.
        let created = try await airtable.primary.dict.create([
            row(suite, 10, "b"),
            row(suite, 10, "a"),
            row(suite, 20, "c"),
        ])
        let ids = created.map { $0.id }

        do {
            let results = try await airtable.primary.dict.get(
                AirtableQuery()
                    .formula(scopeTo(ids))
                    .sort(field: PrimaryFields.numberIntId, direction: .asc)
                    .sort(field: PrimaryFields.singleLineTextId, direction: .asc)
            )
            // (10,a), (10,b), (20,c) — tie on 10 broken by text asc.
            #expect(texts(of: results) == ["a", "b", "c"])

            await tryDeleteMany(ids)
        } catch {
            await tryDeleteMany(ids)
            throw error
        }
    }

    // MARK: - Secondary descending

    @Test("Secondary descending reverses tied group")
    func secondaryDescendingReversesTiedGroup() async throws {
        let suite = TestSetup.primaryKey(for: "Sort", "MixedDir")
        let created = try await airtable.primary.dict.create([
            row(suite, 10, "a"),
            row(suite, 10, "b"),
            row(suite, 20, "c"),
        ])
        let ids = created.map { $0.id }

        do {
            let results = try await airtable.primary.dict.get(
                AirtableQuery()
                    .formula(scopeTo(ids))
                    .sort(field: PrimaryFields.numberIntId, direction: .asc)
                    .sort(field: PrimaryFields.singleLineTextId, direction: .desc)
            )
            // NumberInt asc, then text DESC within the 10-tie: (10,b), (10,a), (20,c).
            #expect(texts(of: results) == ["b", "a", "c"])

            await tryDeleteMany(ids)
        } catch {
            await tryDeleteMany(ids)
            throw error
        }
    }

    // MARK: - Sort + filter

    @Test("Sort combined with a filter")
    func sortCombinedWithAFilter() async throws {
        let suite = TestSetup.primaryKey(for: "Sort", "WithFilter")
        let created = try await airtable.primary.dict.create([
            row(suite, 30, "x"),
            row(suite, 10, "y"),
            row(suite, 20, "z"),
            row(suite, 5, "low"),  // filtered out by NumberInt > 5
        ])
        let ids = created.map { $0.id }

        do {
            let f = PrimaryModel.f
            let filter = Formulas.and(scopeTo(ids), f.numberInt.greaterThan(5))
            let results = try await airtable.primary.dict.get(
                AirtableQuery()
                    .formula(filter)
                    .sort(field: PrimaryFields.numberIntId, direction: .asc)
            )
            // Filtered to NumberInt > 5, sorted asc: 10(y), 20(z), 30(x).
            #expect(texts(of: results) == ["y", "z", "x"])

            await tryDeleteMany(ids)
        } catch {
            await tryDeleteMany(ids)
            throw error
        }
    }
}
