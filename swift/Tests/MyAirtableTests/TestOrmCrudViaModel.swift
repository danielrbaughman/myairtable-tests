// F4.10 — ORM CRUD via model extension methods (`save` / `refresh` / `delete`).
// Parity with rust/tests/test_orm_crud_via_model.rs's `model.save()` /
// `.fetch()` / `.delete()` fluent pattern.

import Foundation
import Testing

@testable import MyAirtable

@Suite("ORM CRUD via model", .serialized)
struct TestOrmCrudViaModel {
    private let airtable: Airtable

    init() {
        // Exercise the direct client-injection path to match the Rust test's
        // `Arc<AirtableClient>` construction style.
        TestSetup.loadDotEnvIfNeeded()
        let env = ProcessInfo.processInfo.environment
        guard let apiKey = env["AIRTABLE_API_KEY"], let baseId = env["AIRTABLE_BASE_ID"] else {
            fatalError("AIRTABLE_API_KEY / AIRTABLE_BASE_ID must be set")
        }
        let client = AirtableClient(baseId: baseId, apiKey: apiKey)
        self.airtable = Airtable(client: client)
    }

    // MARK: - Primary key only

    @Test("Primary-key-only CRUD via model methods")
    func primaryKeyOnlyViaModel() async throws {
        let primaryKey = TestSetup.primaryKey(for: "OrmModel", "PKOnly")

        // Create has to go through the Create payload (class can't self-create).
        let created = try await airtable.primary.orm.createOne(
            CreatePrimaryModel(primaryKey: primaryKey),
            typecast: true
        )
        #expect(created.isNew == false)
        #expect(created.primaryKey == primaryKey)
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }

        do {
            // Refresh — returns a fresh instance with server state.
            let refreshed = try await created.refresh(via: airtable.primary.orm)
            #expect(refreshed.id == recordId)
            #expect(refreshed.primaryKey == primaryKey)
            #expect(refreshed.dirtyFields().isEmpty)

            // Update via model.save()
            refreshed.primaryKey = primaryKey + " Updated"
            #expect(!refreshed.dirtyFields().isEmpty)
            let saved = try await refreshed.save(via: airtable.primary.orm, typecast: true)
            #expect(saved.primaryKey == primaryKey + " Updated")

            // Delete via model.delete()
            try await saved.delete(via: airtable.primary.orm)

            // Verify deleted
            var wasDeleted = false
            do {
                _ = try await saved.refresh(via: airtable.primary.orm)
            } catch {
                wasDeleted = true
            }
            #expect(wasDeleted)
        } catch {
            try? await airtable.primary.orm.deleteOne(recordId)
            throw error
        }
    }

    // MARK: - Dirty tracking across a save cycle

    @Test("Dirty tracking resets after a successful save")
    func dirtyTrackingAcrossSave() async throws {
        let primaryKey = TestSetup.primaryKey(for: "OrmModel", "Dirty")
        let created = try await airtable.primary.orm.createOne(
            CreatePrimaryModel(primaryKey: primaryKey),
            typecast: true
        )
        guard let recordId = created.id else {
            Issue.record("Missing id")
            return
        }

        do {
            // Fresh create — no dirty fields.
            #expect(created.dirtyFields().isEmpty)

            // Mutate and verify dirty set.
            created.primaryKey = primaryKey + " A"
            created.singleLineText = "First"
            let dirty = created.dirtyFields()
            #expect(dirty[PrimaryFields.primaryKeyId] == .string(primaryKey + " A"))
            #expect(dirty[PrimaryFields.singleLineTextId] == .string("First"))

            // Save — the returned model is a fresh instance with a fresh snapshot.
            let saved = try await created.save(via: airtable.primary.orm, typecast: true)
            #expect(saved.dirtyFields().isEmpty)
            #expect(saved.singleLineText == "First")

            try await saved.delete(via: airtable.primary.orm)
        } catch {
            try? await airtable.primary.orm.deleteOne(recordId)
            throw error
        }
    }

    // MARK: - Error cases

    @Test("Delete on an unsaved model throws")
    func deleteOnUnsavedThrows() async throws {
        // Decode a bare envelope (no id) to produce an unsaved model instance.
        let json = """
            {"fields": {"\(PrimaryFields.primaryKeyId)": "x"}}
            """
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let model = try decoder.decode(PrimaryModel.self, from: Data(json.utf8))
        #expect(model.isNew == true)

        await #expect(throws: AirtableError.self) {
            try await model.delete(via: airtable.primary.orm)
        }
    }

    @Test("Refresh on an unsaved model throws")
    func refreshOnUnsavedThrows() async throws {
        let json = """
            {"fields": {"\(PrimaryFields.primaryKeyId)": "x"}}
            """
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let model = try decoder.decode(PrimaryModel.self, from: Data(json.utf8))

        await #expect(throws: AirtableError.self) {
            _ = try await model.refresh(via: airtable.primary.orm)
        }
    }
}
