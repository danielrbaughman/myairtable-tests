import { describe, it, expect } from "vitest";
const { Airtable, PrimaryModel, SecondaryModel } = require("../output");

const airtable = new Airtable();

describe("Primary Key Only", async () => {
	const newRecord = new PrimaryModel({ primaryKey: "New Primary Key" });
	let id;

	describe("Create", async () => {
		const createdRecord = await airtable.primary.create(newRecord);
		id = createdRecord.id;

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
	let id;

	describe("Create", async () => {
		const createdRecord = await airtable.primary.create(newRecord);
		id = createdRecord.id;

		it("should have a valid id", async () => {
			expect(createdRecord.id).toBeTruthy();
		});

		it("should have valid values", async () => {
			expect(createdRecord.primaryKey).toBe("All Props Key");
			expect(createdRecord.singleLineText).toBe("Hello World");
			expect(createdRecord.longText).toBe("Long text content");
			expect(createdRecord.longTextWithRichText).toBe("Rich text content");
			expect(createdRecord.email).toBe("test@example.com");
			expect(createdRecord.url).toBe("https://example.com");
			expect(createdRecord.phoneNumber).toBe("555-1234");
			expect(createdRecord.checkbox).toBe(true);
			expect(createdRecord.numberInt).toBe(42);
			expect(createdRecord.numberFloat).toBe(3.14);
			expect(createdRecord.currencyInt).toBe(10);
			expect(createdRecord.currencyFloat).toBe(9.99);
			expect(createdRecord.percentInt).toBe(0.5);
			expect(createdRecord.percentFloat).toBe(0.333);
			expect(createdRecord.duration).toBe(3600);
			expect(createdRecord.rating).toBe(3);
			expect(createdRecord.date).toBe("2025-01-15");
			expect(createdRecord.dateWithTime).toBe("2025-01-15T10:00:00.000Z");
			expect(createdRecord.singleSelect).toBe("Choice 1");
			expect(createdRecord.multipleSelect).toEqual(["Option 1", "Option 2"]);
		});
	});

	describe("Read", async () => {
		const readRecord = await airtable.primary.get(id);

		it("should have the expected values", async () => {
			expect(readRecord.id).toBe(id);
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
		const r = await airtable.primary.get(id);
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
		const updatedRecord = await airtable.primary.update(r);

		it("should have the updated values", async () => {
			expect(updatedRecord.id).toBe(id);
			expect(updatedRecord.primaryKey).toBe("Updated All Props Key");
			expect(updatedRecord.singleLineText).toBe("Updated Hello");
			expect(updatedRecord.longText).toBe("Updated long text");
			expect(updatedRecord.longTextWithRichText).toBe("Updated rich text");
			expect(updatedRecord.email).toBe("updated@example.com");
			expect(updatedRecord.url).toBe("https://updated.com");
			expect(updatedRecord.phoneNumber).toBe("555-5678");
			expect(updatedRecord.checkbox).toBeFalsy();
			expect(updatedRecord.numberInt).toBe(100);
			expect(updatedRecord.numberFloat).toBe(2.72);
			expect(updatedRecord.currencyInt).toBe(20);
			expect(updatedRecord.currencyFloat).toBe(19.99);
			expect(updatedRecord.percentInt).toBe(0.75);
			expect(updatedRecord.percentFloat).toBe(0.667);
			expect(updatedRecord.duration).toBe(7200);
			expect(updatedRecord.rating).toBe(5);
			expect(updatedRecord.date).toBe("2025-06-15");
			expect(updatedRecord.dateWithTime).toBe("2025-06-15T14:00:00.000Z");
			expect(updatedRecord.singleSelect).toBe("Choice 2");
			expect(updatedRecord.multipleSelect).toEqual(["Option 2", "Option 3"]);
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

describe("Complex Properties", async () => {
	describe("Linked Records", async () => {
		const sec1 = await airtable.secondary.create(new SecondaryModel({ name: "Link Target 1", value: "val1" }));
		const sec2 = await airtable.secondary.create(new SecondaryModel({ name: "Link Target 2", value: "val2" }));
		let id;

		describe("Create", async () => {
			const createdRecord = await airtable.primary.create(
				new PrimaryModel({
					primaryKey: "Link Test",
					linkSingle: sec1.id,
					linkMultiple: [sec1.id, sec2.id],
				}),
			);
			id = createdRecord.id;

			it("should have a valid id", async () => {
				expect(createdRecord.id).toBeTruthy();
			});

			it("should have valid link values", async () => {
				expect(createdRecord.linkSingle.id).toEqual(sec1.id);
				expect(createdRecord.linkMultiple.ids).toEqual([sec1.id, sec2.id]);
			});
		});

		describe("Read", async () => {
			const readRecord = await airtable.primary.get(id);

			it("should have the expected link values", async () => {
				expect(readRecord.linkSingle.id).toEqual(sec1.id);
				expect(readRecord.linkMultiple.ids).toEqual([sec1.id, sec2.id]);
			});
		});

		describe("Update", async () => {
			const r = await airtable.primary.get(id);
			r.linkSingle = sec2;
			r.linkMultiple = [sec1];
			const updatedRecord = await airtable.primary.update(r);

			it("should have the updated link values", async () => {
				expect(updatedRecord.linkSingle.id).toEqual(sec2.id);
				expect(updatedRecord.linkMultiple.ids).toEqual([sec1.id]);
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

		describe("Cleanup", () => {
			it("should clean up secondary records", async () => {
				await airtable.secondary.delete(sec1.id);
				await airtable.secondary.delete(sec2.id);
			});
		});
	});

	describe("Attachments", async () => {
		let id;

		describe("Create", async () => {
			const createdRecord = await airtable.primary.create(
				new PrimaryModel({
					primaryKey: "Attachment Test",
					attachment: [{ url: "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png" }],
				}),
			);
			id = createdRecord.id;

			it("should have a valid id", async () => {
				expect(createdRecord.id).toBeTruthy();
			});

			it("should have an attachment", async () => {
				expect(createdRecord.attachment).toHaveLength(1);
				expect(createdRecord.attachment[0].url).toBeTruthy();
			});
		});

		describe("Read", async () => {
			let readRecord;
			for (let i = 0; i < 10; i++) {
				await new Promise((resolve) => setTimeout(resolve, 5000));
				readRecord = await airtable.primary.get(id);
				if (readRecord.attachment) break;
			}

			it("should have the expected attachment", async () => {
				expect(readRecord.attachment).toHaveLength(1);
				expect(readRecord.attachment[0].url).toBeTruthy();
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

	describe("User", async () => {
		let id;

		describe("Create", async () => {
			const createdRecord = await airtable.primary.create(
				new PrimaryModel({
					primaryKey: "User Test",
					user: { id: "usrnZ4k98m0Ipji4e", email: "9vymqckyxq@privaterelay.appleid.com", name: "Daniel Baughman" },
					userAllowMultiple: [
						{ id: "usrnZ4k98m0Ipji4e", email: "9vymqckyxq@privaterelay.appleid.com", name: "Daniel Baughman" },
					],
				}),
			);
			id = createdRecord.id;

			it("should have a valid id", async () => {
				expect(createdRecord.id).toBeTruthy();
			});

			it("should have user values", async () => {
				expect(createdRecord.user).toBeTruthy();
				expect(createdRecord.user.id).toBe("usrnZ4k98m0Ipji4e");
				expect(createdRecord.userAllowMultiple).toHaveLength(1);
				expect(createdRecord.userAllowMultiple[0].id).toBe("usrnZ4k98m0Ipji4e");
			});
		});

		describe("Read", async () => {
			const readRecord = await airtable.primary.get(id);

			it("should have the expected user values", async () => {
				expect(readRecord.user).toBeTruthy();
				expect(readRecord.user.id).toBe("usrnZ4k98m0Ipji4e");
				expect(readRecord.userAllowMultiple).toHaveLength(1);
				expect(readRecord.userAllowMultiple[0].id).toBe("usrnZ4k98m0Ipji4e");
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

	describe("Computed Fields", async () => {
		let id;

		describe("Create", async () => {
			const createdRecord = await airtable.primary.create(
				new PrimaryModel({
					primaryKey: "Computed Test",
					numberInt: 10,
					numberFloat: 5,
				}),
			);
			id = createdRecord.id;

			it("should have a valid id", async () => {
				expect(createdRecord.id).toBeTruthy();
			});

			it("should have computed field values", async () => {
				expect(createdRecord.autoNumber).toEqual(expect.any(Number));
				expect(createdRecord.createdAtTime).toBeTruthy();
				expect(createdRecord.formulaId).toBeTruthy();
				expect(createdRecord.formulaSimple).toBe(15);
			});
		});

		describe("Read", async () => {
			const readRecord = await airtable.primary.get(id);

			it("should have the expected computed values", async () => {
				expect(readRecord.autoNumber).toEqual(expect.any(Number));
				expect(readRecord.createdAtTime).toBeTruthy();
				expect(readRecord.formulaId).toBe(id);
				expect(readRecord.formulaSimple).toBe(15);
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
});

describe("Batch Operations", async () => {
	const count = 111;
	const newRecords = Array.from({ length: count }, (_, i) => new PrimaryModel({ primaryKey: `Batch Record ${i + 1}` }));
	let ids;

	describe("Create", async () => {
		const createdRecords = await airtable.primary.create(newRecords);
		ids = createdRecords.map((r) => r.id);

		it("should return the correct number of records", async () => {
			expect(createdRecords.length).toBe(count);
		});

		it("should all have valid ids", async () => {
			for (const r of createdRecords) {
				expect(r.id).toBeTruthy();
			}
		});

		it("should have the correct primary keys", async () => {
			for (let i = 0; i < count; i++) {
				expect(createdRecords[i].primaryKey).toBe(`Batch Record ${i + 1}`);
			}
		});
	});

	describe("Read", async () => {
		const readRecords = await airtable.primary.get(ids);

		it("should return the correct number of records", async () => {
			expect(readRecords.length).toBe(count);
		});

		it("should have the expected primary keys", async () => {
			for (const r of readRecords) {
				expect(r.primaryKey).toMatch(/^Batch Record \d+$/);
			}
		});
	});

	describe("Update", async () => {
		const fetched = await airtable.primary.get(ids);
		for (let i = 0; i < fetched.length; i++) {
			fetched[i].primaryKey = `Updated Batch Record ${i + 1}`;
		}
		const updatedRecords = await airtable.primary.update(fetched);

		it("should return the correct number of records", async () => {
			expect(updatedRecords.length).toBe(count);
		});

		it("should have the updated primary keys", async () => {
			for (let i = 0; i < count; i++) {
				expect(updatedRecords[i].primaryKey).toBe(`Updated Batch Record ${i + 1}`);
			}
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

describe("Invalid Record ID", async () => {
	describe("Empty String ID", async () => {
		let threw = false;
		try {
			await airtable.primary.get("");
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
			await airtable.primary.get("rec_INVALID_ID");
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
	let id;

	describe("Upsert as Create", async () => {
		const createdRecord = await airtable.primary.upsert(newRecord);
		id = createdRecord.id;

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
	let id;

	describe("Create", async () => {
		const createdRecord = await airtable.primary.create(newRecord);
		id = createdRecord.id;

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
	let ids;

	describe("Create", async () => {
		const createdRecords = await airtable.primary.create(newRecords);
		ids = createdRecords.map((r) => r.id);

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

describe("Duplicate", async () => {
	// duplicate() is the first verb that POSTs a record read back from Airtable: a server-loaded
	// model has `_isNew === false` and no dirty fields, so the ordinary create payload for it
	// would be empty.
	const sec = await airtable.secondary.create(new SecondaryModel({ name: "JS Duplicate Link" }));
	const source = await airtable.primary.create(
		new PrimaryModel({
			primaryKey: "JS Duplicate Source",
			singleLineText: "copy me",
			// numberInt stays <= 10 and != 20 on purpose: filter-by-formula asserts exact counts
			// for `numberInt = 20` and `AND(numberInt > 10, checkbox = true)`, and vitest runs test
			// files in parallel against the one shared base.
			numberInt: 7,
			rating: 3,
			singleSelect: "Choice 1",
		}),
	);
	source.linkSingle.id = sec.id;
	await source.save();

	const fetched = await airtable.primary.get(source.id);
	const copy = await airtable.primary.duplicate(fetched);
	const trash = [copy.id];

	it("creates a new record rather than updating the source", async () => {
		expect(copy.id).toBeTruthy();
		expect(copy.id).not.toBe(source.id);
	});

	it("copies every writable field, including the primary field", async () => {
		expect(copy.primaryKey).toBe("JS Duplicate Source");
		expect(copy.singleLineText).toBe("copy me");
		expect(copy.numberInt).toBe(7);
		expect(copy.rating).toBe(3);
		expect(copy.singleSelect).toBe("Choice 1");
	});

	it("recomputes computed fields instead of copying them", async () => {
		// Formula (ID) resolves to RECORD_ID(), so on a true copy it is the COPY's id.
		expect(copy.formulaId).toBe(copy.id);
		expect(copy.autoNumber).not.toBe(fetched.autoNumber);
	});

	it("copies the link without moving it off the source", async () => {
		expect(copy.linkSingle.id).toBe(sec.id);
		const sourceAgain = await airtable.primary.get(source.id);
		expect(sourceAgain.linkSingle.id).toBe(sec.id);
	});

	it("duplicates by record id and preserves batch order", async () => {
		const other = await airtable.primary.create(new PrimaryModel({ primaryKey: "JS Duplicate Source B" }));
		trash.push(other.id);
		const copies = await airtable.primary.duplicate([other.id, source.id]);
		trash.push(...copies.map((r) => r.id));
		expect(copies).toHaveLength(2);
		expect(copies[0].primaryKey).toBe("JS Duplicate Source B");
		expect(copies[1].primaryKey).toBe("JS Duplicate Source");
	});

	it("rejects a source with no id", async () => {
		await expect(airtable.primary.duplicate(new PrimaryModel({ primaryKey: "unsaved" }))).rejects.toThrow(/no id/);
	});

	it("cleans up", async () => {
		await airtable.primary.delete([...new Set([...trash, source.id])]);
		await airtable.secondary.delete(sec.id);
		expect(true).toBe(true);
	});
});
