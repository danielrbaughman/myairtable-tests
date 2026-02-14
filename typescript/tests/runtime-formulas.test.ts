import { describe, it, expect, beforeAll } from "vitest";
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
		expect(record.mathFormula()).toEqual(record.mathFormula(true));
	});

	it("Airtable and Runtime Formulas should match (text)", async () => {
		expect(record.textFormula()).toEqual(record.textFormula(true));
	});

	it("Airtable and Runtime Formulas should match (date)", async () => {
		expect(record.dateFormula()).toEqual(record.dateFormula(true));
	});
});
