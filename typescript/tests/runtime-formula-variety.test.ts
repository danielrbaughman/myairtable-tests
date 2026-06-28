import { describe, it, expect } from "vitest";
import { Airtable, FormulasModel } from "../output";

/**
 * TC4 — Runtime-formula input variety. The base runtime-formulas suite only evaluates the
 * kitchen-sink formulas with ONE fully-populated input set, so the IF(OR(...=BLANK()))
 * short-circuit and varied inputs are never exercised. Each case here creates a Formulas
 * record with a specific input set, fetches the API-computed value, and asserts the
 * transpiled runtime evaluation reproduces it.
 *
 * Scope notes (kept deliberately portable across all targets):
 *  • Numbers are positive with LOG/SQRT arguments chosen to be exact (powers of 10 / perfect
 *    squares). The Math formula calls LOG()/SQRT()/EXP() — transcendental results differ by a
 *    ULP between platforms' math libs and Airtable (V8), so irrational results (e.g. LOG(5))
 *    are NOT bit-identical and would make an exact string compare flaky. Negatives/zero are
 *    excluded too: LOG(negative) and MOD(_,0) error inside this formula.
 *  • Text covers empty-ish edges (whitespace), unicode, and reserved punctuation (exercising
 *    the fixed ENCODE_URL_COMPONENT). An all-blank text input is excluded: Airtable returns
 *    blank for the whole formula (REPLACE past end-of-string errors), while the transpiler is
 *    lenient.
 */

const airtable = new Airtable();

function base(label: string): FormulasModel {
	return new FormulasModel({
		primaryKey: "TS Variety " + label,
		firstDate: new Date("2024-01-01T00:00:00.000Z").toISOString(),
		secondDate: new Date("2024-02-01T00:00:00.000Z").toISOString(),
		thirdDate: new Date("2024-03-01T00:00:00.000Z").toISOString(),
	});
}

async function tryDelete(record?: FormulasModel): Promise<void> {
	if (!record?.id) return;
	try {
		await record.delete();
	} catch {
		// best-effort cleanup
	}
}

// First Number: power of 10 (LOG exact). Second Number: perfect square (SQRT exact).
const numberCases: [string, number, number, number][] = [
	["hundreds", 100, 16, 8],
	["ones", 1, 4, 2],
	["tens", 10, 25, 3],
];

describe("Runtime Formula Variety", () => {
	it.each(numberCases)(
		"math formula matches API for varied numbers (%s)",
		async (label, a, b, c) => {
			const model = base("Math " + label);
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
		30000,
	);

	it("math formula returns blank when numbers missing", async () => {
		// First/Second Number left unset -> OR(BLANK, BLANK) is true -> the formula
		// returns BLANK(). This is the IF-true short-circuit the base suite never reaches.
		const model = base("Blank");
		model.firstText = "x";
		model.secondText = "y";
		model.thirdText = "z";
		const created = await airtable.formulas.create(model);
		try {
			const fetched = await airtable.formulas.get(created.id);
			const fromAPI = fetched.mathFormula;
			fetched.evaluateFormulasAtRuntime = true;
			const runtime = fetched.mathFormula;
			expect(fromAPI ?? "").toEqual("");
			expect(runtime ?? "").toEqual("");
		} finally {
			await tryDelete(created);
		}
	}, 30000);

	const textCases: [string, string, string, string][] = [
		["unicode", "café", "naïve", "日本語🎉"],
		["whitespace", "  he llo  ", "a b", "c"],
		["punct", "a.e-i+o", "x/y", "z"], // exercises fixed ENCODE_URL_COMPONENT
	];

	it.each(textCases)(
		"text formula matches API for varied text (%s)",
		async (label, a, b, c) => {
			const model = base("Text " + label);
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
		30000,
	);
});
