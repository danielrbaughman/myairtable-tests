import { describe, it, expect } from "vitest";
const { Record: ATRecord } = require("airtable");
const { Airtable, ApiError, HttpError, AirtableError } = require("../output");

/**
 * TC3 — Error / validation paths (TYPED) — JavaScript twin.
 *
 * Mirrors csharp/tests/TestErrorPaths.cs now that the generated JavaScript client exposes a typed
 * error hierarchy (myairtable-9yrt): `AirtableError` base with `ApiError` (structured `.code` /
 * `.statusCode` / `.apiMessage`), `HttpError`, `RateLimitedError`, etc. The wrapper classifies HTTP
 * failures instead of flattening them to `new Error(String(error))`.
 *
 * These cases are LIVE — they hit the base and require credentials, so they run in CI, not locally.
 *
 * Two documented divergences from the C# parity target remain, by design:
 *   - Scenario 1 uses a well-formed-but-nonexistent id ("rec" + 14 chars) so it passes the client-side
 *     record-id regex; get-by-id is a filterByFormula query, so a missing record yields zero rows and
 *     the client synthesizes ApiError("NOT_FOUND", 404).
 *   - Scenario 4 (unknown field id): the typed writable-field allowlist drops the unknown field
 *     CLIENT-SIDE, so the create SUCCEEDS. Intentional client-side-validation divergence, NOT a
 *     regression — the allowlist is deliberately not weakened.
 */

const airtable = new Airtable();

function newPrimaryRecord() {
	return new ATRecord(airtable.primary._table, "", {});
}

function primaryKey(label) {
	return `ErrJS ${label} ${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
}

/** Run an async op, returning the thrown error (fails the test if it does not throw). */
async function captureThrow(op) {
	try {
		await op();
	} catch (e) {
		return e;
	}
	throw new Error("expected the operation to throw, but it resolved");
}

describe("TC3 — Error / validation paths (typed)", () => {
	// 1. get() a well-formed but nonexistent record id → typed ApiError NOT_FOUND.
	it("get(nonexistent id) throws ApiError NOT_FOUND", async () => {
		const err = await captureThrow(() => airtable.primary.get("rec00000000000000", { returnAs: "record" }));
		expect(err).toBeInstanceOf(ApiError);
		expect(err).toBeInstanceOf(AirtableError);
		expect(err.code).toBe("NOT_FOUND");
	});

	// 2. create with an invalid single-select option → ApiError INVALID_MULTIPLE_CHOICE_OPTIONS.
	it("create(invalid select option) throws ApiError INVALID_MULTIPLE_CHOICE_OPTIONS (422)", async () => {
		const r = newPrimaryRecord();
		r.set("Primary Key", primaryKey("BadSelect"));
		r.set("Single Select", "NotARealOption_zzz");

		let created;
		const err = await captureThrow(async () => {
			created = await airtable.primary.create(r);
		});
		if (created) await airtable.primary.delete(created.id);

		expect(err).toBeInstanceOf(ApiError);
		expect(err.code).toBe("INVALID_MULTIPLE_CHOICE_OPTIONS");
		expect(err.statusCode).toBe(422);
	});

	// 3. create with a wrong-typed value (Number(int) field = string) → ApiError INVALID_VALUE_FOR_COLUMN.
	it("create(wrong-typed value) throws ApiError INVALID_VALUE_FOR_COLUMN (422)", async () => {
		const r = newPrimaryRecord();
		r.set("Primary Key", primaryKey("BadType"));
		r.set("Number (int)", "not a number");

		let created;
		const err = await captureThrow(async () => {
			created = await airtable.primary.create(r);
		});
		if (created) await airtable.primary.delete(created.id);

		expect(err).toBeInstanceOf(ApiError);
		expect(err.code).toBe("INVALID_VALUE_FOR_COLUMN");
		expect(err.statusCode).toBe(422);
	});

	// 4. create with an unknown field id → SUCCEEDS (documented divergence).
	//
	// The typed writable-field allowlist (toWritableRecord keeps only known writable field ids) drops
	// the unknown field id CLIENT-SIDE, so the server never sees it and the create succeeds. This is an
	// intentional client-side-validation divergence from C#'s UNKNOWN_FIELD_NAME, NOT a regression — the
	// allowlist is deliberately not weakened.
	it("create(unknown field id) SUCCEEDS — unknown field dropped client-side (intentional divergence)", async () => {
		const obj = {
			fields: {
				fldol5Q4wmQJQvPRy: primaryKey("BadField"), // Primary > Primary Key
				fldDOESNOTEXIST00000: "x",
			},
		};
		const created = await airtable.primary.create(obj);
		expect(created.id).toBeTruthy(); // unknown field was dropped client-side, not rejected
		await airtable.primary.delete(created.id);
	});

	// 5. real base id + bogus PAT → typed 401 (ApiError/HttpError), AUTHENTICATION_REQUIRED.
	it("bad API key throws a typed 401 (ApiError/HttpError)", async () => {
		const bad = new Airtable({
			baseId: airtable.baseId, // explicit real base id
			apiKey: "patBOGUS00000.deadbeefdeadbeefdeadbeefdeadbeef",
		});
		const err = await captureThrow(() => bad.primary.get({ maxRecords: 1 }));

		expect(err).toBeInstanceOf(AirtableError);
		expect(err instanceof ApiError || err instanceof HttpError).toBe(true);
		expect(err.statusCode).toBe(401);
		if (err instanceof ApiError && err.code) {
			expect(err.code).toBe("AUTHENTICATION_REQUIRED");
		}
	});

	// 6. Distinct rate-limited error class: a typed `RateLimitedError` (carrying `.retryAfter`) now
	// exists in the generated output. The live 429 path is covered by TC8 / the retry-5xx suite.
});
