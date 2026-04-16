// F4.8 — Offline (no-network) serialization tests for generated ORM models.
// Parity with the offline portion of rust/tests/test_serializing.rs.
//
// Exercises JSON decode / encode round-trip against synthetic Airtable
// record envelopes — no API calls. Network-bound serialization parity lands
// in F10.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Serialization (offline)")
struct TestSerializing {
    // Convenience wrapper matching Rust's `record_json` helper.
    private func recordJSON(fields: String, id: String = "recTEST123456789") -> Data {
        let payload = """
            {
              "id": "\(id)",
              "createdTime": "2025-01-15T10:00:00.000Z",
              "fields": \(fields)
            }
            """
        return Data(payload.utf8)
    }

    private func decoder() -> JSONDecoder {
        let d = JSONDecoder()
        d.dateDecodingStrategy = .iso8601
        return d
    }

    private func encoder() -> JSONEncoder {
        let e = JSONEncoder()
        e.dateEncodingStrategy = .iso8601
        return e
    }

    // MARK: - Deserialization (field IDs as keys)

    @Test("Decode primary fields keyed by field ID")
    func decodeByFieldId() throws {
        let json = recordJSON(
            fields: """
                {
                  "\(PrimaryFields.primaryKeyId)": "Hello World",
                  "\(PrimaryFields.singleLineTextId)": "Single line",
                  "\(PrimaryFields.emailId)": "test@example.com",
                  "\(PrimaryFields.checkboxId)": true,
                  "\(PrimaryFields.numberIntId)": 42,
                  "\(PrimaryFields.numberFloatId)": 3.14,
                  "\(PrimaryFields.durationId)": 3600
                }
                """
        )
        let model = try decoder().decode(PrimaryModel.self, from: json)
        #expect(model.id == "recTEST123456789")
        #expect(model.primaryKey == "Hello World")
        #expect(model.singleLineText == "Single line")
        #expect(model.email == "test@example.com")
        #expect(model.checkbox == true)
        #expect(model.numberInt == 42)
        #expect(model.numberFloat == 3.14)
        #expect(model.duration == 3600)
    }

    // MARK: - Missing fields

    @Test("Missing fields are nil")
    func missingFieldsAreNil() throws {
        let json = recordJSON(fields: "{}")
        let model = try decoder().decode(PrimaryModel.self, from: json)
        #expect(model.id == "recTEST123456789")
        #expect(model.primaryKey == nil)
        #expect(model.singleLineText == nil)
        #expect(model.numberInt == nil)
        #expect(model.checkbox == nil)
    }

    // MARK: - Entire 'fields' container missing

    @Test("Record with no 'fields' container decodes cleanly")
    func missingFieldsContainer() throws {
        let json = """
            {"id": "recEmpty", "createdTime": "2025-01-15T10:00:00.000Z"}
            """
        let model = try decoder().decode(PrimaryModel.self, from: Data(json.utf8))
        #expect(model.id == "recEmpty")
        #expect(model.primaryKey == nil)
    }

    // MARK: - Encode (model → JSON)

    @Test("Encode produces fields keyed by field ID")
    func encodeKeyedByFieldId() throws {
        let json = recordJSON(
            fields: """
                { "\(PrimaryFields.primaryKeyId)": "Hello" }
                """
        )
        let model = try decoder().decode(PrimaryModel.self, from: json)
        model.primaryKey = "Updated"

        let data = try encoder().encode(model)
        let text = String(data: data, encoding: .utf8) ?? ""
        // Fields container keyed by field ID.
        #expect(text.contains("\"\(PrimaryFields.primaryKeyId)\":\"Updated\""))
        // Field names should NOT appear as JSON keys.
        #expect(!text.contains("\"Primary Key\":"))
    }

    // MARK: - Round-trip

    @Test("Round-trip preserves all writable field values")
    func roundTripPreservesData() throws {
        let originalJSON = recordJSON(
            fields: """
                {
                  "\(PrimaryFields.primaryKeyId)": "Original",
                  "\(PrimaryFields.singleLineTextId)": "Line",
                  "\(PrimaryFields.numberIntId)": 100,
                  "\(PrimaryFields.checkboxId)": true
                }
                """
        )
        let original = try decoder().decode(PrimaryModel.self, from: originalJSON)
        let encoded = try encoder().encode(original)
        let roundTripped = try decoder().decode(PrimaryModel.self, from: encoded)

        #expect(roundTripped.primaryKey == original.primaryKey)
        #expect(roundTripped.singleLineText == original.singleLineText)
        #expect(roundTripped.numberInt == original.numberInt)
        #expect(roundTripped.checkbox == original.checkbox)
    }

    // MARK: - toRecord()

    @Test("toRecord emits field values keyed by field ID")
    func toRecordKeyedByFieldId() throws {
        let json = recordJSON(
            fields: """
                {
                  "\(PrimaryFields.primaryKeyId)": "Hello",
                  "\(PrimaryFields.numberIntId)": 42
                }
                """
        )
        let model = try decoder().decode(PrimaryModel.self, from: json)
        let record = model.toRecord()
        #expect(record[PrimaryFields.primaryKeyId] == .string("Hello"))
        #expect(record[PrimaryFields.numberIntId] == .int(42))
        // No field-name keys in toRecord output.
        #expect(record["Primary Key"] == nil)
    }

    // MARK: - Dirty tracking

    @Test("Fresh decode has no dirty fields")
    func freshDecodeHasNoDirty() throws {
        let json = recordJSON(
            fields: """
                { "\(PrimaryFields.primaryKeyId)": "Hello" }
                """
        )
        let model = try decoder().decode(PrimaryModel.self, from: json)
        #expect(model.dirtyFields().isEmpty)
    }

    @Test("Mutating a writable field marks only that field dirty")
    func mutatingMarksDirty() throws {
        let json = recordJSON(
            fields: """
                {
                  "\(PrimaryFields.primaryKeyId)": "Hello",
                  "\(PrimaryFields.numberIntId)": 42
                }
                """
        )
        let model = try decoder().decode(PrimaryModel.self, from: json)
        model.singleLineText = "New!"

        let dirty = model.dirtyFields()
        #expect(dirty[PrimaryFields.singleLineTextId] == .string("New!"))
        // Unchanged fields stay out of the diff.
        #expect(dirty[PrimaryFields.primaryKeyId] == nil)
        #expect(dirty[PrimaryFields.numberIntId] == nil)
    }

    @Test("takeSnapshot clears the dirty set")
    func takeSnapshotClearsDirty() throws {
        let json = recordJSON(fields: "{}")
        let model = try decoder().decode(PrimaryModel.self, from: json)
        model.primaryKey = "X"
        #expect(!model.dirtyFields().isEmpty)
        model.takeSnapshot()
        #expect(model.dirtyFields().isEmpty)
    }

    // MARK: - isNew

    @Test("Unsaved model reports isNew=true")
    func unsavedIsNew() throws {
        // Decode a record without an id — matches what an unsaved create payload
        // response might look like in pathological cases. We construct one here
        // by decoding envelope without id.
        let json = """
            {"createdTime": null, "fields": {}}
            """
        let model = try decoder().decode(PrimaryModel.self, from: Data(json.utf8))
        #expect(model.isNew == true)
    }

    @Test("Saved model reports isNew=false")
    func savedIsNotNew() throws {
        let json = recordJSON(fields: "{}")
        let model = try decoder().decode(PrimaryModel.self, from: json)
        #expect(model.isNew == false)
    }

    // MARK: - CreatePrimaryModel encoding

    @Test("CreatePrimaryModel encodes with field-ID keys")
    func createModelEncodesFieldIds() throws {
        // Init params are emitted in schema (alphabetical) order — numberInt,
        // primaryKey, singleLineText — so we respect that ordering here.
        let create = CreatePrimaryModel(
            numberInt: 42,
            primaryKey: "Hello",
            singleLineText: "Line"
        )
        let data = try encoder().encode(create)
        let text = String(data: data, encoding: .utf8) ?? ""
        #expect(text.contains("\"\(PrimaryFields.primaryKeyId)\":\"Hello\""))
        #expect(text.contains("\"\(PrimaryFields.numberIntId)\":42"))
        #expect(!text.contains("\"Primary Key\":"))
    }

    @Test("CreatePrimaryModel omits nil fields (sparse write)")
    func createModelOmitsNils() throws {
        let create = CreatePrimaryModel(primaryKey: "Only Me")
        let data = try encoder().encode(create)
        let text = String(data: data, encoding: .utf8) ?? ""
        #expect(text.contains("\"\(PrimaryFields.primaryKeyId)\":\"Only Me\""))
        // Non-set fields should not appear in the encoded payload.
        #expect(!text.contains(PrimaryFields.singleLineTextId))
        #expect(!text.contains(PrimaryFields.numberIntId))
    }
}
