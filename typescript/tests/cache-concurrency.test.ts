import { describe, it, expect } from "vitest";
import { Record as ATRecord } from "airtable";
import { Airtable, PrimaryFieldSet } from "../output";

/** A primary-key value unique to this run (distinct per scenario via `label`). */
function suiteKey(label: string): string {
	const ts = Date.now();
	const rand = Math.floor(Math.random() * (999999 - 100000) + 100000);
	return `TypeScript CacheConc ${label} ${ts}-${rand}`;
}

function newPrimaryRecord(airtable: Airtable): ATRecord<PrimaryFieldSet> {
	return new ATRecord<PrimaryFieldSet>(airtable.primary._table, "", {});
}

async function bestEffortDelete(airtable: Airtable, id: string | undefined): Promise<void> {
	if (!id) return;
	try {
		await airtable.primary.delete(id);
	} catch {
		// best effort
	}
}

/**
 * TC9 — Cache thread-safety. The TTL cache is shared mutable state on the client. JS is
 * single-threaded but async-concurrent: overlapping in-flight requests share the cache. These fire
 * concurrent reads and concurrent reads+invalidations at one cached client and assert no corruption
 * and a consistent result. Parity target for the other 8 suites.
 */
describe("TC9 — Cache thread-safety", () => {
	it("concurrent gets of the same record are consistent", async () => {
		const airtable = new Airtable({ cacheSeconds: 60 });
		const key = suiteKey("Reads");
		const newRecord = newPrimaryRecord(airtable);
		newRecord.set("Primary Key", key);
		const created = await airtable.primary.create(newRecord);
		const id = created.id;
		try {
			// 25 concurrent gets race to populate/read the shared cache.
			const results = await Promise.all(Array.from({ length: 25 }, () => airtable.primary.get(id)));
			for (const m of results) {
				expect(m.primaryKey).toBe(key);
				expect(m.id).toBe(id);
			}
		} finally {
			await bestEffortDelete(airtable, id);
		}
	}, 60000);

	it("concurrent reads and invalidations do not corrupt the cache", async () => {
		const airtable = new Airtable({ cacheSeconds: 60 });
		const key = suiteKey("Mix");
		const newRecord = newPrimaryRecord(airtable);
		newRecord.set("Primary Key", key);
		const created = await airtable.primary.create(newRecord);
		const id = created.id;
		try {
			// Interleave reads (populate cache), per-table invalidations, and cascade
			// invalidations concurrently.
			const tasks: Promise<unknown>[] = [];
			for (let i = 0; i < 15; i++) {
				tasks.push(airtable.primary.get(id));
				tasks.push(Promise.resolve().then(() => airtable.primary.invalidateCache()));
				tasks.push(Promise.resolve().then(() => airtable.invalidateCache()));
			}
			await Promise.all(tasks);

			// The client is still usable and returns the correct value afterward.
			const fetched = await airtable.primary.get(id);
			expect(fetched.primaryKey).toBe(key);
		} finally {
			await bestEffortDelete(airtable, id);
		}
	}, 60000);
});
