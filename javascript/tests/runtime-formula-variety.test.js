import { describe, it, expect } from "vitest";
const { Airtable, FormulasModel } = require("../output");

// TC4 — Runtime-formula input variety.
//
// The base "Runtime Formulas" suite only evaluates the kitchen-sink formulas with ONE
// fully-populated input set, so the IF(OR(...=BLANK())) short-circuit and varied inputs
// are never exercised. Each case here creates a Formulas record with a specific input set,
// fetches the API-computed value, and asserts the transpiled runtime evaluation reproduces it.
//
// Scope notes (kept deliberately portable across all targets, matching the C# suite):
//  • Numbers are positive with LOG/SQRT arguments chosen to be exact (powers of 10 / perfect
//    squares). Transcendental results (LOG/SQRT/EXP) differ by a ULP between platforms and
//    Airtable (V8), so irrational results are NOT bit-identical and would make an exact string
//    compare flaky. Negatives/zero are excluded too: LOG(negative) and MOD(_,0) error.
//  • Text covers whitespace edges, unicode, and reserved punctuation (exercising the fixed
//    ENCODE_URL_COMPONENT). An all-blank text input is excluded: Airtable returns blank for the
//    whole formula while the transpiler is lenient.

const airtable = new Airtable();

const TIMEOUT = 30000;

function baseModel(label) {
	return new FormulasModel({
		primaryKey: "JavaScript Variety " + label,
		firstDate: new Date("2024-01-01T00:00:00.000Z").toISOString(),
		secondDate: new Date("2024-02-01T00:00:00.000Z").toISOString(),
		thirdDate: new Date("2024-03-01T00:00:00.000Z").toISOString(),
	});
}

async function tryDelete(record) {
	if (!record || !record.id) return;
	try {
		await airtable.formulas.delete(record.id);
	} catch {
		// best-effort cleanup
	}
}

function isBlank(value) {
	return value === null || value === undefined || value === "";
}

describe("TC4 — Runtime-formula input variety", () => {
	// First Number: power of 10 (LOG exact). Second Number: perfect square (SQRT exact).
	const numberCases = [
		["hundreds", 100, 16, 8],
		["ones", 1, 4, 2],
		["tens", 10, 25, 3],
	];

	for (const [label, a, b, c] of numberCases) {
		it(
			`math formula matches API for varied numbers (${label})`,
			async () => {
				const model = baseModel("Math " + label);
				model.firstNumber = a;
				model.secondNumber = b;
				model.thirdNumber = c;
				model.firstText = "x";
				model.secondText = "y";
				model.thirdText = "z";
				const created = await airtable.formulas.create(model);
				try {
					const fetched = await airtable.formulas.get(created.id);
					const fromAPI = fetched.mathFormula;
					fetched.evaluateFormulasAtRuntime = true;
					const runtime = fetched.mathFormula;
					expect(runtime).toEqual(fromAPI);
				} finally {
					await tryDelete(created);
				}
			},
			TIMEOUT,
		);
	}

	it(
		"math formula returns blank when numbers missing",
		async () => {
			// First/Second Number left unset -> OR(BLANK, BLANK) is true -> formula returns BLANK().
			// This is the IF-true short-circuit the base suite never reaches.
			const model = baseModel("Blank");
			model.firstText = "x";
			model.secondText = "y";
			model.thirdText = "z";
			const created = await airtable.formulas.create(model);
			try {
				const fetched = await airtable.formulas.get(created.id);
				const fromAPI = fetched.mathFormula;
				fetched.evaluateFormulasAtRuntime = true;
				const runtime = fetched.mathFormula;
				expect(isBlank(fromAPI)).toBe(true);
				expect(isBlank(runtime)).toBe(true);
			} finally {
				await tryDelete(created);
			}
		},
		TIMEOUT,
	);

	const textCases = [
		["unicode", "café", "naïve", "日本語🎉"],
		["whitespace", "  he llo  ", "a b", "c"],
		["punct", "a.e-i+o", "x/y", "z"], // exercises fixed ENCODE_URL_COMPONENT
	];

	for (const [label, a, b, c] of textCases) {
		it(
			`text formula matches API for varied text (${label})`,
			async () => {
				const model = baseModel("Text " + label);
				model.firstNumber = 10;
				model.secondNumber = 20;
				model.thirdNumber = 30;
				model.firstText = a;
				model.secondText = b;
				model.thirdText = c;
				const created = await airtable.formulas.create(model);
				try {
					const fetched = await airtable.formulas.get(created.id);
					const fromAPI = fetched.textFormula;
					fetched.evaluateFormulasAtRuntime = true;
					const runtime = fetched.textFormula;
					expect(runtime).toEqual(fromAPI);
				} finally {
					await tryDelete(created);
				}
			},
			TIMEOUT,
		);
	}
});
