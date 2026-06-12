package myairtable.tests

import kotlinx.serialization.json.JsonNull
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.jsonPrimitive
import kotlinx.serialization.json.put
import myairtable.AirtableJson
import myairtable.PrimaryFields
import myairtable.PrimaryModel
import java.time.Instant
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * K4.6 — Offline (no-network) model serialization parity with Swift
 * TestSerializing / Rust test_serializing. Network-bound cases (createdTime
 * round-trip, linked records, server-side clears) live in K-F10.
 */
class TestSerializing {
    private val json = AirtableJson.instance

    private fun decodePrimary(builder: kotlinx.serialization.json.JsonObjectBuilder.() -> Unit): PrimaryModel =
        json.decodeFromJsonElement(PrimaryModel.serializer(), buildJsonObject(builder))

    @Test
    fun decodePrimaryFieldsKeyedByFieldId() {
        val model =
            decodePrimary {
                put(PrimaryFields.primaryKeyId, "PK Value")
                put(PrimaryFields.singleLineTextId, "hello")
                put(PrimaryFields.numberIntId, 42)
                put(PrimaryFields.checkboxId, true)
            }
        assertEquals("PK Value", model.primaryKey)
        assertEquals("hello", model.singleLineText)
        assertEquals(42.0, model.numberInt)
        assertEquals(true, model.checkbox)
    }

    @Test
    fun dateOnlyAndDatetimeFieldValuesBothDecode() {
        val model =
            decodePrimary {
                put(PrimaryFields.dateId, "2024-01-15")
                put(PrimaryFields.dateWithTimeId, "2024-01-15T10:30:00.000Z")
            }
        assertEquals(Instant.parse("2024-01-15T00:00:00Z"), model.date)
        assertEquals(Instant.parse("2024-01-15T10:30:00Z"), model.dateWithTime)
    }

    @Test
    fun missingFieldsAreNull() {
        val model = decodePrimary { put(PrimaryFields.primaryKeyId, "only pk") }
        assertEquals("only pk", model.primaryKey)
        assertNull(model.singleLineText)
        assertNull(model.numberInt)
        assertNull(model.date)
    }

    @Test
    fun emptyFieldsObjectDecodesCleanly() {
        val model = decodePrimary { }
        assertNull(model.primaryKey)
        assertTrue(model.isNew)
    }

    @Test
    fun encodeProducesFieldsKeyedByFieldId() {
        val model = PrimaryModel(primaryKey = "x", numberInt = 7.0)
        val record = model.toCreateFields()
        assertEquals(setOf(PrimaryFields.primaryKeyId, PrimaryFields.numberIntId), record.keys)
        assertEquals("x", record[PrimaryFields.primaryKeyId]?.jsonPrimitive?.content)
    }

    @Test
    fun roundTripPreservesAllWritableFieldValues() {
        val original =
            PrimaryModel(
                primaryKey = "rt",
                singleLineText = "text",
                checkbox = true,
                numberInt = 5.0,
                numberFloat = 2.5,
            )
        val encoded = json.encodeToString(PrimaryModel.serializer(), original)
        val decoded = json.decodeFromString(PrimaryModel.serializer(), encoded)
        assertEquals(original.primaryKey, decoded.primaryKey)
        assertEquals(original.singleLineText, decoded.singleLineText)
        assertEquals(original.checkbox, decoded.checkbox)
        assertEquals(original.numberInt, decoded.numberInt)
        assertEquals(original.numberFloat, decoded.numberFloat)
    }

    @Test
    fun toRecordEmitsFieldValuesKeyedByFieldId() {
        val model = PrimaryModel(primaryKey = "rec", email = "a@b.c")
        val record = model.toRecord()
        assertEquals("rec", record[PrimaryFields.primaryKeyId]?.jsonPrimitive?.content)
        assertEquals("a@b.c", record[PrimaryFields.emailId]?.jsonPrimitive?.content)
    }

    @Test
    fun freshDecodeHasNoDirtyFieldsAfterSnapshot() {
        val model = decodePrimary { put(PrimaryFields.primaryKeyId, "clean") }
        model.takeSnapshot()
        assertTrue(model.dirtyFields().isEmpty())
    }

    @Test
    fun mutatingAWritableFieldMarksOnlyThatFieldDirty() {
        val model =
            decodePrimary {
                put(PrimaryFields.primaryKeyId, "pk")
                put(PrimaryFields.singleLineTextId, "before")
            }
        model.takeSnapshot()
        model.singleLineText = "after"
        assertEquals(setOf(PrimaryFields.singleLineTextId), model.dirtyFields().keys)
    }

    @Test
    fun takeSnapshotClearsTheDirtySet() {
        val model = decodePrimary { put(PrimaryFields.primaryKeyId, "pk") }
        model.takeSnapshot()
        model.singleLineText = "mutated"
        assertFalse(model.dirtyFields().isEmpty())
        model.takeSnapshot()
        assertTrue(model.dirtyFields().isEmpty())
    }

    @Test
    fun clearedWritableFieldBecomesJsonNull() {
        val model =
            decodePrimary {
                put(PrimaryFields.primaryKeyId, "pk")
                put(PrimaryFields.numberIntId, 9.5)
            }
        model.takeSnapshot()
        model.numberInt = null
        assertEquals(JsonNull, model.dirtyFields()[PrimaryFields.numberIntId])
    }

    @Test
    fun unsavedModelReportsIsNewTrue() {
        val model = PrimaryModel(primaryKey = "new")
        assertTrue(model.isNew)
        assertNull(model.id)
    }

    @Test
    fun modelWithIdReportsIsNewFalse() {
        val model = PrimaryModel(primaryKey = "saved")
        model.id = "recXYZ"
        assertFalse(model.isNew)
    }

    @Test
    fun freshModelOmitsNullFieldsSparseWrite() {
        val model = PrimaryModel(primaryKey = "sparse")
        val record = model.toCreateFields()
        assertEquals(setOf(PrimaryFields.primaryKeyId), record.keys)
        // Plugin-encode also skips nulls (explicitNulls=false).
        val encoded = json.encodeToString(PrimaryModel.serializer(), model)
        assertEquals("""{"${PrimaryFields.primaryKeyId}":"sparse"}""", encoded)
    }

    @Test
    fun ratingIsTypeErasedJsonElement() {
        // `rating` has no Airtable metadata type mapping → JsonElement (UNKNOWN).
        val model = PrimaryModel(primaryKey = "r", rating = JsonPrimitive(3))
        assertEquals(JsonPrimitive(3), model.rating)
        assertEquals(JsonPrimitive(3), model.toCreateFields()[PrimaryFields.ratingId])
    }
}
