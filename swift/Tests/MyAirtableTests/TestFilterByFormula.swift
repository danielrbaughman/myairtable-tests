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

    // MARK: - Shared helpers (parity with rust scope_to / primary helpers)

    /// Scope a formula to specific record IDs so suite runs don't interfere.
    private func scopeTo(_ ids: [String]) -> String {
        let parts = ids.map { "RECORD_ID()='\($0)'" }
        return parts.count == 1 ? parts[0] : Formulas.or(parts)
    }

    /// Dict-path fields container with just the primary key set.
    private func primaryFields(_ name: String) -> Fields {
        var fields = Fields(nameToId: PrimaryFields.nameToId)
        fields.setString(PrimaryFields.primaryKeyId, name)
        return fields
    }

    private func names(of records: [DictTable.Record]) -> [String] {
        records.compactMap { $0.fields.getString(PrimaryFields.primaryKeyId) }.sorted()
    }

    // MARK: - Filter by view

    @Test("Filter by view returns only view-filtered records")
    func filterByView() async throws {
        var creates: [Fields] = []
        for i in 0..<5 { creates.append(primaryFields("Filter Test \(i)")) }
        for i in 0..<5 { creates.append(primaryFields("Don't Include Test \(i)")) }
        let created = try await airtable.primary.dict.create(creates)
        let ids = created.map { $0.id }

        do {
            let results = try await airtable.primary.dict.get(
                AirtableQuery().view(PrimaryView.filterByView.rawValue)
            )
            #expect(results.count >= 5)
            for record in results {
                let name = record.fields.getString(PrimaryFields.primaryKeyId) ?? ""
                #expect(name.hasPrefix("Filter Test"), "Unexpected record: \(name)")
            }
            try await airtable.primary.dict.delete(ids)
        } catch {
            try? await airtable.primary.dict.delete(ids)
            throw error
        }
    }

    // MARK: - Record ID inList

    @Test("Filter by record ID inList")
    func recordIdInList() async throws {
        let suite = TestSetup.primaryKey(for: "Filter", "InList")
        let created = try await airtable.primary.dict.create(
            (0..<3).map { primaryFields("\(suite) \($0)") }
        )
        let ids = created.map { $0.id }

        do {
            let f = PrimaryModel.f

            // inList multiple
            var results = try await airtable.primary.dict.get(
                AirtableQuery().formula(f.id.inList([ids[0], ids[1]]))
            )
            #expect(results.count == 2)

            // inList single
            results = try await airtable.primary.dict.get(
                AirtableQuery().formula(f.id.inList([ids[0]]))
            )
            #expect(results.count == 1)

            // inList empty
            results = try await airtable.primary.dict.get(
                AirtableQuery().formula(f.id.inList([]))
            )
            #expect(results.isEmpty)

            try await airtable.primary.dict.delete(ids)
        } catch {
            try? await airtable.primary.dict.delete(ids)
            throw error
        }
    }

    // MARK: - Text field predicates

    @Test("Text field predicate coverage")
    func textFieldPredicates() async throws {
        let created = try await airtable.primary.dict.create([
            primaryFields("FbText Alpha One"),
            primaryFields("FbText Alpha Two"),
            primaryFields("FbText Beta One"),
        ])
        let ids = created.map { $0.id }
        let scope = scopeTo(ids)
        let f = PrimaryModel.f

        func count(_ formula: String) async throws -> Int {
            try await airtable.primary.dict.get(
                AirtableQuery().formula(Formulas.and(scope, formula))
            ).count
        }

        do {
            #expect(try await count(f.primaryKey.equals("FbText Alpha One")) == 1)
            #expect(try await count(f.primaryKey.notEquals("FbText Alpha One")) == 2)
            #expect(try await count(f.primaryKey.contains("Alpha", caseSensitive: false, trim: true)) == 2)
            #expect(
                try await count(f.primaryKey.containsAny(["Alpha", "Beta"], caseSensitive: false, trim: true)) == 3)
            #expect(try await count(f.primaryKey.containsAll(["Alpha", "One"], caseSensitive: false, trim: true)) == 1)
            #expect(try await count(f.primaryKey.notContains("Alpha", caseSensitive: false, trim: true)) == 1)
            #expect(try await count(f.primaryKey.startsWith("FbText Alpha", caseSensitive: false, trim: true)) == 2)
            #expect(try await count(f.primaryKey.notStartsWith("FbText Alpha", caseSensitive: false, trim: true)) == 1)

            try await airtable.primary.dict.delete(ids)
        } catch {
            try? await airtable.primary.dict.delete(ids)
            throw error
        }
    }

    // MARK: - Number field predicates

    @Test("Number field predicate coverage")
    func numberFieldPredicates() async throws {
        let suite = TestSetup.primaryKey(for: "Filter", "NumPreds")
        var creates: [Fields] = []
        for (i, n) in [10, 20, 30].enumerated() {
            var fields = primaryFields("\(suite) \(i)")
            fields.setInt(PrimaryFields.numberIntId, n)
            creates.append(fields)
        }
        let created = try await airtable.primary.dict.create(creates)
        let ids = created.map { $0.id }
        let scope = scopeTo(ids)
        let f = PrimaryModel.f

        func count(_ formula: String) async throws -> Int {
            try await airtable.primary.dict.get(
                AirtableQuery().formula(Formulas.and(scope, formula))
            ).count
        }

        do {
            #expect(try await count(f.numberInt.equals(20)) == 1)
            #expect(try await count(f.numberInt.notEquals(20)) == 2)
            #expect(try await count(f.numberInt.greaterThan(10)) == 2)
            #expect(try await count(f.numberInt.lessThan(30)) == 2)
            #expect(try await count(f.numberInt.greaterThanOrEquals(20)) == 2)
            #expect(try await count(f.numberInt.lessThanOrEquals(20)) == 2)
            #expect(try await count(f.numberInt.between(10, 30)) == 3)
            #expect(try await count(f.numberInt.between(10, 30, inclusive: false)) == 1)

            try await airtable.primary.dict.delete(ids)
        } catch {
            try? await airtable.primary.dict.delete(ids)
            throw error
        }
    }

    // MARK: - Boolean field

    @Test("Boolean field filter (equals / isTrue / isFalse)")
    func booleanFieldFilter() async throws {
        let suite = TestSetup.primaryKey(for: "Filter", "Bool")
        var f1 = primaryFields("\(suite) A")
        f1.setBool(PrimaryFields.checkboxId, true)
        var f2 = primaryFields("\(suite) B")
        f2.setBool(PrimaryFields.checkboxId, false)
        let created = try await airtable.primary.dict.create([f1, f2])
        let ids = created.map { $0.id }
        let scope = scopeTo(ids)
        let f = PrimaryModel.f

        func count(_ formula: String) async throws -> Int {
            try await airtable.primary.dict.get(
                AirtableQuery().formula(Formulas.and(scope, formula))
            ).count
        }

        do {
            #expect(try await count(f.checkbox.equals(true)) == 1)
            #expect(try await count(f.checkbox.equals(false)) >= 1)
            #expect(try await count(f.checkbox.isTrue()) == 1)
            #expect(try await count(f.checkbox.isFalse()) >= 1)

            try await airtable.primary.dict.delete(ids)
        } catch {
            try? await airtable.primary.dict.delete(ids)
            throw error
        }
    }

    // MARK: - Attachments field

    @Test("Attachments field empty / notEmpty filter")
    func attachmentsFieldFilter() async throws {
        let suite = TestSetup.primaryKey(for: "Filter", "Attach")
        var f1 = primaryFields("\(suite) A")
        f1.set(
            PrimaryFields.attachmentId,
            .array([
                .object([
                    "url": .string(
                        "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"
                    )
                ])
            ])
        )
        let f2 = primaryFields("\(suite) B")
        let created = try await airtable.primary.dict.create([f1, f2])
        let ids = created.map { $0.id }
        let scope = scopeTo(ids)
        let f = PrimaryModel.f

        do {
            let notEmpty = try await airtable.primary.dict.get(
                AirtableQuery().formula(Formulas.and(scope, f.attachment.notEmpty()))
            )
            #expect(notEmpty.count >= 1)

            let empty = try await airtable.primary.dict.get(
                AirtableQuery().formula(Formulas.and(scope, f.attachment.empty()))
            )
            #expect(empty.count >= 1)

            try await airtable.primary.dict.delete(ids)
        } catch {
            try? await airtable.primary.dict.delete(ids)
            throw error
        }
    }

    // MARK: - Date field predicates

    @Test("Date field predicate coverage")
    func dateFieldPredicates() async throws {
        let suite = TestSetup.primaryKey(for: "Filter", "Date")
        var f1 = primaryFields("\(suite) A")
        f1.setString(PrimaryFields.dateId, "2024-01-15")
        var f2 = primaryFields("\(suite) B")
        f2.setString(PrimaryFields.dateId, "2024-06-15")
        let f3 = primaryFields("\(suite) C")  // no date
        let created = try await airtable.primary.dict.create([f1, f2, f3])
        let ids = created.map { $0.id }
        let scope = scopeTo(ids)
        let f = PrimaryModel.f

        func count(_ formula: String) async throws -> Int {
            try await airtable.primary.dict.get(
                AirtableQuery().formula(Formulas.and(scope, formula))
            ).count
        }

        do {
            #expect(try await count(f.date.on("2024-01-15")) == 1)
            #expect(try await count(f.date.notOn("2024-01-15")) >= 1)
            #expect(try await count(f.date.onOrAfter("2024-06-15")) >= 1)
            #expect(try await count(f.date.onOrBefore("2024-01-15")) >= 1)
            #expect(try await count(f.date.after("2024-01-15")) >= 1)
            #expect(try await count(f.date.before("2024-06-15")) >= 1)
            #expect(try await count(f.date.between("2024-01-15", "2024-06-15")) == 2)
            #expect(try await count(f.date.between("2024-01-01", "2024-12-31", inclusive: false)) == 2)
            #expect(try await count(f.date.notEmpty()) >= 1)
            #expect(try await count(f.date.empty()) >= 1)

            try await airtable.primary.dict.delete(ids)
        } catch {
            try? await airtable.primary.dict.delete(ids)
            throw error
        }
    }

    // MARK: - Date field chained (time-ago)

    @Test("Date field chained time-ago comparisons")
    func dateFieldChained() async throws {
        let suite = TestSetup.primaryKey(for: "Filter", "DateChain")
        var f1 = primaryFields("\(suite) A")
        f1.setString(PrimaryFields.dateId, "2024-01-15")
        var f2 = primaryFields("\(suite) B")
        f2.setString(PrimaryFields.dateId, "2024-06-15")
        let created = try await airtable.primary.dict.create([f1, f2])
        let ids = created.map { $0.id }
        let scope = scopeTo(ids)
        let f = PrimaryModel.f

        do {
            // before().daysAgo(1) — both dates are in the past.
            let past = try await airtable.primary.dict.get(
                AirtableQuery().formula(Formulas.and(scope, f.date.before().daysAgo(1)))
            )
            #expect(past.count >= 1)

            // after().yearsAgo(100) — both dates are within the last 100 years.
            let recent = try await airtable.primary.dict.get(
                AirtableQuery().formula(Formulas.and(scope, f.date.after().yearsAgo(100)))
            )
            #expect(recent.count >= 1)

            try await airtable.primary.dict.delete(ids)
        } catch {
            try? await airtable.primary.dict.delete(ids)
            throw error
        }
    }

    // MARK: - Complex formulas (AND / OR / NOT / XOR)

    @Test("Complex combinator coverage")
    func complexFormulas() async throws {
        let suite = TestSetup.primaryKey(for: "Filter", "Complex")
        var f1 = primaryFields("\(suite) A")
        f1.setInt(PrimaryFields.numberIntId, 10)
        f1.setBool(PrimaryFields.checkboxId, true)
        var f2 = primaryFields("\(suite) B")
        f2.setInt(PrimaryFields.numberIntId, 20)
        f2.setBool(PrimaryFields.checkboxId, false)
        var f3 = primaryFields("\(suite) C")
        f3.setInt(PrimaryFields.numberIntId, 30)
        f3.setBool(PrimaryFields.checkboxId, true)
        let created = try await airtable.primary.dict.create([f1, f2, f3])
        let ids = created.map { $0.id }
        let scope = scopeTo(ids)
        let f = PrimaryModel.f

        func count(_ formula: String) async throws -> Int {
            try await airtable.primary.dict.get(
                AirtableQuery().formula(Formulas.and(scope, formula))
            ).count
        }

        do {
            // AND(number > 10, checkbox) -> only C
            #expect(try await count(Formulas.and(f.numberInt.greaterThan(10), f.checkbox.isTrue())) == 1)
            // AND(contains, gte) -> B and C
            #expect(
                try await count(
                    Formulas.and(
                        f.primaryKey.contains(suite, caseSensitive: false, trim: true),
                        f.numberInt.greaterThanOrEquals(20)
                    )
                ) == 2)
            // OR(number = 10, number = 30) -> A and C
            #expect(try await count(Formulas.or(f.numberInt.equals(10), f.numberInt.equals(30))) == 2)
            // OR(checkbox, number = 20) -> all three
            #expect(try await count(Formulas.or(f.checkbox.isTrue(), f.numberInt.equals(20))) == 3)
            // XOR(checkbox, number >= 30) -> only A
            #expect(
                try await count(
                    Formulas.xor(f.checkbox.isTrue(), f.numberInt.greaterThanOrEquals(30))
                ) == 1)
            // NOT(primaryKey = "A") -> B and C
            #expect(try await count(Formulas.not(f.primaryKey.equals("\(suite) A"))) == 2)
            // AND(OR(number=10, number=30), checkbox) -> A and C
            #expect(
                try await count(
                    Formulas.and(
                        Formulas.or(f.numberInt.equals(10), f.numberInt.equals(30)),
                        f.checkbox.isTrue()
                    )
                ) == 2)
            // OR(AND(number > 10, checkbox), primaryKey = "A") -> A and C
            #expect(
                try await count(
                    Formulas.or(
                        Formulas.and(f.numberInt.greaterThan(10), f.checkbox.isTrue()),
                        f.primaryKey.equals("\(suite) A")
                    )
                ) == 2)
            // NOT(AND(checkbox, number > 20)) -> A and B
            #expect(
                try await count(
                    Formulas.not(Formulas.and(f.checkbox.isTrue(), f.numberInt.greaterThan(20)))
                ) == 2)
            // AND(NOT(checkbox), gte(20)) -> only B
            #expect(
                try await count(
                    Formulas.and(
                        Formulas.not(f.checkbox.isTrue()), f.numberInt.greaterThanOrEquals(20))
                ) == 1)

            try await airtable.primary.dict.delete(ids)
        } catch {
            try? await airtable.primary.dict.delete(ids)
            throw error
        }
    }

    // MARK: - Max records

    @Test("maxRecords caps the result set")
    func maxRecords() async throws {
        let suite = TestSetup.primaryKey(for: "Filter", "MaxRec")
        let created = try await airtable.primary.dict.create(
            (1...5).map { primaryFields("\(suite) \($0)") }
        )
        let ids = created.map { $0.id }

        do {
            let results = try await airtable.primary.dict.get(
                AirtableQuery().formula(scopeTo(ids)).maxRecords(3)
            )
            #expect(results.count == 3)

            try await airtable.primary.dict.delete(ids)
        } catch {
            try? await airtable.primary.dict.delete(ids)
            throw error
        }
    }

    // MARK: - Sort

    @Test("Sort ascending and descending by number field")
    func sortRecords() async throws {
        let suite = TestSetup.primaryKey(for: "Filter", "Sort")
        var f1 = primaryFields("\(suite) C")
        f1.setInt(PrimaryFields.numberIntId, 30)
        var f2 = primaryFields("\(suite) A")
        f2.setInt(PrimaryFields.numberIntId, 10)
        var f3 = primaryFields("\(suite) B")
        f3.setInt(PrimaryFields.numberIntId, 20)
        let created = try await airtable.primary.dict.create([f1, f2, f3])
        let ids = created.map { $0.id }
        let scope = scopeTo(ids)

        do {
            let asc = try await airtable.primary.dict.get(
                AirtableQuery().formula(scope).sort(field: PrimaryFields.numberIntId)
            )
            #expect(asc.compactMap { $0.fields.getInt(PrimaryFields.numberIntId) } == [10, 20, 30])

            let desc = try await airtable.primary.dict.get(
                AirtableQuery().formula(scope)
                    .sort(field: PrimaryFields.numberIntId, direction: .desc)
            )
            #expect(desc.compactMap { $0.fields.getInt(PrimaryFields.numberIntId) } == [30, 20, 10])

            try await airtable.primary.dict.delete(ids)
        } catch {
            try? await airtable.primary.dict.delete(ids)
            throw error
        }
    }

    // MARK: - Field selection

    @Test("fields() restricts returned fields")
    func fieldSelection() async throws {
        let suite = TestSetup.primaryKey(for: "Filter", "FieldSel")
        var fields = primaryFields(suite)
        fields.setString(PrimaryFields.singleLineTextId, "Hello")
        fields.setInt(PrimaryFields.numberIntId, 42)
        let created = try await airtable.primary.dict.create(fields)

        do {
            let results = try await airtable.primary.dict.get(
                AirtableQuery()
                    .formula("RECORD_ID()='\(created.id)'")
                    .fields([PrimaryFields.primaryKeyId])
            )
            #expect(results.count == 1)
            let f = results[0].fields
            #expect(f.get(PrimaryFields.primaryKeyId) != nil)
            #expect(f.get(PrimaryFields.singleLineTextId) == nil)
            #expect(f.get(PrimaryFields.numberIntId) == nil)

            try await airtable.primary.dict.delete(created.id)
        } catch {
            try? await airtable.primary.dict.delete(created.id)
            throw error
        }
    }

    // MARK: - Lookup field (ARRAYJOIN regression)

    @Test("Lookup field string filters work via ARRAYJOIN")
    func lookupFieldFilter() async throws {
        let suite = TestSetup.primaryKey(for: "Filter", "Lookup")

        // Three Secondary records provide the looked-up text. Primary.lookup
        // pulls from Secondary.value through Primary.linkSingle.
        let secA = try await airtable.secondary.create(
            SecondaryModel(name: "\(suite) Sec A", value: "Groundwork BioAg"))
        let secB = try await airtable.secondary.create(
            SecondaryModel(name: "\(suite) Sec B", value: "Groundwork Lab"))
        let secC = try await airtable.secondary.create(
            SecondaryModel(name: "\(suite) Sec C", value: "Other Vendor"))
        guard let secAId = secA.id, let secBId = secB.id, let secCId = secC.id else {
            Issue.record("Missing secondary ids")
            return
        }
        let secIds = [secAId, secBId, secCId]

        let primA = try await airtable.primary.create(
            PrimaryModel(linkSingle: [secAId], primaryKey: "\(suite) A"))
        let primB = try await airtable.primary.create(
            PrimaryModel(linkSingle: [secBId], primaryKey: "\(suite) B"))
        let primC = try await airtable.primary.create(
            PrimaryModel(linkSingle: [secCId], primaryKey: "\(suite) C"))
        guard let primAId = primA.id, let primBId = primB.id, let primCId = primC.id else {
            Issue.record("Missing primary ids")
            return
        }
        let primIds = [primAId, primBId, primCId]
        let scope = scopeTo(primIds)
        let f = PrimaryModel.f

        func matchNames(_ formula: String) async throws -> [String] {
            let records = try await airtable.primary.dict.get(
                AirtableQuery().formula(Formulas.and(scope, formula))
            )
            return names(of: records)
        }

        do {
            // contains — the regression test for the silently-empty bug.
            #expect(
                try await matchNames(f.lookup.contains("groundwork", caseSensitive: false, trim: true))
                    == ["\(suite) A", "\(suite) B"])
            // contains, no match
            #expect(
                try await matchNames(
                    f.lookup.contains("nonexistent-substring-xyz", caseSensitive: false, trim: true)
                ).isEmpty)
            // startsWith
            #expect(
                try await matchNames(f.lookup.startsWith("Ground", caseSensitive: false, trim: true))
                    == ["\(suite) A", "\(suite) B"])
            // endsWith
            #expect(
                try await matchNames(f.lookup.endsWith("BioAg", caseSensitive: false, trim: true))
                    == ["\(suite) A"])
            // equals — Airtable already coerces arrays under `=`.
            #expect(try await matchNames(f.lookup.equals("Groundwork BioAg")) == ["\(suite) A"])
            // notContains
            #expect(
                try await matchNames(f.lookup.notContains("Groundwork", caseSensitive: false, trim: true))
                    == ["\(suite) C"])

            try await airtable.primary.delete(primIds)
            try await airtable.secondary.delete(secIds)
        } catch {
            try? await airtable.primary.delete(primIds)
            try? await airtable.secondary.delete(secIds)
            throw error
        }
    }

    // MARK: - Date field vs date field

    @Test("Date field-to-field comparisons")
    func dateFieldVsField() async throws {
        let suite = TestSetup.primaryKey(for: "Filter", "VsField")
        let f = FormulasModel.f

        func mk(_ name: String, _ first: String, _ second: String, _ third: String) -> FormulasModel {
            FormulasModel(
                firstDate: AirtableDateParser.parse(first)!,
                primaryKey: "\(suite) \(name)",
                secondDate: AirtableDateParser.parse(second)!,
                thirdDate: AirtableDateParser.parse(third)!
            )
        }

        let created = try await airtable.formulas.create([
            mk("Equal", "2024-06-15", "2024-06-15T00:00:00.000Z", "2024-06-15T00:00:00.000Z"),
            mk("FirstBefore", "2024-01-15", "2024-06-15T00:00:00.000Z", "2024-12-15T00:00:00.000Z"),
            mk("FirstAfter", "2024-12-15", "2024-06-15T00:00:00.000Z", "2024-01-15T00:00:00.000Z"),
            mk("Between", "2024-06-15", "2024-01-15T00:00:00.000Z", "2024-12-15T00:00:00.000Z"),
        ])
        let ids = created.compactMap { $0.id }
        guard ids.count == 4 else {
            Issue.record("Missing formulas ids")
            return
        }
        let scope = scopeTo(ids)

        func matchNames(_ formula: String) async throws -> Set<String> {
            let models = try await airtable.formulas.get(
                AirtableQuery().formula(Formulas.and(scope, formula))
            )
            return Set(models.compactMap { $0.primaryKey?.replacingOccurrences(of: "\(suite) ", with: "") })
        }

        do {
            #expect(try await matchNames(f.firstDate.on(f.secondDate)) == ["Equal"])
            #expect(
                try await matchNames(f.firstDate.notOn(f.secondDate))
                    == ["FirstBefore", "FirstAfter", "Between"])
            #expect(try await matchNames(f.firstDate.before(f.secondDate)) == ["FirstBefore"])
            #expect(
                try await matchNames(f.firstDate.after(f.secondDate)) == ["FirstAfter", "Between"])
            #expect(
                try await matchNames(f.firstDate.onOrBefore(f.secondDate))
                    == ["Equal", "FirstBefore"])
            #expect(
                try await matchNames(f.firstDate.onOrAfter(f.secondDate))
                    == ["Equal", "FirstAfter", "Between"])
            #expect(
                try await matchNames(f.firstDate.between(f.secondDate, f.thirdDate))
                    == ["Equal", "Between"])
            #expect(
                try await matchNames(f.firstDate.between(f.secondDate, f.thirdDate, inclusive: false))
                    == ["Between"])
            #expect(
                try await matchNames(f.firstDate.between("2024-01-01", f.secondDate))
                    == ["Equal", "FirstBefore"])

            try await airtable.formulas.delete(ids)
        } catch {
            try? await airtable.formulas.delete(ids)
            throw error
        }
    }
}
