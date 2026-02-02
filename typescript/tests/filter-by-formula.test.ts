import { describe, it, expect, beforeAll, afterAll } from "vitest";
import { Airtable, PrimaryModel } from "../output";

const airtable = new Airtable();

describe("Filter by View", async () => {
	const newRecords: PrimaryModel[] = [];

	beforeAll(async () => {
		const toCreate: PrimaryModel[] = [];
		for (let i = 0; i < 5; i++) {
			toCreate.push(
				new PrimaryModel({
					primaryKey: `Filter Test ${i}`,
				}),
			);
		}
		for (let i = 0; i < 5; i++) {
			toCreate.push(
				new PrimaryModel({
					primaryKey: `Don't Include Test ${i}`,
				}),
			);
		}
		const created = await airtable.primary.create(toCreate);
		newRecords.push(...created);
	});

	it("should only return records in the specified view", async () => {
		const records = await airtable.primary.get({ view: "Filter by View" });
		expect(records.length).toBe(5);
		records.forEach((record) => {
			expect(record.primaryKey!.startsWith("Filter Test")).toBe(true);
		});
	});

	afterAll(async () => {
		const allRecords = await airtable.primary.get(newRecords.map((r) => r.id!));
		await airtable.primary.delete(allRecords.map((r) => r.id!));
	});
});

describe("Filter by ID Formula", async () => {
	const newRecords: PrimaryModel[] = [];

	beforeAll(async () => {
		const toCreate: PrimaryModel[] = [];
		for (let i = 0; i < 3; i++) {
			toCreate.push(
				new PrimaryModel({
					primaryKey: `ID Formula Test ${i}`,
				}),
			);
		}
		const created = await airtable.primary.create(toCreate);
		newRecords.push(...created);
	});

	it("should filter by ID.equals()", async () => {
		const formula = PrimaryModel.f.id.equals(newRecords[0].id!);
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBe(1);
		expect(records[0].id).toBe(newRecords[0].id);
	});

	it("should filter by ID.inList() with multiple IDs", async () => {
		const formula = PrimaryModel.f.id.inList([newRecords[0].id!, newRecords[1].id!]);
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBe(2);
		const ids = records.map((r) => r.id);
		expect(ids).toContain(newRecords[0].id);
		expect(ids).toContain(newRecords[1].id);
	});

	it("should filter by ID.inList() with a single ID", async () => {
		const formula = PrimaryModel.f.id.inList([newRecords[0].id!]);
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBe(1);
		expect(records[0].id).toBe(newRecords[0].id);
	});

	it("should return no records for ID.inList() with empty array", async () => {
		const formula = PrimaryModel.f.id.inList([]);
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBe(0);
	});

	afterAll(async () => {
		const allRecords = await airtable.primary.get(newRecords.map((r) => r.id!));
		await airtable.primary.delete(allRecords.map((r) => r.id!));
	});
});
