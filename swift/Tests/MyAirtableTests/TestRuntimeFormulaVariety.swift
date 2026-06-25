// TC4 — Runtime-formula input variety. The base TestRuntimeFormulas suite only evaluates the
// kitchen-sink formulas with ONE fully-populated input set, so the IF(OR(...=BLANK())) short-circuit
// and varied inputs are never exercised. Each case here creates a Formulas record with a specific
// input set, fetches the API-computed value, and asserts the transpiled evaluate*() reproduces it.
//
// Scope notes (kept deliberately portable across all targets):
//  • Numbers are positive with LOG/SQRT arguments chosen to be exact (powers of 10 / perfect
//    squares). The Math formula calls LOG()/SQRT()/EXP() — transcendental results differ by a ULP
//    between platforms' math libs and Airtable (V8), so irrational results (e.g. LOG(5)) are NOT
//    bit-identical and would make an exact string compare flaky. Negatives/zero are excluded too:
//    LOG(negative) and MOD(_,0) error inside this formula.
//  • Text covers empty-ish edges (whitespace), unicode, and reserved punctuation (exercising the
//    fixed ENCODE_URL_COMPONENT). An all-blank text input is excluded: Airtable returns blank for
//    the whole formula (REPLACE past end-of-string errors), while the transpiler is lenient.
//
// Parity with csharp/tests/TestRuntimeFormulaVariety.cs.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Runtime formula variety", .serialized)
struct TestRuntimeFormulaVariety {
    private let airtable: Airtable

    init() {
        self.airtable = TestSetup.makeAirtable()
    }

    private static func base(_ label: String) -> FormulasModel {
        FormulasModel(
            firstDate: AirtableDateParser.parse("2024-01-01T00:00:00.000Z")!,
            primaryKey: "Swift Variety " + label,
            secondDate: AirtableDateParser.parse("2024-02-01T00:00:00.000Z")!,
            thirdDate: AirtableDateParser.parse("2024-03-01T00:00:00.000Z")!
        )
    }

    // First Number: power of 10 (LOG exact). Second Number: perfect square (SQRT exact).
    struct NumberCase: CustomStringConvertible {
        let label: String
        let a: Double
        let b: Double
        let c: Double
        var description: String { label }
    }

    static let numberCases: [NumberCase] = [
        NumberCase(label: "hundreds", a: 100, b: 16, c: 8),
        NumberCase(label: "ones", a: 1, b: 4, c: 2),
        NumberCase(label: "tens", a: 10, b: 25, c: 3),
    ]

    @Test("Math formula matches API for varied numbers", arguments: numberCases)
    func mathFormulaMatchesApiForVariedNumbers(_ tc: NumberCase) async throws {
        let model = Self.base("Math " + tc.label)
        model.firstNumber = tc.a
        model.secondNumber = tc.b
        model.thirdNumber = tc.c
        model.firstText = "x"
        model.secondText = "y"
        model.thirdText = "z"
        let created = try await airtable.formulas.create(model)
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }
        do {
            let fetched = try await airtable.formulas.get(recordId)
            let runtime = AirtableRuntime.S(fetched.evaluateMathFormula())
            #expect(
                fetched.mathFormula?.value == runtime,
                "\(tc.label): api='\(fetched.mathFormula?.value ?? "nil")' runtime='\(runtime)'"
            )
        } catch {
            try? await airtable.formulas.delete(recordId)
            throw error
        }
        try? await airtable.formulas.delete(recordId)
    }

    @Test("Math formula returns blank when numbers missing")
    func mathFormulaBlankBranchWhenNumbersMissing() async throws {
        // First/Second Number left nil -> OR(BLANK, BLANK) is true -> the formula returns BLANK().
        // This is the IF-true short-circuit that the base suite never reaches.
        let model = Self.base("Blank")
        model.firstText = "x"
        model.secondText = "y"
        model.thirdText = "z"
        let created = try await airtable.formulas.create(model)
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }
        do {
            let fetched = try await airtable.formulas.get(recordId)
            let apiVal = fetched.mathFormula?.value ?? ""
            let runtime = AirtableRuntime.S(fetched.evaluateMathFormula())
            #expect(apiVal.isEmpty, "API expected blank, got '\(apiVal)'")
            #expect(runtime.isEmpty, "runtime expected blank, got '\(runtime)'")
        } catch {
            try? await airtable.formulas.delete(recordId)
            throw error
        }
        try? await airtable.formulas.delete(recordId)
    }

    struct TextCase: CustomStringConvertible {
        let label: String
        let a: String
        let b: String
        let c: String
        var description: String { label }
    }

    static let textCases: [TextCase] = [
        TextCase(label: "unicode", a: "café", b: "naïve", c: "日本語🎉"),
        TextCase(label: "whitespace", a: "  he llo  ", b: "a b", c: "c"),
        TextCase(label: "punct", a: "a.e-i+o", b: "x/y", c: "z"),  // exercises fixed ENCODE_URL_COMPONENT
    ]

    @Test("Text formula matches API for varied text", arguments: textCases)
    func textFormulaMatchesApiForVariedText(_ tc: TextCase) async throws {
        let model = Self.base("Text " + tc.label)
        model.firstNumber = 10
        model.secondNumber = 20
        model.thirdNumber = 30
        model.firstText = tc.a
        model.secondText = tc.b
        model.thirdText = tc.c
        let created = try await airtable.formulas.create(model)
        guard let recordId = created.id else {
            Issue.record("Missing id on created model")
            return
        }
        do {
            let fetched = try await airtable.formulas.get(recordId)
            let runtime = AirtableRuntime.S(fetched.evaluateTextFormula())
            #expect(
                fetched.textFormula?.value == runtime,
                "\(tc.label): api='\(fetched.textFormula?.value ?? "nil")' runtime='\(runtime)'"
            )
        } catch {
            try? await airtable.formulas.delete(recordId)
            throw error
        }
        try? await airtable.formulas.delete(recordId)
    }
}
