// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

// #region REQUIRES
const { AirtableTable } = require("../../static/airtable-table");
const {
    FormulasViewNameIdMapping,
    FormulasFieldNameIdMapping,
    FormulasFieldIdNameMapping,
    FormulasWritableFieldIds,
} = require("../types/formulas");
// #endregion


class FormulasTable extends AirtableTable {
    /** Table name (Formulas) */
    static tableName = "Formulas";
    /** Table name (Formulas) */
    get tableName() { return FormulasTable.tableName; }

    /** Table ID (tblnuYBsMdXNDsuRc) */
    static tableId = "tblnuYBsMdXNDsuRc";
    /** Table ID (tblnuYBsMdXNDsuRc) */
    get tableId() { return FormulasTable.tableId; }

    constructor(baseId, options) {
        super(baseId, "tblnuYBsMdXNDsuRc", FormulasViewNameIdMapping, FormulasFieldNameIdMapping, FormulasFieldIdNameMapping, FormulasWritableFieldIds, (record) => require("../models/formulas").FormulasModel.fromRecord(record, { baseId: this.baseId, ...this._options }), options);
    }
}

module.exports = { FormulasTable };
