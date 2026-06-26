/**
 * typecast option (JavaScript) — deterministic / offline.
 *
 * Airtable's write API accepts a per-request `typecast` boolean (POST/PATCH body). The generated
 * client exposes it on the PUBLIC create/update/upsert methods, threaded into the request body.
 * Default MUST remain false (omitted) so existing behavior is unchanged.
 *
 * These tests are hermetic: for create/update the underlying airtable.js `_table` is stubbed to
 * capture the options argument the client passes through (airtable.js spreads it into the request
 * body — see node_modules/airtable/lib/table.js `_createRecords`/`_updateRecords`). For the
 * server-side merge-upsert path, global `fetch` is stubbed to capture the raw PATCH body.
 */
import { describe, it, expect, vi, afterEach } from "vitest";
const { Airtable } = require("../output");

function offlineTable() {
	return new Airtable({ baseId: "appFAKE1234567890", apiKey: "fake" }).primary;
}

afterEach(() => {
	vi.restoreAllMocks();
});

describe("create typecast", () => {
	it("passes { typecast: true } to airtable.js when opted in", async () => {
		const table = offlineTable();
		let captured;
		table._table = {
			create: vi.fn(async (_records, options) => {
				captured = options;
				return [{ id: "recABCDEFGHIJKLMN", fields: {} }];
			}),
		};
		await table.create({ fields: {} }, { typecast: true });
		expect(captured).toEqual({ typecast: true });
	});

	it("omits typecast by default", async () => {
		const table = offlineTable();
		let captured = "sentinel";
		table._table = {
			create: vi.fn(async (_records, options) => {
				captured = options;
				return [{ id: "recABCDEFGHIJKLMN", fields: {} }];
			}),
		};
		await table.create({ fields: {} });
		expect(captured).toBeUndefined();
	});

	it("omits typecast when explicitly false", async () => {
		const table = offlineTable();
		let captured = "sentinel";
		table._table = {
			create: vi.fn(async (_records, options) => {
				captured = options;
				return [{ id: "recABCDEFGHIJKLMN", fields: {} }];
			}),
		};
		await table.create({ fields: {} }, { typecast: false });
		expect(captured).toBeUndefined();
	});
});

describe("update typecast", () => {
	it("passes { typecast: true } to airtable.js when opted in", async () => {
		const table = offlineTable();
		let captured;
		table._table = {
			update: vi.fn(async (_records, options) => {
				captured = options;
				return [{ id: "recABCDEFGHIJKLMN", fields: {} }];
			}),
		};
		await table.update({ id: "recABCDEFGHIJKLMN", fields: {} }, { typecast: true });
		expect(captured).toEqual({ typecast: true });
	});

	it("omits typecast by default", async () => {
		const table = offlineTable();
		let captured = "sentinel";
		table._table = {
			update: vi.fn(async (_records, options) => {
				captured = options;
				return [{ id: "recABCDEFGHIJKLMN", fields: {} }];
			}),
		};
		await table.update({ id: "recABCDEFGHIJKLMN", fields: {} });
		expect(captured).toBeUndefined();
	});
});

describe("merge-upsert typecast (raw PATCH body)", () => {
	function stubFetch() {
		const calls = [];
		const fetchMock = vi.fn(async (_url, init) => {
			calls.push(JSON.parse(init.body));
			return {
				ok: true,
				status: 200,
				json: async () => ({ records: [{ id: "recABCDEFGHIJKLMN" }] }),
			};
		});
		vi.stubGlobal("fetch", fetchMock);
		return () => calls[0];
	}

	it("sets typecast: true in the PATCH body when opted in", async () => {
		const table = offlineTable();
		const lastBody = stubFetch();
		// Avoid the follow-up get() rehydration touching the network: stub the select pipeline.
		table._table = {
			select: () => ({ all: async () => [{ id: "recABCDEFGHIJKLMN", fields: {} }] }),
		};
		await table.upsert(
			{ id: "recABCDEFGHIJKLMN", fields: { fld0BL2lFo9fqcKv3: "x" } },
			{ fieldsToMergeOn: ["fld0BL2lFo9fqcKv3"], typecast: true },
		);
		expect(lastBody().typecast).toBe(true);
	});

	it("defaults typecast: false in the PATCH body", async () => {
		const table = offlineTable();
		const lastBody = stubFetch();
		table._table = {
			select: () => ({ all: async () => [{ id: "recABCDEFGHIJKLMN", fields: {} }] }),
		};
		await table.upsert(
			{ id: "recABCDEFGHIJKLMN", fields: { fld0BL2lFo9fqcKv3: "x" } },
			{ fieldsToMergeOn: ["fld0BL2lFo9fqcKv3"] },
		);
		expect(lastBody().typecast).toBe(false);
	});
});

describe("id-based upsert threads typecast to create/update", () => {
	it("passes typecast through to the underlying create for a new record", async () => {
		const table = offlineTable();
		let captured = "sentinel";
		table._table = {
			// get() for the existence check returns nothing -> the record is a create.
			select: () => ({ all: async () => [] }),
			create: vi.fn(async (_records, options) => {
				captured = options;
				return [{ id: "recABCDEFGHIJKLMN", fields: {} }];
			}),
		};
		await table.upsert({ fields: {} }, { typecast: true });
		expect(captured).toEqual({ typecast: true });
	});
});
