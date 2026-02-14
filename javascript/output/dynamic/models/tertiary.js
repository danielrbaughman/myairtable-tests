// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

// #region REQUIRES
const { AirtableModel } = require("../../static/airtable-model");
const { LinkedRecord, LinkedRecords } = require("../../static/linked-record");
const { getOptions, getBaseId } = require("../../static/helpers");
const {
	TertiaryFieldNameIdMapping,
	TertiaryFieldIdNameMapping,
	TertiaryFieldNamePropertyMapping,
} = require("../types/tertiary");
const { TertiaryFormulas } = require("../formulas/tertiary");
const { TertiaryTable } = require("../tables/tertiary");
const { TertiarySchema } = require("../zod/tertiary");
// #endregion

// #region TERTIARY
/** Model for `Tertiary` (tblLFoLxEdWlxjmLP) */
class TertiaryModel extends AirtableModel {
	static schema = TertiarySchema;
	static f = TertiaryFormulas;
	static nameToIdMap = TertiaryFieldNameIdMapping;
	static idToNameMap = TertiaryFieldIdNameMapping;
	static nameToPropertyMap = TertiaryFieldNamePropertyMapping;
	/** Table name (Tertiary) */
	static tableName = "Tertiary";
	/** Table name (Tertiary) */
	get tableName() {
		return TertiaryModel.tableName;
	}

	/** Table ID (tblLFoLxEdWlxjmLP) */
	static tableId = "tblLFoLxEdWlxjmLP";
	/** Table ID (tblLFoLxEdWlxjmLP) */
	get tableId() {
		return TertiaryModel.tableId;
	}

	static fieldDescriptors = [
		{ propertyName: "name", fieldId: "fldwzqKxsRnPZJ2Ll", fieldName: "Name", isComputed: false, fieldType: "generic" },
		{
			propertyName: "secondary",
			fieldId: "fld8lCuUXpEXkIeYv",
			fieldName: "Secondary",
			isComputed: false,
			fieldType: "linkedRecords",
			linkedModelFromId: (id, config) => require("./secondary").SecondaryModel.fromId(id, config),
		},
		{
			propertyName: "value",
			fieldId: "fldjNLBh2UccM64h5",
			fieldName: "Value",
			isComputed: false,
			fieldType: "generic",
		},
	];

	/** `Name` (fldwzqKxsRnPZJ2Ll) */
	get name() {
		return this._fields["name"];
	}
	set name(value) {
		this._fields["name"] = value;
		this.markDirty("name");
	}
	/** `Secondary` (fld8lCuUXpEXkIeYv) */
	get secondary() {
		return this._fields["secondary"];
	}
	set secondary(value) {
		this._fields["secondary"] = value;
		this.markDirty("secondary");
	}
	/** `Value` (fldjNLBh2UccM64h5) */
	get value() {
		return this._fields["value"];
	}
	set value(value) {
		this._fields["value"] = value;
		this.markDirty("value");
	}

	constructor(data = {}) {
		super(data.id ?? "");
		this.initializeFields(data);
		this.record = new (require("airtable").Record)(
			new TertiaryTable(this.getInstanceBaseId(), this.getInstanceOptions())._table,
			this.id,
			{},
		);
		this.updateRecord();
	}
}
// #endregion

module.exports = { TertiaryModel };
