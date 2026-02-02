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

describe("Filter by TextField Formula", async () => {
	const newRecords: PrimaryModel[] = [];

	beforeAll(async () => {
		const toCreate = [
			new PrimaryModel({ primaryKey: "TextField Alpha One" }),
			new PrimaryModel({ primaryKey: "TextField Alpha Two" }),
			new PrimaryModel({ primaryKey: "TextField Beta One" }),
		];
		const created = await airtable.primary.create(toCreate);
		newRecords.push(...created);
	});

	it("should filter by equals()", async () => {
		const formula = PrimaryModel.f.primaryKey.equals("TextField Alpha One");
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBe(1);
		expect(records[0].primaryKey).toBe("TextField Alpha One");
	});

	it("should filter by notEquals()", async () => {
		const formula = PrimaryModel.f.primaryKey.notEquals("TextField Alpha One");
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBeGreaterThanOrEqual(2);
		records.forEach((record) => {
			expect(record.primaryKey).not.toBe("TextField Alpha One");
		});
	});

	it("should filter by contains()", async () => {
		const formula = PrimaryModel.f.primaryKey.contains("Alpha");
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBe(2);
		records.forEach((record) => {
			expect(record.primaryKey!.includes("Alpha")).toBe(true);
		});
	});

	it("should filter by containsAny()", async () => {
		const formula = PrimaryModel.f.primaryKey.containsAny(["Alpha", "Beta"]);
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBe(3);
	});

	it("should filter by containsAll()", async () => {
		const formula = PrimaryModel.f.primaryKey.containsAll(["Alpha", "One"]);
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBe(1);
		expect(records[0].primaryKey).toBe("TextField Alpha One");
	});

	it("should filter by notContains()", async () => {
		const formula = PrimaryModel.f.primaryKey.notContains("Alpha");
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBeGreaterThanOrEqual(1);
		records.forEach((record) => {
			expect(record.primaryKey!.includes("Alpha")).toBe(false);
		});
	});

	it("should filter by startsWith()", async () => {
		const formula = PrimaryModel.f.primaryKey.startsWith("TextField Alpha");
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBe(2);
		records.forEach((record) => {
			expect(record.primaryKey!.startsWith("TextField Alpha")).toBe(true);
		});
	});

	it("should filter by notStartsWith()", async () => {
		const formula = PrimaryModel.f.primaryKey.notStartsWith("TextField Alpha");
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBeGreaterThanOrEqual(1);
		records.forEach((record) => {
			expect(record.primaryKey!.startsWith("TextField Alpha")).toBe(false);
		});
	});

	afterAll(async () => {
		const allRecords = await airtable.primary.get(newRecords.map((r) => r.id!));
		await airtable.primary.delete(allRecords.map((r) => r.id!));
	});
});

describe("Filter by NumberField Formula", async () => {
	const newRecords: PrimaryModel[] = [];

	beforeAll(async () => {
		const toCreate = [
			new PrimaryModel({ primaryKey: "NumField Test A", numberInt: 10 }),
			new PrimaryModel({ primaryKey: "NumField Test B", numberInt: 20 }),
			new PrimaryModel({ primaryKey: "NumField Test C", numberInt: 30 }),
		];
		const created = await airtable.primary.create(toCreate);
		newRecords.push(...created);
	});

	it("should filter by equals()", async () => {
		const formula = PrimaryModel.f.numberInt.equals(20);
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBe(1);
		expect(records[0].numberInt).toBe(20);
	});

	it("should filter by notEquals()", async () => {
		const formula = PrimaryModel.f.numberInt.notEquals(20);
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBeGreaterThanOrEqual(2);
		records.forEach((record) => {
			expect(record.numberInt).not.toBe(20);
		});
	});

	it("should filter by greaterThan()", async () => {
		const formula = PrimaryModel.f.numberInt.greaterThan(10);
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBeGreaterThanOrEqual(2);
		records.forEach((record) => {
			expect(record.numberInt).toBeGreaterThan(10);
		});
	});

	it("should filter by lessThan()", async () => {
		const formula = PrimaryModel.f.numberInt.lessThan(30);
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBeGreaterThanOrEqual(2);
		const withValue = records.filter((r) => r.numberInt !== undefined);
		withValue.forEach((record) => {
			expect(record.numberInt).toBeLessThan(30);
		});
	});

	it("should filter by greaterThanOrEquals()", async () => {
		const formula = PrimaryModel.f.numberInt.greaterThanOrEquals(20);
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBeGreaterThanOrEqual(2);
		records.forEach((record) => {
			expect(record.numberInt).toBeGreaterThanOrEqual(20);
		});
	});

	it("should filter by lessThanOrEquals()", async () => {
		const formula = PrimaryModel.f.numberInt.lessThanOrEquals(20);
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBeGreaterThanOrEqual(2);
		const withValue = records.filter((r) => r.numberInt !== undefined);
		withValue.forEach((record) => {
			expect(record.numberInt).toBeLessThanOrEqual(20);
		});
	});

	it("should filter by between() inclusive", async () => {
		const formula = PrimaryModel.f.numberInt.between(10, 30, true);
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBeGreaterThanOrEqual(3);
		records.forEach((record) => {
			expect(record.numberInt).toBeGreaterThanOrEqual(10);
			expect(record.numberInt).toBeLessThanOrEqual(30);
		});
	});

	it("should filter by between() exclusive", async () => {
		const formula = PrimaryModel.f.numberInt.between(10, 30, false);
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBe(1);
		expect(records[0].numberInt).toBe(20);
	});

	afterAll(async () => {
		const allRecords = await airtable.primary.get(newRecords.map((r) => r.id!));
		await airtable.primary.delete(allRecords.map((r) => r.id!));
	});
});

describe("Filter by BooleanField Formula", async () => {
	const newRecords: PrimaryModel[] = [];

	beforeAll(async () => {
		const toCreate = [
			new PrimaryModel({ primaryKey: "BoolField Test A", checkbox: true }),
			new PrimaryModel({ primaryKey: "BoolField Test B", checkbox: false }),
		];
		const created = await airtable.primary.create(toCreate);
		newRecords.push(...created);
	});

	it("should filter by equals(true)", async () => {
		const formula = PrimaryModel.f.checkbox.equals(true);
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBeGreaterThanOrEqual(1);
		records.forEach((record) => {
			expect(record.checkbox).toBe(true);
		});
	});

	it("should filter by equals(false)", async () => {
		const formula = PrimaryModel.f.checkbox.equals(false);
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBeGreaterThanOrEqual(1);
		records.forEach((record) => {
			expect(record.checkbox === false || record.checkbox === undefined).toBe(true);
		});
	});

	it("should filter by true()", async () => {
		const formula = PrimaryModel.f.checkbox.true();
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBeGreaterThanOrEqual(1);
		records.forEach((record) => {
			expect(record.checkbox).toBe(true);
		});
	});

	it("should filter by false()", async () => {
		const formula = PrimaryModel.f.checkbox.false();
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBeGreaterThanOrEqual(1);
		records.forEach((record) => {
			expect(record.checkbox === false || record.checkbox === undefined).toBe(true);
		});
	});

	afterAll(async () => {
		const allRecords = await airtable.primary.get(newRecords.map((r) => r.id!));
		await airtable.primary.delete(allRecords.map((r) => r.id!));
	});
});

describe("Filter by AttachmentsField Formula", async () => {
	const newRecords: PrimaryModel[] = [];

	beforeAll(async () => {
		const toCreate = [
			new PrimaryModel({
				primaryKey: "AttachField Test A",
				attachment: [
					{
						url: "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png",
					},
				],
			}),
			new PrimaryModel({ primaryKey: "AttachField Test B" }),
		];
		const created = await airtable.primary.create(toCreate);
		newRecords.push(...created);
	});

	it("should filter by notEmpty()", async () => {
		const formula = PrimaryModel.f.attachment.notEmpty();
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBeGreaterThanOrEqual(1);
		records.forEach((record) => {
			expect(record.attachment).toBeDefined();
			expect(record.attachment!.length).toBeGreaterThan(0);
		});
	});

	it("should filter by empty()", async () => {
		const formula = PrimaryModel.f.attachment.empty();
		const records = await airtable.primary.get({ formula });
		expect(records.length).toBeGreaterThanOrEqual(1);
		records.forEach((record) => {
			expect(record.attachment === undefined || record.attachment.length === 0).toBe(true);
		});
	});

	afterAll(async () => {
		const allRecords = await airtable.primary.get(newRecords.map((r) => r.id!));
		await airtable.primary.delete(allRecords.map((r) => r.id!));
	});
});
