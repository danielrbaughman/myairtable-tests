// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

// #region IMPORTS
import { AirtableTable } from "../../static/airtable-table";
import {
    FormulasFieldSet,
    FormulasField,
    FormulasView,
    FormulasViewNameIdMapping,
    FormulasFieldNameIdMapping,
    FormulasFieldIdNameMapping,
    FormulasWritableFieldIds,
} from "../types/formulas";
import { FormulasModel } from "../models/formulas";
import { AirtableOptions } from "airtable";
// #endregion


export class FormulasTable extends AirtableTable<FormulasFieldSet, FormulasModel, FormulasView, FormulasField> {
    /** Table name (Formulas) */
    public static tableName: string = "Formulas";
    /** Table name (Formulas) */
    public get tableName(): string { return FormulasTable.tableName; }

    /** Table ID (tblnuYBsMdXNDsuRc) */
    public static tableId: string = "tblnuYBsMdXNDsuRc";
    /** Table ID (tblnuYBsMdXNDsuRc) */
    public get tableId(): string { return FormulasTable.tableId; }

    constructor(baseId: string, options: AirtableOptions) {
        super(baseId, "tblnuYBsMdXNDsuRc", FormulasViewNameIdMapping, FormulasFieldNameIdMapping, FormulasFieldIdNameMapping, FormulasWritableFieldIds, (record) => FormulasModel.fromRecord(record, { baseId: this.baseId, ...this._options }, false), options);
    }
}
