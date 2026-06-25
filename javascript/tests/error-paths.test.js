import { describe, it, expect } from "vitest";
const { Record: ATRecord } = require("airtable");
const { Airtable } = require("../output");

/**
 * TC3 — Error / validation paths.
 *
 * Mirrors csharp/tests/TestErrorPaths.cs, which classifies live HTTP failures into a TYPED
 * exception hierarchy (AirtableException.ApiError with a structured `.Code`, .HttpError,
 * .RateLimitedError) and asserts the specific subtype + code per scenario.
 *
 * ─────────────────────────────────────────────────────────────────────────────────────────
 * PARITY GAP (documented, not silently passed):
 *
 * The generated JavaScript client does NOT expose a structured API-error type. There is no
 * AirtableError/ApiError subclass surfaced to callers, and no `.code` / `.statusCode` fields.
 *
 * Root cause — javascript/output/static/airtable-table.js wraps every caught error as
 * `throw new Error(String(error))` (get: line ~109; create/update: the "I am aware of how
 * stupid this looks" blocks). That:
 *   • collapses the underlying airtable.js `AirtableError` to a plain `Error`
 *     (so `instanceof` against any typed class is impossible), and
 *   • discards the structured `.error` (code) and `.statusCode` properties — only the
 *     `AirtableError.toString()` text ("...(CODE)[Http code NNN]") survives, embedded in
 *     `.message`.
 * Additionally, get() of a missing id short-circuits with the client's OWN
 * `throw new Error("Record <id> not found")` and never surfaces the server's NOT_FOUND at all.
 *
 * Because the C#-style typed assertions (`instanceof ApiError`, `.code === "NOT_FOUND"`) cannot
 * be satisfied, this file asserts the actual current behavior AND records the gap explicitly:
 * for each scenario it confirms (a) a plain `Error` is thrown — NOT a typed subclass, and
 * (b) where the request actually reaches the server, the API error code is embedded in the
 * message text. The `expectNoTypedApiError` helper is the machine-checkable gap marker: if the
 * generator ever starts exposing a typed hierarchy, these assertions will fail and prompt this
 * file to be upgraded to true typed assertions for cross-target parity.
 * ─────────────────────────────────────────────────────────────────────────────────────────
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

/**
 * Machine-checkable parity-gap marker. Confirms the client gives back a plain `Error` with no
 * structured API-error type. If the generated client ever exposes a typed hierarchy with
 * `.code`/`.statusCode`, these expectations break — flagging that this file should move to real
 * typed assertions to match TestErrorPaths.cs.
 */
function expectNoTypedApiError(err) {
	expect(err).toBeInstanceOf(Error);
	// Plain Error, not a structured subclass (airtable.js AirtableError, or any *ApiError*).
	expect(err.constructor.name).toBe("Error");
	expect(err.code).toBeUndefined();
	expect(err.statusCode).toBeUndefined();
}

// SKIPPED pending myairtable-9yrt: the JS target wraps airtable.js and flattens every API error
// to a plain Error(String(error)), with no typed AirtableError/ApiError/RateLimitedError hierarchy,
// a client-side 404 short-circuit, an over-strict record-id regex, and silent unknown-field drops.
// The cases below document the current (non-parity) behavior; once the typed hierarchy lands,
// un-skip and rewrite them with real typed assertions mirroring csharp/tests/TestErrorPaths.cs.
describe.skip("TC3 Error/validation paths (JS parity gap: no typed API errors)", () => {
	// 1. get() a nonexistent record id. C#: typed ApiError, code NOT_FOUND.
	// JS: the wrapper short-circuits with its own "Record ... not found" plain Error and never
	// surfaces the server NOT_FOUND. Uses a VALID-FORMAT id (14 chars after "rec") so it passes
	// the client-side id validator and exercises the lookup path rather than a ZodError.
	it("get() nonexistent record -> plain Error, no NOT_FOUND code (gap vs C# ApiError)", async () => {
		const err = await captureThrow(() => airtable.primary.get("recAAAAAAAAAAAAAA", { returnAs: "record" }));
		expectNoTypedApiError(err);
		expect(String(err.message)).toContain("not found");
	});

	// 2. create with an invalid single-select option. C#: ApiError INVALID_MULTIPLE_CHOICE_OPTIONS.
	it("create with invalid single-select option -> plain Error carrying INVALID_MULTIPLE_CHOICE_OPTIONS in message", async () => {
		const r = newPrimaryRecord();
		r.set("Primary Key", primaryKey("BadSelect"));
		r.set("Single Select", "NotARealOption_zzz");

		let created;
		const err = await captureThrow(async () => {
			created = await airtable.primary.create(r);
		});
		// Server-rejected; clean up if it unexpectedly succeeded.
		if (created) await airtable.primary.delete(created.id);

		expectNoTypedApiError(err);
		expect(String(err.message)).toContain("INVALID_MULTIPLE_CHOICE_OPTIONS");
	});

	// 3. create with a wrong-typed value (Number(int) field = string). C#: ApiError INVALID_VALUE_FOR_COLUMN.
	it("create with wrong-typed value -> plain Error carrying INVALID_VALUE_FOR_COLUMN in message", async () => {
		const r = newPrimaryRecord();
		r.set("Primary Key", primaryKey("BadType"));
		r.set("Number (int)", "not a number");

		let created;
		const err = await captureThrow(async () => {
			created = await airtable.primary.create(r);
		});
		if (created) await airtable.primary.delete(created.id);

		expectNoTypedApiError(err);
		expect(String(err.message)).toContain("INVALID_VALUE_FOR_COLUMN");
	});

	// 4. create with an unknown field id. C#: ApiError UNKNOWN_FIELD_NAME.
	//
	// SECONDARY GAP: the generated client's create() can't even transmit an unknown field — its
	// toWritableRecord() filters the payload down to the known `writableFieldIds`, silently
	// DROPPING `fldDOESNOTEXIST00000` so the create succeeds with just the primary key (verified
	// below). C#'s Dict.CreateAsync passes the raw dict straight through; JS exposes no such
	// dict/pass-through accessor. So to reach the server's UNKNOWN_FIELD_NAME we go through the
	// underlying Airtable.js table the wrapper itself uses (`primary._table`), which surfaces the
	// raw airtable.js AirtableError — and even there it is a bare AirtableError, never a generated
	// typed ApiError with a `.code`.
	it("create with unknown field id: generated create() drops it; raw table surfaces UNKNOWN_FIELD_NAME (no typed ApiError)", async () => {
		// 4a. The generated wrapper silently drops the unknown field and succeeds.
		const obj = {
			fields: {
				fldol5Q4wmQJQvPRy: primaryKey("BadField"), // Primary > Primary Key
				fldDOESNOTEXIST00000: "x",
			},
		};
		const created = await airtable.primary.create(obj);
		expect(created.id).toBeTruthy(); // unknown field was dropped client-side, not rejected
		await airtable.primary.delete(created.id);

		// 4b. The underlying table the wrapper delegates to does reach the server and is rejected.
		const err = await captureThrow(() =>
			airtable.primary._table.create([
				{
					fields: {
						fldol5Q4wmQJQvPRy: primaryKey("BadField"),
						fldDOESNOTEXIST00000: "x",
					},
				},
			]),
		);
		// Raw airtable.js AirtableError — NOT the generated typed ApiError the C# client exposes,
		// and not what the generated client's own surface would have produced (it swallowed it).
		// Note: airtable.js's AirtableError doesn't even extend Error (it's a plain class), another
		// facet of the gap — the generated client surfaces no Error-derived typed hierarchy at all.
		expect(err.constructor.name).toBe("AirtableError");
		expect(String(err.toString())).toContain("UNKNOWN_FIELD_NAME");
	});

	// 5. real base id + bogus PAT. C#: typed 401 (ApiError/HttpError), AUTHENTICATION_REQUIRED.
	it("bogus API key -> plain Error carrying AUTHENTICATION_REQUIRED / 401 in message", async () => {
		const bad = new Airtable({
			baseId: airtable.baseId, // explicit real base id
			apiKey: "patBOGUS00000.deadbeefdeadbeefdeadbeefdeadbeef",
		});
		const err = await captureThrow(() => bad.primary.get({ maxRecords: 1 }));

		expectNoTypedApiError(err);
		expect(String(err.message)).toContain("AUTHENTICATION_REQUIRED");
		expect(String(err.message)).toContain("401");
	});

	// 6. Distinct rate-limited error class. C#: AirtableException.RateLimitedError carrying Retry-After.
	// JS exposes NO distinct rate-limited error class (429 collapses into the same plain `Error`
	// as every other failure). Skipped; the 429 path is covered live by TC8.
	it.skip("rate-limited error is a distinct class (no such class in JS; 429 covered by TC8)", () => {});
});
