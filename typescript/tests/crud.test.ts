import { describe, it, expect } from "vitest";
import { Airtable, PrimaryModel } from "../output";

const airtable = new Airtable({ apiKey: process.env.AIRTABLE_API_KEY, baseId: process.env.AIRTABLE_BASE_ID });

describe("Basic CRUD", async () => {
	const newRecord = new PrimaryModel({ primaryKey: "New Primary Key" });
	let id: string;

	describe("Create", async () => {
		const createdRecord = await airtable.primary.create(newRecord);
		id = createdRecord.id!;

		it("should have a valid id", async () => {
			expect(createdRecord.id).toBeTruthy();
		});

		it("should have valid values", async () => {
			expect(createdRecord.primaryKey).toBe("New Primary Key");
		});
	});

	describe("Read", async () => {
		const readRecord = await airtable.primary.get(id);

		it("should have the expected values", async () => {
			expect(readRecord.id).toBe(id);
			expect(readRecord.primaryKey).toBe("New Primary Key");
		});
	});

	describe("Update", async () => {
		const r = await airtable.primary.get(id);
		r.primaryKey = "Updated Primary Key";
		const updatedRecord = await airtable.primary.update(r);

		it("should have the updated values", async () => {
			expect(updatedRecord.id).toBe(id);
			expect(updatedRecord.primaryKey).toBe("Updated Primary Key");
		});
	});

	describe("Delete", async () => {
		await airtable.primary.delete(id);
		let deleted = false;
		try {
			await airtable.primary.get(id);
		} catch {
			deleted = true;
		}

		it("should be deleted", async () => {
			expect(deleted).toBe(true);
		});
	});
});
