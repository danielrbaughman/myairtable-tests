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
    SecondaryFieldSet,
    SecondaryField,
    SecondaryFieldNameIdMapping,
    SecondaryFieldIdNameMapping,
    SecondaryFieldNamePropertyMapping,
} from "../types/secondary";
import { SecondaryFormulas } from '../formulas/secondary';
import {
    PrimaryModel,
} from "../models";
import { SecondaryTable } from '../tables/secondary';
import { SecondarySchema, ISecondary } from '../zod/secondary';
// #endregion


// #region SECONDARY
/** Model for `Secondary` (tblPPScS3XMuFkDYN) */
export class SecondaryModel extends AirtableModel<SecondaryFieldSet, ISecondary, SecondaryField> {
    protected static schema = SecondarySchema;
    public static f = SecondaryFormulas
    protected nameToIdMap = SecondaryFieldNameIdMapping;
    protected idToNameMap = SecondaryFieldIdNameMapping;
    protected nameToPropertyMap = SecondaryFieldNamePropertyMapping;

    /** Table name (Secondary) */
    public static tableName: string = 'Secondary';
    /** Table name (Secondary) */
    public get tableName(): string { return SecondaryModel.tableName; }

    /** Table ID (tblPPScS3XMuFkDYN) */
    public static tableId: string = 'tblPPScS3XMuFkDYN';
    /** Table ID (tblPPScS3XMuFkDYN) */
    public get tableId(): string { return SecondaryModel.tableId; }

    protected static fieldDescriptors: FieldDescriptor[] = [
        { propertyName: "name", fieldId: "fld1RagdJ09mpWhzM", fieldName: "Name", isComputed: false, fieldType: "generic" },
        { propertyName: "primary", fieldId: "fldl0nB9WRFSdqlii", fieldName: "Primary", isComputed: false, fieldType: "linkedRecords", linkedModelFromId: (id, baseId, options) => PrimaryModel.fromId(id, baseId, options) },
        { propertyName: "primary2", fieldId: "fldgoE2oZmXmKkQca", fieldName: "Primary 2", isComputed: false, fieldType: "linkedRecords", linkedModelFromId: (id, baseId, options) => PrimaryModel.fromId(id, baseId, options) },
        { propertyName: "value", fieldId: "fldi6Mxh5H1gPGxFX", fieldName: "Value", isComputed: false, fieldType: "generic" },
    ];

    /** `Name` (fld1RagdJ09mpWhzM) */
    public get name(): string | undefined { return this._fields["name"] as string; }
    public set name(value: string | undefined) { this._fields["name"] = value; this.markDirty('name'); }
    /** `Primary` (fldl0nB9WRFSdqlii) */
    public get primary(): LinkedRecords<PrimaryModel> { return this._fields["primary"] as LinkedRecords<PrimaryModel>; }
    public set primary(value: LinkedRecords<PrimaryModel> | undefined) { this._fields["primary"] = value!; this.markDirty('primary'); }
    /** `Primary 2` (fldgoE2oZmXmKkQca) */
    public get primary2(): LinkedRecords<PrimaryModel> { return this._fields["primary2"] as LinkedRecords<PrimaryModel>; }
    public set primary2(value: LinkedRecords<PrimaryModel> | undefined) { this._fields["primary2"] = value!; this.markDirty('primary2'); }
    /** `Value` (fldi6Mxh5H1gPGxFX) */
    public get value(): string | undefined { return this._fields["value"] as string; }
    public set value(value: string | undefined) { this._fields["value"] = value; this.markDirty('value'); }

    constructor(data: ISecondary = {}) {
        super(data.id ?? '');
        this.initializeFields(data);
        this.record = new Record<SecondaryFieldSet>(new SecondaryTable(this.getInstanceBaseId(), this.getInstanceOptions())._table, this.id, {});
        this.updateRecord();
    }

}
// #endregion

