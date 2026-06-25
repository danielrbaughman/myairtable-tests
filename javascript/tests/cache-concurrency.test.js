import { describe, it, expect } from "vitest";
const { Record: ATRecord } = require("airtable");
const { Airtable } = require("../output");

/**
 * TC9 — Cache thread-safety.
 *
 * Mirrors csharp/tests/TestCacheConcurrency.cs. The TTL cache is shared mutable state on the
 * client; the caching suite only exercises it single-threaded. JavaScript is single-threaded but
 * async-concurrent: many in-flight promises can interleave around the cache between awaits. These
 * tests fire concurrent reads and concurrent reads+invalidations at one cached client and assert
 * no corruption and a consistent result afterward.
 *
 * Per-table invalidation -> airtable.primary.invalidateCache(); invalidate-all (cascade across all
 * tables) -> airtable.invalidateCache().
 */

function cached() {
	return new Airtable({ cacheSeconds: 60 });
}

function newPrimaryRecord(airtable) {
	return new ATRecord(airtable.primary._table, "", {});
}

function primaryKey(label) {
	return `CacheConcJS ${label} ${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
}

async function bestEffortDelete(airtable, id) {
	if (!id) return;
	try {
		await airtable.primary.delete(id);
	} catch {
		// best-effort cleanup
	}
}

describe("Concurrent gets of the same record are consistent", async () => {
	const airtable = cached();
	const key = primaryKey("Reads");
	const newRecord = newPrimaryRecord(airtable);
	newRecord.set("Primary Key", key);
	const created = await airtable.primary.create(newRecord);
	const id = created.id;

	it("25 concurrent gets all return the correct primaryKey + id", async () => {
		// 25 concurrent gets race to populate/read the shared cache.
		const results = await Promise.all(Array.from({ length: 25 }, () => airtable.primary.get(id)));
		expect(results).toHaveLength(25);
		for (const m of results) {
			expect(m.primaryKey).toBe(key);
			expect(m.id).toBe(id);
		}
	}, 30000);

	it("cleanup", async () => {
		await bestEffortDelete(airtable, id);
	});
});

describe("Concurrent reads and invalidations do not corrupt the cache", async () => {
	const airtable = cached();
	const key = primaryKey("Mix");
	const newRecord = newPrimaryRecord(airtable);
	newRecord.set("Primary Key", key);
	const created = await airtable.primary.create(newRecord);
	const id = created.id;

	it("interleaved reads + invalidations leave the client usable and correct", async () => {
		// Interleave reads (populate cache), per-table invalidations, and invalidate-all
		// (cascade) concurrently.
		const ops = [];
		for (let i = 0; i < 15; i++) {
			ops.push(airtable.primary.get(id));
			ops.push(Promise.resolve().then(() => airtable.primary.invalidateCache()));
			ops.push(Promise.resolve().then(() => airtable.invalidateCache()));
		}
		await Promise.all(ops);

		// The client is still usable and returns the correct value afterward.
		const fetched = await airtable.primary.get(id);
		expect(fetched.primaryKey).toBe(key);
		expect(fetched.id).toBe(id);
	}, 30000);

	it("cleanup", async () => {
		await bestEffortDelete(airtable, id);
	});
});
