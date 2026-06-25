import { describe, it, expect } from "vitest";
import { Airtable, PrimaryModel, AND } from "../output";

/**
 * TC11 — Multi-field sort + sort combined with a filter. The base filter suite only covers a
 * single-field sort. This verifies a two-field sort (primary key with ties broken by a secondary
 * key) and sorting within a filtered scope. Parity target for the other 8 suites.
 */
const airtable = new Airtable();

function row(suite: string, number: number, text: string): PrimaryModel {
	return new PrimaryModel({
		primaryKey: `${suite} ${text}`,
		numberInt: number,
		singleLineText: text,
	});
}

const scopeTo = (ids: string[]) => PrimaryModel.f.id.inList(ids);

const texts = (records: PrimaryModel[]) => records.map((r) => r.singleLineText!);

async function tryDeleteMany(ids: string[]) {
	if (ids.length === 0) return;
	try {
		await airtable.primary.delete(ids);
	} catch {
		// best-effort cleanup
	}
}

describe("TC11 — Multi-field sort", () => {
	it("two-field sort breaks ties on the secondary key", { timeout: 30000 }, async () => {
		const suite = "Sort TwoField";
		// NumberInt ties at 10 (rows "b" and "a"); the secondary SingleLineText sort orders them.
		const created = await airtable.primary.create([row(suite, 10, "b"), row(suite, 10, "a"), row(suite, 20, "c")]);
		const ids = created.map((r) => r.id!);
		try {
			const results = await airtable.primary.get({
				formula: scopeTo(ids),
				sort: [
					{ field: "Number (int)", direction: "asc" },
					{ field: "Single Line Text", direction: "asc" },
				],
			});
			// (10,a), (10,b), (20,c) — tie on 10 broken by text asc.
			expect(texts(results)).toEqual(["a", "b", "c"]);
		} finally {
			await tryDeleteMany(ids);
		}
	});

	it("secondary descending reverses the tied group", { timeout: 30000 }, async () => {
		const suite = "Sort MixedDir";
		const created = await airtable.primary.create([row(suite, 10, "a"), row(suite, 10, "b"), row(suite, 20, "c")]);
		const ids = created.map((r) => r.id!);
		try {
			const results = await airtable.primary.get({
				formula: scopeTo(ids),
				sort: [
					{ field: "Number (int)", direction: "asc" },
					{ field: "Single Line Text", direction: "desc" },
				],
			});
			// NumberInt asc, then text DESC within the 10-tie: (10,b), (10,a), (20,c).
			expect(texts(results)).toEqual(["b", "a", "c"]);
		} finally {
			await tryDeleteMany(ids);
		}
	});

	it("sort combined with a filter", { timeout: 30000 }, async () => {
		const suite = "Sort WithFilter";
		const created = await airtable.primary.create([
			row(suite, 30, "x"),
			row(suite, 10, "y"),
			row(suite, 20, "z"),
			row(suite, 5, "low"), // filtered out by NumberInt > 5
		]);
		const ids = created.map((r) => r.id!);
		try {
			const formula = AND(scopeTo(ids), PrimaryModel.f.numberInt.greaterThan(5));
			const results = await airtable.primary.get({
				formula,
				sort: [{ field: "Number (int)", direction: "asc" }],
			});
			// Filtered to NumberInt > 5, sorted asc: 10(y), 20(z), 30(x).
			expect(texts(results)).toEqual(["y", "z", "x"]);
		} finally {
			await tryDeleteMany(ids);
		}
	});
});
