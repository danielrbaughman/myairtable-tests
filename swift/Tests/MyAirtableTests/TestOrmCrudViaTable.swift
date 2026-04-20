// F4.9 — ORM CRUD via table parity with rust/tests/test_orm_crud_via_table.rs.
// Complex field types (attachments, users, computed) are covered in F5.
// Batch 111-record CRUD is covered by the F5 chunking helper task.

import Foundation
import Testing

@testable import MyAirtable

@Suite("ORM CRUD via table", .serialized)
struct TestOrmCrudViaTable {
    private let airtable: Airtable

    init() {
        self.airtable = TestSetup.makeAirtable()
    }

    // MARK: - Primary key only

    @Test("Primary-key-only CRUD round-trip")
    func primaryKeyOnlyCrud() async throws {
        let primaryKey = TestSetup.primaryKey(for: "OrmTable", "PKOnly")
        let new = PrimaryModel(primaryKey: primaryKey)

        // Create
        let created = try await airtable.primary.create(new)
        #expect(created.id != nil)
        #expect(created.primaryKey == primaryKey)
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }

        do {
            // Read
            let fetched = try await airtable.primary.get(recordId)
            #expect(fetched.id == recordId)
            #expect(fetched.primaryKey == primaryKey)
            #expect(fetched.dirtyFields().isEmpty)  // fresh decode

            // Update via model + dirty tracking
            fetched.primaryKey = primaryKey + " Updated"
            #expect(!fetched.dirtyFields().isEmpty)
            let updated = try await airtable.primary.update(fetched)
            #expect(updated.primaryKey == primaryKey + " Updated")

            // Delete + verify gone
            try await airtable.primary.delete(recordId)
            var deleted = false
            do {
                _ = try await airtable.primary.get(recordId)
            } catch {
                deleted = true
            }
            #expect(deleted)
        } catch {
            try? await airtable.primary.delete(recordId)
            throw error
        }
    }

    // MARK: - All simple properties

    @Test("All simple properties round-trip")
    func allSimplePropertiesCrud() async throws {
        // F4 scope: basic scalar types (String / Bool / Int / Double). Typed
        // select enums, attachments, users, links, and specially-encoded dates
        // are covered in F5 and F6. Rating + currency(int) map to
        // `AirtableJSONValue` today — exercising them end-to-end waits on a
        // tightening of AIRTABLE_TO_GENERIC that's out of scope here.
        let primaryKey = TestSetup.primaryKey(for: "OrmTable", "AllProps")
        let new = PrimaryModel(
            checkbox: true,
            currencyFloat: 9.99,
            email: "test@example.com",
            longText: "Long text content",
            longTextWithRichText: "Rich text content",
            numberFloat: 3.14,
            numberInt: 42,
            percentFloat: 0.333,
            phoneNumber: "555-1234",
            primaryKey: primaryKey,
            singleLineText: "Hello World",
            url: "https://example.com"
        )

        let created = try await airtable.primary.create(new)
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }

        do {
            #expect(created.primaryKey == primaryKey)
            #expect(created.singleLineText == "Hello World")
            #expect(created.email == "test@example.com")
            #expect(created.checkbox == true)
            #expect(created.numberInt == 42)
            #expect(created.numberFloat == 3.14)
            #expect(created.currencyFloat == 9.99)
            #expect(created.url == "https://example.com")
            #expect(created.phoneNumber == "555-1234")

            try await airtable.primary.delete(recordId)
        } catch {
            try? await airtable.primary.delete(recordId)
            throw error
        }
    }

    // MARK: - List records

    @Test("get(query) returns a paginated record set")
    func listRecords() async throws {
        // Create a handful of records so the list is non-trivial.
        let suite = TestSetup.primaryKey(for: "OrmTable", "ListRecords")
        var createdIds: [String] = []

        do {
            for i in 0..<3 {
                let model = try await airtable.primary.create(
                    PrimaryModel(primaryKey: "\(suite) \(i)")
                )
                if let id = model.id { createdIds.append(id) }
            }

            // Filter to only our seeded records by formula.
            let escaped = suite.replacingOccurrences(of: "\"", with: "\\\"")
            let filterFormula = "FIND(\"\(escaped)\", {Primary Key}) > 0"
            let query = AirtableQuery().formula(filterFormula).maxRecords(10)
            let results = try await airtable.primary.get(query)
            #expect(results.count == 3)
            #expect(Set(results.compactMap { $0.id }) == Set(createdIds))
        }

        // Cleanup
        for id in createdIds {
            try? await airtable.primary.delete(id)
        }
    }

    // MARK: - Invalid record ID

    @Test("getOne with a well-formed-but-nonexistent record ID throws")
    func invalidRecordId() async throws {
        await #expect(throws: AirtableError.self) {
            _ = try await airtable.primary.get("recNONEXISTENT__")
        }
    }

    // MARK: - Batch create / update / delete (chunks by 10)

    @Test("Batch create / update / delete chunks above Airtable's 10-per-call limit")
    func batchCreateUpdateDelete() async throws {
        // 25 records exercises the chunking path (3 batches: 10 + 10 + 5).
        let count = 25
        let suite = TestSetup.primaryKey(for: "OrmTable", "Batch")

        let creates = (1...count).map { i in
            // Init params are alphabetical (numberInt before primaryKey).
            PrimaryModel(numberInt: i, primaryKey: "\(suite) \(i)")
        }
        let created = try await airtable.primary.create(creates)
        #expect(created.count == count)
        for (i, model) in created.enumerated() {
            #expect(model.primaryKey == "\(suite) \(i + 1)")
        }

        let ids = created.compactMap { $0.id }
        guard ids.count == count else {
            Issue.record("Some created records were missing ids")
            return
        }

        do {
            // Update all: mutate each model's primaryKey, then batch-update.
            for (i, model) in created.enumerated() {
                model.primaryKey = "\(suite) \(i + 1) Updated"
            }
            let updated = try await airtable.primary.update(created)
            #expect(updated.count == count)
            for (i, model) in updated.enumerated() {
                #expect(model.primaryKey == "\(suite) \(i + 1) Updated")
            }

            // Delete all by ID list.
            try await airtable.primary.delete(ids)
            await #expect(throws: AirtableError.self) {
                _ = try await airtable.primary.get(ids[0])
            }
        } catch {
            try? await airtable.primary.delete(ids)
            throw error
        }
    }

    // MARK: - Upsert

    @Test("upsertOne inserts when no match exists")
    func upsertAsCreate() async throws {
        let primaryKey = TestSetup.primaryKey(for: "OrmTable", "UpsertCreate")
        let new = PrimaryModel(primaryKey: primaryKey)

        let (model, wasCreated) = try await airtable.primary.upsert(
            new,
            matchFieldsToMerge: [PrimaryFields.primaryKeyId]
        )
        #expect(wasCreated)
        #expect(model.primaryKey == primaryKey)

        if let id = model.id {
            try? await airtable.primary.delete(id)
        }
    }

    @Test("upsertOne updates an existing record matched by primary key")
    func upsertAsUpdate() async throws {
        let primaryKey = TestSetup.primaryKey(for: "OrmTable", "UpsertUpdate")

        // Seed a record.
        let seeded = try await airtable.primary.create(
            PrimaryModel(primaryKey: primaryKey, singleLineText: "before")
        )
        guard let seededId = seeded.id else {
            Issue.record("Missing id on seeded model")
            return
        }

        do {
            // Upsert by primary key; should match the seeded record and update it.
            let (merged, wasCreated) = try await airtable.primary.upsert(
                PrimaryModel(primaryKey: primaryKey, singleLineText: "after"),
                matchFieldsToMerge: [PrimaryFields.primaryKeyId]
            )
            #expect(!wasCreated)
            #expect(merged.id == seededId)
            #expect(merged.singleLineText == "after")

            try await airtable.primary.delete(seededId)
        } catch {
            try? await airtable.primary.delete(seededId)
            throw error
        }
    }
}
