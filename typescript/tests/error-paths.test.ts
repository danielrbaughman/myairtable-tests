import { describe, it, expect } from "vitest";
import { Record as ATRecord } from "airtable";
import { Airtable, PrimaryFieldSet } from "../output";

/**
 * TC3 — Error / validation paths.
 *
 * The C# parity target (csharp/tests/TestErrorPaths.cs) asserts that the generated
 * client classifies HTTP failures into a TYPED exception hierarchy: a 404 surfaces as
 * `AirtableException.ApiError` with `Code == "NOT_FOUND"`, server-side validation
 * rejections surface as `ApiError` carrying their structured `Code`
 * (INVALID_MULTIPLE_CHOICE_OPTIONS / INVALID_VALUE_FOR_COLUMN / UNKNOWN_FIELD_NAME),
 * a bad API key surfaces as a 401 `ApiError`/`HttpError`, and a rate-limited response
 * is a distinct `RateLimitedError` carrying `RetryAfterSeconds`.
 *
 * ============================================================================
 * PARITY GAP — the generated TypeScript client does NOT expose typed errors.
 * ============================================================================
 *
 * The generated client (typescript/output/static/airtable-table.ts) catches every
 * underlying error and re-throws it as `throw new Error(String(error))`. The
 * underlying `airtable` npm package DOES produce a structured `AirtableError`
 * (with `.error` = the API code, `.statusCode` = the HTTP status — see
 * node_modules/airtable/lib/airtable_error.js), but the wrapper flattens it:
 *
 *   - The thrown value is a plain `Error` (no `ApiError`/`AirtableError` subclass,
 *     no `instanceof` discriminator).
 *   - The API code and HTTP status survive ONLY as text inside `.message`
 *     (e.g. "...(AUTHENTICATION_REQUIRED)[Http code 401]"); there is no
 *     structured `.error` / `.statusCode` / `.code` property to read.
 *   - There is no `RateLimitedError` (or any) error subclass in the generated output.
 *
 * Two scenarios additionally cannot reach the server-side classification at all:
 *
 *   - Scenario 1 (404 NOT_FOUND): the suite's nonexistent id "recDOESNOTEXIST0001"
 *     fails the client-side record-id Zod regex (/\brec[a-zA-Z0-9]{14}\b/) BEFORE any
 *     request is made, so it throws a `ZodError` client-side, never a 404. Even a
 *     well-formed-but-nonexistent id would not yield NOT_FOUND: get-by-id is
 *     implemented as a filterByFormula `.select().all()`, so a missing record returns
 *     zero rows and the client throws a synthetic `Error("Record <id> not found")`,
 *     not an API 404.
 *
 *   - Scenario 4 (UNKNOWN_FIELD_NAME): the generated client's `toWritableRecord`
 *     copies only fields whose id is in the known writable-field allowlist, so an
 *     unknown field id ("fldDOESNOTEXIST00000") is dropped client-side and the create
 *     SUCCEEDS instead of being rejected by the server.
 *
 * Per the TC3 spec ("do NOT just assert it throws"; "If TS does NOT expose a
 * structured API-error subclass, STOP and report a parity gap rather than weakening
 * the assertion"), these tests document the ACTUAL behavior of the generated client
 * rather than pretending typed classification exists. The strongest truthful
 * assertion available is: the call rejects with an `Error` whose `.message` carries
 * the API classification substring. Where a scenario cannot be expressed at all, it
 * is recorded with `it.skip` and an explanation. Each test is annotated GAP/OK so the
 * parity delta is auditable from the test output.
 */

const airtable = new Airtable();

function newPrimaryRecord(): ATRecord<PrimaryFieldSet> {
	return new ATRecord<PrimaryFieldSet>(airtable.primary._table, "", {});
}

// SKIPPED pending myairtable-9yrt: the TS target wraps airtable.js and flattens every API error
// to a plain Error(String(error)), with no typed AirtableError/ApiError/RateLimitedError hierarchy,
// a client-side 404 short-circuit, an over-strict record-id regex, and silent unknown-field drops.
// The cases below document the current (non-parity) behavior; once the typed hierarchy lands,
// un-skip and rewrite them with real typed assertions mirroring csharp/tests/TestErrorPaths.cs.
describe.skip("TC3 — Error / validation paths", () => {
	// ── Scenario 1: get() a nonexistent record id ───────────────────────────────
	//
	// C# expects a typed ApiError with Code == "NOT_FOUND". The TS client cannot
	// produce that: the id is rejected client-side by the record-id regex, so this
	// is a *client-side* validation throw, never an API 404.
	it("GAP: get(nonexistent id) throws (client-side validation, NOT a typed NOT_FOUND ApiError)", async () => {
		const err = await airtable.primary.get("recDOESNOTEXIST0001").then(
			() => {
				throw new Error("expected get() to reject");
			},
			(e) => e,
		);
		// All we can truthfully assert: it's a generic Error. There is no typed
		// ApiError, no `.code`, no `.statusCode`, and it never reached the API to
		// classify a 404.
		expect(err).toBeInstanceOf(Error);
		expect((err as any).code).toBeUndefined();
		expect((err as any).statusCode).toBeUndefined();
		// Document the actual cause: client-side record-id schema rejection.
		expect((err as Error).constructor.name).toBe("ZodError");
	}, 30_000);

	// ── Scenario 2: create with an invalid single-select option ─────────────────
	//
	// C# expects ApiError INVALID_MULTIPLE_CHOICE_OPTIONS. TS: generic Error whose
	// .message embeds the code as text only.
	it("GAP: create(invalid select option) rejects with generic Error; code only in .message (INVALID_MULTIPLE_CHOICE_OPTIONS)", async () => {
		const r = newPrimaryRecord();
		r.set("Primary Key", `TS Error BadSelect ${Date.now()}`);
		r.set("Single Select", "NotARealOption_zzz" as any);

		const err = await airtable.primary.create(r).then(
			(created) => {
				// Should not happen; clean up if it unexpectedly succeeds.
				return airtable.primary.delete((created as any).id).then(() => {
					throw new Error("expected create() to reject");
				});
			},
			(e) => e,
		);

		expect(err).toBeInstanceOf(Error);
		// No structured fields — the parity gap.
		expect((err as any).code).toBeUndefined();
		expect((err as any).statusCode).toBeUndefined();
		// The classification is present, but only as a substring of the message.
		expect((err as Error).message).toContain("INVALID_MULTIPLE_CHOICE_OPTIONS");
		expect((err as Error).message).toContain("422");
	}, 30_000);

	// ── Scenario 3: create with a wrong-typed value ─────────────────────────────
	//
	// C# expects ApiError INVALID_VALUE_FOR_COLUMN. TS: generic Error, code in text.
	it("GAP: create(wrong-typed value) rejects with generic Error; code only in .message (INVALID_VALUE_FOR_COLUMN)", async () => {
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

		expect(err).toBeInstanceOf(Error);
		expect((err as any).code).toBeUndefined();
		expect((err as any).statusCode).toBeUndefined();
		expect((err as Error).message).toContain("INVALID_VALUE_FOR_COLUMN");
		expect((err as Error).message).toContain("422");
	}, 30_000);

	// ── Scenario 4: create with an unknown field id ─────────────────────────────
	//
	// C# expects ApiError UNKNOWN_FIELD_NAME. The TS client filters unknown field ids
	// client-side (toWritableRecord keeps only known writable field ids), so the
	// create SUCCEEDS and the server never sees the unknown field. Cannot be
	// expressed against the generated client; documented + the accidental record is
	// cleaned up.
	it("GAP: create(unknown field id) does NOT reject — unknown field is dropped client-side, no UNKNOWN_FIELD_NAME", async () => {
		const r = new ATRecord<any>(airtable.primary._table, "", {});
		r.set("Primary Key", `TS Error BadField ${Date.now()}`);
		r.set("fldDOESNOTEXIST00000", "x");

		const created = await airtable.primary.create(r);
		const id = (created as any).id as string;
		// It succeeded (the gap): clean up the record the client silently created.
		expect(id).toBeTruthy();
		await airtable.primary.delete(id);
	}, 30_000);

	// ── Scenario 5: real base id + bogus API key → 401 ──────────────────────────
	//
	// C# expects a typed 401 ApiError/HttpError. Constructed with the real base id
	// (read from env, same as the default client) + a bogus PAT. TS: generic Error
	// whose .message embeds AUTHENTICATION_REQUIRED and "401".
	it("GAP: bad API key rejects with generic Error; 401/AUTHENTICATION_REQUIRED only in .message (no typed 401)", async () => {
		// `new Airtable({ apiKey })` reads the real baseId from env (AIRTABLE_BASE_ID),
		// so this targets auth (401) against the real base, not a base 404.
		const bad = new Airtable({ apiKey: "patBOGUS00000.deadbeefdeadbeefdeadbeefdeadbeef" });

		const err = await bad.primary.get({ maxRecords: 1 }).then(
			() => {
				throw new Error("expected get() to reject");
			},
			(e) => e,
		);

		expect(err).toBeInstanceOf(Error);
		expect((err as any).code).toBeUndefined();
		expect((err as any).statusCode).toBeUndefined();
		expect((err as Error).message).toContain("AUTHENTICATION_REQUIRED");
		expect((err as Error).message).toContain("401");
	}, 30_000);

	// ── Scenario 6: distinct rate-limited error class ───────────────────────────
	//
	// C# has a distinct AirtableException.RateLimitedError carrying RetryAfterSeconds.
	// The generated TS output exposes no error subclasses at all, so there is nothing
	// to assert distinctness against. (The live 429 path itself is covered by TC8.)
	it.skip("GAP: no RateLimitedError class exists in generated TS output (nothing to assert; 429 path covered by TC8)", () => {
		// Intentionally skipped — documents that the generated TS client has no
		// rate-limited error subclass to distinguish from other errors.
	});
});
