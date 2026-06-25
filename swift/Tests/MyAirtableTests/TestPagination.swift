// TC1 — Multi-page pagination: list more records than Airtable's 100-record
// default page in a single query (no maxRecords) and assert the client's
// offset loop reassembles every page. Exercises the page-reassembly path in the
// generated client. Parity with csharp/tests/TestPagination.cs.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Pagination", .serialized)
struct TestPagination {
    private let airtable: Airtable

    init() {
        self.airtable = TestSetup.makeAirtable()
    }

    // 105 > the 100-record default page size, so a no-maxRecords list spans two pages.
    private static let count = 105

    // MARK: - ORM / typed accessor

    @Test("ORM get spanning multiple pages returns every record")
    func ormGetSpanningMultiplePagesReturnsEveryRecord() async throws {
        let suite = TestSetup.primaryKey(for: "Pagination", "Orm")
        let models = (1...Self.count).map { PrimaryModel(primaryKey: "\(suite) \($0)") }
        var createdIds: [String] = []

        do {
            let created = try await airtable.primary.create(models)
            createdIds = created.compactMap { $0.id }
            #expect(created.count == Self.count)

            // No maxRecords: the offset loop must walk both pages and return all 105.
            let results = try await airtable.primary.get(
                AirtableQuery().formula(formula(suite))
            )
            #expect(results.count == Self.count)
            #expect(Set(results.compactMap { $0.id }) == Set(createdIds))

            try await tryDeleteMany(createdIds, suite: suite)
        } catch {
            try await tryDeleteMany(createdIds, suite: suite)
            throw error
        }
    }

    // MARK: - Dict / untyped accessor

    @Test("Dict get spanning multiple pages returns every record")
    func dictGetSpanningMultiplePagesReturnsEveryRecord() async throws {
        let suite = TestSetup.primaryKey(for: "Pagination", "Dict")
        let creates = (1...Self.count).map { i -> Fields in
            var fields = Fields(nameToId: PrimaryFields.nameToId)
            fields.setString(PrimaryFields.primaryKeyId, "\(suite) \(i)")
            return fields
        }
        var createdIds: [String] = []

        do {
            let created = try await airtable.primary.dict.create(creates)
            createdIds = created.map { $0.id }
            #expect(created.count == Self.count)

            let results = try await airtable.primary.dict.get(
                AirtableQuery().formula(formula(suite))
            )
            #expect(results.count == Self.count)
            #expect(Set(results.map { $0.id }) == Set(createdIds))

            try await tryDeleteMany(createdIds, suite: suite)
        } catch {
            try await tryDeleteMany(createdIds, suite: suite)
            throw error
        }
    }

    // MARK: - TC5 — explicit page size

    // 25 records over pageSize 10 spans 3 pages (10 + 10 + 5).
    private static let pageSizeCount = 25

    @Test("Explicit pageSize returns all records across pages")
    func explicitPageSizeReturnsAllRecordsAcrossPages() async throws {
        let suite = TestSetup.primaryKey(for: "Pagination", "PageSize")
        let models = (1...Self.pageSizeCount).map { PrimaryModel(primaryKey: "\(suite) \($0)") }
        var createdIds: [String] = []

        do {
            let created = try await airtable.primary.create(models)
            createdIds = created.compactMap { $0.id }

            // pageSize 10, NO maxRecords: the offset loop must walk all 3 pages and return all 25.
            let results = try await airtable.primary.get(
                AirtableQuery().formula(formula(suite)).pageSize(10)
            )
            #expect(results.count == Self.pageSizeCount)

            try await tryDeleteMany(createdIds, suite: suite)
        } catch {
            try await tryDeleteMany(createdIds, suite: suite)
            throw error
        }
    }

    @Test("pageSize with maxRecords caps the total")
    func pageSizeWithMaxRecordsCapsTheTotal() async throws {
        let suite = TestSetup.primaryKey(for: "Pagination", "PageCap")
        let models = (1...Self.pageSizeCount).map { PrimaryModel(primaryKey: "\(suite) \($0)") }
        var createdIds: [String] = []

        do {
            let created = try await airtable.primary.create(models)
            createdIds = created.compactMap { $0.id }

            // pageSize 10 + maxRecords 15: maxRecords caps the total mid-stream (not a page multiple).
            let results = try await airtable.primary.get(
                AirtableQuery().formula(formula(suite)).pageSize(10).maxRecords(15)
            )
            #expect(results.count == 15)

            try await tryDeleteMany(createdIds, suite: suite)
        } catch {
            try await tryDeleteMany(createdIds, suite: suite)
            throw error
        }
    }

    // MARK: - Helpers

    /// FIND("<suite>", {Primary Key}) — scopes a list to this run's records.
    private func formula(_ suite: String) -> String {
        let escaped = suite.replacingOccurrences(of: "\"", with: "\\\"")
        return "FIND(\"\(escaped)\", {Primary Key})"
    }

    /// Best-effort cleanup: delete created ids; on failure, sweep by FIND prefix.
    private func tryDeleteMany(_ ids: [String], suite: String) async throws {
        if !ids.isEmpty {
            do {
                try await airtable.primary.dict.delete(ids)
                return
            } catch {
                // fall through to a prefix sweep
            }
        }
        do {
            let stray = try await airtable.primary.dict.get(
                AirtableQuery().formula(formula(suite))
            )
            if !stray.isEmpty {
                try await airtable.primary.dict.delete(stray.map { $0.id })
            }
        } catch {
            // best-effort cleanup
        }
    }
}
