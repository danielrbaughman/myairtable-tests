package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertThrows;

import com.fasterxml.jackson.core.type.TypeReference;
import java.util.List;
import java.util.Objects;
import myairtable.AirtableJson;
import myairtable.ErrorValue;
import myairtable.MaybeSpecialOrError;
import myairtable.PrimaryFields;
import myairtable.PrimaryModel;
import myairtable.SpecialNumber;
import myairtable.VecOrValue;
import org.junit.jupiter.api.Test;

/**
 * J5.7 — Parity with rust test_special_error_deser / Kotlin TestSpecialErrorDeser / Swift
 * TestSpecialErrorDeser: offline decoding of Airtable's {@code {"specialValue": ...}} / {@code
 * {"error": ...}} computed-field shapes through the generated module, plus a generated-model decode
 * test proving erroring formula fields don't break deserialization.
 */
class TestSpecialErrorDeser {

  private static <T> T decode(String raw, TypeReference<T> type) throws Exception {
    return AirtableJson.MAPPER.readValue(raw, type);
  }

  @Test
  void specialNumberAcceptsObjectFormRejectsArrayForm() throws Exception {
    assertEquals(
        "NaN",
        decode("{\"specialValue\":\"NaN\"}", new TypeReference<SpecialNumber>() {}).specialValue());
    assertThrows(Exception.class, () -> decode("[\"NaN\"]", new TypeReference<SpecialNumber>() {}));
  }

  @Test
  void errorValueAcceptsObjectFormRejectsArrayForm() throws Exception {
    assertEquals(
        "#ERROR!", decode("{\"error\":\"#ERROR!\"}", new TypeReference<ErrorValue>() {}).error());
    assertThrows(
        Exception.class, () -> decode("[\"#ERROR!\"]", new TypeReference<ErrorValue>() {}));
  }

  @Test
  void maybeSpecialOrErrorStringParsesPlainStringRejectsLookupArray() throws Exception {
    MaybeSpecialOrError<String> plain =
        decode("\"Stukenholtz\"", new TypeReference<MaybeSpecialOrError<String>>() {});
    assertEquals("Stukenholtz", plain.value());
    // Pre-fix regression: ["Stukenholtz"] accidentally succeeded as Special.
    assertThrows(
        Exception.class,
        () -> decode("[\"Stukenholtz\"]", new TypeReference<MaybeSpecialOrError<String>>() {}));
  }

  @Test
  void vecOrValueLookupShapes() throws Exception {
    VecOrValue<MaybeSpecialOrError<String>> single =
        decode("\"hello\"", new TypeReference<VecOrValue<MaybeSpecialOrError<String>>>() {});
    assertInstanceOf(VecOrValue.Single.class, single);
    assertEquals(
        "hello", ((VecOrValue.Single<MaybeSpecialOrError<String>>) single).value().value());

    VecOrValue<MaybeSpecialOrError<String>> multiple =
        decode(
            "[\"Stukenholtz Laboratory Inc\"]",
            new TypeReference<VecOrValue<MaybeSpecialOrError<String>>>() {});
    assertEquals(
        List.of("Stukenholtz Laboratory Inc"),
        multiple.values().stream()
            .map(v -> v == null ? null : v.value())
            .filter(Objects::nonNull)
            .toList());

    VecOrValue<MaybeSpecialOrError<String>> withNulls =
        decode(
            "[\"a\", null, \"b\"]",
            new TypeReference<VecOrValue<MaybeSpecialOrError<String>>>() {});
    VecOrValue.Multiple<MaybeSpecialOrError<String>> multipleShape =
        assertInstanceOf(VecOrValue.Multiple.class, withNulls);
    assertEquals(3, multipleShape.values().size());
    assertNull(multipleShape.values().get(1));

    VecOrValue<MaybeSpecialOrError<Double>> special =
        decode(
            "{\"specialValue\":\"NaN\"}",
            new TypeReference<VecOrValue<MaybeSpecialOrError<Double>>>() {});
    MaybeSpecialOrError.Special<Double> specialValue =
        assertInstanceOf(
            MaybeSpecialOrError.Special.class,
            assertInstanceOf(VecOrValue.Single.class, special).value());
    assertEquals("NaN", specialValue.special().specialValue());

    VecOrValue<MaybeSpecialOrError<Double>> error =
        decode(
            "{\"error\":\"#ERROR!\"}",
            new TypeReference<VecOrValue<MaybeSpecialOrError<Double>>>() {});
    MaybeSpecialOrError.Error<Double> errorValue =
        assertInstanceOf(
            MaybeSpecialOrError.Error.class,
            assertInstanceOf(VecOrValue.Single.class, error).value());
    assertEquals("#ERROR!", errorValue.error().error());
  }

  // region Generated model decode with erroring computed fields

  @Test
  void modelDecodesErroringComputedFields() throws Exception {
    // The exact failure mode that used to throw in other targets: Airtable returns
    // error/special objects for computed fields, while lookup returns an array with nulls.
    String fieldsJson =
        "{\n"
            + "  \""
            + PrimaryFields.primaryKeyId
            + "\": \"ErrDeser\",\n"
            + "  \""
            + PrimaryFields.formulaSimpleId
            + "\": {\"error\": \"#ERROR!\"},\n"
            + "  \""
            + PrimaryFields.formulaIdId
            + "\": \"recERRDESER123456\",\n"
            + "  \""
            + PrimaryFields.lookupId
            + "\": [\"Lab A\", null],\n"
            + "  \""
            + PrimaryFields.rollupId
            + "\": {\"specialValue\": \"NaN\"}\n"
            + "}";
    PrimaryModel model = AirtableJson.MAPPER.readValue(fieldsJson, PrimaryModel.class);

    assertEquals("ErrDeser", model.getPrimaryKey());
    assertInstanceOf(MaybeSpecialOrError.Error.class, model.getFormulaSimple());
    assertNull(model.getFormulaSimple().value());
    assertEquals("recERRDESER123456", model.getFormulaId().value());
    assertEquals(
        List.of("Lab A"),
        model.getLookup().values().stream()
            .map(v -> v == null ? null : v.value())
            .filter(Objects::nonNull)
            .toList());
    VecOrValue.Multiple<MaybeSpecialOrError<String>> lookupShape =
        assertInstanceOf(VecOrValue.Multiple.class, model.getLookup());
    assertEquals(2, lookupShape.values().size());
    MaybeSpecialOrError.Special<String> rollupSpecial =
        assertInstanceOf(
            MaybeSpecialOrError.Special.class,
            assertInstanceOf(VecOrValue.Single.class, model.getRollup()).value());
    assertEquals("NaN", rollupSpecial.special().specialValue());
  }

  // endregion
}
