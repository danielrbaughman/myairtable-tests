import { describe, it, expect } from "vitest";
import { Record as ATRecord } from "airtable";
import { Airtable, PrimaryFieldSet, ApiError, HttpError, AirtableError } from "../output";

/**
 * TC3 — Error / validation paths (TYPED).
 *
 * Mirrors csharp/tests/TestErrorPaths.cs now that the generated TypeScript client exposes a typed
 * error hierarchy (myairtable-9yrt): `AirtableError` base with `ApiError` (structured `.code` /
 * `.statusCode` / `.apiMessage`), `HttpError`, `RateLimitedError`, etc. The wrapper classifies HTTP
 * failures instead of flattening them to `new Error(String(error))`.
 *
 * These cases are LIVE — they hit the base and require credentials, so they run in CI, not locally.
 *
 * Two documented divergences from the C# parity target remain, by design:
 *   - Scenario 1 uses a well-formed-but-nonexistent id ("rec" + 14 chars) so it passes the client-side
 *     record-id regex; get-by-id is a filterByFormula query, so a missing record yields zero rows and
 *     the client synthesizes ApiError("NOT_FOUND", 404). (A malformed id is a separate ZodError case.)
 *   - Scenario 4 (unknown field id): the typed writable-field allowlist drops the unknown field
 *     CLIENT-SIDE, so the create SUCCEEDS. This is an intentional client-side-validation divergence,
 *     NOT a regression — the allowlist is deliberately not weakened.
 */

const airtable = new Airtable();

function newPrimaryRecord(): ATRecord<PrimaryFieldSet> {
	return new ATRecord<PrimaryFieldSet>(airtable.primary._table, "", {});
}

describe("TC3 — Error / validation paths", () => {
	// ── Scenario 1: get() a well-formed but nonexistent record id → typed NOT_FOUND ─────────────
	it("get(nonexistent id) throws ApiError NOT_FOUND", async () => {
		const err = await airtable.primary.get("rec00000000000000").then(
			() => {
				throw new Error("expected get() to reject");
			},
			(e) => e,
		);
		expect(err).toBeInstanceOf(ApiError);
		expect(err).toBeInstanceOf(AirtableError);
		expect((err as ApiError).code).toBe("NOT_FOUND");
	}, 30_000);

	// ── Scenario 1b: malformed record id → client-side ZodError (separate case) ──────────────────
	it("get(malformed id) throws a client-side validation error (ZodError, not an ApiError)", async () => {
		const err = await airtable.primary.get("recDOESNOTEXIST0001").then(
			() => {
				throw new Error("expected get() to reject");
			},
			(e) => e,
		);
		expect(err).toBeInstanceOf(Error);
		expect((err as Error).constructor.name).toBe("ZodError");
	}, 30_000);

	// ── Scenario 2: create with an invalid single-select option → INVALID_MULTIPLE_CHOICE_OPTIONS ─
	it("create(invalid select option) throws ApiError INVALID_MULTIPLE_CHOICE_OPTIONS (422)", async () => {
		const r = newPrimaryRecord();
		r.set("Primary Key", `TS Error BadSelect ${Date.now()}`);
		r.set("Single Select", "NotARealOption_zzz" as any);

		const err = await airtable.primary.create(r).then(
			(created) => {
				return airtable.primary.delete((created as any).id).then(() => {
					throw new Error("expected create() to reject");
				});
			},
			(e) => e,
		);

		expect(err).toBeInstanceOf(ApiError);
		expect((err as ApiError).code).toBe("INVALID_MULTIPLE_CHOICE_OPTIONS");
		expect((err as ApiError).statusCode).toBe(422);
	}, 30_000);

	// ── Scenario 3: create with a wrong-typed value → INVALID_VALUE_FOR_COLUMN ───────────────────
	it("create(wrong-typed value) throws ApiError INVALID_VALUE_FOR_COLUMN (422)", async () => {
		const r = newPrimaryRecord();
		r.set("Primary Key", `TS Error BadType ${Date.now()}`);
		r.set("Number (int)", "not a number" as any);

		const err = await airtable.primary.create(r).then(
			(created) => {
				return airtable.primary.delete((created as any).id).then(() => {
					throw new Error("expected create() to reject");
				});
			},
			(e) => e,
		);

		expect(err).toBeInstanceOf(ApiError);
		expect((err as ApiError).code).toBe("INVALID_VALUE_FOR_COLUMN");
		expect((err as ApiError).statusCode).toBe(422);
	}, 30_000);

	// ── Scenario 4: create with an unknown field id → SUCCEEDS (documented divergence) ───────────
	//
	// The typed writable-field allowlist (toWritableRecord keeps only known writable field ids) drops
	// the unknown field id CLIENT-SIDE, so the server never sees it and the create succeeds. This is an
	// intentional client-side-validation divergence from C#'s UNKNOWN_FIELD_NAME (which passes the raw
	// dict straight through), NOT a regression. The allowlist is deliberately not weakened.
	it("create(unknown field id) SUCCEEDS — unknown field dropped client-side (intentional divergence)", async () => {
		const r = new ATRecord<any>(airtable.primary._table, "", {});
		r.set("Primary Key", `TS Error BadField ${Date.now()}`);
		r.set("fldDOESNOTEXIST00000", "x");

		const created = await airtable.primary.create(r);
		const id = (created as any).id as string;
		expect(id).toBeTruthy();
		await airtable.primary.delete(id);
	}, 30_000);

	// ── Scenario 5: real base id + bogus API key → typed 401 ─────────────────────────────────────
	//
	// `new Airtable({ apiKey })` reads the real baseId from env (AIRTABLE_BASE_ID), so this targets auth
	// (401) against the real base. Accept either a structured ApiError(code) or a raw HttpError(401).
	it("bad API key throws a typed 401 (ApiError/HttpError)", async () => {
		const bad = new Airtable({ apiKey: "patBOGUS00000.deadbeefdeadbeefdeadbeefdeadbeef" });

		const err = await bad.primary.get({ maxRecords: 1 }).then(
			() => {
				throw new Error("expected get() to reject");
			},
			(e) => e,
		);

		expect(err).toBeInstanceOf(AirtableError);
		expect(err instanceof ApiError || err instanceof HttpError).toBe(true);
		expect((err as ApiError | HttpError).statusCode).toBe(401);
		if (err instanceof ApiError && err.code) {
			expect(err.code).toBe("AUTHENTICATION_REQUIRED");
		}
	}, 30_000);

	// ── Scenario 6: distinct rate-limited error class ────────────────────────────────────────────
	//
	// A typed `RateLimitedError` (carrying `.retryAfter`) now exists in the generated output. The live
	// 429 path is exercised by TC8 / the retry-5xx suite; no need to force a live 429 here.
});
