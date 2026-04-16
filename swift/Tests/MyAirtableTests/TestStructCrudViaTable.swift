// F3.8 — Dict-table CRUD parity with rust/tests/test_struct_crud_via_table.rs.
//
// `@Suite(.serialized)` forces within-file sequential execution (matches
// vitest's default). Between-file parallelism is still enabled.

import Foundation
import Testing

@testable import MyAirtable

@Suite("DictTable CRUD via table", .serialized)
struct TestStructCrudViaTable {
    private let airtable: Airtable

    init() {
        self.airtable = TestSetup.makeAirtable()
    }

    // MARK: - Primary key only

    @Test("Primary-key-only CRUD round-trip")
    func primaryKeyOnlyCrud() async throws {
        let primaryKey = TestSetup.primaryKey(for: "StructCrud", "PKOnly")
        var fields = Fields(nameToId: PrimaryFields.nameToId)
        fields.setString(PrimaryFields.primaryKeyId, primaryKey)

        // Create
        let created = try await airtable.primary.dict.create(fields)
        #expect(!created.id.isEmpty)
        #expect(created.fields.getString(PrimaryFields.primaryKeyId) == primaryKey)

        let recordId = created.id

        do {
            // Read
            let fetched = try await airtable.primary.dict.get(recordId)
            #expect(fetched.id == recordId)
            #expect(fetched.fields.getString(PrimaryFields.primaryKeyId) == primaryKey)

            // Update
            var update = Fields(nameToId: PrimaryFields.nameToId)
            let updatedKey = primaryKey + " Updated"
            update.setString(PrimaryFields.primaryKeyId, updatedKey)
            let updated = try await airtable.primary.dict.update(
                recordId, fields: update)
            #expect(updated.fields.getString(PrimaryFields.primaryKeyId) == updatedKey)

            // Delete + verify gone
            try await airtable.primary.dict.delete(recordId)

            var deleted = false
            do {
                _ = try await airtable.primary.dict.get(recordId)
            } catch {
                deleted = true
            }
            #expect(deleted)
        } catch {
            // Best-effort cleanup on any intermediate failure.
            try? await airtable.primary.dict.delete(recordId)
            throw error
        }
    }

    // MARK: - Field-name access works alongside ID access (§2.2.5)

    @Test("DictTable supports dual ID/name field access")
    func dualIdNameAccess() async throws {
        let primaryKey = TestSetup.primaryKey(for: "StructCrud", "DualAccess")
        var fields = Fields(nameToId: PrimaryFields.nameToId)
        // Set by ID…
        fields.setString(PrimaryFields.primaryKeyId, primaryKey)

        let created = try await airtable.primary.dict.create(fields)
        let recordId = created.id

        do {
            // …read back by name.
            let viaName = created.fields.getString(PrimaryFields.primaryKeyName)
            #expect(viaName == primaryKey)
            // Also readable by ID.
            let viaId = created.fields.getString(PrimaryFields.primaryKeyId)
            #expect(viaId == primaryKey)
            try await airtable.primary.dict.delete(recordId)
        } catch {
            try? await airtable.primary.dict.delete(recordId)
            throw error
        }
    }

    // MARK: - All simple properties

    @Test("All simple properties round-trip")
    func allSimplePropertiesCrud() async throws {
        let primaryKey = TestSetup.primaryKey(for: "StructCrud", "AllProps")
        var fields = Fields(nameToId: PrimaryFields.nameToId)
        fields.setString(PrimaryFields.primaryKeyId, primaryKey)
        fields.setString(PrimaryFields.singleLineTextId, "Hello World")
        fields.setString(PrimaryFields.longTextId, "Long text content")
        fields.setString(PrimaryFields.longTextWithRichTextId, "Rich text content")
        fields.setString(PrimaryFields.emailId, "test@example.com")
        fields.setString(PrimaryFields.urlId, "https://example.com")
        fields.setString(PrimaryFields.phoneNumberId, "555-1234")
        fields.setBool(PrimaryFields.checkboxId, true)
        fields.setInt(PrimaryFields.numberIntId, 42)
        fields.setDouble(PrimaryFields.numberFloatId, 3.14)
        fields.setInt(PrimaryFields.currencyIntId, 10)
        fields.setDouble(PrimaryFields.currencyFloatId, 9.99)
        fields.setDouble(PrimaryFields.percentIntId, 0.5)
        fields.setDouble(PrimaryFields.percentFloatId, 0.333)
        fields.setInt(PrimaryFields.durationId, 3600)
        fields.setInt(PrimaryFields.ratingId, 3)
        fields.setString(PrimaryFields.dateId, "2025-01-15")
        fields.setString(PrimaryFields.dateWithTimeId, "2025-01-15T10:00:00.000Z")
        fields.setString(PrimaryFields.singleSelectId, "Choice 1")
        fields.setStrings(PrimaryFields.multipleSelectId, ["Option 1", "Option 2"])

        let created = try await airtable.primary.dict.create(fields)
        let recordId = created.id

        do {
            let f = created.fields
            #expect(f.getString(PrimaryFields.primaryKeyId) == primaryKey)
            #expect(f.getString(PrimaryFields.singleLineTextId) == "Hello World")
            #expect(f.getString(PrimaryFields.emailId) == "test@example.com")
            #expect(f.getBool(PrimaryFields.checkboxId) == true)
            #expect(f.getInt(PrimaryFields.numberIntId) == 42)
            #expect(f.getDouble(PrimaryFields.numberFloatId) == 3.14)
            #expect(f.getInt(PrimaryFields.ratingId) == 3)
            #expect(f.getString(PrimaryFields.singleSelectId) == "Choice 1")

            let multi = f.getArray(PrimaryFields.multipleSelectId) ?? []
            #expect(multi.count == 2)

            try await airtable.primary.dict.delete(recordId)
        } catch {
            try? await airtable.primary.dict.delete(recordId)
            throw error
        }
    }
}
