package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.IntNode;
import com.fasterxml.jackson.databind.node.NullNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import java.time.Instant;
import java.util.Map;
import java.util.Set;
import myairtable.AirtableJson;
import myairtable.PrimaryFields;
import myairtable.PrimaryModel;
import org.junit.jupiter.api.Test;

/**
 * J4.6 — Offline (no-network) model serialization parity with Kotlin TestSerializing / Swift
 * TestSerializing / Rust test_serializing. Network-bound cases live in {@link
 * TestSerializingIntegration} below.
 */
class TestSerializing {

  private static ObjectNode fields() {
    return AirtableJson.MAPPER.createObjectNode();
  }

  private static PrimaryModel decodePrimary(ObjectNode fields) throws Exception {
    return AirtableJson.MAPPER.treeToValue(fields, PrimaryModel.class);
  }

  @Test
  void decodePrimaryFieldsKeyedByFieldId() throws Exception {
    ObjectNode json =
        fields()
            .put(PrimaryFields.primaryKeyId, "PK Value")
            .put(PrimaryFields.singleLineTextId, "hello")
            .put(PrimaryFields.numberIntId, 42)
            .put(PrimaryFields.checkboxId, true);
    PrimaryModel model = decodePrimary(json);
    assertEquals("PK Value", model.getPrimaryKey());
    assertEquals("hello", model.getSingleLineText());
    assertEquals(42.0, model.getNumberInt());
    assertEquals(true, model.getCheckbox());
  }

  @Test
  void dateOnlyAndDatetimeFieldValuesBothDecode() throws Exception {
    ObjectNode json =
        fields()
            .put(PrimaryFields.dateId, "2024-01-15")
            .put(PrimaryFields.dateWithTimeId, "2024-01-15T10:30:00.000Z");
    PrimaryModel model = decodePrimary(json);
    assertEquals(Instant.parse("2024-01-15T00:00:00Z"), model.getDate());
    assertEquals(Instant.parse("2024-01-15T10:30:00Z"), model.getDateWithTime());
  }

  @Test
  void missingFieldsAreNull() throws Exception {
    PrimaryModel model = decodePrimary(fields().put(PrimaryFields.primaryKeyId, "only pk"));
    assertEquals("only pk", model.getPrimaryKey());
    assertNull(model.getSingleLineText());
    assertNull(model.getNumberInt());
    assertNull(model.getDate());
  }

  @Test
  void emptyFieldsObjectDecodesCleanly() throws Exception {
    PrimaryModel model = decodePrimary(fields());
    assertNull(model.getPrimaryKey());
    assertTrue(model.isNew());
  }

  @Test
  void encodeProducesFieldsKeyedByFieldId() {
    PrimaryModel model = PrimaryModel.builder().primaryKey("x").numberInt(7.0).build();
    Map<String, JsonNode> record = model.toCreateFields();
    assertEquals(Set.of(PrimaryFields.primaryKeyId, PrimaryFields.numberIntId), record.keySet());
    assertEquals("x", record.get(PrimaryFields.primaryKeyId).asText());
  }

  @Test
  void roundTripPreservesAllWritableFieldValues() throws Exception {
    PrimaryModel original =
        PrimaryModel.builder()
            .primaryKey("rt")
            .singleLineText("text")
            .checkbox(true)
            .numberInt(5.0)
            .numberFloat(2.5)
            .build();
    String encoded = AirtableJson.MAPPER.writeValueAsString(original);
    PrimaryModel decoded = AirtableJson.MAPPER.readValue(encoded, PrimaryModel.class);
    assertEquals(original.getPrimaryKey(), decoded.getPrimaryKey());
    assertEquals(original.getSingleLineText(), decoded.getSingleLineText());
    assertEquals(original.getCheckbox(), decoded.getCheckbox());
    assertEquals(original.getNumberInt(), decoded.getNumberInt());
    assertEquals(original.getNumberFloat(), decoded.getNumberFloat());
  }

  @Test
  void toRecordEmitsFieldValuesKeyedByFieldId() {
    PrimaryModel model = PrimaryModel.builder().primaryKey("rec").email("a@b.c").build();
    Map<String, JsonNode> record = model.toRecord();
    assertEquals("rec", record.get(PrimaryFields.primaryKeyId).asText());
    assertEquals("a@b.c", record.get(PrimaryFields.emailId).asText());
  }

  @Test
  void freshDecodeHasNoDirtyFieldsAfterSnapshot() throws Exception {
    PrimaryModel model = decodePrimary(fields().put(PrimaryFields.primaryKeyId, "clean"));
    model.takeSnapshot();
    assertTrue(model.dirtyFields().isEmpty());
  }

  @Test
  void mutatingAWritableFieldMarksOnlyThatFieldDirty() throws Exception {
    PrimaryModel model =
        decodePrimary(
            fields()
                .put(PrimaryFields.primaryKeyId, "pk")
                .put(PrimaryFields.singleLineTextId, "before"));
    model.takeSnapshot();
    model.setSingleLineText("after");
    assertEquals(Set.of(PrimaryFields.singleLineTextId), model.dirtyFields().keySet());
  }

  @Test
  void takeSnapshotClearsTheDirtySet() throws Exception {
    PrimaryModel model = decodePrimary(fields().put(PrimaryFields.primaryKeyId, "pk"));
    model.takeSnapshot();
    model.setSingleLineText("mutated");
    assertFalse(model.dirtyFields().isEmpty());
    model.takeSnapshot();
    assertTrue(model.dirtyFields().isEmpty());
  }

  @Test
  void clearedWritableFieldBecomesJsonNull() throws Exception {
    PrimaryModel model =
        decodePrimary(
            fields().put(PrimaryFields.primaryKeyId, "pk").put(PrimaryFields.numberIntId, 9.5));
    model.takeSnapshot();
    model.setNumberInt(null);
    assertEquals(NullNode.getInstance(), model.dirtyFields().get(PrimaryFields.numberIntId));
  }

  @Test
  void unsavedModelReportsIsNewTrue() {
    PrimaryModel model = PrimaryModel.builder().primaryKey("new").build();
    assertTrue(model.isNew());
    assertNull(model.getId());
  }

  @Test
  void modelWithIdReportsIsNewFalse() {
    PrimaryModel model = PrimaryModel.builder().primaryKey("saved").build();
    model.setId("recXYZ");
    assertFalse(model.isNew());
  }

  @Test
  void freshModelOmitsNullFieldsSparseWrite() throws Exception {
    PrimaryModel model = PrimaryModel.builder().primaryKey("sparse").build();
    Map<String, JsonNode> record = model.toCreateFields();
    assertEquals(Set.of(PrimaryFields.primaryKeyId), record.keySet());
    // Jackson-encode also skips nulls (NON_NULL inclusion).
    String encoded = AirtableJson.MAPPER.writeValueAsString(model);
    assertEquals("{\"" + PrimaryFields.primaryKeyId + "\":\"sparse\"}", encoded);
  }

  @Test
  void ratingIsTypeErasedJsonNode() {
    // `rating` has no Airtable metadata type mapping → JsonNode (UNKNOWN).
    PrimaryModel model = PrimaryModel.builder().primaryKey("r").rating(IntNode.valueOf(3)).build();
    assertEquals(IntNode.valueOf(3), model.getRating());
    assertEquals(IntNode.valueOf(3), model.toCreateFields().get(PrimaryFields.ratingId));
  }
}

/**
 * J10.1 — Network-bound serialization cases (split from the offline suite above, parity with Kotlin
 * TestSerializingIntegration / the integration portion of Swift TestSerializing).
 */
class TestSerializingIntegration {
  private final myairtable.Airtable airtable = TestSetup.makeAirtable();

  @Test
  void roundTripPreservesIdAndCreatedTimeAfterFetch() {
    String primaryKey = TestSetup.primaryKey("Serializing", "IdTime");
    PrimaryModel created =
        airtable.primary().create(PrimaryModel.builder().primaryKey(primaryKey).build());
    String recordId = created.getId();
    try {
      assertNotNull(created.getCreatedTime(), "createdTime populated on create");
      PrimaryModel fetched = airtable.primary().get(recordId);
      assertEquals(recordId, fetched.getId());
      assertEquals(created.getCreatedTime(), fetched.getCreatedTime());
      airtable.primary().delete(recordId);
    } catch (Throwable e) {
      try {
        airtable.primary().delete(recordId);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  @Test
  void linkedRecordFieldsSerializeAsRecordIdArrays() {
    String suite = TestSetup.primaryKey("Serializing", "Links");
    myairtable.SecondaryModel sec =
        airtable
            .secondary()
            .create(myairtable.SecondaryModel.builder().name(suite + " Target").build());
    String secId = sec.getId();
    PrimaryModel prim =
        airtable
            .primary()
            .create(
                PrimaryModel.builder()
                    .primaryKey(suite)
                    .linkSingle(java.util.List.of(secId))
                    .build());
    String primId = prim.getId();
    try {
      Map<String, JsonNode> record = prim.toRecord();
      JsonNode linkElement = record.get(PrimaryFields.linkSingleId);
      assertNotNull(linkElement);
      assertTrue(linkElement.isArray(), "links serialize as arrays");
      assertEquals(secId, linkElement.get(0).asText());
      airtable.primary().delete(primId);
      airtable.secondary().delete(secId);
    } catch (Throwable e) {
      try {
        airtable.primary().delete(primId);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      try {
        airtable.secondary().delete(secId);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  @Test
  void explicitNullOnAWritableFieldClearsItServerSide() {
    String primaryKey = TestSetup.primaryKey("Serializing", "Clear");
    PrimaryModel created =
        airtable
            .primary()
            .create(
                PrimaryModel.builder()
                    .primaryKey(primaryKey)
                    .singleLineText("to be cleared")
                    .build());
    String recordId = created.getId();
    try {
      created.setSingleLineText(null);
      PrimaryModel saved = airtable.primary().update(created);
      assertNull(saved.getSingleLineText(), "null dirty entry clears the field");

      PrimaryModel fetched = airtable.primary().get(recordId);
      assertNull(fetched.getSingleLineText());
      airtable.primary().delete(recordId);
    } catch (Throwable e) {
      try {
        airtable.primary().delete(recordId);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }
}
