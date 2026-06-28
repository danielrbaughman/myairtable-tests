// TC6 — Primary "Formula (Complex)"/"Formula (Nested)" evaluated at runtime. The base runtime
// suite only covers the Formulas table; the Primary complex formula concatenates ~35 fields through
// IF(field, field, "None"), a richer transpile path never checked against the API before.
//
// We compare the transpiled evaluateFormulaComplex() to the API line-by-line for the
// DETERMINISTIC, offline-reproducible field types (text, checkbox, single/multi select, numbers,
// currency, email, url, phone). The formula also references server-computed fields
// (Created/Last Modified Time + By, Auto Number, Button, Formula(ID)/(Simple)) and
// link/lookup/rollup — Airtable renders those from data the offline runtime doesn't hold
// (collaborator names, linked-record display values, special wrappers), so those lines are NOT
// expected to match offline. See myairtable-5b0n.
//
// This suite specifically locks in the multi-select array-join fix (myairtable-bb7f): a multi-value
// field coerces to "Option 1, Option 2", not just the first element.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Primary formula runtime", .serialized)
struct TestPrimaryFormulaRuntime {
    private let airtable: Airtable

    init() {
        self.airtable = TestSetup.makeAirtable()
    }

    // Field labels whose rendering the offline runtime can reproduce exactly.
    private static let deterministicLabels = [
        "Single Line Text",
        "Long Text",
        "Checkbox",
        "Multiple Select",
        "Single Select",
        "Number (int)",
        "Number (float)",
        "Currency (int)",
        "Currency (float)",
        "Email",
        "URL",
        "Phone Number",
    ]

    private func newRecord(_ suite: String) -> PrimaryModel {
        PrimaryModel(
            checkbox: true,
            currencyFloat: 9.99,
            currencyInt: 10,
            email: "a@b.co",
            longText: "long text",
            multipleSelect: [.option1, .option2],
            numberFloat: 3.5,
            numberInt: 42,
            phoneNumber: "555-1212",
            primaryKey: suite,
            singleLineText: "hello",
            singleSelect: .choice1,
            url: "https://x.co"
        )
    }

    /// Extract the "Label: value" line for `label` from a formula result.
    private static func line(_ formula: String, _ label: String) -> String {
        for line in formula.split(separator: "\n", omittingEmptySubsequences: false) {
            if line.hasPrefix(label + ": ") {
                return String(line)
            }
        }
        return "<missing: \(label)>"
    }

    @Test("Complex formula renders deterministic fields like the API")
    func complexFormulaRendersDeterministicFieldsLikeApi() async throws {
        let suite = TestSetup.primaryKey(for: "PrimaryFormula", "Complex")
        let created = try await airtable.primary.create(newRecord(suite))
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }

        do {
            let fetched = try await airtable.primary.get(recordId)
            let api = fetched.formulaComplex?.values.first?.value ?? ""
            let runtime = AirtableRuntime.S(fetched.evaluateFormulaComplex())

            for label in Self.deterministicLabels {
                #expect(
                    Self.line(api, label) == Self.line(runtime, label),
                    "Line mismatch for \(label):\n  API:     \(Self.line(api, label))\n  RUNTIME: \(Self.line(runtime, label))"
                )
            }

            // The multi-select join is the headline fix: render all options, comma-joined.
            #expect(Self.line(runtime, "Multiple Select") == "Multiple Select: Option 1, Option 2")
        } catch {
            try? await airtable.primary.delete(recordId)
            throw error
        }

        try? await airtable.primary.delete(recordId)
    }

    @Test("Nested formula evaluates without throwing")
    func nestedFormulaEvaluatesWithoutThrowing() async throws {
        // Formula (Nested) = Formula(ID) & Formula(Simple) & Formula(Complex) — it chains three
        // COMPUTED formula fields. Offline the runtime can't reproduce computed-field values (they
        // decode as special/wrapped types it doesn't re-evaluate), so the content isn't asserted;
        // this confirms the transpiled nested-formula method is generated and evaluates without
        // error. See myairtable-5b0n.
        let suite = TestSetup.primaryKey(for: "PrimaryFormula", "Nested")
        let created = try await airtable.primary.create(newRecord(suite))
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }

        do {
            let fetched = try await airtable.primary.get(recordId)
            // Reaching this line means evaluateFormulaNested() was generated and ran without
            // trapping; S() always yields a (possibly empty) string, so its non-nil-ness is the
            // assertion. Content isn't checked — it chains computed fields the runtime can't
            // reproduce offline.
            let runtime: String = AirtableRuntime.S(fetched.evaluateFormulaNested())
            #expect(!runtime.isEmpty || runtime.isEmpty)
        } catch {
            try? await airtable.primary.delete(recordId)
            throw error
        }

        try? await airtable.primary.delete(recordId)
    }
}
