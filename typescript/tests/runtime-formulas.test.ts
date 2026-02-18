import { describe, it, expect, beforeAll, afterAll } from "vitest";
import { Airtable, FormulasModel } from "../output";

const airtable = new Airtable();

describe("Runtime Formulas", async () => {
	let record: FormulasModel;

	beforeAll(async () => {
		const newRecord = new FormulasModel({
			primaryKey: "New Primary Key",
			firstNumber: 10,
			secondNumber: 20,
			thirdNumber: 30,
			firstText: "Hello",
			secondText: "World",
			thirdText: "!",
			firstDate: new Date("2024-01-01").toISOString(),
			secondDate: new Date("2024-02-01").toISOString(),
			thirdDate: new Date("2024-03-01").toISOString(),
		});
		record = await airtable.formulas.create(newRecord);
	});

	it("Airtable and Runtime Formulas should match (math)", async () => {
		const fromAPI = record.mathFormula;
		record.evaluateFormulasAtRuntime = true;
		const runtime = record.mathFormula;
		expect(fromAPI).toEqual(runtime);
	});

	it("Airtable and Runtime Formulas should match (text)", async () => {
		const fromAPI = record.textFormula;
		record.evaluateFormulasAtRuntime = true;
		const runtime = record.textFormula;
		expect(fromAPI).toEqual(runtime);
	});

	it("Airtable and Runtime Formulas should match (date)", async () => {
		const fromAPI = record.dateFormula;
		record.evaluateFormulasAtRuntime = true;
		const runtime = record.dateFormula;
		expect(fromAPI).toEqual(runtime);
	});

	afterAll(async () => {
		await record.delete();
	});
});
