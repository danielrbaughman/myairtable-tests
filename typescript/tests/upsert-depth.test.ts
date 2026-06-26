/**
 * TC10 - Upsert depth (TypeScript).
 *
 * Mirrors the C# `TestUpsertDepth` / Python / Rust suites: a multi-field merge key (a match must
 * agree on ALL merge fields), the multiple-match error path (a merge key matching more than one
 * record is rejected with 422), and a batched merge-upsert. Exercises the generated
 * `airtable.primary.upsert(model, { fieldsToMergeOn })` (Airtable's server-side performUpsert).
 * Insert-vs-update is inferred from the returned record id (upsert returns the upserted model(s),
 * not a wasCreated flag). As elsewhere in the generated client, API errors surface as a generic
 * `Error` whose `.message` carries the 422 status (see error-paths.test.ts).
 */
import { describe, it, expect, afterAll } from "vitest";
import { Airtable, PrimaryModel } from "../output";

// Field IDs for the Primary table (analogue of the C# PrimaryFields.*Id merge-key constants).
const PRIMARY_KEY_ID = "fldol5Q4wmQJQvPRy";
const SINGLE_LINE_TEXT_ID = "fld0BL2lFo9fqcKv3";

const airtable = new Airtable();
const SUITE = `TsUpsert ${Date.now()}`;
const createdIds: string[] = [];

afterAll(async () => {
	if (createdIds.length > 0) await airtable.primary.delete(createdIds).catch(() => {});
});

describe("TC10 upsert depth", () => {
	it("matches on multiple merge fields (update vs insert by the returned id)", async () => {
		const suite = `${SUITE} MultiKey`;
		const mergeOn = [PRIMARY_KEY_ID, SINGLE_LINE_TEXT_ID];

		// Seed a record identified by the (Primary Key, Single Line Text) pair.
		const seed = await airtable.primary.create(new PrimaryModel({ primaryKey: suite, singleLineText: "anchor" }));
		createdIds.push(seed.id!);

		// Same pair -> UPDATE the seed (matched on BOTH fields).
		const updated = await airtable.primary.upsert(
			new PrimaryModel({ primaryKey: suite, singleLineText: "anchor", longText: "updated" }),
			{ fieldsToMergeOn: mergeOn },
		);
		expect(updated.id).toBe(seed.id);
		expect(updated.longText).toBe("updated");

		// Same Primary Key but a DIFFERENT Single Line Text -> no match on the pair -> INSERT.
		const inserted = await airtable.primary.upsert(
			new PrimaryModel({ primaryKey: suite, singleLineText: "different" }),
			{ fieldsToMergeOn: mergeOn },
		);
		createdIds.push(inserted.id!);
		expect(inserted.id).not.toBe(seed.id);
	});

	it("rejects a merge key that matches multiple records (422)", async () => {
		const suite = `${SUITE} MultiMatch`;

		// Two records share the same Single Line Text value.
		const created = await airtable.primary.create([
			new PrimaryModel({ primaryKey: `${suite} A`, singleLineText: "dupe" }),
			new PrimaryModel({ primaryKey: `${suite} B`, singleLineText: "dupe" }),
		]);
		createdIds.push(...created.map((r) => r.id!));

		// Merging only on Single Line Text="dupe" matches BOTH -> Airtable rejects it.
		let err: unknown;
		try {
			await airtable.primary.upsert(
				new PrimaryModel({ primaryKey: "ignored", singleLineText: "dupe", longText: "x" }),
				{
					fieldsToMergeOn: [SINGLE_LINE_TEXT_ID],
				},
			);
		} catch (e) {
			err = e;
		}
		expect(err).toBeInstanceOf(Error);
		expect((err as Error).message).toContain("422");
	});

	it("batches inserts and updates in one merge-upsert", async () => {
		const suite = `${SUITE} Batch`;
		const mergeOn = [PRIMARY_KEY_ID, SINGLE_LINE_TEXT_ID];

		// Seed one record; the batch will UPDATE it and INSERT a second.
		const seed = await airtable.primary.create(new PrimaryModel({ primaryKey: suite, singleLineText: "keep" }));
		createdIds.push(seed.id!);

		const out = await airtable.primary.upsert(
			[
				new PrimaryModel({ primaryKey: suite, singleLineText: "keep", longText: "batched" }),
				new PrimaryModel({ primaryKey: suite, singleLineText: "fresh" }),
			],
			{ fieldsToMergeOn: mergeOn },
		);
		expect(out).toHaveLength(2);

		const updated = out.find((r) => r.singleLineText === "keep")!;
		expect(updated.id).toBe(seed.id);
		expect(updated.longText).toBe("batched");

		const fresh = out.find((r) => r.singleLineText === "fresh")!;
		expect(fresh.id).not.toBe(seed.id);
		createdIds.push(fresh.id!);
	});
});
