package myairtable.tests

import myairtable.AirtableJson
import myairtable.ErrorValue
import myairtable.MaybeSpecialOrError
import myairtable.PrimaryFields
import myairtable.PrimaryModel
import myairtable.SpecialNumber
import myairtable.VecOrValue
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * K5.7 — Parity with rust test_special_error_deser / Swift
 * TestSpecialErrorDeser: offline decoding of Airtable's `{"specialValue": ...}`
 * / `{"error": ...}` computed-field shapes through the generated module, plus
 * a generated-model decode test proving erroring formula fields don't break
 * deserialization.
 */
class TestSpecialErrorDeser {
    private val json = AirtableJson.instance

    private inline fun <reified T> decode(raw: String): T = json.decodeFromString<T>(raw)

    @Test
    fun specialNumberAcceptsObjectFormRejectsArrayForm() {
        assertEquals("NaN", decode<SpecialNumber>("""{"specialValue":"NaN"}""").specialValue)
        assertFailsWith<Exception> { decode<SpecialNumber>("""["NaN"]""") }
    }

    @Test
    fun errorValueAcceptsObjectFormRejectsArrayForm() {
        assertEquals("#ERROR!", decode<ErrorValue>("""{"error":"#ERROR!"}""").error)
        assertFailsWith<Exception> { decode<ErrorValue>("""["#ERROR!"]""") }
    }

    @Test
    fun maybeSpecialOrErrorStringParsesPlainStringRejectsLookupArray() {
        assertEquals("Stukenholtz", decode<MaybeSpecialOrError<String>>("\"Stukenholtz\"").valueOrNull)
        // Pre-fix regression: ["Stukenholtz"] accidentally succeeded as Special.
        assertFailsWith<Exception> { decode<MaybeSpecialOrError<String>>("""["Stukenholtz"]""") }
    }

    @Test
    fun vecOrValueLookupShapes() {
        val single = decode<VecOrValue<MaybeSpecialOrError<String>>>("\"hello\"")
        assertEquals("hello", single.single?.valueOrNull)

        val multiple = decode<VecOrValue<MaybeSpecialOrError<String>>>("""["Stukenholtz Laboratory Inc"]""")
        assertEquals(listOf("Stukenholtz Laboratory Inc"), multiple.values.mapNotNull { it.valueOrNull })

        val withNulls = decode<VecOrValue<MaybeSpecialOrError<String>>>("""["a", null, "b"]""")
        assertEquals(3, withNulls.multiple?.size)
        assertNull(withNulls.multiple?.get(1))

        val special = decode<VecOrValue<MaybeSpecialOrError<Double>>>("""{"specialValue":"NaN"}""")
        assertEquals("NaN", special.single?.specialOrNull?.specialValue)

        val error = decode<VecOrValue<MaybeSpecialOrError<Double>>>("""{"error":"#ERROR!"}""")
        assertEquals("#ERROR!", error.single?.errorOrNull?.error)
    }

    // region Generated model decode with erroring computed fields

    @Test
    fun modelDecodesErroringComputedFields() {
        // The exact failure mode that used to throw in other targets: Airtable
        // returns error/special objects for computed fields, while lookup
        // returns an array with nulls.
        val fieldsJson =
            """
            {
              "${PrimaryFields.primaryKeyId}": "ErrDeser",
              "${PrimaryFields.formulaSimpleId}": {"error": "#ERROR!"},
              "${PrimaryFields.formulaIdId}": "recERRDESER123456",
              "${PrimaryFields.lookupId}": ["Lab A", null],
              "${PrimaryFields.rollupId}": {"specialValue": "NaN"}
            }
            """.trimIndent()
        val model = json.decodeFromString(PrimaryModel.serializer(), fieldsJson)

        assertEquals("ErrDeser", model.primaryKey)
        assertTrue(model.formulaSimple?.isError == true)
        assertNull(model.formulaSimple?.valueOrNull)
        assertEquals("recERRDESER123456", model.formulaId?.valueOrNull)
        assertEquals(listOf("Lab A"), model.lookup?.values?.mapNotNull { it.valueOrNull })
        assertEquals(2, model.lookup?.multiple?.size)
        assertEquals(
            "NaN",
            model.rollup
                ?.single
                ?.specialOrNull
                ?.specialValue,
        )
    }

    // endregion
}
