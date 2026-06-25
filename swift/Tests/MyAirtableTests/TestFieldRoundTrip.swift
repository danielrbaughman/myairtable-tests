// TC7 — Field-type round-trip completeness via re-fetch. Several field types were only
// asserted on the create response (or only offline-decoded), never written and read back
// through the live API, and clearing/removing multi-value fields was untested. Each case
// here creates, optionally updates, re-fetches, and asserts the server-side value. Parity
// with csharp/tests/TestFieldRoundTrip.cs.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Field-type round-trip via re-fetch", .serialized)
struct TestFieldRoundTrip {
    private let airtable: Airtable

    init() {
        self.airtable = TestSetup.makeAirtable()
    }

    /// Shared-base user, as in TestComplexProperties.
    private static let userId = "usrnZ4k98m0Ipji4e"

    private static func utc(_ s: String) -> Date {
        let iso = ISO8601DateFormatter()
        iso.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
        if let date = iso.date(from: s) { return date }
        iso.formatOptions = [.withInternetDateTime]
        return iso.date(from: s)!
    }

    private func tryDelete(_ recordId: String?) async {
        guard let recordId, !recordId.isEmpty else { return }
        try? await airtable.primary.delete(recordId)
    }

    // MARK: - Date with time

    @Test("DateWithTime writes and reads back")
    func dateWithTimeWritesAndReadsBack() async throws {
        let suite = TestSetup.primaryKey(for: "FieldRT", "DateTime")
        let dt = Self.utc("2024-03-15T14:30:00.000Z")
        let created = try await airtable.primary.create(
            PrimaryModel(dateWithTime: dt, primaryKey: suite)
        )
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }
        do {
            let fetched = try await airtable.primary.get(recordId)
            #expect(fetched.dateWithTime == dt)
            await tryDelete(recordId)
        } catch {
            await tryDelete(recordId)
            throw error
        }
    }

    // MARK: - Rich text + percent/currency

    @Test("Rich text and percent/currency read back")
    func richTextAndPercentCurrencyReadBack() async throws {
        let suite = TestSetup.primaryKey(for: "FieldRT", "Rich")
        let created = try await airtable.primary.create(
            PrimaryModel(
                currencyFloat: 19.99,
                currencyInt: 100.0,
                longTextWithRichText: "**bold** and _italic_ text",
                percentFloat: 0.333,
                percentInt: 0.5,
                primaryKey: suite
            )
        )
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }
        do {
            let fetched = try await airtable.primary.get(recordId)
            #expect(fetched.longTextWithRichText == "**bold** and _italic_ text")
            #expect(fetched.percentInt == 0.5)
            #expect(fetched.percentFloat == 0.333)
            #expect(fetched.currencyInt == 100.0)
            #expect(fetched.currencyFloat == 19.99)
            await tryDelete(recordId)
        } catch {
            await tryDelete(recordId)
            throw error
        }
    }

    // MARK: - Clear single + multi select

    @Test("Clearing single and multi select reads back empty")
    func clearingSingleAndMultiSelectReadsBackEmpty() async throws {
        let suite = TestSetup.primaryKey(for: "FieldRT", "ClearSelect")
        let created = try await airtable.primary.create(
            PrimaryModel(
                multipleSelect: [.option1, .option2],
                primaryKey: suite,
                singleSelect: .choice1
            )
        )
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }
        do {
            #expect(created.singleSelect == .choice1)

            created.singleSelect = nil
            created.multipleSelect = []
            _ = try await airtable.primary.update(created)

            let fetched = try await airtable.primary.get(recordId)
            #expect(fetched.singleSelect == nil)
            #expect((fetched.multipleSelect ?? []).isEmpty)
            await tryDelete(recordId)
        } catch {
            await tryDelete(recordId)
            throw error
        }
    }

    // MARK: - Remove collaborator

    @Test("Removing a collaborator reads back nil")
    func removingACollaboratorReadsBackNil() async throws {
        let suite = TestSetup.primaryKey(for: "FieldRT", "RemoveUser")
        let created = try await airtable.primary.create(
            PrimaryModel(primaryKey: suite, user: AirtableCollaborator(id: Self.userId))
        )
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }
        do {
            #expect(created.user?.id == Self.userId)

            created.user = nil
            _ = try await airtable.primary.update(created)

            let fetched = try await airtable.primary.get(recordId)
            #expect(fetched.user == nil)
            await tryDelete(recordId)
        } catch {
            await tryDelete(recordId)
            throw error
        }
    }

    // MARK: - Attachment replace + remove

    @Test("Attachment replace and remove read back")
    func attachmentReplaceAndRemoveReadBack() async throws {
        let urlA =
            "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"
        let urlB = "https://www.w3.org/Icons/w3c_home.png"
        let suite = TestSetup.primaryKey(for: "FieldRT", "Attach")
        let created = try await airtable.primary.create(
            PrimaryModel(attachment: [AirtableAttachment(url: urlA)], primaryKey: suite)
        )
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }
        do {
            // Replace the attachment with a different one.
            created.attachment = [AirtableAttachment(url: urlB)]
            _ = try await airtable.primary.update(created)
            let replaced = try await airtable.primary.get(recordId)
            #expect((replaced.attachment ?? []).count == 1)

            // Remove all attachments.
            replaced.attachment = []
            _ = try await airtable.primary.update(replaced)
            let cleared = try await airtable.primary.get(recordId)
            #expect((cleared.attachment ?? []).isEmpty)
            await tryDelete(recordId)
        } catch {
            await tryDelete(recordId)
            throw error
        }
    }
}
