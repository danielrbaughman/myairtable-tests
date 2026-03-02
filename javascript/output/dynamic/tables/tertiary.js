// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

// #region REQUIRES
const { AirtableTable } = require("../../static/airtable-table");
const {
    TertiaryViewNameIdMapping,
    TertiaryFieldNameIdMapping,
    TertiaryFieldIdNameMapping,
    TertiaryWritableFieldIds,
} = require("../types/tertiary");
// #endregion


class TertiaryTable extends AirtableTable {
    /** Table name (Tertiary) */
    static tableName = "Tertiary";
    /** Table name (Tertiary) */
    get tableName() { return TertiaryTable.tableName; }

    /** Table ID (tblLFoLxEdWlxjmLP) */
    static tableId = "tblLFoLxEdWlxjmLP";
    /** Table ID (tblLFoLxEdWlxjmLP) */
    get tableId() { return TertiaryTable.tableId; }

    constructor(baseId, options) {
        super(baseId, "tblLFoLxEdWlxjmLP", TertiaryViewNameIdMapping, TertiaryFieldNameIdMapping, TertiaryFieldIdNameMapping, TertiaryWritableFieldIds, (record) => require("../models/tertiary").TertiaryModel.fromRecord(record, { baseId: this.baseId, ...this._options }, false), options);
    }
}

module.exports = { TertiaryTable };
