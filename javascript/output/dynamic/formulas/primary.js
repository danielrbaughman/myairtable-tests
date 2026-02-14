// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

const {
	ID,
	AttachmentsField,
	BooleanField,
	DateField,
	NumberField,
	TextField,
	SingleSelectField,
	MultiSelectField,
} = require("../../static/formula");
const { PrimaryMultipleSelectOptions, PrimarySingleSelectOptions } = require("../types/primary");

const PrimaryFormulas = {
	id: new ID(),
	attachment: new AttachmentsField("fldhF2AEuSC1haCZd"),
	autoNumber: new NumberField("fldizvTkxgIn0mC3L"),
	button: new TextField("fldY48yKPG16AajtU"),
	checkbox: new BooleanField("fldjQIaAZVegb1FUa"),
	createdBy: new TextField("fldGLQhDz2UjjiHG6"),
	createdAtTime: new DateField("fld2YgW382Kt9xltA"),
	currencyFloat: new NumberField("fldyh8pzDXiy5abEr"),
	currencyInt: new NumberField("fldBfo74z9hD78hP8"),
	date: new DateField("fldC6LfNVvVIxKyQH"),
	dateWithTime: new DateField("fldizYmjpXABGDLTG"),
	duration: new NumberField("fldLTyf6ljS0rhur8"),
	email: new TextField("fldHCJoYBiFVsNvP4"),
	formulaComplex: new TextField("fld2vnFc0Bl5IOFUQ"),
	formulaId: new TextField("fldcf62YFeIIDHElt"),
	formulaNested: new TextField("fldXFeHRPBLz6AiWh"),
	formulaSimple: new NumberField("fldy1axxaoUToLVC6"),
	lastModifiedBy: new TextField("fldF8iDttqP0AgzWC"),
	lastModifiedTime: new DateField("fldMinKh4pa3YX86g"),
	linkMultiple: new TextField("fldFyFheQWczd8oux"),
	linkSingle: new TextField("fld7F5onkDo6mkmbN"),
	longText: new TextField("fld8ulc6J0W29M6La"),
	longTextWithRichText: new TextField("fldHJkxCMC0xo343u"),
	lookup: new TextField("fldbmFmrzYKBktJvE"),
	multipleSelect: new MultiSelectField("fld6GTabFmu1xKPvZ"),
	numberFloat: new NumberField("fldmU0X2l4RWd21dd"),
	numberInt: new NumberField("fldOfPKGmnRPv94QH"),
	percentFloat: new NumberField("fldiGui9ll69N7WOj"),
	percentInt: new NumberField("fldbAAyWboGulpb4s"),
	phoneNumber: new TextField("fld38tnNpHmoks8C8"),
	primaryKey: new TextField("fldol5Q4wmQJQvPRy"),
	rating: new TextField("fldRsmwFwQNZkKLp4"),
	rollup: new TextField("fldGaFgBsDC3IBUdV"),
	singleLineText: new TextField("fld0BL2lFo9fqcKv3"),
	singleSelect: new SingleSelectField("fldn0GFFtMFpCXUNU"),
	url: new TextField("fldLYloz2oP4ymf3B"),
	user: new TextField("fldU6SbLp8CSkLcA4"),
	userAllowMultiple: new TextField("fldBwCDbAVxRj9yg7"),
};

module.exports = { PrimaryFormulas };
