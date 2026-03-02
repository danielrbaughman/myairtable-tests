// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

// #region IMPORTS
import { AirtableTable } from "../../static/airtable-table";
import {
    PrimaryFieldSet,
    PrimaryField,
    PrimaryView,
    PrimaryViewNameIdMapping,
    PrimaryFieldNameIdMapping,
    PrimaryFieldIdNameMapping,
    PrimaryWritableFieldIds,
} from "../types/primary";
import { PrimaryModel } from '../models/primary';
import { AirtableOptions } from "airtable";
// #endregion


export class PrimaryTable extends AirtableTable<PrimaryFieldSet, PrimaryModel, PrimaryView, PrimaryField> {
    /** Table name (Primary) */
    public static tableName: string = "Primary";
    /** Table name (Primary) */
    public get tableName(): string { return PrimaryTable.tableName; }

    /** Table ID (tblmb3iqgpNS1ysV2) */
    public static tableId: string = "tblmb3iqgpNS1ysV2";
    /** Table ID (tblmb3iqgpNS1ysV2) */
    public get tableId(): string { return PrimaryTable.tableId; }

    constructor(baseId: string, options: AirtableOptions) {
        super(baseId, "tblmb3iqgpNS1ysV2", PrimaryViewNameIdMapping, PrimaryFieldNameIdMapping, PrimaryFieldIdNameMapping, PrimaryWritableFieldIds, (record) => PrimaryModel.fromRecord(record, { baseId: this.baseId, ...this._options }, false), options);
    }
}
