import { describe, it, expect } from "vitest";
const { Airtable, PrimaryModel } = require("../output");

const airtable = new Airtable();

// 105 > the 100-record default page size, so a no-maxRecords list spans two pages.
const COUNT = 105;

/** A primary-key prefix unique to this run (distinct per scenario via `label`). */
function uniqueSuite(label) {
	const ts = Date.now();
	const rand = Math.floor(Math.random() * (999999 - 100000) + 100000);
	return `JavaScript Pagination ${label} ${ts}-${rand}`;
}

/** Best-effort cleanup: delete by id, then sweep any stragglers by the suite prefix. */
async function tryDeleteMany(ids, suite) {
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

// Each scenario creates 105 records over the live API, which comfortably
// exceeds the 30s default; give the offset-walk + cleanup room to breathe.
const TIMEOUT_MS = 120_000;

describe("TC1 — Multi-page pagination", () => {
	it(
		"typed model accessor: a no-maxRecords get spanning multiple pages returns every record",
		async () => {
			const suite = uniqueSuite("Orm");
			const models = Array.from({ length: COUNT }, (_, i) => new PrimaryModel({ primaryKey: `${suite} ${i + 1}` }));
			let createdIds = [];
			try {
				const created = await airtable.primary.create(models);
				createdIds = created.map((m) => m.id);
				expect(created.length).toBe(COUNT);

				// No maxRecords: the offset loop must walk both pages and return all 105.
				const results = await airtable.primary.get({
					formula: `FIND("${suite}", {Primary Key})`,
				});
				expect(results.length).toBe(COUNT);
				expect(new Set(results.map((m) => m.id))).toEqual(new Set(createdIds));
			} finally {
				await tryDeleteMany(createdIds, suite);
			}
		},
		TIMEOUT_MS,
	);

	it(
		"record accessor: a no-maxRecords get spanning multiple pages returns every record",
		async () => {
			const suite = uniqueSuite("Record");
			const { Record: ATRecord } = require("airtable");
			const records = Array.from({ length: COUNT }, (_, i) => {
				const r = new ATRecord(airtable.primary._table, "", {});
				r.set("Primary Key", `${suite} ${i + 1}`);
				return r;
			});
			let createdIds = [];
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
		},
		TIMEOUT_MS,
	);
});

// TC5 — explicit page size. 25 records over pageSize 10 spans 3 pages (10+10+5).
const PAGE_SIZE_COUNT = 25;

describe("TC5 — explicit page size", () => {
	it(
		"pageSize 10, no maxRecords: the offset loop walks all 3 pages and returns every record",
		async () => {
			const suite = uniqueSuite("PageSize");
			const models = Array.from(
				{ length: PAGE_SIZE_COUNT },
				(_, i) => new PrimaryModel({ primaryKey: `${suite} ${i + 1}` }),
			);
			let createdIds = [];
			try {
				const created = await airtable.primary.create(models);
				createdIds = created.map((m) => m.id);
				expect(created.length).toBe(PAGE_SIZE_COUNT);

				// pageSize 10, NO maxRecords: the offset loop must walk all 3 pages and return all 25.
				const results = await airtable.primary.get({
					formula: `FIND("${suite}", {Primary Key})`,
					pageSize: 10,
				});
				expect(results.length).toBe(PAGE_SIZE_COUNT);
			} finally {
				await tryDeleteMany(createdIds, suite);
			}
		},
		TIMEOUT_MS,
	);

	it(
		"pageSize 10 + maxRecords 15: maxRecords caps the total mid-stream",
		async () => {
			const suite = uniqueSuite("PageCap");
			const models = Array.from(
				{ length: PAGE_SIZE_COUNT },
				(_, i) => new PrimaryModel({ primaryKey: `${suite} ${i + 1}` }),
			);
			let createdIds = [];
			try {
				const created = await airtable.primary.create(models);
				createdIds = created.map((m) => m.id);
				expect(created.length).toBe(PAGE_SIZE_COUNT);

				// pageSize 10 + maxRecords 15: maxRecords caps the total (not a page multiple).
				const results = await airtable.primary.get({
					formula: `FIND("${suite}", {Primary Key})`,
					pageSize: 10,
					maxRecords: 15,
				});
				expect(results.length).toBe(15);
			} finally {
				await tryDeleteMany(createdIds, suite);
			}
		},
		TIMEOUT_MS,
	);
});
