// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

// #region IMPORTS
import { AirtableTable } from "../../static/airtable-table";
import {
    TertiaryFieldSet,
    TertiaryField,
    TertiaryView,
    TertiaryViewNameIdMapping,
    TertiaryFieldNameIdMapping,
    TertiaryFieldIdNameMapping,
    TertiaryWritableFieldIds,
} from "../types/tertiary";
import { TertiaryModel } from '../models/tertiary';
import { AirtableOptions } from "airtable";
// #endregion


export class TertiaryTable extends AirtableTable<TertiaryFieldSet, TertiaryModel, TertiaryView, TertiaryField> {
    /** Table name (Tertiary) */
    public static tableName: string = "Tertiary";
    /** Table name (Tertiary) */
    public get tableName(): string { return TertiaryTable.tableName; }

    /** Table ID (tblLFoLxEdWlxjmLP) */
    public static tableId: string = "tblLFoLxEdWlxjmLP";
    /** Table ID (tblLFoLxEdWlxjmLP) */
    public get tableId(): string { return TertiaryTable.tableId; }

    constructor(baseId: string, options: AirtableOptions) {
        super(baseId, "tblLFoLxEdWlxjmLP", TertiaryViewNameIdMapping, TertiaryFieldNameIdMapping, TertiaryFieldIdNameMapping, TertiaryWritableFieldIds, (record) => TertiaryModel.fromRecord(record, { baseId: this.baseId, ...this._options }), options);
    }
}
