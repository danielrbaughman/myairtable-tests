package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.Callable;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.concurrent.TimeUnit;
import myairtable.Airtable;
import myairtable.PrimaryModel;
import myairtable.PrimaryTable;
import org.junit.jupiter.api.Test;

/**
 * TC9 — Cache thread-safety. The TTL cache is shared mutable state on the client; the caching suite
 * only exercises it single-threaded. These fire concurrent reads and concurrent mutations at one
 * cached client and assert no corruption/exception and a consistent result. Parity with C#
 * TestCacheConcurrency.
 */
class TestCacheConcurrency {

  private static Airtable cachedAirtable() {
    return TestSetup.makeAirtable(60.0);
  }

  private static void bestEffortDelete(Airtable at, String recordId) {
    if (recordId == null) {
      return;
    }
    try {
      at.primary().delete(recordId);
    } catch (RuntimeException ignored) {
      // best-effort cleanup
    }
  }

  /** Await every future, propagating the first failure as a RuntimeException. */
  private static List<Object> awaitAll(List<? extends Future<?>> futures) {
    List<Object> results = new ArrayList<>(futures.size());
    RuntimeException firstFailure = null;
    for (Future<?> f : futures) {
      try {
        results.add(f.get());
      } catch (Exception e) {
        if (firstFailure == null) {
          firstFailure =
              (e.getCause() instanceof RuntimeException)
                  ? (RuntimeException) e.getCause()
                  : new RuntimeException(e.getCause() != null ? e.getCause() : e);
        }
      }
    }
    if (firstFailure != null) {
      throw firstFailure;
    }
    return results;
  }

  @Test
  void concurrentGetsOfTheSameRecordAreConsistent() {
    Airtable at = cachedAirtable();
    String key = TestSetup.primaryKey("CacheConc", "Reads");
    PrimaryModel created = at.primary().create(PrimaryModel.builder().primaryKey(key).build());
    String id = created.getId();
    // 25 concurrent gets race to populate/read the shared cache.
    ExecutorService pool = Executors.newFixedThreadPool(8);
    try {
      List<Future<PrimaryModel>> futures = new ArrayList<>();
      for (int i = 0; i < 25; i++) {
        Callable<PrimaryModel> task = () -> at.primary().get(id);
        futures.add(pool.submit(task));
      }
      for (Object result : awaitAll(futures)) {
        PrimaryModel m = (PrimaryModel) result;
        assertEquals(key, m.getPrimaryKey());
        assertEquals(id, m.getId());
      }
    } catch (Throwable e) {
      bestEffortDelete(at, id);
      throw e;
    } finally {
      pool.shutdown();
    }
    bestEffortDelete(at, id);
  }

  @Test
  void concurrentReadsAndMutationsDoNotCorruptTheCache() throws InterruptedException {
    Airtable at = cachedAirtable();
    String key = TestSetup.primaryKey("CacheConc", "Mix");
    PrimaryModel created = at.primary().create(PrimaryModel.builder().primaryKey(key).build());
    String id = created.getId();
    // Interleave reads (populate cache), manual per-table invalidations, and invalidate-all
    // concurrently.
    ExecutorService pool = Executors.newFixedThreadPool(8);
    try {
      List<Future<?>> futures = new ArrayList<>();
      for (int i = 0; i < 15; i++) {
        futures.add(pool.submit((Callable<PrimaryModel>) () -> at.primary().get(id)));
        futures.add(
            pool.submit(
                () -> {
                  at.client().invalidateCache(PrimaryTable.TABLE_ID);
                  return null;
                }));
        futures.add(
            pool.submit(
                () -> {
                  at.invalidateAllCaches();
                  return null;
                }));
      }
      awaitAll(futures);

      // The client is still usable and returns the correct value afterward.
      PrimaryModel fetched = at.primary().get(id);
      assertEquals(key, fetched.getPrimaryKey());
    } catch (Throwable e) {
      bestEffortDelete(at, id);
      throw e;
    } finally {
      pool.shutdown();
      pool.awaitTermination(10, TimeUnit.SECONDS);
    }
    bestEffortDelete(at, id);
  }
}
