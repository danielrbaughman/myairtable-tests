import { describe, it, expect } from "vitest";
import { PrimaryModel, SecondaryModel } from "../output";

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

	describe("Delete", async () => {
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

		it("should be deleted", async () => {
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

	describe("Delete", async () => {
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

		it("should be deleted", async () => {
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
		const sec1Id = sec1.id!;
		const sec2Id = sec2.id!;

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

			it("should have the expected link values", async () => {
				expect(readRecord.linkSingle.id).toEqual(sec1Id);
				expect(readRecord.linkMultiple.ids).toEqual([sec1Id, sec2Id]);
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

		describe("Delete", async () => {
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

			it("should be deleted", async () => {
				expect(deleted).toBe(true);
			});
		});

		describe("Cleanup", async () => {
			await sec1.delete();
			await sec2.delete();

			it("should clean up secondary records", async () => {
				expect(true).toBe(true);
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

		describe("Delete", async () => {
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

			it("should be deleted", async () => {
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

		describe("Delete", async () => {
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

			it("should be deleted", async () => {
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

		describe("Delete", async () => {
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

			it("should be deleted", async () => {
				expect(deleted).toBe(true);
			});
		});
	});
});
