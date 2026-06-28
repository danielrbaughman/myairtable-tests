/**
 * typecast option (TypeScript) — deterministic / offline.
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
import { Airtable } from "../output";

function offlineTable() {
	return new Airtable({ baseId: "appFAKE1234567890", apiKey: "fake" }).primary;
}

afterEach(() => {
	vi.restoreAllMocks();
});

describe("create typecast", () => {
	it("passes { typecast: true } to airtable.js when opted in", async () => {
		const table = offlineTable();
		let captured: unknown;
		(table as any)._table = {
			create: vi.fn(async (_records: unknown, options: unknown) => {
				captured = options;
				return [{ id: "recABCDEFGHIJKLMN", fields: {} }];
			}),
		};
		await table.create({ fields: {} } as any, { typecast: true });
		expect(captured).toEqual({ typecast: true });
	});

	it("omits typecast by default", async () => {
		const table = offlineTable();
		let captured: unknown = "sentinel";
		(table as any)._table = {
			create: vi.fn(async (_records: unknown, options: unknown) => {
				captured = options;
				return [{ id: "recABCDEFGHIJKLMN", fields: {} }];
			}),
		};
		await table.create({ fields: {} } as any);
		expect(captured).toBeUndefined();
	});

	it("omits typecast when explicitly false", async () => {
		const table = offlineTable();
		let captured: unknown = "sentinel";
		(table as any)._table = {
			create: vi.fn(async (_records: unknown, options: unknown) => {
				captured = options;
				return [{ id: "recABCDEFGHIJKLMN", fields: {} }];
			}),
		};
		await table.create({ fields: {} } as any, { typecast: false });
		expect(captured).toBeUndefined();
	});
});

describe("update typecast", () => {
	it("passes { typecast: true } to airtable.js when opted in", async () => {
		const table = offlineTable();
		let captured: unknown;
		(table as any)._table = {
			update: vi.fn(async (_records: unknown, options: unknown) => {
				captured = options;
				return [{ id: "recABCDEFGHIJKLMN", fields: {} }];
			}),
		};
		await table.update({ id: "recABCDEFGHIJKLMN", fields: {} } as any, { typecast: true });
		expect(captured).toEqual({ typecast: true });
	});

	it("omits typecast by default", async () => {
		const table = offlineTable();
		let captured: unknown = "sentinel";
		(table as any)._table = {
			update: vi.fn(async (_records: unknown, options: unknown) => {
				captured = options;
				return [{ id: "recABCDEFGHIJKLMN", fields: {} }];
			}),
		};
		await table.update({ id: "recABCDEFGHIJKLMN", fields: {} } as any);
		expect(captured).toBeUndefined();
	});
});

describe("merge-upsert typecast (raw PATCH body)", () => {
	function stubFetch(): () => any {
		const calls: any[] = [];
		const fetchMock = vi.fn(async (_url: string, init: any) => {
			calls.push(JSON.parse(init.body));
			return {
				ok: true,
				status: 200,
				json: async () => ({ records: [{ id: "recABCDEFGHIJKLMN" }] }),
			} as any;
		});
		vi.stubGlobal("fetch", fetchMock);
		return () => calls[0];
	}

	it("sets typecast: true in the PATCH body when opted in", async () => {
		const table = offlineTable();
		const lastBody = stubFetch();
		// Avoid the follow-up get() rehydration touching the network: stub the select pipeline.
		(table as any)._table = {
			select: () => ({ all: async () => [{ id: "recABCDEFGHIJKLMN", fields: {} }] }),
		};
		await table.upsert({ id: "recABCDEFGHIJKLMN", fields: { fld0BL2lFo9fqcKv3: "x" } } as any, {
			fieldsToMergeOn: ["fld0BL2lFo9fqcKv3"],
			typecast: true,
		});
		expect(lastBody().typecast).toBe(true);
	});

	it("defaults typecast: false in the PATCH body", async () => {
		const table = offlineTable();
		const lastBody = stubFetch();
		(table as any)._table = {
			select: () => ({ all: async () => [{ id: "recABCDEFGHIJKLMN", fields: {} }] }),
		};
		await table.upsert({ id: "recABCDEFGHIJKLMN", fields: { fld0BL2lFo9fqcKv3: "x" } } as any, {
			fieldsToMergeOn: ["fld0BL2lFo9fqcKv3"],
		});
		expect(lastBody().typecast).toBe(false);
	});
});

describe("id-based upsert threads typecast to create/update", () => {
	it("passes typecast through to the underlying create for a new record", async () => {
		const table = offlineTable();
		let captured: unknown = "sentinel";
		(table as any)._table = {
			// get() for the existence check returns nothing -> the record is a create.
			select: () => ({ all: async () => [] }),
			create: vi.fn(async (_records: unknown, options: unknown) => {
				captured = options;
				return [{ id: "recABCDEFGHIJKLMN", fields: {} }];
			}),
		};
		await table.upsert({ fields: {} } as any, { typecast: true });
		expect(captured).toEqual({ typecast: true });
	});
});
