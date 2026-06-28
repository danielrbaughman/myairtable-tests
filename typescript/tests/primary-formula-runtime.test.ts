import { describe, it, expect } from "vitest";
import { Airtable, PrimaryModel } from "../output";

// TC6 — Primary "Formula (Complex)"/"Formula (Nested)" evaluated at runtime.
//
// The Primary complex formula concatenates ~35 fields through IF(field, field, "None"),
// a richer transpile path than the Formulas-table runtime suite covers. We compare the
// transpiled runtime evaluation to the API line-by-line for the DETERMINISTIC,
// offline-reproducible field types (text, checkbox, single/multi select, numbers,
// currency, email, url, phone). The formula also references server-computed fields
// (Created/Last Modified Time + By, Auto Number, Button, Formula(ID)/(Simple)) and
// link/lookup/rollup — Airtable renders those from data the offline runtime doesn't hold,
// so those lines are NOT expected to match offline.
//
// This suite specifically locks in the multi-select array-join fix: a multi-value field
// coerces to "Option 1, Option 2", not just the first element.

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

function newRecord(suite: string): PrimaryModel {
	return new PrimaryModel({
		primaryKey: suite,
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
function line(formula: string, label: string): string {
	for (const l of formula.split("\n")) {
		if (l.startsWith(label + ": ")) return l;
	}
	return `<missing: ${label}>`;
}

describe("TC6 — Primary Complex/Nested formula at runtime", () => {
	it("Complex formula renders deterministic fields like API", async () => {
		const created = await airtable.primary.create(newRecord("PrimaryFormula Complex"));
		const recordId = created.id!;
		try {
			const fetched = await airtable.primary.get(recordId);
			const api = fetched.formulaComplex ?? "";
			fetched.evaluateFormulasAtRuntime = true;
			const runtime = fetched.formulaComplex ?? "";
			// eslint-disable-next-line no-console
			console.log(`--- API ---\n${api}\n--- RUNTIME ---\n${runtime}`);

			for (const label of DETERMINISTIC_LABELS) {
				expect(line(runtime, label)).toEqual(line(api, label));
			}

			// The multi-select join is the headline fix: both sides render all options, comma-joined.
			expect(line(runtime, "Multiple Select")).toEqual("Multiple Select: Option 1, Option 2");
		} finally {
			try {
				await airtable.primary.delete(recordId);
			} catch {
				/* ignore */
			}
		}
	}, 60000);

	it("Nested formula evaluates without throwing", async () => {
		// Formula (Nested) = Formula(ID) & Formula(Simple) & Formula(Complex) — it chains three
		// COMPUTED formula fields. Offline the runtime can't reproduce computed-field values, so
		// the content isn't asserted; this confirms the transpiled nested-formula getter evaluates
		// without error.
		const created = await airtable.primary.create(newRecord("PrimaryFormula Nested"));
		const recordId = created.id!;
		try {
			const fetched = await airtable.primary.get(recordId);
			fetched.evaluateFormulasAtRuntime = true;
			const runtime = fetched.formulaNested; // must not throw
			expect(runtime).not.toBeNull();
		} finally {
			try {
				await airtable.primary.delete(recordId);
			} catch {
				/* ignore */
			}
		}
	}, 60000);
});
