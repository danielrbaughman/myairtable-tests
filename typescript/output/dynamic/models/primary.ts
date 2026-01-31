// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

// #region IMPORTS
import { AirtableOptions, Attachment, Collaborator, FieldSet, Record } from "airtable";
import { AirtableModel, FieldDescriptor } from "../../static/airtable-model";
import { RecordId, AirtableButton } from "../../static/special-types";
import { LinkedRecord, LinkedRecords } from "../../static/linked-record";
import { getOptions, getBaseId } from "../../static/helpers";
import {
    PrimaryFieldSet,
    PrimaryField,
    PrimaryFieldNameIdMapping,
    PrimaryFieldIdNameMapping,
    PrimaryFieldNamePropertyMapping,
} from "../types/primary";
import { PrimaryFormulas } from '../formulas/primary';
import {
} from "../models";
import { PrimaryTable } from '../tables/primary';
import { PrimarySchema, IPrimary } from '../zod/primary';
// #endregion


// #region PRIMARY
/** Model for `Primary` (tblmb3iqgpNS1ysV2) */
export class PrimaryModel extends AirtableModel<PrimaryFieldSet, IPrimary, PrimaryField> {
    protected static schema = PrimarySchema;
    public static f = PrimaryFormulas
    protected nameToIdMap = PrimaryFieldNameIdMapping;
    protected idToNameMap = PrimaryFieldIdNameMapping;
    protected nameToPropertyMap = PrimaryFieldNamePropertyMapping;

    /** Table name (Primary) */
    public static tableName: string = 'Primary';
    /** Table name (Primary) */
    public get tableName(): string { return PrimaryModel.tableName; }

    /** Table ID (tblmb3iqgpNS1ysV2) */
    public static tableId: string = 'tblmb3iqgpNS1ysV2';
    /** Table ID (tblmb3iqgpNS1ysV2) */
    public get tableId(): string { return PrimaryModel.tableId; }

    protected static fieldDescriptors: FieldDescriptor[] = [
        { propertyName: "primarykey", fieldId: "fldol5Q4wmQJQvPRy", fieldName: "PrimaryKey", isComputed: false, fieldType: "generic" },
        { propertyName: "singlelinetext", fieldId: "fld0BL2lFo9fqcKv3", fieldName: "SingleLineText", isComputed: false, fieldType: "generic" },
    ];

    /** `PrimaryKey` (fldol5Q4wmQJQvPRy) */
    public get primarykey(): string | undefined { return this._fields["primarykey"] as string; }
    public set primarykey(value: string | undefined) { this._fields["primarykey"] = value; this.markDirty('primarykey'); }
    /** `SingleLineText` (fld0BL2lFo9fqcKv3) */
    public get singlelinetext(): string | undefined { return this._fields["singlelinetext"] as string; }
    public set singlelinetext(value: string | undefined) { this._fields["singlelinetext"] = value; this.markDirty('singlelinetext'); }

    constructor(data: IPrimary = {}) {
        super(data.id ?? '');
        this.initializeFields(data);
        this.record = new Record<PrimaryFieldSet>(new PrimaryTable(this.getInstanceBaseId(), this.getInstanceOptions())._table, this.id, {});
        this.updateRecord();
    }

}
// #endregion

