// Parity with rust/tests/test_special_error_deser.rs — offline decoding of
// Airtable's `{"specialValue": ...}` / `{"error": ...}` computed-field shapes
// through the generated module, plus a generated-model decode test proving
// erroring formula fields no longer break `init(from:)`.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Special / error deserialization")
struct TestSpecialErrorDeser {
    private func decode<T: Decodable>(_ type: T.Type, _ json: String) throws -> T {
        try JSONDecoder().decode(T.self, from: Data(json.utf8))
    }

    @Test func specialNumberAcceptsObjectFormRejectsArrayForm() throws {
        let v = try decode(SpecialNumber.self, #"{"specialValue":"NaN"}"#)
        #expect(v.specialValue == "NaN")
        #expect(throws: (any Error).self) {
            _ = try decode(SpecialNumber.self, #"["NaN"]"#)
        }
    }

    @Test func errorValueAcceptsObjectFormRejectsArrayForm() throws {
        let v = try decode(ErrorValue.self, ##"{"error":"#ERROR!"}"##)
        #expect(v.error == "#ERROR!")
        #expect(throws: (any Error).self) {
            _ = try decode(ErrorValue.self, ##"["#ERROR!"]"##)
        }
    }

    @Test func maybeSpecialOrErrorStringParsesPlainStringRejectsLookupArray() throws {
        let v = try decode(MaybeSpecialOrError<String>.self, #""Stukenholtz""#)
        #expect(v.value == "Stukenholtz")
        // Pre-fix regression: ["Stukenholtz"] accidentally succeeded as Special.
        #expect(throws: (any Error).self) {
            _ = try decode(MaybeSpecialOrError<String>.self, #"["Stukenholtz"]"#)
        }
    }

    @Test func vecOrValueLookupShapes() throws {
        let single = try decode(VecOrValue<MaybeSpecialOrError<String>>.self, #""hello""#)
        #expect(single.single?.value == "hello")

        let multiple = try decode(
            VecOrValue<MaybeSpecialOrError<String>>.self, #"["Stukenholtz Laboratory Inc"]"#)
        #expect(multiple.values.compactMap(\.value) == ["Stukenholtz Laboratory Inc"])

        let withNulls = try decode(
            VecOrValue<MaybeSpecialOrError<String>>.self, #"["a", null, "b"]"#)
        #expect(withNulls.multiple?.count == 3)
        #expect(withNulls.multiple?[1] == nil)

        let special = try decode(
            VecOrValue<MaybeSpecialOrError<Double>>.self, #"{"specialValue":"NaN"}"#)
        #expect(special.single?.special?.specialValue == "NaN")

        let error = try decode(
            VecOrValue<MaybeSpecialOrError<Double>>.self, ##"{"error":"#ERROR!"}"##)
        #expect(error.single?.errorValue?.error == "#ERROR!")
    }

    // MARK: - Generated model decode with erroring computed fields

    @Test func modelDecodesErroringComputedFields() throws {
        // The exact failure mode that used to throw: Airtable returns error /
        // special objects for computed fields, while lookup returns an array.
        let json = """
            {
              "id": "recERRDESER123456",
              "createdTime": "2025-01-15T10:00:00.000Z",
              "fields": {
                "\(PrimaryFields.primaryKeyId)": "ErrDeser",
                "\(PrimaryFields.formulaSimpleId)": {"error": "#ERROR!"},
                "\(PrimaryFields.formulaIdId)": "recERRDESER123456",
                "\(PrimaryFields.lookupId)": ["Lab A", null],
                "\(PrimaryFields.rollupId)": {"specialValue": "NaN"}
              }
            }
            """
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .airtable
        let model = try decoder.decode(PrimaryModel.self, from: Data(json.utf8))

        #expect(model.primaryKey == "ErrDeser")
        #expect(model.formulaSimple?.isError == true)
        #expect(model.formulaSimple?.value == nil)
        #expect(model.formulaId?.value == "recERRDESER123456")
        #expect(model.lookup?.values.compactMap(\.value) == ["Lab A"])
        #expect(model.lookup?.multiple?.count == 2)
        #expect(model.rollup?.single?.special?.specialValue == "NaN")
    }
}
