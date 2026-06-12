package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import myairtable.Airtable;
import myairtable.AirtableException;
import myairtable.PrimaryModel;
import org.junit.jupiter.api.Test;

/**
 * J4.7 — Model-side CRUD (save/fetch/delete instance methods). Parity with Kotlin
 * TestOrmCrudViaModel / Swift TestOrmCrudViaModel / Rust test_orm_crud_via_model.
 */
class TestOrmCrudViaModel {
  private final Airtable airtable = TestSetup.makeAirtable();

  @Test
  void primaryKeyOnlyCrudViaModelMethods() {
    String primaryKey = TestSetup.primaryKey("OrmModel", "PKOnly");
    PrimaryModel created =
        airtable.primary().create(PrimaryModel.builder().primaryKey(primaryKey).build());
    String recordId = created.getId();
    try {
      // Mutate + save() — no table argument needed.
      created.setPrimaryKey(primaryKey + " Updated");
      PrimaryModel saved = created.save();
      assertEquals(primaryKey + " Updated", saved.getPrimaryKey());
      assertEquals(recordId, saved.getId());

      // fetch() returns a fresh server copy.
      PrimaryModel fetched = saved.fetch();
      assertEquals(primaryKey + " Updated", fetched.getPrimaryKey());

      // delete() via the model.
      fetched.delete();
      assertThrows(
          AirtableException.class,
          () -> airtable.primary().get(recordId),
          "record should be gone after delete");
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
  void dirtyTrackingResetsAfterASuccessfulSave() {
    String primaryKey = TestSetup.primaryKey("OrmModel", "Dirty");
    PrimaryModel created =
        airtable.primary().create(PrimaryModel.builder().primaryKey(primaryKey).build());
    String recordId = created.getId();
    try {
      assertTrue(created.dirtyFields().isEmpty(), "freshly created model is clean");
      created.setSingleLineText("dirty now");
      assertEquals(1, created.dirtyFields().size());

      PrimaryModel saved = created.save();
      assertTrue(saved.dirtyFields().isEmpty(), "saved model is clean again");
      assertEquals("dirty now", saved.getSingleLineText());

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
  void deleteOnAnUnsavedModelThrows() {
    PrimaryModel model = PrimaryModel.builder().primaryKey("never saved").build();
    // Unsaved AND detached — the detached check fires first.
    assertThrows(AirtableException.Api.class, model::delete);
  }

  @Test
  void fetchOnAnUnsavedModelThrows() {
    PrimaryModel model = PrimaryModel.builder().primaryKey("never saved").build();
    assertThrows(AirtableException.Api.class, model::fetch);
  }
}
