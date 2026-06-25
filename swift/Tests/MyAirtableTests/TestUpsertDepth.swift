// TC10 — Upsert depth. The base CRUD suite only covers single-record,
// single-merge-field upsert. This adds a multi-field merge key (a match must
// agree on ALL merge fields) and the multiple-match error path (a merge key
// matching more than one record is rejected by Airtable). Mirrors
// csharp/tests/TestUpsertDepth.cs. Note: the generated client exposes only
// single-record upsert, so batch upsert isn't covered here.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Upsert depth", .serialized)
struct TestUpsertDepth {
    private let airtable: Airtable

    init() {
        self.airtable = TestSetup.makeAirtable()
    }

    @Test("Upsert matches on multiple merge fields")
    func upsertMatchesOnMultipleMergeFields() async throws {
        let suite = TestSetup.primaryKey(for: "Upsert", "MultiKey")
        var ids: [String] = []

        do {
            // Seed a record identified by the (primaryKey, singleLineText) pair.
            let seed = try await airtable.primary.create(
                PrimaryModel(primaryKey: suite, singleLineText: "anchor")
            )
            guard let seedId = seed.id else {
                Issue.record("Missing id on seeded model")
                return
            }
            ids.append(seedId)

            let mergeOn = [PrimaryFields.primaryKeyId, PrimaryFields.singleLineTextId]

            // Same pair -> UPDATE the seed (matched on both fields).
            let (update, updateWasCreated) = try await airtable.primary.upsert(
                PrimaryModel(longText: "updated", primaryKey: suite, singleLineText: "anchor"),
                matchFieldsToMerge: mergeOn
            )
            #expect(!updateWasCreated)
            #expect(update.id == seedId)
            #expect(update.longText == "updated")

            // Same primaryKey but a DIFFERENT singleLineText -> no match on the pair -> INSERT.
            let (insert, insertWasCreated) = try await airtable.primary.upsert(
                PrimaryModel(primaryKey: suite, singleLineText: "different"),
                matchFieldsToMerge: mergeOn
            )
            if let insertId = insert.id { ids.append(insertId) }
            #expect(insertWasCreated)
            #expect(insert.id != seedId)
        } catch {
            await tryDeleteMany(ids)
            throw error
        }

        await tryDeleteMany(ids)
    }

    @Test("Upsert with multiple matches throws")
    func upsertWithMultipleMatchesThrows() async throws {
        let suite = TestSetup.primaryKey(for: "Upsert", "MultiMatch")
        var ids: [String] = []

        do {
            // Two records share the same singleLineText value.
            let a = try await airtable.primary.create(
                PrimaryModel(primaryKey: suite + " A", singleLineText: "dupe")
            )
            let b = try await airtable.primary.create(
                PrimaryModel(primaryKey: suite + " B", singleLineText: "dupe")
            )
            if let aId = a.id { ids.append(aId) }
            if let bId = b.id { ids.append(bId) }

            // Upsert merging only on singleLineText="dupe" matches BOTH -> Airtable rejects it.
            var thrown: Error?
            do {
                _ = try await airtable.primary.upsert(
                    PrimaryModel(longText: "x", singleLineText: "dupe"),
                    matchFieldsToMerge: [PrimaryFields.singleLineTextId]
                )
            } catch {
                thrown = error
            }

            guard let thrown else {
                Issue.record("Expected upsert with multiple matches to throw")
                return
            }
            guard let airtableError = thrown as? AirtableError else {
                Issue.record("Expected AirtableError, got \(type(of: thrown)): \(thrown)")
                return
            }
            if case .api = airtableError {
                // expected typed API error
            } else {
                Issue.record("Expected AirtableError.api, got \(airtableError)")
            }
        } catch {
            await tryDeleteMany(ids)
            throw error
        }

        await tryDeleteMany(ids)
    }

    /// Best-effort cleanup; swallows delete failures.
    private func tryDeleteMany(_ ids: [String]) async {
        guard !ids.isEmpty else { return }
        try? await airtable.primary.delete(ids)
    }
}
