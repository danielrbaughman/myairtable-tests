// F8.13 — Runtime formula parity: create a record with all input fields,
// fetch it back with API-computed formula values, and verify the transpiled
// evaluate*() methods reproduce them. Parity with
// rust/tests/test_runtime_formulas.rs `runtime_formulas_match_api`.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Runtime formulas", .serialized)
struct TestRuntimeFormulas {
    private let airtable: Airtable

    init() {
        self.airtable = TestSetup.makeAirtable()
    }

    @Test("Runtime formulas match API-computed values")
    func runtimeFormulasMatchApi() async throws {
        let new = FormulasModel(
            firstDate: AirtableDateParser.parse("2024-01-01T00:00:00.000Z")!,
            firstNumber: 10,
            firstText: "Hello",
            primaryKey: "SwiftRuntimeTest",
            secondDate: AirtableDateParser.parse("2024-02-01T00:00:00.000Z")!,
            secondNumber: 20,
            secondText: "World",
            thirdDate: AirtableDateParser.parse("2024-03-01T00:00:00.000Z")!,
            thirdNumber: 30,
            thirdText: "!"
        )
        let created = try await airtable.formulas.create(new)
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }

        do {
            // Re-fetch to get formula values computed by Airtable.
            let model = try await airtable.formulas.get(recordId)

            // --- Math formula: exact match ---
            let runtimeMath = AirtableRuntime.S(model.evaluateMathFormula())
            #expect(model.mathFormula?.value == runtimeMath, "Math formula mismatch")

            // --- Text formula: exact match ---
            let runtimeText = AirtableRuntime.S(model.evaluateTextFormula())
            #expect(model.textFormula?.value == runtimeText, "Text formula mismatch")

            // --- Date formula: partial match (TODAY/TONOW/FROMNOW are time-dependent) ---
            let apiDate = model.dateFormula?.value ?? ""
            let runtimeDate = AirtableRuntime.S(model.evaluateDateFormula())

            // Check all deterministic parts.
            #expect(runtimeDate.contains("YEAR: 2024"), "got: \(runtimeDate)")
            #expect(runtimeDate.contains("MONTH: 1"), "got: \(runtimeDate)")
            #expect(runtimeDate.contains("DAY: 1"), "got: \(runtimeDate)")
            #expect(runtimeDate.contains("HOUR: 0"), "got: \(runtimeDate)")
            #expect(runtimeDate.contains("MINUTE: 0"), "got: \(runtimeDate)")
            #expect(runtimeDate.contains("SECOND: 0"), "got: \(runtimeDate)")
            #expect(runtimeDate.contains("WORKDAY_DIFF: "), "got: \(runtimeDate)")
            #expect(runtimeDate.contains("DATEADD+2d: 2024-01-03"), "got: \(runtimeDate)")
            #expect(runtimeDate.contains("IS_BEFORE: Yes"), "got: \(runtimeDate)")
            #expect(runtimeDate.contains("IS_SAME day: No"), "got: \(runtimeDate)")
            #expect(runtimeDate.contains("IS_AFTER: Yes"), "got: \(runtimeDate)")
            #expect(runtimeDate.contains("DATESTR: "), "got: \(runtimeDate)")
            #expect(runtimeDate.contains("TIMESTR: "), "got: \(runtimeDate)")

            // Compare the deterministic prefix of API vs runtime (before TODAY: which varies).
            let apiPrefix = apiDate.components(separatedBy: ", TODAY:").first ?? ""
            let runtimePrefix = runtimeDate.components(separatedBy: ", TODAY:").first ?? ""
            #expect(apiPrefix == runtimePrefix, "Date formula mismatch (pre-TODAY portion)")
        } catch {
            try await airtable.formulas.delete(recordId)
            throw error
        }

        try await airtable.formulas.delete(recordId)
    }

    @Test("Transpiled evaluate*() methods execute on a fresh offline model")
    func evaluateRunsOffline() throws {
        // Offline smoke check: a model decoded from a synthetic envelope (with
        // dates) still evaluates every generated method without trapping.
        let json = """
            {
              "id": "recRUNTIMETEST",
              "createdTime": "2024-01-01T00:00:00Z",
              "fields": {
                "\(FormulasFields.primaryKeyId)": "RuntimeTest",
                "\(FormulasFields.firstNumberId)": 10,
                "\(FormulasFields.secondNumberId)": 20,
                "\(FormulasFields.thirdNumberId)": 30,
                "\(FormulasFields.firstTextId)": "Hello",
                "\(FormulasFields.secondTextId)": "World",
                "\(FormulasFields.thirdTextId)": "!",
                "\(FormulasFields.firstDateId)": "2024-01-01",
                "\(FormulasFields.secondDateId)": "2024-02-01",
                "\(FormulasFields.thirdDateId)": "2024-03-01"
              }
            }
            """
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .airtable
        let model = try decoder.decode(FormulasModel.self, from: Data(json.utf8))

        #expect(model.evaluateMathFormula() != .null)
        let text = AirtableRuntime.S(model.evaluateTextFormula())
        #expect(text.contains("Hello"))
        #expect(text.contains("World"))
        let date = AirtableRuntime.S(model.evaluateDateFormula())
        #expect(date.contains("YEAR: 2024"))
    }
}
