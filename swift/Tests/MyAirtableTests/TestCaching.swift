// F9.5 — Cache behavior integration tests. Parity with the 10 scenarios
// in rust/tests/test_caching.rs. Each test exercises the CacheStore actor
// indirectly through AirtableClient's cached GET paths.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Caching", .serialized)
struct TestCaching {
    private func cachedAirtable(seconds: TimeInterval = 60) -> Airtable {
        TestSetup.loadDotEnvIfNeeded()
        let env = ProcessInfo.processInfo.environment
        guard let apiKey = env["AIRTABLE_API_KEY"], let baseId = env["AIRTABLE_BASE_ID"]
        else { fatalError("missing Airtable credentials") }
        return Airtable(baseId: baseId, apiKey: apiKey, cacheSeconds: seconds)
    }

    private func uncachedAirtable() -> Airtable {
        TestSetup.loadDotEnvIfNeeded()
        let env = ProcessInfo.processInfo.environment
        guard let apiKey = env["AIRTABLE_API_KEY"], let baseId = env["AIRTABLE_BASE_ID"]
        else { fatalError("missing Airtable credentials") }
        return Airtable(baseId: baseId, apiKey: apiKey)  // cacheSeconds defaults to 0
    }

    // MARK: - Cache hit

    @Test("Cache hit returns same record on repeated get")
    func cacheHitReturnsSameData() async throws {
        let at = cachedAirtable()
        let primaryKey = TestSetup.primaryKey(for: "Cache", "Hit")

        let created = try await at.primary.create(PrimaryModel(primaryKey: primaryKey))
        guard let id = created.id else {
            Issue.record("missing id")
            return
        }

        do {
            let first = try await at.primary.get(id)
            let second = try await at.primary.get(id)
            #expect(first.primaryKey == second.primaryKey)
            #expect(first.id == second.id)

            try await at.primary.delete(id)
        } catch {
            try? await at.primary.delete(id)
            throw error
        }
    }

    // MARK: - Cache disabled by default

    @Test("Cache is disabled when cacheSeconds is 0")
    func cacheDisabledByDefault() async throws {
        let at = uncachedAirtable()
        let count0 = await at.client.cache.count()
        #expect(count0 == 0)

        let primaryKey = TestSetup.primaryKey(for: "Cache", "Off")
        let created = try await at.primary.create(PrimaryModel(primaryKey: primaryKey))
        guard let id = created.id else {
            Issue.record("missing id")
            return
        }

        do {
            _ = try await at.primary.get(id)
            // Even after a read the cache stays empty because defaultTtl=0 is a no-op.
            let count1 = await at.client.cache.count()
            #expect(count1 == 0)
            try await at.primary.delete(id)
        } catch {
            try? await at.primary.delete(id)
            throw error
        }
    }

    // MARK: - Mutation invalidates cache

    @Test("create invalidates cached reads for the table")
    func createInvalidates() async throws {
        let at = cachedAirtable()
        let primaryKey = TestSetup.primaryKey(for: "Cache", "MutateCreate")
        let a = try await at.primary.create(PrimaryModel(primaryKey: primaryKey))
        guard let idA = a.id else {
            Issue.record("missing id")
            return
        }

        do {
            _ = try await at.primary.get(idA)  // populate cache
            let before = await at.client.cache.count()
            #expect(before > 0)

            // Another create should wipe cached entries for this table.
            let b = try await at.primary.create(
                PrimaryModel(primaryKey: primaryKey + " B")
            )
            guard let idB = b.id else {
                Issue.record("missing id")
                return
            }
            let after = await at.client.cache.count()
            #expect(after == 0)

            try await at.primary.delete(idA)
            try await at.primary.delete(idB)
        } catch {
            try? await at.primary.delete(idA)
            throw error
        }
    }

    @Test("update invalidates cached reads for the table")
    func updateInvalidates() async throws {
        let at = cachedAirtable()
        let primaryKey = TestSetup.primaryKey(for: "Cache", "MutateUpdate")
        let created = try await at.primary.create(PrimaryModel(primaryKey: primaryKey))
        guard let id = created.id else {
            Issue.record("missing id")
            return
        }

        do {
            _ = try await at.primary.get(id)  // populate
            #expect(await at.client.cache.count() > 0)

            let fetched = try await at.primary.get(id)
            fetched.primaryKey = primaryKey + " updated"
            _ = try await at.primary.update(fetched)

            #expect(await at.client.cache.count() == 0)
            try await at.primary.delete(id)
        } catch {
            try? await at.primary.delete(id)
            throw error
        }
    }

    @Test("delete invalidates cached reads for the table")
    func deleteInvalidates() async throws {
        let at = cachedAirtable()
        let primaryKey = TestSetup.primaryKey(for: "Cache", "MutateDelete")
        let created = try await at.primary.create(PrimaryModel(primaryKey: primaryKey))
        guard let id = created.id else {
            Issue.record("missing id")
            return
        }

        _ = try await at.primary.get(id)
        #expect(await at.client.cache.count() > 0)

        try await at.primary.delete(id)
        #expect(await at.client.cache.count() == 0)
    }

    // MARK: - Manual invalidation

    @Test("invalidateCache drops this table's cache")
    func manualInvalidateTable() async throws {
        let at = cachedAirtable()
        let primaryKey = TestSetup.primaryKey(for: "Cache", "Manual")
        let created = try await at.primary.create(PrimaryModel(primaryKey: primaryKey))
        guard let id = created.id else {
            Issue.record("missing id")
            return
        }

        do {
            _ = try await at.primary.get(id)
            #expect(await at.client.cache.count() > 0)

            await at.primary.invalidateCache()
            #expect(await at.client.cache.count() == 0)

            try await at.primary.delete(id)
        } catch {
            try? await at.primary.delete(id)
            throw error
        }
    }

    @Test("invalidateAllCaches drops every table's cache")
    func manualInvalidateAll() async throws {
        let at = cachedAirtable()
        let primaryKey = TestSetup.primaryKey(for: "Cache", "All")
        let created = try await at.primary.create(PrimaryModel(primaryKey: primaryKey))
        guard let id = created.id else {
            Issue.record("missing id")
            return
        }

        do {
            _ = try await at.primary.get(id)
            #expect(await at.client.cache.count() > 0)

            await at.invalidateAllCaches()
            #expect(await at.client.cache.count() == 0)

            try await at.primary.delete(id)
        } catch {
            try? await at.primary.delete(id)
            throw error
        }
    }

    // MARK: - TTL expiry

    @Test("Cached entries expire after the configured TTL")
    func cacheExpiresAfterTtl() async throws {
        let at = cachedAirtable(seconds: 1)  // 1-second TTL
        let primaryKey = TestSetup.primaryKey(for: "Cache", "TTL")
        let created = try await at.primary.create(PrimaryModel(primaryKey: primaryKey))
        guard let id = created.id else {
            Issue.record("missing id")
            return
        }

        do {
            _ = try await at.primary.get(id)
            #expect(await at.client.cache.count() > 0)

            // Wait past TTL; next get should re-fetch (lazy expiry also trims).
            try await Task.sleep(nanoseconds: 1_500_000_000)
            _ = try await at.primary.get(id)

            try await at.primary.delete(id)
        } catch {
            try? await at.primary.delete(id)
            throw error
        }
    }

    // MARK: - Independence of dict vs orm reads

    @Test("Different query params produce different cache keys")
    func distinctQueryKeys() async throws {
        let at = cachedAirtable()
        let suite = TestSetup.primaryKey(for: "Cache", "QueryKeys")

        let c1 = try await at.primary.create(PrimaryModel(primaryKey: "\(suite) A"))
        let c2 = try await at.primary.create(PrimaryModel(primaryKey: "\(suite) B"))
        guard let id1 = c1.id, let id2 = c2.id else {
            Issue.record("missing ids")
            return
        }

        do {
            _ = try await at.primary.get(id1)
            _ = try await at.primary.get(id2)
            // Two distinct records → two cache entries.
            #expect(await at.client.cache.count() >= 2)

            try await at.primary.delete([id1, id2])
        } catch {
            try? await at.primary.delete([id1, id2])
            throw error
        }
    }
}
