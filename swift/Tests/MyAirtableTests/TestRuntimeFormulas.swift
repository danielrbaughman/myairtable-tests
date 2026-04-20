// F8.13 — Integration / offline test: verify the transpiler-generated
// evaluate*() methods compile and execute without errors. Parity with
// rust/tests/test_runtime_formulas.rs's intent: exercise the full
// transpiled-formula + runtime stack end-to-end.
//
// Full-fidelity comparison against Airtable's server-computed values
// (the way the Rust test does it) is limited here by a known decoding
// issue: date-only Airtable fields come back as "YYYY-MM-DD", but
// JSONDecoder's .iso8601 strategy expects a full datetime. That's a
// generator-level date-format upgrade tracked separately. For this
// test we exercise the transpiled formulas offline using a synthetic
// JSON envelope that avoids date-decoding.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Runtime formulas")
struct TestRuntimeFormulas {
    // Synthetic Airtable record envelope sufficient to exercise math + text
    // formulas. Date fields are omitted to dodge the date-decode mismatch
    // noted above; date-heavy formulas return `.null` propagated through the
    // runtime, which is acceptable for the "doesn't crash" contract.
    private func makeModel() throws -> FormulasModel {
        // Build the fields object dynamically to use the generator's field IDs
        // as the source of truth — avoids ID drift if the schema changes.
        let fieldsBody = [
            "\"\(FormulasFields.primaryKeyId)\": \"RuntimeTest\"",
            "\"\(FormulasFields.firstNumberId)\": 10",
            "\"\(FormulasFields.secondNumberId)\": 20",
            "\"\(FormulasFields.thirdNumberId)\": 30",
            "\"\(FormulasFields.firstTextId)\": \"Hello\"",
            "\"\(FormulasFields.secondTextId)\": \"World\"",
            "\"\(FormulasFields.thirdTextId)\": \"!\"",
        ].joined(separator: ", ")

        let json = """
            {
              "id": "recRUNTIMETEST",
              "createdTime": "2024-01-01T00:00:00Z",
              "fields": { \(fieldsBody) }
            }
            """
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        return try decoder.decode(FormulasModel.self, from: Data(json.utf8))
    }

    @Test("Transpiled evaluate*() methods execute without crashing")
    func evaluateRunsWithoutCrashing() throws {
        let model = try makeModel()

        // Exercise every generated evaluate method. The point is to catch
        // transpiler bugs that would surface at first use (missing runtime
        // functions, wrong arg shapes, etc.). Return values aren't asserted
        // here — TestAirtableRuntime.swift already verifies runtime semantics.
        _ = model.evaluateMathFormula()
        _ = model.evaluateTextFormula()
        #expect(true)  // reached without throwing
    }

    @Test("Math formula produces a non-null result when numeric inputs are set")
    func mathFormulaIsNonNull() throws {
        let model = try makeModel()
        let result = model.evaluateMathFormula()
        // Should be numeric-ish (not .null).
        #expect(result != .null)
    }

    @Test("Text formula concatenates the three text inputs")
    func textFormulaContainsInputs() throws {
        let model = try makeModel()
        let asString = AirtableRuntime.S(model.evaluateTextFormula())
        // The text formula in the schema concatenates firstText + secondText + thirdText.
        #expect(asString.contains("Hello"))
        #expect(asString.contains("World"))
    }
}
