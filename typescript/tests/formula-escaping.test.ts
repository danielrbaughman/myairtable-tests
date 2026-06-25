import { describe, it, expect } from "vitest";
import { Airtable, PrimaryModel, AND } from "../output";

/**
 * TC2 — Formula-value escaping. Filter predicates build formulas by interpolating user values into
 * quoted string literals; a value containing a quote, backslash, or other meta-character must be
 * escaped or the formula breaks (or silently mis-matches). These tests round-trip special-character
 * values through storage AND through the generated `.equals()/.contains()` filter DSL against the
 * live base, proving the escaping is correct. Parity target for the C# suite (TestFormulaEscaping.cs).
 */

const airtable = new Airtable();

// Live API is slow; give each test generous headroom.
const TIMEOUT = 60_000;

// Values that break naive string interpolation into an Airtable formula literal.
// Newline is covered separately (storage only) — Airtable formula string literals can't carry a
// raw newline, so it's a storage concern, not a filter one.
const specialValues = [
	"O'Brien single quote",
	'say "hi" double quote',
	"back\\slash path",
	"trailing backslash\\",
	"mixed a\"b'c\\d",
	"unicode café ☕ 日本語 🎉",
];

/** A primary-key value unique to this run (distinct per file via `suite`/`label`). */
function uniqueKey(suite: string, label: string): string {
	const ts = Date.now();
	const rand = Math.floor(Math.random() * (999999 - 100000) + 100000);
	return `TypeScript ${suite} ${label} ${ts}-${rand}`;
}

async function tryDelete(recordId: string | undefined): Promise<void> {
	if (!recordId) return;
	try {
		await airtable.primary.delete(recordId);
	} catch {
		// best-effort cleanup
	}
}

describe("Formula-value escaping (TC2)", () => {
	describe(".equals() matches a value with special chars", () => {
		for (const special of specialValues) {
			it(
				`eq: ${JSON.stringify(special)}`,
				async () => {
					const suite = uniqueKey("Escaping", "Eq");
					const created = await airtable.primary.create(
						new PrimaryModel({ primaryKey: suite, singleLineText: special }),
					);
					const recordId = created.id!;
					try {
						// 1. Storage round-trips the exact value.
						expect(created.singleLineText).toBe(special);

						// 2. The .equals() filter, scoped to this record, matches via correctly-escaped interpolation.
						const filter = AND(`FIND("${suite}", {Primary Key})`, PrimaryModel.f.singleLineText.equals(special));
						const results = await airtable.primary.get({ formula: filter });
						const ids = results.map((r) => r.id);
						expect(ids).toContain(recordId);
						results.forEach((r) => expect(r.singleLineText).toBe(special));
					} finally {
						await tryDelete(recordId);
					}
				},
				TIMEOUT,
			);
		}
	});

	describe(".contains() matches a value with special chars", () => {
		for (const special of specialValues) {
			it(
				`contains: ${JSON.stringify(special)}`,
				async () => {
					const suite = uniqueKey("Escaping", "Contains");
					// Embed the special token inside a longer value so contains (FIND>0) is a real substring test.
					const stored = "prefix " + special + " suffix";
					const created = await airtable.primary.create(
						new PrimaryModel({ primaryKey: suite, singleLineText: stored }),
					);
					const recordId = created.id!;
					try {
						const filter = AND(`FIND("${suite}", {Primary Key})`, PrimaryModel.f.singleLineText.contains(special));
						const results = await airtable.primary.get({ formula: filter });
						const ids = results.map((r) => r.id);
						expect(ids).toContain(recordId);
					} finally {
						await tryDelete(recordId);
					}
				},
				TIMEOUT,
			);
		}
	});

	it(
		"special characters in the primary key round-trip and filter via .equals()",
		async () => {
			// The primary key itself carries special chars, and we match on it with .equals().
			const suite = uniqueKey("Escaping", "PK");
			const pk = suite + ' O\'Brien "quote" back\\slash';
			const created = await airtable.primary.create(new PrimaryModel({ primaryKey: pk }));
			const recordId = created.id!;
			try {
				expect(created.primaryKey).toBe(pk);
				const results = await airtable.primary.get({ formula: PrimaryModel.f.primaryKey.equals(pk) });
				const ids = results.map((r) => r.id);
				expect(ids).toContain(recordId);
				expect(results.length).toBe(1);
			} finally {
				await tryDelete(recordId);
			}
		},
		TIMEOUT,
	);

	it(
		"newline/tab values round-trip through storage (create -> get)",
		async () => {
			// Newlines/tabs are a storage concern (long text), not expressible in a formula literal.
			// Verify they survive create -> fetch unchanged.
			const suite = uniqueKey("Escaping", "Newline");
			const value = "line1\nline2\twith tab\r\nwindows";
			const created = await airtable.primary.create(new PrimaryModel({ primaryKey: suite, longText: value }));
			const recordId = created.id!;
			try {
				const fetched = await airtable.primary.get(recordId);
				expect(fetched.longText).toBe(value);
			} finally {
				await tryDelete(recordId);
			}
		},
		TIMEOUT,
	);
});
