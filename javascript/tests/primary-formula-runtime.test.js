import { describe, it, expect } from "vitest";
const { Airtable, PrimaryModel } = require("../output");

// TC6 — Primary "Formula (Complex)"/"Formula (Nested)" evaluated at runtime.
//
// The base runtime suite only covers the Formulas table; the Primary complex formula concatenates
// ~35 fields through IF(field, field, "None"), a richer transpile path. We compare the transpiled
// runtime-evaluated formula to the API line-by-line for the DETERMINISTIC, offline-reproducible
// field types (text, checkbox, single/multi select, numbers, currency, email, url, phone). The
// formula also references server-computed fields (Created/Last Modified Time + By, Auto Number,
// Button, Formula(ID)/(Simple)) and link/lookup/rollup — Airtable renders those from data the
// offline runtime doesn't hold, so those lines are NOT expected to match offline.
//
// This suite also locks in the multi-select array-join fix: a multi-value field coerces to
// "Option 1, Option 2", not just the first element.

const airtable = new Airtable();

// Field labels whose rendering the offline runtime can reproduce exactly.
const DETERMINISTIC_LABELS = [
	"Single Line Text",
	"Long Text",
	"Checkbox",
	"Multiple Select",
	"Single Select",
	"Number (int)",
	"Number (float)",
	"Currency (int)",
	"Currency (float)",
	"Email",
	"URL",
	"Phone Number",
];

function uniqueKey(label) {
	const ts = Date.now();
	const rand = Math.floor(Math.random() * (999999 - 100000) + 100000);
	return `JavaScript PrimaryFormula ${label} ${ts}-${rand}`;
}

function newRecord(key) {
	return new PrimaryModel({
		primaryKey: key,
		singleLineText: "hello",
		longText: "long text",
		email: "a@b.co",
		url: "https://x.co",
		phoneNumber: "555-1212",
		checkbox: true,
		numberInt: 42,
		numberFloat: 3.5,
		currencyInt: 10,
		currencyFloat: 9.99,
		singleSelect: "Choice 1",
		multipleSelect: ["Option 1", "Option 2"],
	});
}

// Extract the "Label: value" line for `label` from a formula result.
function line(formula, label) {
	for (const l of String(formula).split("\n")) {
		if (l.startsWith(label + ": ")) return l;
	}
	return `<missing: ${label}>`;
}

async function tryDelete(id) {
	if (!id) return;
	try {
		await airtable.primary.delete(id);
	} catch {
		// best-effort cleanup
	}
}

describe("Primary Formula Runtime (TC6)", () => {
	it("Complex formula renders deterministic fields like the API", async () => {
		const created = newRecord(uniqueKey("Complex"));
		await created.save();
		const recordId = created.id;
		try {
			const fetched = PrimaryModel.fromId(recordId);
			await fetched.fetch();

			const api = fetched.formulaComplex;
			fetched.evaluateFormulasAtRuntime = true;
			const runtime = fetched.formulaComplex;

			for (const label of DETERMINISTIC_LABELS) {
				expect(line(runtime, label)).toEqual(line(api, label));
			}

			// The multi-select join is the headline fix: both sides render all options, comma-joined.
			expect(line(runtime, "Multiple Select")).toEqual("Multiple Select: Option 1, Option 2");
		} finally {
			await tryDelete(recordId);
		}
	}, 60000);

	it("Nested formula evaluates without throwing", async () => {
		const created = newRecord(uniqueKey("Nested"));
		await created.save();
		const recordId = created.id;
		try {
			const fetched = PrimaryModel.fromId(recordId);
			await fetched.fetch();

			fetched.evaluateFormulasAtRuntime = true;
			const runtime = fetched.formulaNested; // must not throw
			expect(runtime).not.toBeUndefined();
		} finally {
			await tryDelete(recordId);
		}
	}, 60000);
});
