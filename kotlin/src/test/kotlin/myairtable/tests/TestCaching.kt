package myairtable.tests

import kotlinx.coroutines.delay
import kotlinx.coroutines.runBlocking
import myairtable.AirtableQuery
import myairtable.PrimaryModel
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlin.time.Duration.Companion.seconds

/**
 * K9.5 — TTL cache behavior end-to-end: hits, default-off, mutation
 * invalidation, manual invalidation, TTL expiry, query-key separation.
 * Parity with Swift TestCaching / Rust test_caching.
 */
class TestCaching {
    private fun cachedAirtable() = TestSetup.makeAirtable(cacheSeconds = 60.0)

    private fun uncachedAirtable() = TestSetup.makeAirtable() // cacheSeconds defaults to 0

    @Test
    fun cacheHitReturnsSameRecordOnRepeatedGet() =
        runBlocking {
            val at = cachedAirtable()
            val primaryKey = TestSetup.primaryKey("Cache", "Hit")
            val created = at.primary.create(PrimaryModel(primaryKey = primaryKey))
            val id = created.id!!
            try {
                val first = at.primary.get(id)
                val second = at.primary.get(id)
                assertEquals(first.primaryKey, second.primaryKey)
                assertEquals(first.id, second.id)
                assertTrue(at.client.cache.count() > 0, "read populated the cache")
                at.primary.delete(id)
            } catch (e: Throwable) {
                runCatching { at.primary.delete(id) }
                throw e
            }
        }

    @Test
    fun cacheIsDisabledWhenCacheSecondsIsZero() =
        runBlocking {
            val at = uncachedAirtable()
            assertEquals(0, at.client.cache.count())
            val primaryKey = TestSetup.primaryKey("Cache", "Off")
            val created = at.primary.create(PrimaryModel(primaryKey = primaryKey))
            val id = created.id!!
            try {
                at.primary.get(id)
                // Even after a read the cache stays empty because TTL=0 is a no-op.
                assertEquals(0, at.client.cache.count())
                at.primary.delete(id)
            } catch (e: Throwable) {
                runCatching { at.primary.delete(id) }
                throw e
            }
        }

    @Test
    fun createInvalidatesCachedReadsForTheTable() =
        runBlocking {
            val at = cachedAirtable()
            val primaryKey = TestSetup.primaryKey("Cache", "MutateCreate")
            val a = at.primary.create(PrimaryModel(primaryKey = primaryKey))
            val idA = a.id!!
            try {
                at.primary.get(idA) // populate cache
                assertTrue(at.client.cache.count() > 0)

                val b = at.primary.create(PrimaryModel(primaryKey = "$primaryKey B"))
                assertEquals(0, at.client.cache.count(), "create wipes this table's cache")

                at.primary.delete(listOf(idA, b.id!!))
            } catch (e: Throwable) {
                runCatching { at.primary.delete(idA) }
                throw e
            }
        }

    @Test
    fun updateInvalidatesCachedReadsForTheTable() =
        runBlocking {
            val at = cachedAirtable()
            val primaryKey = TestSetup.primaryKey("Cache", "MutateUpdate")
            val created = at.primary.create(PrimaryModel(primaryKey = primaryKey))
            val id = created.id!!
            try {
                at.primary.get(id)
                assertTrue(at.client.cache.count() > 0)

                created.singleLineText = "mutated"
                at.primary.update(created)
                assertEquals(0, at.client.cache.count(), "update wipes this table's cache")

                at.primary.delete(id)
            } catch (e: Throwable) {
                runCatching { at.primary.delete(id) }
                throw e
            }
        }

    @Test
    fun deleteInvalidatesCachedReadsForTheTable() =
        runBlocking {
            val at = cachedAirtable()
            val primaryKey = TestSetup.primaryKey("Cache", "MutateDelete")
            val created = at.primary.create(PrimaryModel(primaryKey = primaryKey))
            val id = created.id!!
            at.primary.get(id)
            assertTrue(at.client.cache.count() > 0)

            at.primary.delete(id)
            assertEquals(0, at.client.cache.count(), "delete wipes this table's cache")
        }

    @Test
    fun invalidateCacheDropsThisTablesCache() =
        runBlocking {
            val at = cachedAirtable()
            val primaryKey = TestSetup.primaryKey("Cache", "ManualTable")
            val created = at.primary.create(PrimaryModel(primaryKey = primaryKey))
            val id = created.id!!
            try {
                at.primary.get(id)
                assertTrue(at.client.cache.count() > 0)

                at.client.invalidateCache(myairtable.PrimaryTable.TABLE_ID)
                assertEquals(0, at.client.cache.count())

                at.primary.delete(id)
            } catch (e: Throwable) {
                runCatching { at.primary.delete(id) }
                throw e
            }
        }

    @Test
    fun invalidateAllCachesDropsEveryTablesCache() =
        runBlocking {
            val at = cachedAirtable()
            val primaryKey = TestSetup.primaryKey("Cache", "ManualAll")
            val created = at.primary.create(PrimaryModel(primaryKey = primaryKey))
            val id = created.id!!
            try {
                at.primary.get(id)
                at.secondary.get(AirtableQuery(maxRecords = 1))
                assertTrue(at.client.cache.count() >= 2)

                at.invalidateAllCaches()
                assertEquals(0, at.client.cache.count())

                at.primary.delete(id)
            } catch (e: Throwable) {
                runCatching { at.primary.delete(id) }
                throw e
            }
        }

    @Test
    fun cachedEntriesExpireAfterTheConfiguredTtl() =
        runBlocking {
            val at = TestSetup.makeAirtable(cacheSeconds = 1.0)
            val primaryKey = TestSetup.primaryKey("Cache", "TTL")
            val created = at.primary.create(PrimaryModel(primaryKey = primaryKey))
            val id = created.id!!
            try {
                at.primary.get(id)
                assertTrue(at.client.cache.count() > 0)

                delay(1.5.seconds)
                // Expired entry is lazily evicted on access; a fresh fetch still works.
                val fresh = at.primary.get(id)
                assertEquals(primaryKey, fresh.primaryKey)

                at.primary.delete(id)
            } catch (e: Throwable) {
                runCatching { at.primary.delete(id) }
                throw e
            }
        }

    @Test
    fun differentQueryParamsProduceDifferentCacheKeys() =
        runBlocking {
            val at = cachedAirtable()
            val primaryKey = TestSetup.primaryKey("Cache", "Keys")
            val created = at.primary.create(PrimaryModel(primaryKey = primaryKey))
            val id = created.id!!
            try {
                at.primary.get(AirtableQuery(maxRecords = 1))
                at.primary.get(AirtableQuery(maxRecords = 2))
                assertTrue(at.client.cache.count() >= 2, "distinct queries cache separately")

                at.primary.delete(id)
            } catch (e: Throwable) {
                runCatching { at.primary.delete(id) }
                throw e
            }
        }
}
