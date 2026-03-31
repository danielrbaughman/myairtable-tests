import { describe, it, expect } from "vitest";
import { Airtable, PrimaryModel, SecondaryModel, TertiaryModel } from "../output";
import { LinkedRecord } from "../output/static/linked-record";

const airtable = new Airtable();

describe("Linked Record Chaining", async () => {
	// Setup: create Tertiary → Secondary → Primary chain
	const tert = await airtable.tertiary.create(new TertiaryModel({ name: "Chain Tert", value: "tval" }));
	const sec = await airtable.secondary.create(
		new SecondaryModel({ name: "Chain Sec", value: "sval", linkToTertiary: [tert.id!] }),
	);
	const prim = await airtable.primary.create(
		new PrimaryModel({
			primaryKey: "Chain Primary",
			linkSingle: sec.id!,
			linkMultiple: [sec.id!],
		}),
	);
	// A primary with no links for null safety tests
	const primEmpty = await airtable.primary.create(new PrimaryModel({ primaryKey: "Chain Empty" }));

	// Re-fetch to get fully initialized models with proxy wrapping
	const primary = await airtable.primary.get(prim.id!);
	const primaryEmpty = await airtable.primary.get(primEmpty.id!);

	describe("Thenable single", () => {
		it("await linkSingle resolves to the linked model", async () => {
			const result = await primary.linkSingle;
			expect(result).toBeDefined();
			expect(result!.name).toBe("Chain Sec");
			expect(result!.value).toBe("sval");
		});
	});

	describe("Thenable plural", () => {
		it("await linkMultiple resolves to an array of models", async () => {
			const results = await primary.linkMultiple;
			expect(results).toHaveLength(1);
			expect(results[0].name).toBe("Chain Sec");
		});
	});

	describe("Chain single → plural", () => {
		it("await linkSingle.linkToTertiary resolves through the chain", async () => {
			const results = await primary.linkSingle.linkToTertiary;
			expect(results).toHaveLength(1);
			expect(results[0].name).toBe("Chain Tert");
			expect(results[0].value).toBe("tval");
		});
	});

	describe("Null safety", () => {
		it("await linkSingle with no link returns undefined", async () => {
			const result = await primaryEmpty.linkSingle;
			expect(result).toBeUndefined();
		});

		it("await linkSingle.linkToTertiary with no linkSingle returns empty array", async () => {
			const result = await primaryEmpty.linkSingle.linkToTertiary;
			expect(result).toEqual([]);
		});
	});

	describe("Existing API preserved", () => {
		it(".id returns the RecordId", () => {
			expect(primary.linkSingle.id).toBe(sec.id);
		});

		it(".get() still works", async () => {
			const result = await primary.linkSingle.get();
			expect(result).toBeDefined();
			expect(result!.name).toBe("Chain Sec");
		});

		it("instanceof LinkedRecord is true", () => {
			expect(primary.linkSingle instanceof LinkedRecord).toBe(true);
		});
	});

	describe("Serialization safe", () => {
		it("toJson() correctly serializes linked record fields", () => {
			const json = primary.toJson();
			expect(json.linkSingle).toBe(sec.id);
			expect(json.linkMultiple).toEqual([sec.id]);
		});
	});

	describe("Direct assignment setters", async () => {
		const sec2 = await airtable.secondary.create(new SecondaryModel({ name: "Setter Sec", value: "sv2" }));

		describe("Single link — assign model", async () => {
			const r = await airtable.primary.get(prim.id!);
			r.linkSingle = sec2;
			const updated = await airtable.primary.update(r);

			it("should update to the new linked record", () => {
				expect(updated.linkSingle.id).toBe(sec2.id);
			});
		});

		describe("Single link — assign RecordId", async () => {
			const r = await airtable.primary.get(prim.id!);
			r.linkSingle = sec.id!;
			const updated = await airtable.primary.update(r);

			it("should update to the original linked record", () => {
				expect(updated.linkSingle.id).toBe(sec.id);
			});
		});

		describe("Multiple link — assign model array", async () => {
			const r = await airtable.primary.get(prim.id!);
			r.linkMultiple = [sec, sec2];
			const updated = await airtable.primary.update(r);

			it("should have both linked records", () => {
				expect(updated.linkMultiple.ids).toEqual([sec.id, sec2.id]);
			});
		});

		describe("Multiple link — assign RecordId array", async () => {
			const r = await airtable.primary.get(prim.id!);
			r.linkMultiple = [sec.id!];
			const updated = await airtable.primary.update(r);

			it("should have one linked record", () => {
				expect(updated.linkMultiple.ids).toEqual([sec.id]);
			});
		});

		describe("Setter cleanup", () => {
			it("should clean up sec2", async () => {
				await airtable.secondary.delete(sec2.id!);
			});
		});
	});

	describe("Cleanup", () => {
		it("should clean up test records", async () => {
			await airtable.primary.delete(prim.id!);
			await airtable.primary.delete(primEmpty.id!);
			await airtable.secondary.delete(sec.id!);
			await airtable.tertiary.delete(tert.id!);
		});
	});
});
