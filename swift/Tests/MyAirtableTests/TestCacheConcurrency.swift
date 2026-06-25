// TC9 — Cache thread-safety. The TTL cache is shared mutable state on the
// client; the caching suite only exercises it single-threaded. These fire
// concurrent reads and concurrent mutations at one cached client and assert no
// corruption/exception and a consistent result. Mirrors
// csharp/tests/TestCacheConcurrency.cs.
//
// In the Swift port the cache lives in a `CacheStore` actor owned by the
// `AirtableClient` actor; `Airtable`/`OrmTable` are `Sendable` struct
// front-ends. That makes the client safe to share across `TaskGroup` children,
// so this suite is a behavioral parity check rather than a data-race probe.

import Foundation
import Testing

@testable import MyAirtable

@Suite("Cache concurrency", .serialized)
struct TestCacheConcurrency {
    private func cachedAirtable(seconds: TimeInterval = 60) -> Airtable {
        TestSetup.loadDotEnvIfNeeded()
        let env = ProcessInfo.processInfo.environment
        guard let apiKey = env["AIRTABLE_API_KEY"], let baseId = env["AIRTABLE_BASE_ID"]
        else { fatalError("missing Airtable credentials") }
        return Airtable(baseId: baseId, apiKey: apiKey, cacheSeconds: seconds)
    }

    // MARK: - Concurrent gets of the same record

    @Test("Concurrent gets of the same record are consistent")
    func concurrentGetsOfTheSameRecordAreConsistent() async throws {
        let at = cachedAirtable()
        let key = TestSetup.primaryKey(for: "CacheConc", "Reads")
        let created = try await at.primary.create(PrimaryModel(primaryKey: key))
        guard let id = created.id else {
            Issue.record("missing id")
            return
        }

        do {
            // 25 concurrent gets race to populate/read the shared cache. Each
            // child extracts the Sendable scalars (PrimaryModel is a
            // non-Sendable @Observable class, so the model itself can't cross
            // the task-group boundary).
            let results = try await withThrowingTaskGroup(of: (String?, String?).self) {
                group in
                for _ in 0..<25 {
                    group.addTask {
                        let m = try await at.primary.get(id)
                        return (m.primaryKey, m.id)
                    }
                }
                var collected: [(String?, String?)] = []
                for try await pair in group {
                    collected.append(pair)
                }
                return collected
            }

            #expect(results.count == 25)
            for (primaryKey, recordId) in results {
                #expect(primaryKey == key)
                #expect(recordId == id)
            }

            await bestEffortDelete(at, id)
        } catch {
            await bestEffortDelete(at, id)
            throw error
        }
    }

    // MARK: - Concurrent reads + invalidations

    @Test("Concurrent reads and mutations do not corrupt the cache")
    func concurrentReadsAndMutationsDoNotCorruptTheCache() async throws {
        let at = cachedAirtable()
        let key = TestSetup.primaryKey(for: "CacheConc", "Mix")
        let created = try await at.primary.create(PrimaryModel(primaryKey: key))
        guard let id = created.id else {
            Issue.record("missing id")
            return
        }

        do {
            // Interleave reads (populate cache), per-table invalidations, and
            // invalidate-all calls concurrently.
            try await withThrowingTaskGroup(of: Void.self) { group in
                for _ in 0..<15 {
                    group.addTask { _ = try await at.primary.get(id) }
                    group.addTask {
                        await at.client.invalidateCache(tableId: PrimaryTable.tableId)
                    }
                    group.addTask { await at.invalidateAllCaches() }
                }
                try await group.waitForAll()
            }

            // The client is still usable and returns the correct value afterward.
            let fetched = try await at.primary.get(id)
            #expect(fetched.primaryKey == key)

            await bestEffortDelete(at, id)
        } catch {
            await bestEffortDelete(at, id)
            throw error
        }
    }

    private func bestEffortDelete(_ at: Airtable, _ id: String?) async {
        guard let id, !id.isEmpty else { return }
        try? await at.primary.delete(id)
    }
}
