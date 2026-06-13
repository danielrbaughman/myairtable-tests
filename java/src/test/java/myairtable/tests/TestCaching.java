package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.List;
import myairtable.Airtable;
import myairtable.AirtableQuery;
import myairtable.CacheStore;
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
    // Track every created id so a failing assert still cleans up all of them, not just idA.
    String idB = null;
    try {
      at.primary().get(idA); // populate cache
      assertTrue(at.client().getCache().count() > 0);

      PrimaryModel b =
          at.primary().create(PrimaryModel.builder().primaryKey(primaryKey + " B").build());
      idB = b.getId();
      assertEquals(0, at.client().getCache().count(), "create wipes this table's cache");

      at.primary().deleteAll(List.of(idA, idB));
    } catch (Throwable e) {
      bestEffortDelete(at, idA);
      if (idB != null) {
        bestEffortDelete(at, idB);
      }
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
    // The cache-populating read and its assert were previously outside any try, so a failing
    // count() assert leaked the created record. Guard the whole sequence.
    boolean deleted = false;
    try {
      at.primary().get(id);
      assertTrue(at.client().getCache().count() > 0);

      at.primary().delete(id);
      deleted = true;
      assertEquals(0, at.client().getCache().count(), "delete wipes this table's cache");
    } catch (Throwable e) {
      if (!deleted) {
        bestEffortDelete(at, id);
      }
      throw e;
    }
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
      // Distinguish genuine TTL expiry from a stale hit: simply re-reading the same payload would
      // pass whether or not expiry works (a stale entry returns the same record). Probe the cache
      // directly for the single-record entry — CacheStore.get() lazily evicts an entry past its
      // TTL and returns null, so a null here proves the pre-sleep entry actually expired rather
      // than being served stale. (Key shape mirrors AirtableClient.getRecord: "rec:" + id.)
      CacheStore.Key recordKey = new CacheStore.Key(PrimaryTable.TABLE_ID, "rec:" + id);
      assertNull(
          at.client().getCache().get(recordKey),
          "TTL-expired entry must be evicted on access, not served stale");

      // A fresh fetch still works (and re-populates the cache).
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
