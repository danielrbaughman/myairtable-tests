package myairtable.tests

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.runBlocking
import myairtable.Airtable
import myairtable.PrimaryModel
import myairtable.PrimaryTable
import kotlin.test.Test
import kotlin.test.assertEquals

/**
 * TC9 — Cache thread-safety. The TTL cache is shared mutable state on the client; the
 * caching suite only exercises it single-threaded. These fire concurrent reads and
 * concurrent mutations at one cached client and assert no corruption/exception and a
 * consistent result. Parity with C# TestCacheConcurrency.
 */
class TestCacheConcurrency {
    private fun cachedAirtable() = TestSetup.makeAirtable(cacheSeconds = 60.0)

    @Test
    fun concurrentGetsOfTheSameRecordAreConsistent(): Unit =
        runBlocking {
            val at = cachedAirtable()
            val key = TestSetup.primaryKey("CacheConc", "Reads")
            val created = at.primary.create(PrimaryModel(primaryKey = key))
            val id = created.id!!
            try {
                // 25 concurrent gets race to populate/read the shared cache.
                val results =
                    (0 until 25)
                        .map { async(Dispatchers.Default) { at.primary.get(id) } }
                        .awaitAll()
                for (m in results) {
                    assertEquals(key, m.primaryKey)
                    assertEquals(id, m.id)
                }
            } finally {
                bestEffortDelete(at, id)
            }
        }

    @Test
    fun concurrentReadsAndMutationsDoNotCorruptTheCache(): Unit =
        runBlocking {
            val at = cachedAirtable()
            val key = TestSetup.primaryKey("CacheConc", "Mix")
            val created = at.primary.create(PrimaryModel(primaryKey = key))
            val id = created.id!!
            try {
                // Interleave reads (populate cache), per-table invalidations, and
                // invalidate-all concurrently.
                val jobs =
                    (0 until 15).flatMap {
                        listOf(
                            async(Dispatchers.Default) {
                                at.primary.get(id)
                                Unit
                            },
                            async(Dispatchers.Default) { at.client.invalidateCache(PrimaryTable.TABLE_ID) },
                            async(Dispatchers.Default) { at.invalidateAllCaches() },
                        )
                    }
                jobs.awaitAll()

                // The client is still usable and returns the correct value afterward.
                val fetched = at.primary.get(id)
                assertEquals(key, fetched.primaryKey)
            } finally {
                bestEffortDelete(at, id)
            }
        }

    private suspend fun bestEffortDelete(
        at: Airtable,
        id: String?,
    ) {
        if (id.isNullOrEmpty()) return
        runCatching { at.primary.delete(id) }
    }
}
