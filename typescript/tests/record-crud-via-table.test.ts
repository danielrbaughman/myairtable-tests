import { describe, it, expect } from "vitest";
import { Record as ATRecord } from "airtable";
import { Airtable, PrimaryFieldSet, SecondaryFieldSet } from "../output";

const airtable = new Airtable();

function newPrimaryRecord(): ATRecord<PrimaryFieldSet> {
	return new ATRecord<PrimaryFieldSet>(airtable.primary._table, "", {});
}

function newSecondaryRecord(): ATRecord<SecondaryFieldSet> {
	return new ATRecord<SecondaryFieldSet>(airtable.secondary._table, "", {});
}

describe("Primary Key Only", async () => {
	const newRecord = newPrimaryRecord();
	newRecord.set("Primary Key", "New Primary Key");
	let id: string;

	describe("Create", async () => {
		const createdRecord = await airtable.primary.create(newRecord);
		id = createdRecord.id;

		it("should have a valid id", async () => {
			expect(createdRecord.id).toBeTruthy();
		});

		it("should have valid values", async () => {
			expect(createdRecord.fields["Primary Key"]).toBe("New Primary Key");
		});
	});

	describe("Read", async () => {
		const readRecord = await airtable.primary.get(id, { returnAs: "record" });

		it("should have the expected values", async () => {
			expect(readRecord.id).toBe(id);
			expect(readRecord.fields["Primary Key"]).toBe("New Primary Key");
		});
	});

	describe("Update", async () => {
		const r = await airtable.primary.get(id, { returnAs: "record" });
		r.set("Primary Key", "Updated Primary Key");
		const updatedRecord = await airtable.primary.update(r);

		it("should have the updated values", async () => {
			expect(updatedRecord.id).toBe(id);
			expect(updatedRecord.fields["Primary Key"]).toBe("Updated Primary Key");
		});
	});

	describe("Delete", async () => {
		await airtable.primary.delete(id);
		let deleted = false;
		try {
			await airtable.primary.get(id, { returnAs: "record" });
		} catch {
			deleted = true;
		}

		it("should be deleted", async () => {
			expect(deleted).toBe(true);
		});
	});
});

describe("All Simple Properties", async () => {
	const newRecord = newPrimaryRecord();
	newRecord.set("Primary Key", "All Props Key");
	newRecord.set("Single Line Text", "Hello World");
	newRecord.set("Long Text", "Long text content");
	newRecord.set("Long Text with Rich Text", "Rich text content");
	newRecord.set("Email", "test@example.com");
	newRecord.set("URL", "https://example.com");
	newRecord.set("Phone Number", "555-1234");
	newRecord.set("Checkbox", true);
	newRecord.set("Number (int)", 42);
	newRecord.set("Number (float)", 3.14);
	newRecord.set("Currency (int)", 10);
	newRecord.set("Currency (float)", 9.99);
	newRecord.set("Percent (int)", 0.5);
	newRecord.set("Percent (float)", 0.333);
	newRecord.set("Duration", 3600);
	newRecord.set("Rating", 3);
	newRecord.set("Date", "2025-01-15");
	newRecord.set("Date (with time)", "2025-01-15T10:00:00.000Z");
	newRecord.set("Single Select", "Choice 1");
	newRecord.set("Multiple Select", ["Option 1", "Option 2"]);
	let id: string;

	describe("Create", async () => {
		const createdRecord = await airtable.primary.create(newRecord);
		id = createdRecord.id;

		it("should have a valid id", async () => {
			expect(createdRecord.id).toBeTruthy();
		});

		it("should have valid values", async () => {
			expect(createdRecord.fields["Primary Key"]).toBe("All Props Key");
			expect(createdRecord.fields["Single Line Text"]).toBe("Hello World");
			expect(createdRecord.fields["Long Text"]).toBe("Long text content");
			expect(createdRecord.fields["Long Text with Rich Text"]).toBe("Rich text content");
			expect(createdRecord.fields["Email"]).toBe("test@example.com");
			expect(createdRecord.fields["URL"]).toBe("https://example.com");
			expect(createdRecord.fields["Phone Number"]).toBe("555-1234");
			expect(createdRecord.fields["Checkbox"]).toBe(true);
			expect(createdRecord.fields["Number (int)"]).toBe(42);
			expect(createdRecord.fields["Number (float)"]).toBe(3.14);
			expect(createdRecord.fields["Currency (int)"]).toBe(10);
			expect(createdRecord.fields["Currency (float)"]).toBe(9.99);
			expect(createdRecord.fields["Percent (int)"]).toBe(0.5);
			expect(createdRecord.fields["Percent (float)"]).toBe(0.333);
			expect(createdRecord.fields["Duration"]).toBe(3600);
			expect(createdRecord.fields["Rating"]).toBe(3);
			expect(createdRecord.fields["Date"]).toBe("2025-01-15");
			expect(createdRecord.fields["Date (with time)"]).toBe("2025-01-15T10:00:00.000Z");
			expect(createdRecord.fields["Single Select"]).toBe("Choice 1");
			expect(createdRecord.fields["Multiple Select"]).toEqual(["Option 1", "Option 2"]);
		});
	});

	describe("Read", async () => {
		const readRecord = await airtable.primary.get(id, { returnAs: "record" });

		it("should have the expected values", async () => {
			expect(readRecord.id).toBe(id);
			expect(readRecord.fields["Primary Key"]).toBe("All Props Key");
			expect(readRecord.fields["Single Line Text"]).toBe("Hello World");
			expect(readRecord.fields["Long Text"]).toBe("Long text content");
			expect(readRecord.fields["Long Text with Rich Text"]).toBe("Rich text content");
			expect(readRecord.fields["Email"]).toBe("test@example.com");
			expect(readRecord.fields["URL"]).toBe("https://example.com");
			expect(readRecord.fields["Phone Number"]).toBe("555-1234");
			expect(readRecord.fields["Checkbox"]).toBe(true);
			expect(readRecord.fields["Number (int)"]).toBe(42);
			expect(readRecord.fields["Number (float)"]).toBe(3.14);
			expect(readRecord.fields["Currency (int)"]).toBe(10);
			expect(readRecord.fields["Currency (float)"]).toBe(9.99);
			expect(readRecord.fields["Percent (int)"]).toBe(0.5);
			expect(readRecord.fields["Percent (float)"]).toBe(0.333);
			expect(readRecord.fields["Duration"]).toBe(3600);
			expect(readRecord.fields["Rating"]).toBe(3);
			expect(readRecord.fields["Date"]).toBe("2025-01-15");
			expect(readRecord.fields["Date (with time)"]).toBe("2025-01-15T10:00:00.000Z");
			expect(readRecord.fields["Single Select"]).toBe("Choice 1");
			expect(readRecord.fields["Multiple Select"]).toEqual(["Option 1", "Option 2"]);
		});
	});

	describe("Update", async () => {
		const r = await airtable.primary.get(id, { returnAs: "record" });
		r.set("Primary Key", "Updated All Props Key");
		r.set("Single Line Text", "Updated Hello");
		r.set("Long Text", "Updated long text");
		r.set("Long Text with Rich Text", "Updated rich text");
		r.set("Email", "updated@example.com");
		r.set("URL", "https://updated.com");
		r.set("Phone Number", "555-5678");
		r.set("Checkbox", false);
		r.set("Number (int)", 100);
		r.set("Number (float)", 2.72);
		r.set("Currency (int)", 20);
		r.set("Currency (float)", 19.99);
		r.set("Percent (int)", 0.75);
		r.set("Percent (float)", 0.667);
		r.set("Duration", 7200);
		r.set("Rating", 5);
		r.set("Date", "2025-06-15");
		r.set("Date (with time)", "2025-06-15T14:00:00.000Z");
		r.set("Single Select", "Choice 2");
		r.set("Multiple Select", ["Option 2", "Option 3"]);
		const updatedRecord = await airtable.primary.update(r);

		it("should have the updated values", async () => {
			expect(updatedRecord.id).toBe(id);
			expect(updatedRecord.fields["Primary Key"]).toBe("Updated All Props Key");
			expect(updatedRecord.fields["Single Line Text"]).toBe("Updated Hello");
			expect(updatedRecord.fields["Long Text"]).toBe("Updated long text");
			expect(updatedRecord.fields["Long Text with Rich Text"]).toBe("Updated rich text");
			expect(updatedRecord.fields["Email"]).toBe("updated@example.com");
			expect(updatedRecord.fields["URL"]).toBe("https://updated.com");
			expect(updatedRecord.fields["Phone Number"]).toBe("555-5678");
			expect(updatedRecord.fields["Checkbox"]).toBeFalsy();
			expect(updatedRecord.fields["Number (int)"]).toBe(100);
			expect(updatedRecord.fields["Number (float)"]).toBe(2.72);
			expect(updatedRecord.fields["Currency (int)"]).toBe(20);
			expect(updatedRecord.fields["Currency (float)"]).toBe(19.99);
			expect(updatedRecord.fields["Percent (int)"]).toBe(0.75);
			expect(updatedRecord.fields["Percent (float)"]).toBe(0.667);
			expect(updatedRecord.fields["Duration"]).toBe(7200);
			expect(updatedRecord.fields["Rating"]).toBe(5);
			expect(updatedRecord.fields["Date"]).toBe("2025-06-15");
			expect(updatedRecord.fields["Date (with time)"]).toBe("2025-06-15T14:00:00.000Z");
			expect(updatedRecord.fields["Single Select"]).toBe("Choice 2");
			expect(updatedRecord.fields["Multiple Select"]).toEqual(["Option 2", "Option 3"]);
		});
	});

	describe("Delete", async () => {
		await airtable.primary.delete(id);
		let deleted = false;
		try {
			await airtable.primary.get(id, { returnAs: "record" });
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
		const secRecord1 = newSecondaryRecord();
		secRecord1.set("Name", "Link Target 1");
		secRecord1.set("Value", "val1");
		const sec1 = await airtable.secondary.create(secRecord1);

		const secRecord2 = newSecondaryRecord();
		secRecord2.set("Name", "Link Target 2");
		secRecord2.set("Value", "val2");
		const sec2 = await airtable.secondary.create(secRecord2);
		let id: string;

		describe("Create", async () => {
			const newRecord = newPrimaryRecord();
			newRecord.set("Primary Key", "Link Test");
			newRecord.set("Link (single)", [sec1.id]);
			newRecord.set("Link (multiple)", [sec1.id, sec2.id]);
			const createdRecord = await airtable.primary.create(newRecord);
			id = createdRecord.id;

			it("should have a valid id", async () => {
				expect(createdRecord.id).toBeTruthy();
			});

			it("should have valid link values", async () => {
				expect(createdRecord.fields["Link (single)"]).toEqual([sec1.id]);
				expect(createdRecord.fields["Link (multiple)"]).toEqual([sec1.id, sec2.id]);
			});
		});

		describe("Read", async () => {
			const readRecord = await airtable.primary.get(id, { returnAs: "record" });

			it("should have the expected link values", async () => {
				expect(readRecord.fields["Link (single)"]).toEqual([sec1.id]);
				expect(readRecord.fields["Link (multiple)"]).toEqual([sec1.id, sec2.id]);
			});
		});

		describe("Update", async () => {
			const r = await airtable.primary.get(id, { returnAs: "record" });
			r.set("Link (single)", [sec2.id]);
			r.set("Link (multiple)", [sec1.id]);
			const updatedRecord = await airtable.primary.update(r);

			it("should have the updated link values", async () => {
				expect(updatedRecord.fields["Link (single)"]).toEqual([sec2.id]);
				expect(updatedRecord.fields["Link (multiple)"]).toEqual([sec1.id]);
			});
		});

		describe("Delete", async () => {
			await airtable.primary.delete(id);
			let deleted = false;
			try {
				await airtable.primary.get(id, { returnAs: "record" });
			} catch {
				deleted = true;
			}

			it("should be deleted", async () => {
				expect(deleted).toBe(true);
			});
		});

		describe("Cleanup", async () => {
			await airtable.secondary.delete(sec1.id);
			await airtable.secondary.delete(sec2.id);

			it("should clean up secondary records", async () => {
				expect(true).toBe(true);
			});
		});
	});

	describe("Attachments", async () => {
		let id: string;

		describe("Create", async () => {
			const newRecord = newPrimaryRecord();
			newRecord.set("Primary Key", "Attachment Test");
			newRecord.set("Attachment", [
				{ url: "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png" },
			]);
			const createdRecord = await airtable.primary.create(newRecord);
			id = createdRecord.id;

			it("should have a valid id", async () => {
				expect(createdRecord.id).toBeTruthy();
			});

			it("should have an attachment", async () => {
				expect(createdRecord.fields["Attachment"]).toHaveLength(1);
				expect(createdRecord.fields["Attachment"]![0].url).toBeTruthy();
			});
		});

		describe("Read", async () => {
			let readRecord!: ATRecord<PrimaryFieldSet>;
			for (let i = 0; i < 10; i++) {
				await new Promise((resolve) => setTimeout(resolve, 5000));
				readRecord = await airtable.primary.get(id, { returnAs: "record" });
				if (readRecord.fields["Attachment"]) break;
			}

			it("should have the expected attachment", async () => {
				expect(readRecord.fields["Attachment"]).toHaveLength(1);
				expect(readRecord.fields["Attachment"]![0].url).toBeTruthy();
			});
		});

		describe("Delete", async () => {
			await airtable.primary.delete(id);
			let deleted = false;
			try {
				await airtable.primary.get(id, { returnAs: "record" });
			} catch {
				deleted = true;
			}

			it("should be deleted", async () => {
				expect(deleted).toBe(true);
			});
		});
	});

	describe("User", async () => {
		let id: string;

		describe("Create", async () => {
			const newRecord = newPrimaryRecord();
			newRecord.set("Primary Key", "User Test");
			newRecord.set("User", {
				id: "usrnZ4k98m0Ipji4e",
				email: "9vymqckyxq@privaterelay.appleid.com",
				name: "Daniel Baughman",
			});
			newRecord.set("User (allow multiple)", [
				{ id: "usrnZ4k98m0Ipji4e", email: "9vymqckyxq@privaterelay.appleid.com", name: "Daniel Baughman" },
			]);
			const createdRecord = await airtable.primary.create(newRecord);
			id = createdRecord.id;

			it("should have a valid id", async () => {
				expect(createdRecord.id).toBeTruthy();
			});

			it("should have user values", async () => {
				expect(createdRecord.fields["User"]).toBeTruthy();
				expect(createdRecord.fields["User"]!.id).toBe("usrnZ4k98m0Ipji4e");
				expect(createdRecord.fields["User (allow multiple)"]).toHaveLength(1);
				expect(createdRecord.fields["User (allow multiple)"]![0].id).toBe("usrnZ4k98m0Ipji4e");
			});
		});

		describe("Read", async () => {
			const readRecord = await airtable.primary.get(id, { returnAs: "record" });

			it("should have the expected user values", async () => {
				expect(readRecord.fields["User"]).toBeTruthy();
				expect(readRecord.fields["User"]!.id).toBe("usrnZ4k98m0Ipji4e");
				expect(readRecord.fields["User (allow multiple)"]).toHaveLength(1);
				expect(readRecord.fields["User (allow multiple)"]![0].id).toBe("usrnZ4k98m0Ipji4e");
			});
		});

		describe("Delete", async () => {
			await airtable.primary.delete(id);
			let deleted = false;
			try {
				await airtable.primary.get(id, { returnAs: "record" });
			} catch {
				deleted = true;
			}

			it("should be deleted", async () => {
				expect(deleted).toBe(true);
			});
		});
	});

	describe("Computed Fields", async () => {
		let id: string;

		describe("Create", async () => {
			const newRecord = newPrimaryRecord();
			newRecord.set("Primary Key", "Computed Test");
			newRecord.set("Number (int)", 10);
			newRecord.set("Number (float)", 5);
			const createdRecord = await airtable.primary.create(newRecord);
			id = createdRecord.id;

			it("should have a valid id", async () => {
				expect(createdRecord.id).toBeTruthy();
			});

			it("should have computed field values", async () => {
				expect(createdRecord.fields["Auto Number"]).toEqual(expect.any(Number));
				expect(createdRecord.fields["Created Time"]).toBeTruthy();
				expect(createdRecord.fields["Formula (ID)"]).toBeTruthy();
				expect(createdRecord.fields["Formula (Simple)"]).toBe(15);
			});
		});

		describe("Read", async () => {
			const readRecord = await airtable.primary.get(id, { returnAs: "record" });

			it("should have the expected computed values", async () => {
				expect(readRecord.fields["Auto Number"]).toEqual(expect.any(Number));
				expect(readRecord.fields["Created Time"]).toBeTruthy();
				expect(readRecord.fields["Formula (ID)"]).toBe(id);
				expect(readRecord.fields["Formula (Simple)"]).toBe(15);
			});
		});

		describe("Delete", async () => {
			await airtable.primary.delete(id);
			let deleted = false;
			try {
				await airtable.primary.get(id, { returnAs: "record" });
			} catch {
				deleted = true;
			}

			it("should be deleted", async () => {
				expect(deleted).toBe(true);
			});
		});
	});
});

describe("Batch Operations", async () => {
	const count = 111;
	const newRecords = Array.from({ length: count }, (_, i) => {
		const r = newPrimaryRecord();
		r.set("Primary Key", `Batch Record ${i + 1}`);
		return r;
	});
	let ids: string[];

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
				expect(createdRecords[i].fields["Primary Key"]).toBe(`Batch Record ${i + 1}`);
			}
		});
	});

	describe("Read", async () => {
		const readRecords = await airtable.primary.get(ids, { returnAs: "record" });

		it("should return the correct number of records", async () => {
			expect(readRecords.length).toBe(count);
		});

		it("should have the expected primary keys", async () => {
			for (const r of readRecords) {
				expect(r.fields["Primary Key"]).toMatch(/^Batch Record \d+$/);
			}
		});
	});

	describe("Update", async () => {
		const fetched = await airtable.primary.get(ids, { returnAs: "record" });
		for (let i = 0; i < fetched.length; i++) {
			fetched[i].set("Primary Key", `Updated Batch Record ${i + 1}`);
		}
		const updatedRecords = await airtable.primary.update(fetched);

		it("should return the correct number of records", async () => {
			expect(updatedRecords.length).toBe(count);
		});

		it("should have the updated primary keys", async () => {
			for (let i = 0; i < count; i++) {
				expect(updatedRecords[i].fields["Primary Key"]).toBe(`Updated Batch Record ${i + 1}`);
			}
		});
	});

	describe("Delete", async () => {
		await airtable.primary.delete(ids);
		const remaining = await airtable.primary.get(ids, { returnAs: "record" });

		it("should be deleted", async () => {
			expect(remaining.length).toBe(0);
		});
	});
});

describe("Invalid Record ID", async () => {
	describe("Empty String ID", async () => {
		let threw = false;
		try {
			await airtable.primary.get("", { returnAs: "record" });
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
			await airtable.primary.get("rec_INVALID_ID", { returnAs: "record" });
		} catch {
			threw = true;
		}

		it("should throw an error", async () => {
			expect(threw).toBe(true);
		});
	});
});

describe("Upsert", async () => {
	const newRecord = newPrimaryRecord();
	newRecord.set("Primary Key", "Upsert Create");
	let id: string;

	describe("Upsert as Create", async () => {
		const createdRecord = await airtable.primary.upsert(newRecord);
		id = createdRecord.id;

		it("should have a valid id", async () => {
			expect(createdRecord.id).toBeTruthy();
		});

		it("should have valid values", async () => {
			expect(createdRecord.fields["Primary Key"]).toBe("Upsert Create");
		});
	});

	describe("Upsert as Update", async () => {
		const r = await airtable.primary.get(id, { returnAs: "record" });
		r.set("Primary Key", "Upsert Update");
		const updatedRecord = await airtable.primary.upsert(r);

		it("should have the same id", async () => {
			expect(updatedRecord.id).toBe(id);
		});

		it("should have the updated values", async () => {
			expect(updatedRecord.fields["Primary Key"]).toBe("Upsert Update");
		});
	});

	describe("Read", async () => {
		const readRecord = await airtable.primary.get(id, { returnAs: "record" });

		it("should have the expected values", async () => {
			expect(readRecord.id).toBe(id);
			expect(readRecord.fields["Primary Key"]).toBe("Upsert Update");
		});
	});

	describe("Delete", async () => {
		await airtable.primary.delete(id);
		let deleted = false;
		try {
			await airtable.primary.get(id, { returnAs: "record" });
		} catch {
			deleted = true;
		}

		it("should be deleted", async () => {
			expect(deleted).toBe(true);
		});
	});
});
