// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

// #region IMPORTS
import { AirtableTable } from "../../static/airtable-table";
import {
    SecondaryFieldSet,
    SecondaryField,
    SecondaryView,
    SecondaryViewNameIdMapping,
    SecondaryFieldNameIdMapping,
    SecondaryFieldIdNameMapping,
    SecondaryWritableFieldIds,
} from "../types/secondary";
import { SecondaryModel } from '../models/secondary';
import { AirtableOptions } from "airtable";
// #endregion


export class SecondaryTable extends AirtableTable<SecondaryFieldSet, SecondaryModel, SecondaryView, SecondaryField> {
    /** Table name (Secondary) */
    public static tableName: string = "Secondary";
    /** Table name (Secondary) */
    public get tableName(): string { return SecondaryTable.tableName; }

    /** Table ID (tblPPScS3XMuFkDYN) */
    public static tableId: string = "tblPPScS3XMuFkDYN";
    /** Table ID (tblPPScS3XMuFkDYN) */
    public get tableId(): string { return SecondaryTable.tableId; }

    constructor(baseId: string, options: AirtableOptions) {
        super(baseId, "tblPPScS3XMuFkDYN", SecondaryViewNameIdMapping, SecondaryFieldNameIdMapping, SecondaryFieldIdNameMapping, SecondaryWritableFieldIds, (record) => SecondaryModel.fromRecord(record, { baseId: this.baseId, ...this._options }), options);
    }
}
