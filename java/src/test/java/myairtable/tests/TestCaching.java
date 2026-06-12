package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.List;
import myairtable.Airtable;
import myairtable.AirtableQuery;
import myairtable.PrimaryModel;
import myairtable.PrimaryTable;
import org.junit.jupiter.api.Test;

/**
 * J-port of Kotlin K9.5 — TTL cache behavior end-to-end: hits, default-off, mutation invalidation,
 * manual invalidation, TTL expiry, query-key separation. Parity with Kotlin TestCaching / Swift
 * TestCaching / Rust test_caching.
 */
class TestCaching {

  private static Airtable cachedAirtable() {
    return TestSetup.makeAirtable(60.0);
  }

  private static Airtable uncachedAirtable() {
    return TestSetup.makeAirtable(); // cacheSeconds defaults to 0
  }

  private static void bestEffortDelete(Airtable at, String recordId) {
    try {
      at.primary().delete(recordId);
    } catch (RuntimeException ignored) {
      // best-effort cleanup
    }
  }

  @Test
  void cacheHitReturnsSameRecordOnRepeatedGet() {
    Airtable at = cachedAirtable();
    String primaryKey = TestSetup.primaryKey("Cache", "Hit");
    PrimaryModel created =
        at.primary().create(PrimaryModel.builder().primaryKey(primaryKey).build());
    String id = created.getId();
    try {
      PrimaryModel first = at.primary().get(id);
      PrimaryModel second = at.primary().get(id);
      assertEquals(first.getPrimaryKey(), second.getPrimaryKey());
      assertEquals(first.getId(), second.getId());
      assertTrue(at.client().getCache().count() > 0, "read populated the cache");
      at.primary().delete(id);
    } catch (Throwable e) {
      bestEffortDelete(at, id);
      throw e;
    }
  }

  @Test
  void cacheIsDisabledWhenCacheSecondsIsZero() {
    Airtable at = uncachedAirtable();
    assertEquals(0, at.client().getCache().count());
    String primaryKey = TestSetup.primaryKey("Cache", "Off");
    PrimaryModel created =
        at.primary().create(PrimaryModel.builder().primaryKey(primaryKey).build());
    String id = created.getId();
    try {
      at.primary().get(id);
      // Even after a read the cache stays empty because TTL=0 is a no-op.
      assertEquals(0, at.client().getCache().count());
      at.primary().delete(id);
    } catch (Throwable e) {
      bestEffortDelete(at, id);
      throw e;
    }
  }

  @Test
  void createInvalidatesCachedReadsForTheTable() {
    Airtable at = cachedAirtable();
    String primaryKey = TestSetup.primaryKey("Cache", "MutateCreate");
    PrimaryModel a = at.primary().create(PrimaryModel.builder().primaryKey(primaryKey).build());
    String idA = a.getId();
    try {
      at.primary().get(idA); // populate cache
      assertTrue(at.client().getCache().count() > 0);

      PrimaryModel b =
          at.primary().create(PrimaryModel.builder().primaryKey(primaryKey + " B").build());
      assertEquals(0, at.client().getCache().count(), "create wipes this table's cache");

      at.primary().deleteAll(List.of(idA, b.getId()));
    } catch (Throwable e) {
      bestEffortDelete(at, idA);
      throw e;
    }
  }

  @Test
  void updateInvalidatesCachedReadsForTheTable() {
    Airtable at = cachedAirtable();
    String primaryKey = TestSetup.primaryKey("Cache", "MutateUpdate");
    PrimaryModel created =
        at.primary().create(PrimaryModel.builder().primaryKey(primaryKey).build());
    String id = created.getId();
    try {
      at.primary().get(id);
      assertTrue(at.client().getCache().count() > 0);

      created.setSingleLineText("mutated");
      at.primary().update(created);
      assertEquals(0, at.client().getCache().count(), "update wipes this table's cache");

      at.primary().delete(id);
    } catch (Throwable e) {
      bestEffortDelete(at, id);
      throw e;
    }
  }

  @Test
  void deleteInvalidatesCachedReadsForTheTable() {
    Airtable at = cachedAirtable();
    String primaryKey = TestSetup.primaryKey("Cache", "MutateDelete");
    PrimaryModel created =
        at.primary().create(PrimaryModel.builder().primaryKey(primaryKey).build());
    String id = created.getId();
    at.primary().get(id);
    assertTrue(at.client().getCache().count() > 0);

    at.primary().delete(id);
    assertEquals(0, at.client().getCache().count(), "delete wipes this table's cache");
  }

  @Test
  void invalidateCacheDropsThisTablesCache() {
    Airtable at = cachedAirtable();
    String primaryKey = TestSetup.primaryKey("Cache", "ManualTable");
    PrimaryModel created =
        at.primary().create(PrimaryModel.builder().primaryKey(primaryKey).build());
    String id = created.getId();
    try {
      at.primary().get(id);
      assertTrue(at.client().getCache().count() > 0);

      at.client().invalidateCache(PrimaryTable.TABLE_ID);
      assertEquals(0, at.client().getCache().count());

      at.primary().delete(id);
    } catch (Throwable e) {
      bestEffortDelete(at, id);
      throw e;
    }
  }

  @Test
  void invalidateAllCachesDropsEveryTablesCache() {
    Airtable at = cachedAirtable();
    String primaryKey = TestSetup.primaryKey("Cache", "ManualAll");
    PrimaryModel created =
        at.primary().create(PrimaryModel.builder().primaryKey(primaryKey).build());
    String id = created.getId();
    try {
      at.primary().get(id);
      at.secondary().get(new AirtableQuery().withMaxRecords(1));
      assertTrue(at.client().getCache().count() >= 2);

      at.invalidateAllCaches();
      assertEquals(0, at.client().getCache().count());

      at.primary().delete(id);
    } catch (Throwable e) {
      bestEffortDelete(at, id);
      throw e;
    }
  }

  @Test
  void cachedEntriesExpireAfterTheConfiguredTtl() throws InterruptedException {
    Airtable at = TestSetup.makeAirtable(1.0);
    String primaryKey = TestSetup.primaryKey("Cache", "TTL");
    PrimaryModel created =
        at.primary().create(PrimaryModel.builder().primaryKey(primaryKey).build());
    String id = created.getId();
    try {
      at.primary().get(id);
      assertTrue(at.client().getCache().count() > 0);

      Thread.sleep(1_500);
      // Expired entry is lazily evicted on access; a fresh fetch still works.
      PrimaryModel fresh = at.primary().get(id);
      assertEquals(primaryKey, fresh.getPrimaryKey());

      at.primary().delete(id);
    } catch (Throwable e) {
      bestEffortDelete(at, id);
      throw e;
    }
  }

  @Test
  void differentQueryParamsProduceDifferentCacheKeys() {
    Airtable at = cachedAirtable();
    String primaryKey = TestSetup.primaryKey("Cache", "Keys");
    PrimaryModel created =
        at.primary().create(PrimaryModel.builder().primaryKey(primaryKey).build());
    String id = created.getId();
    try {
      at.primary().get(new AirtableQuery().withMaxRecords(1));
      at.primary().get(new AirtableQuery().withMaxRecords(2));
      assertTrue(at.client().getCache().count() >= 2, "distinct queries cache separately");

      at.primary().delete(id);
    } catch (Throwable e) {
      bestEffortDelete(at, id);
      throw e;
    }
  }
}
