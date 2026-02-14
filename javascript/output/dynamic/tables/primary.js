// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

// #region REQUIRES
const { AirtableTable } = require("../../static/airtable-table");
const {
    PrimaryViewNameIdMapping,
    PrimaryFieldNameIdMapping,
    PrimaryFieldIdNameMapping,
    PrimaryWritableFieldIds,
} = require("../types/primary");
// #endregion


class PrimaryTable extends AirtableTable {
    /** Table name (Primary) */
    static tableName = "Primary";
    /** Table name (Primary) */
    get tableName() { return PrimaryTable.tableName; }

    /** Table ID (tblmb3iqgpNS1ysV2) */
    static tableId = "tblmb3iqgpNS1ysV2";
    /** Table ID (tblmb3iqgpNS1ysV2) */
    get tableId() { return PrimaryTable.tableId; }

    constructor(baseId, options) {
        super(baseId, "tblmb3iqgpNS1ysV2", PrimaryViewNameIdMapping, PrimaryFieldNameIdMapping, PrimaryFieldIdNameMapping, PrimaryWritableFieldIds, (record) => require("../models/primary").PrimaryModel.fromRecord(record, { baseId: this.baseId, ...this._options }), options);
    }
}

module.exports = { PrimaryTable };
