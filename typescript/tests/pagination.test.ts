import { describe, it, expect } from "vitest";
import { Record as ATRecord } from "airtable";
import { Airtable, PrimaryModel, PrimaryFieldSet } from "../output";

const airtable = new Airtable();

// 105 > the 100-record default page size, so a no-maxRecords list spans two pages.
const COUNT = 105;

/** A primary-key prefix unique to this run (distinct per scenario via `label`). */
function suiteKey(label: string): string {
	const ts = Date.now();
	const rand = Math.floor(Math.random() * (999999 - 100000) + 100000);
	return `TypeScript Pagination ${label} ${ts}-${rand}`;
}

function newPrimaryRecord(): ATRecord<PrimaryFieldSet> {
	return new ATRecord<PrimaryFieldSet>(airtable.primary._table, "", {});
}

/**
 * TC1 — Multi-page pagination: list more records than Airtable's 100-record default page in a
 * single query (no maxRecords) and assert the client's offset loop reassembles every page.
 * Exercises the untested page-reassembly path in the generated client. Parity target for the
 * other 8 suites.
 */
describe("TC1 — Multi-page pagination", () => {
	it("model accessor: a no-maxRecords query spanning multiple pages returns every record", async () => {
		const suite = suiteKey("Model");
		const models = Array.from({ length: COUNT }, (_, i) => new PrimaryModel({ primaryKey: `${suite} ${i + 1}` }));
		let createdIds: string[] = [];
		try {
			const created = await airtable.primary.create(models);
			createdIds = created.map((m) => m.id!);
			expect(created.length).toBe(COUNT);

			// No maxRecords: the offset loop must walk both pages and return all 105.
			const results = await airtable.primary.get({ formula: `FIND("${suite}", {Primary Key})` });
			expect(results.length).toBe(COUNT);
			expect(new Set(results.map((m) => m.id!))).toEqual(new Set(createdIds));
		} finally {
			await tryDeleteMany(createdIds, suite);
		}
	});

	it("record accessor: a no-maxRecords query spanning multiple pages returns every record", async () => {
		const suite = suiteKey("Record");
		const records = Array.from({ length: COUNT }, (_, i) => {
			const r = newPrimaryRecord();
			r.set("Primary Key", `${suite} ${i + 1}`);
			return r;
		});
		let createdIds: string[] = [];
		try {
			const created = await airtable.primary.create(records);
			createdIds = created.map((r) => r.id);
			expect(created.length).toBe(COUNT);

			const results = await airtable.primary.get({
				formula: `FIND("${suite}", {Primary Key})`,
				returnAs: "record",
			});
			expect(results.length).toBe(COUNT);
			expect(new Set(results.map((r) => r.id))).toEqual(new Set(createdIds));
		} finally {
			await tryDeleteMany(createdIds, suite);
		}
	});
});

// ---- cleanup ----

async function tryDeleteMany(ids: string[], suite: string): Promise<void> {
	if (ids.length > 0) {
		try {
			await airtable.primary.delete(ids);
			return;
		} catch {
			// fall through to a prefix sweep
		}
	}
	try {
		const stray = await airtable.primary.get({
			formula: `FIND("${suite}", {Primary Key})`,
			returnAs: "record",
		});
		if (stray.length > 0) {
			await airtable.primary.delete(stray.map((r) => r.id));
		}
	} catch {
		// best-effort cleanup
	}
}
