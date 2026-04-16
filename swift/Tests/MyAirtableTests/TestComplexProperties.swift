// F5.6 — Complex field-type CRUD: attachments, collaborators (users), and
// computed fields (auto-number, formulas, created-time). Parity with
// rust/tests/test_orm_crud_via_table.rs `attachment_crud`, `user_fields_crud`,
// `computed_fields`.
//
// Linked records are covered by F6 (TestLinkedRecordChaining). Buttons are
// read-only — exercised here as part of the computed-fields path.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Complex properties — attachments, users, computed", .serialized)
struct TestComplexProperties {
    private let airtable: Airtable

    init() {
        self.airtable = TestSetup.makeAirtable()
    }

    // MARK: - Attachments (URL-based create, async processing)

    @Test("Attachment field round-trips via URL")
    func attachmentCrud() async throws {
        let primaryKey = TestSetup.primaryKey(for: "Complex", "Attach")
        let url =
            "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"

        let new = CreatePrimaryModel(
            attachment: [AirtableAttachment(url: url)],
            primaryKey: primaryKey
        )
        let created = try await airtable.primary.create(new)
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }

        do {
            // Airtable processes attachments asynchronously — the freshly-created
            // record may have the attachment present with just the URL set, or
            // fully populated with id/size/thumbnails once processing completes.
            let initialAttachments = created.attachment ?? []
            #expect(initialAttachments.count == 1)

            // Poll until Airtable has enriched the attachment (id populated).
            var current = created
            for _ in 0..<10 {
                let first = current.attachment?.first
                if let id = first?.id, !id.isEmpty { break }
                try await Task.sleep(nanoseconds: 2_000_000_000)
                current = try await airtable.primary.get(recordId)
            }
            let enriched = current.attachment ?? []
            #expect(enriched.count == 1)
            #expect((enriched.first?.url ?? "").isEmpty == false)

            try await airtable.primary.delete(recordId)
        } catch {
            try? await airtable.primary.delete(recordId)
            throw error
        }
    }

    // MARK: - Collaborators (users)

    @Test("User + multi-user fields round-trip")
    func userFieldsCrud() async throws {
        let primaryKey = TestSetup.primaryKey(for: "Complex", "Users")
        // Matches the user ID used by the Rust test in the shared test base.
        let userId = "usrnZ4k98m0Ipji4e"

        let new = CreatePrimaryModel(
            primaryKey: primaryKey,
            user: AirtableCollaborator(id: userId),
            userAllowMultiple: [AirtableCollaborator(id: userId)]
        )
        let created = try await airtable.primary.create(new)
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }

        do {
            #expect(created.user?.id == userId)
            let multi = created.userAllowMultiple ?? []
            #expect(multi.count == 1)
            #expect(multi.first?.id == userId)

            // Read-back round-trip.
            let fetched = try await airtable.primary.get(recordId)
            #expect(fetched.user?.id == userId)

            try await airtable.primary.delete(recordId)
        } catch {
            try? await airtable.primary.delete(recordId)
            throw error
        }
    }

    // MARK: - Computed fields

    @Test("Computed fields populate on create and read-back")
    func computedFields() async throws {
        let primaryKey = TestSetup.primaryKey(for: "Complex", "Computed")
        let new = CreatePrimaryModel(
            numberFloat: 5.0,
            numberInt: 10,
            primaryKey: primaryKey
        )
        let created = try await airtable.primary.create(new)
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }

        do {
            // Server-owned: auto number assigned; created time populated.
            #expect(created.autoNumber != nil)
            #expect(created.createdTime != nil)
            #expect(created.createdAtTime != nil)  // dedicated createdTime field
            // Formula(ID) reflects the record id.
            #expect(created.formulaId == recordId)
            // Formula(Simple) = numberInt + numberFloat = 15.
            #expect(created.formulaSimple == 15.0)
            // Created-by is populated with the caller's user info.
            #expect(created.createdBy != nil)

            // Read-back round-trip.
            let fetched = try await airtable.primary.get(recordId)
            #expect(fetched.formulaId == recordId)
            #expect(fetched.formulaSimple == 15.0)
            #expect(fetched.autoNumber == created.autoNumber)

            try await airtable.primary.delete(recordId)
        } catch {
            try? await airtable.primary.delete(recordId)
            throw error
        }
    }

    // MARK: - Button field (read-only)

    @Test("Button field decodes on read")
    func buttonDecodes() async throws {
        let primaryKey = TestSetup.primaryKey(for: "Complex", "Button")
        let new = CreatePrimaryModel(primaryKey: primaryKey)
        let created = try await airtable.primary.create(new)
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }

        do {
            // Button fields carry a label + url read from the schema; the
            // record-shaped payload echoes them back as a structured value.
            // We just assert the field decodes without throwing and exposes
            // the schema's label/url (or nil for records where the button
            // has no URL resolved yet).
            if let btn = created.button {
                #expect((btn.label ?? "").isEmpty == false || btn.url != nil)
            }

            try await airtable.primary.delete(recordId)
        } catch {
            try? await airtable.primary.delete(recordId)
            throw error
        }
    }
}
