package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.List;
import myairtable.Airtable;
import myairtable.PrimaryModel;
import myairtable.SecondaryModel;
import org.junit.jupiter.api.Test;

/**
 * J6.2 — Linked records as raw record-ID lists (Swift/Rust final shape — no wrapper type).
 * Resolution happens by passing the IDs to the linked table's {@code get}. Parity with Kotlin
 * TestLinkedRecords / Swift TestLinkedRecords.
 */
class TestLinkedRecords {
  private final Airtable airtable = TestSetup.makeAirtable();

  @Test
  void linkedRecordFieldsRoundTripViaRecordIds() {
    String suite = TestSetup.primaryKey("Linked", "RoundTrip");
    SecondaryModel sec1 =
        airtable.secondary().create(SecondaryModel.builder().name(suite + " S1").build());
    SecondaryModel sec2 =
        airtable.secondary().create(SecondaryModel.builder().name(suite + " S2").build());
    String sec1Id = sec1.getId();
    String sec2Id = sec2.getId();

    PrimaryModel prim =
        airtable
            .primary()
            .create(
                PrimaryModel.builder()
                    .primaryKey(suite)
                    .linkSingle(List.of(sec1Id))
                    .linkMultiple(List.of(sec1Id, sec2Id))
                    .build());
    String primId = prim.getId();
    try {
      assertEquals(List.of(sec1Id), prim.getLinkSingle());
      assertEquals(List.of(sec1Id, sec2Id), prim.getLinkMultiple());

      PrimaryModel fetched = airtable.primary().get(primId);
      assertEquals(List.of(sec1Id), fetched.getLinkSingle());
      assertEquals(List.of(sec1Id, sec2Id), fetched.getLinkMultiple());

      fetched.setLinkSingle(List.of(sec2Id));
      fetched.setLinkMultiple(List.of(sec1Id));
      PrimaryModel updated = airtable.primary().update(fetched);
      assertEquals(List.of(sec2Id), updated.getLinkSingle());
      assertEquals(List.of(sec1Id), updated.getLinkMultiple());

      airtable.primary().delete(primId);
      airtable.secondary().deleteAll(List.of(sec1Id, sec2Id));
    } catch (Throwable e) {
      try {
        airtable.primary().delete(primId);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      try {
        airtable.secondary().deleteAll(List.of(sec1Id, sec2Id));
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  @Test
  void linkedSingleRecordResolvesViaSecondaryGet() {
    String suite = TestSetup.primaryKey("Linked", "Single");
    SecondaryModel sec =
        airtable
            .secondary()
            .create(SecondaryModel.builder().name(suite + " Target").value("sv").build());
    String secId = sec.getId();

    PrimaryModel prim =
        airtable
            .primary()
            .create(PrimaryModel.builder().primaryKey(suite).linkSingle(List.of(secId)).build());
    String primId = prim.getId();
    try {
      assertEquals(1, prim.getLinkSingle().size());
      String linkedId = prim.getLinkSingle().get(0);
      assertEquals(secId, linkedId);

      SecondaryModel linked = airtable.secondary().get(linkedId);
      assertEquals(suite + " Target", linked.getName());
      assertEquals("sv", linked.getValue());

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
  void emptyLinkSingleDecodesAsNull() {
    String suite = TestSetup.primaryKey("Linked", "FetchNil");
    PrimaryModel prim =
        airtable.primary().create(PrimaryModel.builder().primaryKey(suite + " No Links").build());
    String primId = prim.getId();
    try {
      // Airtable omits empty link fields entirely; linkSingle decodes as null.
      assertTrue(
          prim.getLinkSingle() == null || prim.getLinkSingle().isEmpty(),
          "empty link field decodes as null/empty");
      airtable.primary().delete(primId);
    } catch (Throwable e) {
      try {
        airtable.primary().delete(primId);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }

  @Test
  void linkedMultiRecordsResolveViaSecondaryGet() {
    String suite = TestSetup.primaryKey("Linked", "Multi");
    SecondaryModel sec1 =
        airtable.secondary().create(SecondaryModel.builder().name(suite + " T1").build());
    SecondaryModel sec2 =
        airtable.secondary().create(SecondaryModel.builder().name(suite + " T2").build());
    List<String> secIds = List.of(sec1.getId(), sec2.getId());

    PrimaryModel prim =
        airtable
            .primary()
            .create(PrimaryModel.builder().primaryKey(suite).linkMultiple(secIds).build());
    String primId = prim.getId();
    try {
      List<SecondaryModel> linked =
          airtable
              .secondary()
              .get(prim.getLinkMultiple() == null ? List.<String>of() : prim.getLinkMultiple());
      assertEquals(2, linked.size());
      List<String> names = linked.stream().map(SecondaryModel::getName).toList();
      assertTrue(names.contains(suite + " T1"));
      assertTrue(names.contains(suite + " T2"));

      airtable.primary().delete(primId);
      airtable.secondary().deleteAll(secIds);
    } catch (Throwable e) {
      try {
        airtable.primary().delete(primId);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      try {
        airtable.secondary().deleteAll(secIds);
      } catch (RuntimeException ignored) {
        // best-effort cleanup
      }
      throw e;
    }
  }
}
