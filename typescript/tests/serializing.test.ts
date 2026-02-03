import { describe, it, expect } from "vitest";
import { PrimaryModel, SecondaryModel } from "../output";

describe("Simple Fields — toJson, toRecord, toIRecord", async () => {
	const newRecord = new PrimaryModel({
		primaryKey: "Serialize Simple Test",
		singleLineText: "Hello World",
		longText: "Long text content",
		longTextWithRichText: "Rich text content",
		email: "test@example.com",
		url: "https://example.com",
		phoneNumber: "555-1234",
		checkbox: true,
		numberInt: 42,
		numberFloat: 3.14,
		currencyInt: 10,
		currencyFloat: 9.99,
		percentInt: 0.5,
		percentFloat: 0.333,
		duration: 3600,
		rating: 3,
		date: "2025-01-15",
		dateWithTime: "2025-01-15T10:00:00.000Z",
		singleSelect: "Choice 1",
		multipleSelect: ["Option 1", "Option 2"],
	});

	await newRecord.save();

	describe("toJson()", () => {
		const json = newRecord.toJson();

		it("should have id", () => {
			expect(json.id).toBe(newRecord.id);
		});

		it("should have all simple property values", () => {
			expect(json.primaryKey).toBe("Serialize Simple Test");
			expect(json.singleLineText).toBe("Hello World");
			expect(json.longText).toBe("Long text content");
			expect(json.longTextWithRichText).toBe("Rich text content");
			expect(json.email).toBe("test@example.com");
			expect(json.url).toBe("https://example.com");
			expect(json.phoneNumber).toBe("555-1234");
			expect(json.checkbox).toBe(true);
			expect(json.numberInt).toBe(42);
			expect(json.numberFloat).toBe(3.14);
			expect(json.currencyInt).toBe(10);
			expect(json.currencyFloat).toBe(9.99);
			expect(json.percentInt).toBe(0.5);
			expect(json.percentFloat).toBe(0.333);
			expect(json.duration).toBe(3600);
			expect(json.rating).toBe(3);
			expect(json.date).toBe("2025-01-15");
			expect(json.dateWithTime).toBe("2025-01-15T10:00:00.000Z");
			expect(json.singleSelect).toBe("Choice 1");
			expect(json.multipleSelect).toEqual(["Option 1", "Option 2"]);
		});

		it("should have linked record fields as raw IDs (not LinkedRecord objects)", () => {
			// linkSingle and linkMultiple were not set, so they should be undefined/empty
			expect(json.linkSingle).toBeUndefined();
			expect(json.linkMultiple).toBeUndefined();
		});
	});

	describe("toRecord()", () => {
		it("should return a record with matching id", () => {
			const r = newRecord.toRecord();
			expect(r.id).toBe(newRecord.id);
		});

		it("should use field names as keys by default", () => {
			const r = newRecord.toRecord();
			expect(r.fields["Primary Key"]).toBe("Serialize Simple Test");
			expect(r.fields["Single Line Text"]).toBe("Hello World");
			expect(r.fields["Checkbox"]).toBe(true);
			expect(r.fields["Number (int)"]).toBe(42);
			expect(r.fields["Date"]).toBe("2025-01-15");
			expect(r.fields["Single Select"]).toBe("Choice 1");
			expect(r.fields["Multiple Select"]).toEqual(["Option 1", "Option 2"]);
		});

		it("should use field IDs as keys when useFieldIds=true", () => {
			const r = newRecord.toRecord(true);
			// "Primary Key" field ID is "fldol5Q4wmQJQvPRy"
			expect((r.fields as any)["fldol5Q4wmQJQvPRy"]).toBe("Serialize Simple Test");
			// "Single Line Text" field ID is "fld0BL2lFo9fqcKv3"
			expect((r.fields as any)["fld0BL2lFo9fqcKv3"]).toBe("Hello World");
			// "Checkbox" field ID is "fldjQIaAZVegb1FUa"
			expect((r.fields as any)["fldjQIaAZVegb1FUa"]).toBe(true);
		});
	});

	describe("toIRecord()", () => {
		it("should return an object with only id and fields", () => {
			const ir = newRecord.toIRecord();
			expect(ir.id).toBe(newRecord.id);
			expect(ir.fields).toBeDefined();
			// Should only have id and fields (no ATRecord methods)
			const keys = Object.keys(ir);
			expect(keys).toContain("id");
			expect(keys).toContain("fields");
			expect(keys.length).toBe(2);
		});

		it("should use field names as keys by default", () => {
			const ir = newRecord.toIRecord();
			expect(ir.fields["Primary Key"]).toBe("Serialize Simple Test");
			expect(ir.fields["Single Line Text"]).toBe("Hello World");
			expect(ir.fields["Number (int)"]).toBe(42);
		});

		it("should use field IDs as keys when useFieldIds=true", () => {
			const ir = newRecord.toIRecord(true);
			expect((ir.fields as any)["fldol5Q4wmQJQvPRy"]).toBe("Serialize Simple Test");
			expect((ir.fields as any)["fld0BL2lFo9fqcKv3"]).toBe("Hello World");
		});
	});

	describe("Cleanup", () => {
		it("should delete the record", async () => {
			await newRecord.delete();
		});
	});
});

describe("Linked Records — toJson, toRecord, toIRecord", async () => {
	const sec1 = new SecondaryModel({ name: "Ser Link 1", value: "val1" });
	await sec1.save();
	const sec2 = new SecondaryModel({ name: "Ser Link 2", value: "val2" });
	await sec2.save();
	const sec1Id = sec1.id!;
	const sec2Id = sec2.id!;

	const newRecord = new PrimaryModel({
		primaryKey: "Serialize Link Test",
		linkSingle: sec1Id,
		linkMultiple: [sec1Id, sec2Id],
	});
	await newRecord.save();

	describe("toJson()", () => {
		const json = newRecord.toJson();

		it("should have linkSingle as a single record ID string", () => {
			expect(json.linkSingle).toBe(sec1Id);
		});

		it("should have linkMultiple as an array of record ID strings", () => {
			expect(json.linkMultiple).toEqual([sec1Id, sec2Id]);
		});
	});

	describe("toRecord()", () => {
		it("should have linked fields as arrays of record IDs with field name keys", () => {
			const r = newRecord.toRecord();
			// In Airtable's native format, single link is [id], multiple is [id1, id2]
			expect(r.fields["Link (single)"]).toEqual([sec1Id]);
			expect(r.fields["Link (multiple)"]).toEqual([sec1Id, sec2Id]);
		});
	});

	describe("toIRecord()", () => {
		it("should have linked fields as arrays of record IDs with { id, fields } shape", () => {
			const ir = newRecord.toIRecord();
			expect(ir.fields["Link (single)"]).toEqual([sec1Id]);
			expect(ir.fields["Link (multiple)"]).toEqual([sec1Id, sec2Id]);
			// Confirm only id and fields
			const keys = Object.keys(ir);
			expect(keys).toContain("id");
			expect(keys).toContain("fields");
			expect(keys.length).toBe(2);
		});
	});

	describe("Cleanup", () => {
		it("should clean up records", async () => {
			await newRecord.delete();
			await sec1.delete();
			await sec2.delete();
		});
	});
});

describe("Unfetched model", async () => {
	describe("toJson() on a new model before saving", () => {
		it("should work and return the local data", () => {
			const m = new PrimaryModel({ primaryKey: "Unsaved Model" });
			const json = m.toJson();
			expect(json.primaryKey).toBe("Unsaved Model");
			expect(json.id).toBeUndefined();
		});
	});

	describe("toRecord() on a new model before saving", () => {
		it("should work since the constructor creates the internal record", () => {
			const m = new PrimaryModel({ primaryKey: "Unsaved Model" });
			const r = m.toRecord();
			expect(r.fields["Primary Key"]).toBe("Unsaved Model");
		});
	});

	describe("toRecord() on fromId without fetch", async () => {
		// Create a real record to get a valid ID
		const temp = new PrimaryModel({ primaryKey: "FromId No Fetch" });
		await temp.save();
		const tempId = temp.id!;

		it("should work but fields will be empty (no field data fetched)", () => {
			const m = PrimaryModel.fromId(tempId);
			const r = m.toRecord();
			expect(r.id).toBe(tempId);
			// Fields should be empty since we never called fetch()
			expect(r.fields["Primary Key"]).toBeUndefined();
		});

		it("cleanup", async () => {
			await temp.delete();
		});
	});
});
