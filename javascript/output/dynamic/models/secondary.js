// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

// #region REQUIRES
const { AirtableModel } = require("../../static/airtable-model");
const {
    LinkedRecord,
    LinkedRecords,
} = require("../../static/linked-record");
const {
    getOptions,
    getBaseId,
    buildUrl,
} = require("../../static/helpers");
const {
    SecondaryFieldNameIdMapping,
    SecondaryFieldIdNameMapping,
    SecondaryFieldNamePropertyMapping,
    SecondaryViewNameIdMapping,
} = require("../types/secondary");
const { SecondaryFormulas } = require("../formulas/secondary");
const { SecondaryTable } = require("../tables/secondary");
const { SecondarySchema } = require("../zod/secondary");
// #endregion


// #region SECONDARY
/** Model for `Secondary` (tblPPScS3XMuFkDYN) */
class SecondaryModel extends AirtableModel {
    static schema = SecondarySchema;
    static f = SecondaryFormulas;
    static nameToIdMap = SecondaryFieldNameIdMapping;
    static idToNameMap = SecondaryFieldIdNameMapping;
    static nameToPropertyMap = SecondaryFieldNamePropertyMapping;
    /** Table name (Secondary) */
    static tableName = 'Secondary';
    /** Table name (Secondary) */
    get tableName() { return SecondaryModel.tableName; }

    /** Table ID (tblPPScS3XMuFkDYN) */
    static tableId = 'tblPPScS3XMuFkDYN';
    /** Table ID (tblPPScS3XMuFkDYN) */
    get tableId() { return SecondaryModel.tableId; }

    static fieldDescriptors = [
        { propertyName: "linkToTertiary", fieldId: "fldKR6tdbnOBRCtdQ", fieldName: "Link to Tertiary", isComputed: false, fieldType: "linkedRecords", linkedModelFromId: (id, config) => require("./tertiary").TertiaryModel.fromId(id, config) },
        { propertyName: "name", fieldId: "fld1RagdJ09mpWhzM", fieldName: "Name", isComputed: false, fieldType: "generic" },
        { propertyName: "primary", fieldId: "fldl0nB9WRFSdqlii", fieldName: "Primary", isComputed: false, fieldType: "linkedRecords", linkedModelFromId: (id, config) => require("./primary").PrimaryModel.fromId(id, config) },
        { propertyName: "primary2", fieldId: "fldgoE2oZmXmKkQca", fieldName: "Primary 2", isComputed: false, fieldType: "linkedRecords", linkedModelFromId: (id, config) => require("./primary").PrimaryModel.fromId(id, config) },
        { propertyName: "value", fieldId: "fldi6Mxh5H1gPGxFX", fieldName: "Value", isComputed: false, fieldType: "generic" },
    ];

    /** `Link to Tertiary` (fldKR6tdbnOBRCtdQ) */
    get linkToTertiary() { return this._fields["linkToTertiary"]; }
    set linkToTertiary(value) { this._fields["linkToTertiary"] = value; this.markDirty('linkToTertiary'); }
    /** `Name` (fld1RagdJ09mpWhzM) */
    get name() { return this._fields["name"]; }
    set name(value) { this._fields["name"] = value; this.markDirty('name'); }
    /** `Primary` (fldl0nB9WRFSdqlii) */
    get primary() { return this._fields["primary"]; }
    set primary(value) { this._fields["primary"] = value; this.markDirty('primary'); }
    /** `Primary 2` (fldgoE2oZmXmKkQca) */
    get primary2() { return this._fields["primary2"]; }
    set primary2(value) { this._fields["primary2"] = value; this.markDirty('primary2'); }
    /** `Value` (fldi6Mxh5H1gPGxFX) */
    get value() { return this._fields["value"]; }
    set value(value) { this._fields["value"] = value; this.markDirty('value'); }

    constructor(data = {}) {
        super(data.id ?? '');
        this.initializeFields(data);
        this.record = new (require('airtable').Record)(new SecondaryTable(this.getInstanceBaseId(), this.getInstanceOptions())._table, this.id, {});
        this.updateRecord();
    }

    /** Get the URL for this record in Airtable, with optional view. */
    url(view) {
        if (view) {
            return buildUrl(this.getInstanceBaseId(), 'tblPPScS3XMuFkDYN', this.id, SecondaryViewNameIdMapping[view]);
        } else {
            return buildUrl(this.getInstanceBaseId(), 'tblPPScS3XMuFkDYN', this.id);
        }
    }

}
// #endregion

module.exports = { SecondaryModel };
