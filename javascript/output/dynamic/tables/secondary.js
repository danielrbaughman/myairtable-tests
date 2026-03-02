// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

// #region REQUIRES
const { AirtableTable } = require("../../static/airtable-table");
const {
    SecondaryViewNameIdMapping,
    SecondaryFieldNameIdMapping,
    SecondaryFieldIdNameMapping,
    SecondaryWritableFieldIds,
} = require("../types/secondary");
// #endregion


class SecondaryTable extends AirtableTable {
    /** Table name (Secondary) */
    static tableName = "Secondary";
    /** Table name (Secondary) */
    get tableName() { return SecondaryTable.tableName; }

    /** Table ID (tblPPScS3XMuFkDYN) */
    static tableId = "tblPPScS3XMuFkDYN";
    /** Table ID (tblPPScS3XMuFkDYN) */
    get tableId() { return SecondaryTable.tableId; }

    constructor(baseId, options) {
        super(baseId, "tblPPScS3XMuFkDYN", SecondaryViewNameIdMapping, SecondaryFieldNameIdMapping, SecondaryFieldIdNameMapping, SecondaryWritableFieldIds, (record) => require("../models/secondary").SecondaryModel.fromRecord(record, { baseId: this.baseId, ...this._options }, false), options);
    }
}

module.exports = { SecondaryTable };
