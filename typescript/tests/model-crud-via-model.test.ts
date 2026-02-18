import { describe, it, expect } from "vitest";
import { Airtable, PrimaryModel, SecondaryModel, TertiaryModel } from "../output";

const airtable = new Airtable();

describe("Primary Key Only", async () => {
	const newRecord = new PrimaryModel({ primaryKey: "New Primary Key" });

	describe("Create", async () => {
		await newRecord.save();

		it("should have a valid id", async () => {
			expect(newRecord.id).toBeTruthy();
		});

		it("should have valid values", async () => {
			expect(newRecord.primaryKey).toBe("New Primary Key");
		});
	});

	describe("Read", async () => {
		const readRecord = PrimaryModel.fromId(newRecord.id);
		await readRecord.fetch();

		it("should have the expected values", async () => {
			expect(readRecord.id).toBe(newRecord.id);
			expect(readRecord.primaryKey).toBe("New Primary Key");
		});
	});

	describe("Update", async () => {
		const r = PrimaryModel.fromId(newRecord.id);
		await r.fetch();
		r.primaryKey = "Updated Primary Key";
		await r.save();

		it("should have the updated values", async () => {
			expect(r.id).toBe(newRecord.id);
			expect(r.primaryKey).toBe("Updated Primary Key");
		});
	});

	describe("Delete", () => {
		it("should be deleted", async () => {
			const r = PrimaryModel.fromId(newRecord.id);
			await r.fetch();
			await r.delete();
			let deleted = false;
			try {
				const check = PrimaryModel.fromId(newRecord.id);
				await check.fetch();
			} catch {
				deleted = true;
			}
			expect(deleted).toBe(true);
		});
	});
});

describe("All Simple Properties", async () => {
	const newRecord = new PrimaryModel({
		primaryKey: "All Props Key",
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

	describe("Create", async () => {
		await newRecord.save();

		it("should have a valid id", async () => {
			expect(newRecord.id).toBeTruthy();
		});

		it("should have valid values", async () => {
			expect(newRecord.primaryKey).toBe("All Props Key");
			expect(newRecord.singleLineText).toBe("Hello World");
			expect(newRecord.longText).toBe("Long text content");
			expect(newRecord.longTextWithRichText).toBe("Rich text content");
			expect(newRecord.email).toBe("test@example.com");
			expect(newRecord.url).toBe("https://example.com");
			expect(newRecord.phoneNumber).toBe("555-1234");
			expect(newRecord.checkbox).toBe(true);
			expect(newRecord.numberInt).toBe(42);
			expect(newRecord.numberFloat).toBe(3.14);
			expect(newRecord.currencyInt).toBe(10);
			expect(newRecord.currencyFloat).toBe(9.99);
			expect(newRecord.percentInt).toBe(0.5);
			expect(newRecord.percentFloat).toBe(0.333);
			expect(newRecord.duration).toBe(3600);
			expect(newRecord.rating).toBe(3);
			expect(newRecord.date).toBe("2025-01-15");
			expect(newRecord.dateWithTime).toBe("2025-01-15T10:00:00.000Z");
			expect(newRecord.singleSelect).toBe("Choice 1");
			expect(newRecord.multipleSelect).toEqual(["Option 1", "Option 2"]);
		});
	});

	describe("Read", async () => {
		const readRecord = PrimaryModel.fromId(newRecord.id);
		await readRecord.fetch();

		it("should have the expected values", async () => {
			expect(readRecord.id).toBe(newRecord.id);
			expect(readRecord.primaryKey).toBe("All Props Key");
			expect(readRecord.singleLineText).toBe("Hello World");
			expect(readRecord.longText).toBe("Long text content");
			expect(readRecord.longTextWithRichText).toBe("Rich text content");
			expect(readRecord.email).toBe("test@example.com");
			expect(readRecord.url).toBe("https://example.com");
			expect(readRecord.phoneNumber).toBe("555-1234");
			expect(readRecord.checkbox).toBe(true);
			expect(readRecord.numberInt).toBe(42);
			expect(readRecord.numberFloat).toBe(3.14);
			expect(readRecord.currencyInt).toBe(10);
			expect(readRecord.currencyFloat).toBe(9.99);
			expect(readRecord.percentInt).toBe(0.5);
			expect(readRecord.percentFloat).toBe(0.333);
			expect(readRecord.duration).toBe(3600);
			expect(readRecord.rating).toBe(3);
			expect(readRecord.date).toBe("2025-01-15");
			expect(readRecord.dateWithTime).toBe("2025-01-15T10:00:00.000Z");
			expect(readRecord.singleSelect).toBe("Choice 1");
			expect(readRecord.multipleSelect).toEqual(["Option 1", "Option 2"]);
		});
	});

	describe("Update", async () => {
		const r = PrimaryModel.fromId(newRecord.id);
		await r.fetch();
		r.primaryKey = "Updated All Props Key";
		r.singleLineText = "Updated Hello";
		r.longText = "Updated long text";
		r.longTextWithRichText = "Updated rich text";
		r.email = "updated@example.com";
		r.url = "https://updated.com";
		r.phoneNumber = "555-5678";
		r.checkbox = false;
		r.numberInt = 100;
		r.numberFloat = 2.72;
		r.currencyInt = 20;
		r.currencyFloat = 19.99;
		r.percentInt = 0.75;
		r.percentFloat = 0.667;
		r.duration = 7200;
		r.rating = 5;
		r.date = "2025-06-15";
		r.dateWithTime = "2025-06-15T14:00:00.000Z";
		r.singleSelect = "Choice 2";
		r.multipleSelect = ["Option 2", "Option 3"];
		await r.save();

		it("should have the updated values", async () => {
			expect(r.id).toBe(newRecord.id);
			expect(r.primaryKey).toBe("Updated All Props Key");
			expect(r.singleLineText).toBe("Updated Hello");
			expect(r.longText).toBe("Updated long text");
			expect(r.longTextWithRichText).toBe("Updated rich text");
			expect(r.email).toBe("updated@example.com");
			expect(r.url).toBe("https://updated.com");
			expect(r.phoneNumber).toBe("555-5678");
			expect(r.checkbox).toBeFalsy();
			expect(r.numberInt).toBe(100);
			expect(r.numberFloat).toBe(2.72);
			expect(r.currencyInt).toBe(20);
			expect(r.currencyFloat).toBe(19.99);
			expect(r.percentInt).toBe(0.75);
			expect(r.percentFloat).toBe(0.667);
			expect(r.duration).toBe(7200);
			expect(r.rating).toBe(5);
			expect(r.date).toBe("2025-06-15");
			expect(r.dateWithTime).toBe("2025-06-15T14:00:00.000Z");
			expect(r.singleSelect).toBe("Choice 2");
			expect(r.multipleSelect).toEqual(["Option 2", "Option 3"]);
		});
	});

	describe("Delete", () => {
		it("should be deleted", async () => {
			const r = PrimaryModel.fromId(newRecord.id);
			await r.fetch();
			await r.delete();
			let deleted = false;
			try {
				const check = PrimaryModel.fromId(newRecord.id);
				await check.fetch();
			} catch {
				deleted = true;
			}
			expect(deleted).toBe(true);
		});
	});
});

describe("Complex Properties", async () => {
	describe("Linked Records", async () => {
		const sec1 = new SecondaryModel({ name: "Link Target 1", value: "val1" });
		await sec1.save();
		const sec2 = new SecondaryModel({ name: "Link Target 2", value: "val2" });
		await sec2.save();
		const sec3 = new SecondaryModel({ name: "Link Target 3", value: "val3" });
		await sec3.save();
		const sec1Id = sec1.id!;
		const sec2Id = sec2.id!;
		const sec3Id = sec3.id!;

		const newRecord = new PrimaryModel({
			primaryKey: "Link Test",
			linkSingle: sec1Id,
			linkMultiple: [sec1Id, sec2Id],
		});

		describe("Create", async () => {
			await newRecord.save();

			it("should have a valid id", async () => {
				expect(newRecord.id).toBeTruthy();
			});

			it("should have valid link values", async () => {
				expect(newRecord.linkSingle.id).toEqual(sec1Id);
				expect(newRecord.linkMultiple.ids).toEqual([sec1Id, sec2Id]);
			});
		});

		describe("Read", async () => {
			const readRecord = PrimaryModel.fromId(newRecord.id);
			await readRecord.fetch();
			const linkedSingle = await readRecord.linkSingle.get();
			const linkedMultiple = await readRecord.linkMultiple.get();

			it("should have the expected link values", async () => {
				expect(readRecord.linkSingle.id).toEqual(sec1Id);
				expect(readRecord.linkMultiple.ids).toEqual([sec1Id, sec2Id]);
			});

			it("should fetch linked single record via get()", async () => {
				expect(linkedSingle).toBeTruthy();
				expect(linkedSingle!.id).toEqual(sec1Id);
				expect(linkedSingle!.name).toBe("Link Target 1");
			});

			it("should fetch linked multiple records via get()", async () => {
				expect(linkedMultiple).toHaveLength(2);
				expect(linkedMultiple.map((r) => r.id)).toEqual([sec1Id, sec2Id]);
				expect(linkedMultiple[0].name).toBe("Link Target 1");
				expect(linkedMultiple[1].name).toBe("Link Target 2");
			});
		});

		describe("Update", async () => {
			const r = PrimaryModel.fromId(newRecord.id);
			await r.fetch();
			r.linkSingle.set(sec2);
			r.linkMultiple.set([sec1]);
			await r.save();

			it("should have the updated link values", async () => {
				expect(r.linkSingle.id).toEqual(sec2Id);
				expect(r.linkMultiple.ids).toEqual([sec1Id]);
			});
		});

		describe("Add", async () => {
			const r = PrimaryModel.fromId(newRecord.id);
			await r.fetch();
			r.linkMultiple.add(sec3);
			await r.save();

			it("should have the added link value", async () => {
				expect(r.linkMultiple.ids).toEqual([sec1Id, sec3Id]);
			});

			it("should persist after re-fetch", async () => {
				const check = PrimaryModel.fromId(newRecord.id);
				await check.fetch();
				expect(check.linkMultiple.ids).toEqual([sec1Id, sec3Id]);
			});
		});

		describe("Delete", () => {
			it("should be deleted", async () => {
				const r = PrimaryModel.fromId(newRecord.id);
				await r.fetch();
				await r.delete();
				let deleted = false;
				try {
					const check = PrimaryModel.fromId(newRecord.id);
					await check.fetch();
				} catch {
					deleted = true;
				}
				expect(deleted).toBe(true);
			});
		});

		describe("Cleanup", () => {
			it("should clean up secondary records", async () => {
				await sec1.delete();
				await sec2.delete();
				await sec3.delete();
			});
		});
	});

	describe("Nested Linked Records", async () => {
		const tert1 = new TertiaryModel({ name: "Tertiary 1", value: "tval1" });
		await tert1.save();
		const tert1Id = tert1.id!;

		const sec1 = new SecondaryModel({ name: "Nested Link Target", value: "sval1", linkToTertiary: [tert1Id] });
		await sec1.save();
		const sec1Id = sec1.id!;

		const primary = new PrimaryModel({
			primaryKey: "Nested Link Test",
			linkSingle: sec1Id,
		});
		await primary.save();

		describe("Read nested links", async () => {
			const readPrimary = PrimaryModel.fromId(primary.id);
			await readPrimary.fetch();
			const linkedSecondary = await readPrimary.linkSingle.get();
			const linkedTertiaries = await linkedSecondary!.linkToTertiary.get();

			it("should traverse Primary -> Secondary", async () => {
				expect(linkedSecondary).toBeTruthy();
				expect(linkedSecondary!.id).toEqual(sec1Id);
				expect(linkedSecondary!.name).toBe("Nested Link Target");
			});

			it("should traverse Secondary -> Tertiary", async () => {
				expect(linkedTertiaries).toHaveLength(1);
				expect(linkedTertiaries[0].id).toEqual(tert1Id);
				expect(linkedTertiaries[0].name).toBe("Tertiary 1");
				expect(linkedTertiaries[0].value).toBe("tval1");
			});
		});

		describe("Cleanup", () => {
			it("should clean up nested records", async () => {
				await primary.delete();
				await sec1.delete();
				await tert1.delete();
			});
		});
	});

	describe("Attachments", async () => {
		const newRecord = new PrimaryModel({
			primaryKey: "Attachment Test",
			attachment: [
				{ url: "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png" },
			] as any,
		});

		describe("Create", async () => {
			await newRecord.save();

			it("should have a valid id", async () => {
				expect(newRecord.id).toBeTruthy();
			});

			it("should have an attachment", async () => {
				expect(newRecord.attachment).toHaveLength(1);
				expect(newRecord.attachment![0].url).toBeTruthy();
			});
		});

		describe("Read", async () => {
			let readRecord!: PrimaryModel;
			for (let i = 0; i < 10; i++) {
				await new Promise((resolve) => setTimeout(resolve, 5000));
				readRecord = PrimaryModel.fromId(newRecord.id);
				await readRecord.fetch();
				if (readRecord.attachment) break;
			}

			it("should have the expected attachment", async () => {
				expect(readRecord.attachment).toHaveLength(1);
				expect(readRecord.attachment![0].url).toBeTruthy();
			});
		});

		describe("Delete", () => {
			it("should be deleted", async () => {
				const r = PrimaryModel.fromId(newRecord.id);
				await r.fetch();
				await r.delete();
				let deleted = false;
				try {
					const check = PrimaryModel.fromId(newRecord.id);
					await check.fetch();
				} catch {
					deleted = true;
				}
				expect(deleted).toBe(true);
			});
		});
	});

	describe("User", async () => {
		const newRecord = new PrimaryModel({
			primaryKey: "User Test",
			user: { id: "usrnZ4k98m0Ipji4e", email: "9vymqckyxq@privaterelay.appleid.com", name: "Daniel Baughman" },
			userAllowMultiple: [
				{ id: "usrnZ4k98m0Ipji4e", email: "9vymqckyxq@privaterelay.appleid.com", name: "Daniel Baughman" },
			],
		});

		describe("Create", async () => {
			await newRecord.save();

			it("should have a valid id", async () => {
				expect(newRecord.id).toBeTruthy();
			});

			it("should have user values", async () => {
				expect(newRecord.user).toBeTruthy();
				expect(newRecord.user!.id).toBe("usrnZ4k98m0Ipji4e");
				expect(newRecord.userAllowMultiple).toHaveLength(1);
				expect(newRecord.userAllowMultiple![0].id).toBe("usrnZ4k98m0Ipji4e");
			});
		});

		describe("Read", async () => {
			const readRecord = PrimaryModel.fromId(newRecord.id);
			await readRecord.fetch();

			it("should have the expected user values", async () => {
				expect(readRecord.user).toBeTruthy();
				expect(readRecord.user!.id).toBe("usrnZ4k98m0Ipji4e");
				expect(readRecord.userAllowMultiple).toHaveLength(1);
				expect(readRecord.userAllowMultiple![0].id).toBe("usrnZ4k98m0Ipji4e");
			});
		});

		describe("Delete", () => {
			it("should be deleted", async () => {
				const r = PrimaryModel.fromId(newRecord.id);
				await r.fetch();
				await r.delete();
				let deleted = false;
				try {
					const check = PrimaryModel.fromId(newRecord.id);
					await check.fetch();
				} catch {
					deleted = true;
				}
				expect(deleted).toBe(true);
			});
		});
	});

	describe("Computed Fields", async () => {
		const newRecord = new PrimaryModel({
			primaryKey: "Computed Test",
			numberInt: 10,
			numberFloat: 5,
		});

		describe("Create", async () => {
			await newRecord.save();

			it("should have a valid id", async () => {
				expect(newRecord.id).toBeTruthy();
			});

			it("should have computed field values", async () => {
				expect(newRecord.autoNumber).toEqual(expect.any(Number));
				expect(newRecord.createdAtTime).toBeTruthy();
				expect(newRecord.formulaId).toBeTruthy();
				expect(newRecord.formulaSimple).toBe(15);
			});
		});

		describe("Read", async () => {
			const readRecord = PrimaryModel.fromId(newRecord.id);
			await readRecord.fetch();

			it("should have the expected computed values", async () => {
				expect(readRecord.autoNumber).toEqual(expect.any(Number));
				expect(readRecord.createdAtTime).toBeTruthy();
				expect(readRecord.formulaId).toBe(newRecord.id);
				expect(readRecord.formulaSimple).toBe(15);
			});
		});

		describe("Delete", () => {
			it("should be deleted", async () => {
				const r = PrimaryModel.fromId(newRecord.id);
				await r.fetch();
				await r.delete();
				let deleted = false;
				try {
					const check = PrimaryModel.fromId(newRecord.id);
					await check.fetch();
				} catch {
					deleted = true;
				}
				expect(deleted).toBe(true);
			});
		});
	});
});

describe("Invalid Record ID", async () => {
	describe("Empty String ID", async () => {
		let threw = false;
		try {
			const record = PrimaryModel.fromId("");
			await record.fetch();
		} catch {
			threw = true;
		}

		it("should throw an error", async () => {
			expect(threw).toBe(true);
		});
	});

	describe("Invalid ID", async () => {
		let threw = false;
		try {
			const record = PrimaryModel.fromId("rec_INVALID_ID");
			await record.fetch();
		} catch {
			threw = true;
		}

		it("should throw an error", async () => {
			expect(threw).toBe(true);
		});
	});
});

describe("Upsert", async () => {
	const newRecord = new PrimaryModel({ primaryKey: "Upsert Create" });
	let id: string;

	describe("Upsert as Create", async () => {
		const createdRecord = await airtable.primary.upsert(newRecord);
		id = createdRecord.id!;

		it("should have a valid id", async () => {
			expect(createdRecord.id).toBeTruthy();
		});

		it("should have valid values", async () => {
			expect(createdRecord.primaryKey).toBe("Upsert Create");
		});
	});

	describe("Upsert as Update", async () => {
		const r = await airtable.primary.get(id);
		r.primaryKey = "Upsert Update";
		const updatedRecord = await airtable.primary.upsert(r);

		it("should have the same id", async () => {
			expect(updatedRecord.id).toBe(id);
		});

		it("should have the updated values", async () => {
			expect(updatedRecord.primaryKey).toBe("Upsert Update");
		});
	});

	describe("Read", async () => {
		const readRecord = await airtable.primary.get(id);

		it("should have the expected values", async () => {
			expect(readRecord.id).toBe(id);
			expect(readRecord.primaryKey).toBe("Upsert Update");
		});
	});

	describe("Delete", () => {
		it("should be deleted", async () => {
			await airtable.primary.delete(id);
			let deleted = false;
			try {
				await airtable.primary.get(id);
			} catch {
				deleted = true;
			}
			expect(deleted).toBe(true);
		});
	});
});

describe("Field Selection", async () => {
	const newRecord = new PrimaryModel({ primaryKey: "Field Select", singleLineText: "Hello" });
	let id: string;

	describe("Create", async () => {
		const createdRecord = await airtable.primary.create(newRecord);
		id = createdRecord.id!;

		it("should have a valid id", async () => {
			expect(createdRecord.id).toBeTruthy();
		});
	});

	describe("Read with fields option", async () => {
		const readRecord = await airtable.primary.get(id, { fields: ["Primary Key"] });

		it("should return the requested field", async () => {
			expect(readRecord.primaryKey).toBe("Field Select");
		});

		it("should not return unrequested fields", async () => {
			expect(readRecord.singleLineText).toBeUndefined();
		});
	});

	describe("Delete", () => {
		it("should be deleted", async () => {
			await airtable.primary.delete(id);
			let deleted = false;
			try {
				await airtable.primary.get(id);
			} catch {
				deleted = true;
			}
			expect(deleted).toBe(true);
		});
	});
});

describe("Max Records", async () => {
	const newRecords = Array.from({ length: 5 }, (_, i) => new PrimaryModel({ primaryKey: `Max Records ${i + 1}` }));
	let ids: string[];

	describe("Create", async () => {
		const createdRecords = await airtable.primary.create(newRecords);
		ids = createdRecords.map((r) => r.id!);

		it("should have created 5 records", async () => {
			expect(createdRecords.length).toBe(5);
		});
	});

	describe("Read with maxRecords option", async () => {
		const readRecords = await airtable.primary.get(ids, { maxRecords: 3 });

		it("should return only 3 records", async () => {
			expect(readRecords.length).toBe(3);
		});
	});

	describe("Delete", () => {
		it("should be deleted", async () => {
			await airtable.primary.delete(ids);
			const remaining = await airtable.primary.get(ids);
			expect(remaining.length).toBe(0);
		});
	});
});
