import { describe, it, expect } from "vitest";
import { Airtable, PrimaryModel } from "../output";

// TC7 — Field-type round-trip completeness via re-fetch. Several field types were only asserted on
// the create response (or only offline-decoded), never written and read back through the live API,
// and clearing/removing multi-value fields was untested. Each case here creates, optionally updates,
// re-fetches, and asserts the server-side value. Parity target for the other 8 suites.

const airtable = new Airtable();

const USER_ID = "usrnZ4k98m0Ipji4e"; // shared-base user, as in the complex-properties suite

const TIMEOUT = 60_000;

async function tryDelete(recordId: string | undefined): Promise<void> {
	if (!recordId) return;
	try {
		await airtable.primary.delete(recordId);
	} catch {
		// best-effort cleanup
	}
}

describe("TC7 — Field-type round-trip via re-fetch", () => {
	it(
		"DateWithTime writes and reads back",
		async () => {
			const dt = "2024-03-15T14:30:00.000Z";
			const created = await airtable.primary.create(
				new PrimaryModel({ primaryKey: "FieldRT DateTime", dateWithTime: dt }),
			);
			const recordId = created.id!;
			try {
				const fetched = await airtable.primary.get(recordId);
				expect(fetched.dateWithTime).toBe(dt);
			} finally {
				await tryDelete(recordId);
			}
		},
		TIMEOUT,
	);

	it(
		"rich text and percent/currency read back",
		async () => {
			const created = await airtable.primary.create(
				new PrimaryModel({
					primaryKey: "FieldRT Rich",
					longTextWithRichText: "**bold** and _italic_ text",
					percentInt: 0.5,
					percentFloat: 0.333,
					currencyInt: 100,
					currencyFloat: 19.99,
				}),
			);
			const recordId = created.id!;
			try {
				const fetched = await airtable.primary.get(recordId);
				expect(fetched.longTextWithRichText).toBe("**bold** and _italic_ text");
				expect(fetched.percentInt).toBe(0.5);
				expect(fetched.percentFloat).toBe(0.333);
				expect(fetched.currencyInt).toBe(100);
				expect(fetched.currencyFloat).toBe(19.99);
			} finally {
				await tryDelete(recordId);
			}
		},
		TIMEOUT,
	);

	it(
		"clearing single and multi select reads back empty",
		async () => {
			const created = await airtable.primary.create(
				new PrimaryModel({
					primaryKey: "FieldRT ClearSelect",
					singleSelect: "Choice 1",
					multipleSelect: ["Option 1", "Option 2"],
				}),
			);
			const recordId = created.id!;
			try {
				expect(created.singleSelect).toBe("Choice 1");

				created.singleSelect = undefined;
				created.multipleSelect = [];
				await airtable.primary.update(created);

				const fetched = await airtable.primary.get(recordId);
				expect(fetched.singleSelect).toBeFalsy();
				expect(fetched.multipleSelect ?? []).toHaveLength(0);
			} finally {
				await tryDelete(recordId);
			}
		},
		TIMEOUT,
	);

	it(
		"removing a collaborator reads back null",
		async () => {
			const created = await airtable.primary.create(
				new PrimaryModel({
					primaryKey: "FieldRT RemoveUser",
					user: { id: USER_ID },
				}),
			);
			const recordId = created.id!;
			try {
				expect(created.user!.id).toBe(USER_ID);

				created.user = undefined;
				await airtable.primary.update(created);

				const fetched = await airtable.primary.get(recordId);
				expect(fetched.user).toBeFalsy();
			} finally {
				await tryDelete(recordId);
			}
		},
		TIMEOUT,
	);

	it(
		"attachment replace and remove read back",
		async () => {
			const urlA = "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png";
			const urlB = "https://www.w3.org/Icons/w3c_home.png";
			const created = await airtable.primary.create(
				new PrimaryModel({
					primaryKey: "FieldRT Attach",
					attachment: [{ url: urlA }] as any,
				}),
			);
			const recordId = created.id!;
			try {
				// Replace the attachment with a different one.
				created.attachment = [{ url: urlB }] as any;
				await airtable.primary.update(created);
				const replaced = await airtable.primary.get(recordId);
				expect(replaced.attachment).toHaveLength(1);

				// Remove all attachments.
				replaced.attachment = [];
				await airtable.primary.update(replaced);
				const cleared = await airtable.primary.get(recordId);
				expect(cleared.attachment ?? []).toHaveLength(0);
			} finally {
				await tryDelete(recordId);
			}
		},
		TIMEOUT,
	);
});
