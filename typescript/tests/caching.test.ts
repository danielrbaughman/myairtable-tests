import { describe, it, expect } from "vitest";
import { Record as ATRecord } from "airtable";
import { Airtable, PrimaryFieldSet, SecondaryFieldSet } from "../output";

function newPrimaryRecord(airtable: Airtable): ATRecord<PrimaryFieldSet> {
	return new ATRecord<PrimaryFieldSet>(airtable.primary._table, "", {});
}

function newSecondaryRecord(airtable: Airtable): ATRecord<SecondaryFieldSet> {
	return new ATRecord<SecondaryFieldSet>(airtable.secondary._table, "", {});
}

describe("Cache Hit (reference equality)", async () => {
	const airtable = new Airtable({ cacheSeconds: 60 });
	const newRecord = newPrimaryRecord(airtable);
	newRecord.set("Primary Key", "Cache Hit Test");
	const created = await airtable.primary.create(newRecord);
	const id = created.id;

	const first = await airtable.primary.get(id);
	const second = await airtable.primary.get(id);

	it("should return the exact same object", () => {
		expect(second).toBe(first);
	});

	it("cleanup", async () => {
		await airtable.primary.delete(id);
	});
});

describe("Cache Disabled By Default", async () => {
	const airtable = new Airtable();
	const newRecord = newPrimaryRecord(airtable);
	newRecord.set("Primary Key", "No Cache Test");
	const created = await airtable.primary.create(newRecord);
	const id = created.id;

	await airtable.primary.get(id);
	const cacheSize = (airtable.primary as any)._cache.size;

	it("should have empty cache", () => {
		expect(cacheSize).toBe(0);
	});

	it("cleanup", async () => {
		await airtable.primary.delete(id);
	});
});

describe("Mutation Invalidation", async () => {
	const airtable = new Airtable({ cacheSeconds: 60 });

	const r1 = newPrimaryRecord(airtable);
	r1.set("Primary Key", "Mutation Test");
	const created = await airtable.primary.create(r1);
	const id = created.id;

	it("get populates cache, create invalidates it", async () => {
		await airtable.primary.get(id);
		expect((airtable.primary as any)._cache.size).toBeGreaterThan(0);

		const r2 = newPrimaryRecord(airtable);
		r2.set("Primary Key", "Mutation Test 2");
		const created2 = await airtable.primary.create(r2);
		expect((airtable.primary as any)._cache.size).toBe(0);
		await airtable.primary.delete(created2.id);
	});

	it("update invalidates cache", async () => {
		await airtable.primary.get(id);
		expect((airtable.primary as any)._cache.size).toBeGreaterThan(0);

		const r = await airtable.primary.get(id, { returnAs: "record" });
		r.set("Primary Key", "Mutation Updated");
		await airtable.primary.update(r);
		expect((airtable.primary as any)._cache.size).toBe(0);
	});

	it("delete invalidates cache", async () => {
		await airtable.primary.get(id);
		expect((airtable.primary as any)._cache.size).toBeGreaterThan(0);

		await airtable.primary.delete(id);
		expect((airtable.primary as any)._cache.size).toBe(0);
	});
});

describe("Manual Invalidation", async () => {
	const airtable = new Airtable({ cacheSeconds: 60 });
	const newRecord = newPrimaryRecord(airtable);
	newRecord.set("Primary Key", "Manual Invalidation Test");
	const created = await airtable.primary.create(newRecord);
	const id = created.id;

	it("invalidateCache clears cache", async () => {
		await airtable.primary.get(id);
		expect((airtable.primary as any)._cache.size).toBeGreaterThan(0);

		airtable.primary.invalidateCache();
		expect((airtable.primary as any)._cache.size).toBe(0);
	});

	it("get still works after invalidation", async () => {
		const result = await airtable.primary.get(id);
		expect(result.id).toBe(id);
	});

	it("cleanup", async () => {
		await airtable.primary.delete(id);
	});
});

describe("Main Client Cascade Invalidation", async () => {
	const airtable = new Airtable({ cacheSeconds: 60 });

	const pRec = newPrimaryRecord(airtable);
	pRec.set("Primary Key", "Cascade Primary");
	const pCreated = await airtable.primary.create(pRec);

	const sRec = newSecondaryRecord(airtable);
	sRec.set("Name", "Cascade Secondary");
	const sCreated = await airtable.secondary.create(sRec);

	it("invalidateCache cascades to all tables", async () => {
		await airtable.primary.get(pCreated.id);
		await airtable.secondary.get(sCreated.id);
		expect((airtable.primary as any)._cache.size).toBeGreaterThan(0);
		expect((airtable.secondary as any)._cache.size).toBeGreaterThan(0);

		airtable.invalidateCache();
		expect((airtable.primary as any)._cache.size).toBe(0);
		expect((airtable.secondary as any)._cache.size).toBe(0);
	});

	it("cleanup", async () => {
		await airtable.primary.delete(pCreated.id);
		await airtable.secondary.delete(sCreated.id);
	});
});

describe("Cache Expiry", async () => {
	const airtable = new Airtable({ cacheSeconds: 2 });
	const newRecord = newPrimaryRecord(airtable);
	newRecord.set("Primary Key", "Expiry Test");
	const created = await airtable.primary.create(newRecord);
	const id = created.id;

	it("expires after TTL", async () => {
		const first = await airtable.primary.get(id);
		const cached = await airtable.primary.get(id);
		expect(cached).toBe(first);

		await new Promise((r) => setTimeout(r, 3000));

		const afterExpiry = await airtable.primary.get(id);
		expect(afterExpiry).not.toBe(first);
	});

	it("cleanup", async () => {
		await airtable.primary.delete(id);
	});
});

describe("Cache Serves Stale Data", async () => {
	const airtable = new Airtable({ cacheSeconds: 60 });

	const newRecord = newPrimaryRecord(airtable);
	newRecord.set("Primary Key", "Stale Data Test");
	const created = await airtable.primary.create(newRecord);
	const id = created.id;

	it("returns stale cached data after external update", async () => {
		// Populate cache with original value
		const first = await airtable.primary.get(id);
		expect(first.primaryKey).toBe("Stale Data Test");

		// Update via underlying Airtable.js API directly (bypasses cache invalidation)
		await airtable.primary._table.update(id, { "Primary Key": "Updated Directly" });

		// Cached instance should return stale data
		const stale = await airtable.primary.get(id);
		expect(stale.primaryKey).toBe("Stale Data Test");
	});

	it("cleanup", async () => {
		await airtable.primary.delete(id);
	});
});

describe("Different Parameters Different Cache Keys", async () => {
	const airtable = new Airtable({ cacheSeconds: 60 });

	const r1 = newPrimaryRecord(airtable);
	r1.set("Primary Key", "Cache Key 1");
	const c1 = await airtable.primary.create(r1);

	const r2 = newPrimaryRecord(airtable);
	r2.set("Primary Key", "Cache Key 2");
	const c2 = await airtable.primary.create(r2);

	it("different IDs are cached independently", async () => {
		const get1 = await airtable.primary.get(c1.id);
		const get2 = await airtable.primary.get(c2.id);
		expect((airtable.primary as any)._cache.size).toBe(2);

		const reGet1 = await airtable.primary.get(c1.id);
		const reGet2 = await airtable.primary.get(c2.id);
		expect(reGet1).toBe(get1);
		expect(reGet2).toBe(get2);
	});

	it("cleanup", async () => {
		await airtable.primary.delete(c1.id);
		await airtable.primary.delete(c2.id);
	});
});

describe("Batch Get Caching", async () => {
	const airtable = new Airtable({ cacheSeconds: 60 });

	const r1 = newPrimaryRecord(airtable);
	r1.set("Primary Key", "Batch Cache 1");
	const c1 = await airtable.primary.create(r1);

	const r2 = newPrimaryRecord(airtable);
	r2.set("Primary Key", "Batch Cache 2");
	const c2 = await airtable.primary.create(r2);

	it("batch get is cached separately from single get", async () => {
		// Batch get caches as one entry
		const batch = await airtable.primary.get([c1.id, c2.id]);
		expect((airtable.primary as any)._cache.size).toBe(1);

		// Single get creates a separate cache entry
		const single = await airtable.primary.get(c1.id);
		expect((airtable.primary as any)._cache.size).toBe(2);

		// Both return cached results on re-get
		const batch2 = await airtable.primary.get([c1.id, c2.id]);
		const single2 = await airtable.primary.get(c1.id);
		expect(batch2).toBe(batch);
		expect(single2).toBe(single);
	});

	it("cleanup", async () => {
		await airtable.primary.delete(c1.id);
		await airtable.primary.delete(c2.id);
	});
});

describe("Fields Option Affects Cache Key", async () => {
	const airtable = new Airtable({ cacheSeconds: 60 });

	const newRecord = newPrimaryRecord(airtable);
	newRecord.set("Primary Key", "Fields Cache Test");
	newRecord.set("Single Line Text", "Hello");
	const created = await airtable.primary.create(newRecord);
	const id = created.id;

	it("different fields options produce different cache entries", async () => {
		// Get with no fields filter
		const full = await airtable.primary.get(id);
		expect((airtable.primary as any)._cache.size).toBe(1);

		// Get with fields filter creates a separate cache entry
		const partial = await airtable.primary.get(id, { fields: ["Primary Key"] });
		expect((airtable.primary as any)._cache.size).toBe(2);

		// Both return cached results on re-get
		const full2 = await airtable.primary.get(id);
		const partial2 = await airtable.primary.get(id, { fields: ["Primary Key"] });
		expect(full2).toBe(full);
		expect(partial2).toBe(partial);
	});

	it("cleanup", async () => {
		await airtable.primary.delete(id);
	});
});

describe("Formula Query Caching", async () => {
	const airtable = new Airtable({ cacheSeconds: 60 });

	const newRecord = newPrimaryRecord(airtable);
	newRecord.set("Primary Key", "Formula Cache Test");
	const created = await airtable.primary.create(newRecord);
	const id = created.id;

	it("formula query result is cached", async () => {
		const formula = "{Primary Key} = 'Formula Cache Test'";
		const first = await airtable.primary.get({ formula });
		expect((airtable.primary as any)._cache.size).toBe(1);

		const second = await airtable.primary.get({ formula });
		expect(second).toBe(first);
	});

	it("different formulas produce different cache entries", async () => {
		airtable.primary.invalidateCache();

		const f1 = "{Primary Key} = 'Formula Cache Test'";
		const f2 = "{Primary Key} != 'nonexistent'";
		await airtable.primary.get({ formula: f1 });
		await airtable.primary.get({ formula: f2 });
		expect((airtable.primary as any)._cache.size).toBe(2);
	});

	it("cleanup", async () => {
		await airtable.primary.delete(id);
	});
});
