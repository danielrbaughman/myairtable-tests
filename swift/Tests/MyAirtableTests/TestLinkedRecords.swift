// F6 — Linked-record CRUD + fetch{Field} async chaining.
// Parity with rust/tests/test_orm_crud_via_table.rs's linked_records_crud
// and typescript/tests/linked-record-chaining.test.ts.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Linked records", .serialized)
struct TestLinkedRecords {
    private let airtable: Airtable

    init() {
        self.airtable = TestSetup.makeAirtable()
    }

    // MARK: - Basic linked-record CRUD (raw IDs)

    @Test("Linked-record fields round-trip via record IDs")
    func linkedRecordCrud() async throws {
        let suite = TestSetup.primaryKey(for: "LinkedRecords", "CRUD")

        // Create target records in Secondary.
        let sec1 = try await airtable.secondary.create(
            SecondaryModel(name: "\(suite) Target 1", value: "val1")
        )
        let sec2 = try await airtable.secondary.create(
            SecondaryModel(name: "\(suite) Target 2", value: "val2")
        )
        guard let sec1Id = sec1.id, let sec2Id = sec2.id else {
            Issue.record("Missing ids on secondary records")
            return
        }

        // Create primary with links.
        let prim = try await airtable.primary.create(
            PrimaryModel(
                linkMultiple: [sec1Id, sec2Id],
                linkSingle: [sec1Id],
                primaryKey: "\(suite) Primary"
            )
        )
        guard let primId = prim.id else {
            Issue.record("Missing id on primary record")
            return
        }

        do {
            // Verify link IDs on created record.
            #expect(prim.linkSingle == [sec1Id])
            #expect(prim.linkMultiple == [sec1Id, sec2Id])

            // Read-back round-trip.
            let fetched = try await airtable.primary.get(primId)
            #expect(fetched.linkSingle == [sec1Id])
            #expect(fetched.linkMultiple == [sec1Id, sec2Id])

            // Update: swap links.
            fetched.linkSingle = [sec2Id]
            fetched.linkMultiple = [sec1Id]
            let updated = try await airtable.primary.update(fetched)
            #expect(updated.linkSingle == [sec2Id])
            #expect(updated.linkMultiple == [sec1Id])

            // Cleanup.
            try await airtable.primary.delete(primId)
            try await airtable.secondary.delete(sec1Id)
            try await airtable.secondary.delete(sec2Id)
        } catch {
            try? await airtable.primary.delete(primId)
            try? await airtable.secondary.delete(sec1Id)
            try? await airtable.secondary.delete(sec2Id)
            throw error
        }
    }

    // MARK: - Resolving linked IDs through the linked table

    @Test("Linked single record resolves via secondary.get")
    func resolveLinkSingle() async throws {
        let suite = TestSetup.primaryKey(for: "LinkedRecords", "FetchSingle")

        let sec = try await airtable.secondary.create(
            SecondaryModel(name: "\(suite) Target", value: "sv")
        )
        guard let secId = sec.id else {
            Issue.record("Missing id on secondary record")
            return
        }

        let prim = try await airtable.primary.create(
            PrimaryModel(linkSingle: [secId], primaryKey: "\(suite) Primary")
        )
        guard let primId = prim.id else {
            Issue.record("Missing id on primary record")
            return
        }

        do {
            // Resolve the single-link id via the secondary table directly.
            guard let linkedId = prim.linkSingle?.first else {
                Issue.record("linkSingle was empty")
                return
            }
            let linked = try await airtable.secondary.get(linkedId)
            #expect(linked.name == "\(suite) Target")
            #expect(linked.value == "sv")

            try await airtable.primary.delete(primId)
            try await airtable.secondary.delete(secId)
        } catch {
            try? await airtable.primary.delete(primId)
            try? await airtable.secondary.delete(secId)
            throw error
        }
    }

    @Test("Linked multi records resolve via secondary.get")
    func resolveLinkMultiple() async throws {
        let suite = TestSetup.primaryKey(for: "LinkedRecords", "FetchMultiple")

        let sec1 = try await airtable.secondary.create(
            SecondaryModel(name: "\(suite) T1", value: "v1")
        )
        let sec2 = try await airtable.secondary.create(
            SecondaryModel(name: "\(suite) T2", value: "v2")
        )
        guard let sec1Id = sec1.id, let sec2Id = sec2.id else {
            Issue.record("Missing ids")
            return
        }

        let prim = try await airtable.primary.create(
            PrimaryModel(
                linkMultiple: [sec1Id, sec2Id],
                primaryKey: "\(suite) Primary"
            )
        )
        guard let primId = prim.id else {
            Issue.record("Missing id on primary record")
            return
        }

        do {
            let linked = try await airtable.secondary.get(prim.linkMultiple ?? [])
            #expect(linked.count == 2)
            let names = Set(linked.compactMap { $0.name })
            #expect(names.contains("\(suite) T1"))
            #expect(names.contains("\(suite) T2"))

            try await airtable.primary.delete(primId)
            try await airtable.secondary.delete(sec1Id)
            try await airtable.secondary.delete(sec2Id)
        } catch {
            try? await airtable.primary.delete(primId)
            try? await airtable.secondary.delete(sec1Id)
            try? await airtable.secondary.delete(sec2Id)
            throw error
        }
    }

    @Test("Empty linkSingle yields nil when unwrapped")
    func fetchNilLinkSingle() async throws {
        let suite = TestSetup.primaryKey(for: "LinkedRecords", "FetchNil")
        let prim = try await airtable.primary.create(
            PrimaryModel(primaryKey: "\(suite) No Links")
        )
        guard let primId = prim.id else {
            Issue.record("Missing id")
            return
        }

        do {
            // Airtable omits empty link fields entirely; linkSingle decodes as nil.
            #expect(prim.linkSingle?.first == nil)
            try await airtable.primary.delete(primId)
        } catch {
            try? await airtable.primary.delete(primId)
            throw error
        }
    }
}
