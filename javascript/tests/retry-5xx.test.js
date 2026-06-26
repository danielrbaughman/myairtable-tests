/**
 * Transient 5xx retry (JavaScript) — the airtable.js gap.
 *
 * airtable.js retries 429 only (base.js: `resp.status === 429`); a 503 SERVICE_UNAVAILABLE surfaces
 * immediately. The generated client adds its own `withRetry` wrapper around airtable.js calls that
 * retries 429 + 5xx with exponential backoff. These are deterministic offline tests: the underlying
 * `_table` is stubbed to throw an airtable.js-shaped error (carrying `.statusCode`) so no live API or
 * real 503 is needed. `delete()` is the simplest wrapped path (no record rehydration).
 */
import { describe, it, expect, vi } from "vitest";
const { Airtable } = require("../output");

// A valid-format record id (rec + 14) so validateRecordIds() passes before the stub is hit.
const ID = "recABCDEFGHIJKLMN";

const err503 = Object.assign(new Error("The service is temporarily unavailable. Please retry shortly."), {
	statusCode: 503,
	error: "SERVICE_UNAVAILABLE",
});
const err422 = Object.assign(new Error("The operation cannot be processed"), {
	statusCode: 422,
	error: "INVALID_VALUE_FOR_COLUMN",
});

// Walk to the class that actually owns RETRY_BASE_MS (the AirtableTable base, not the table
// subclass) so the override is seen by retryDelayMs.
function retryConfigClass(table) {
	let cls = table.constructor;
	while (cls && !Object.prototype.hasOwnProperty.call(cls, "RETRY_BASE_MS")) cls = Object.getPrototypeOf(cls);
	return cls;
}

function offlineTable() {
	const table = new Airtable({ baseId: "appFAKE1234567890", apiKey: "fake" }).primary;
	// Shrink backoff so the retries run effectively instantly.
	retryConfigClass(table).RETRY_BASE_MS = 1;
	return table;
}

describe("transient 5xx retry", () => {
	it("retries a 503 then succeeds", async () => {
		const table = offlineTable();
		let calls = 0;
		table._table = {
			destroy: vi.fn(async () => {
				calls++;
				if (calls < 3) throw err503;
				return [{ id: ID }];
			}),
		};
		await table.delete(ID);
		expect(calls).toBe(3); // 2 failures + 1 success
	});

	it("does not retry a non-transient 422", async () => {
		const table = offlineTable();
		let calls = 0;
		table._table = {
			destroy: vi.fn(async () => {
				calls++;
				throw err422;
			}),
		};
		await expect(table.delete(ID)).rejects.toThrow();
		expect(calls).toBe(1); // no retry
	});

	it("gives up after the max attempts on persistent 503", async () => {
		const table = offlineTable();
		let calls = 0;
		table._table = {
			destroy: vi.fn(async () => {
				calls++;
				throw err503;
			}),
		};
		await expect(table.delete(ID)).rejects.toThrow();
		// initial try + RETRY_MAX_ATTEMPTS retries
		expect(calls).toBe(retryConfigClass(table).RETRY_MAX_ATTEMPTS + 1);
	});
});

// Idempotency policy: create (POST) is non-idempotent. A retried 5xx after a write that may have
// partially succeeded could insert duplicates, so create must NOT retry 5xx — it surfaces immediately
// (exactly 1 attempt). 429 (request rejected, nothing applied) is unaffected by this and still retries
// for create, but that is handled inside airtable.js and not exercised by these offline stubs.
describe("create is non-idempotent: 5xx does not retry", () => {
	it("does not retry a 503 on create (single attempt)", async () => {
		const table = offlineTable();
		let calls = 0;
		table._table = {
			create: vi.fn(async () => {
				calls++;
				throw err503;
			}),
		};
		await expect(table.create({ fields: {} })).rejects.toThrow();
		expect(calls).toBe(1); // no retry — 5xx on a non-idempotent op surfaces immediately
	});

	it("does not retry a 422 on create either (single attempt)", async () => {
		const table = offlineTable();
		let calls = 0;
		table._table = {
			create: vi.fn(async () => {
				calls++;
				throw err422;
			}),
		};
		await expect(table.create({ fields: {} })).rejects.toThrow();
		expect(calls).toBe(1);
	});
});
