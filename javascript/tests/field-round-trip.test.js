import { describe, it, expect } from "vitest";
const { PrimaryModel } = require("../output");

// TC7 — Field-type round-trip completeness via re-fetch.
//
// Several field types were only asserted on the create response (or only offline-decoded),
// never written and read back through the live API, and clearing/removing multi-value fields
// was untested. Each case here creates, optionally updates, re-fetches, and asserts the
// server-side value. Mirrors csharp/tests/TestFieldRoundTrip.cs.

const USER_ID = "usrnZ4k98m0Ipji4e"; // shared-base user, as in the Complex Properties suite

function primaryKey(label) {
	const ts = Date.now();
	const rand = Math.floor(Math.random() * (999999 - 100000) + 100000);
	return `JavaScript FieldRT ${label} ${ts}-${rand}`;
}

// Retry transient 429s (and other transient network errors) with backoff.
async function withRetry(fn, attempts = 5) {
	let lastErr;
	for (let i = 0; i < attempts; i++) {
		try {
			return await fn();
		} catch (err) {
			lastErr = err;
			const status = err?.statusCode ?? err?.status;
			const msg = String(err?.message ?? err);
			const transient = status === 429 || /429|rate limit|ECONNRESET|ETIMEDOUT|socket hang up/i.test(msg);
			if (!transient || i === attempts - 1) throw err;
			await new Promise((resolve) => setTimeout(resolve, 1000 * (i + 1)));
		}
	}
	throw lastErr;
}

async function tryDelete(id) {
	if (!id) return;
	try {
		const r = PrimaryModel.fromId(id);
		await r.fetch();
		await r.delete();
	} catch {
		// best-effort cleanup
	}
}

describe("Field-type round-trip via re-fetch", () => {
	it("DateWithTime writes and reads back", async () => {
		const record = new PrimaryModel({
			primaryKey: primaryKey("DateTime"),
			dateWithTime: "2024-03-15T14:30:00.000Z",
		});
		await withRetry(() => record.save());
		const id = record.id;
		try {
			const fetched = PrimaryModel.fromId(id);
			await withRetry(() => fetched.fetch());
			expect(fetched.dateWithTime).toBe("2024-03-15T14:30:00.000Z");
		} finally {
			await tryDelete(id);
		}
	});

	it("Rich text and percent/currency read back", async () => {
		const record = new PrimaryModel({
			primaryKey: primaryKey("Rich"),
			longTextWithRichText: "**bold** and _italic_ text",
			percentInt: 0.5,
			percentFloat: 0.333,
			currencyInt: 100,
			currencyFloat: 19.99,
		});
		await withRetry(() => record.save());
		const id = record.id;
		try {
			const fetched = PrimaryModel.fromId(id);
			await withRetry(() => fetched.fetch());
			expect(fetched.longTextWithRichText).toBe("**bold** and _italic_ text");
			expect(fetched.percentInt).toBe(0.5);
			expect(fetched.percentFloat).toBe(0.333);
			expect(fetched.currencyInt).toBe(100);
			expect(fetched.currencyFloat).toBe(19.99);
		} finally {
			await tryDelete(id);
		}
	});

	it("clearing single and multi select reads back empty", async () => {
		const record = new PrimaryModel({
			primaryKey: primaryKey("ClearSelect"),
			singleSelect: "Choice 1",
			multipleSelect: ["Option 1", "Option 2"],
		});
		await withRetry(() => record.save());
		const id = record.id;
		try {
			expect(record.singleSelect).toBe("Choice 1");

			record.singleSelect = undefined;
			record.multipleSelect = [];
			await withRetry(() => record.save());

			const fetched = PrimaryModel.fromId(id);
			await withRetry(() => fetched.fetch());
			expect(fetched.singleSelect == null).toBe(true);
			expect(!fetched.multipleSelect || fetched.multipleSelect.length === 0).toBe(true);
		} finally {
			await tryDelete(id);
		}
	});

	it("removing a collaborator reads back null", async () => {
		const record = new PrimaryModel({
			primaryKey: primaryKey("RemoveUser"),
			user: { id: USER_ID },
		});
		await withRetry(() => record.save());
		const id = record.id;
		try {
			expect(record.user.id).toBe(USER_ID);

			record.user = undefined;
			await withRetry(() => record.save());

			const fetched = PrimaryModel.fromId(id);
			await withRetry(() => fetched.fetch());
			expect(fetched.user == null).toBe(true);
		} finally {
			await tryDelete(id);
		}
	});

	it("attachment replace and remove read back", { timeout: 120_000 }, async () => {
		const urlA = "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png";
		const urlB = "https://www.w3.org/Icons/w3c_home.png";
		const record = new PrimaryModel({
			primaryKey: primaryKey("Attach"),
			attachment: [{ url: urlA }],
		});
		await withRetry(() => record.save());
		const id = record.id;
		try {
			// Replace the attachment with a different one.
			record.attachment = [{ url: urlB }];
			await withRetry(() => record.save());

			let replaced;
			for (let i = 0; i < 10; i++) {
				await new Promise((resolve) => setTimeout(resolve, 5000));
				replaced = PrimaryModel.fromId(id);
				await withRetry(() => replaced.fetch());
				if (replaced.attachment && replaced.attachment.length === 1) break;
			}
			expect(replaced.attachment).toHaveLength(1);

			// Remove all attachments.
			replaced.attachment = [];
			await withRetry(() => replaced.save());

			const cleared = PrimaryModel.fromId(id);
			await withRetry(() => cleared.fetch());
			expect(!cleared.attachment || cleared.attachment.length === 0).toBe(true);
		} finally {
			await tryDelete(id);
		}
	});
});
