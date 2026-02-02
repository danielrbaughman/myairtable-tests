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
