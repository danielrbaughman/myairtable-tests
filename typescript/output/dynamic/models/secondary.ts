// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

import { AirtableOptions, Attachment, Collaborator, FieldSet, Record } from "airtable";
import { AirtableModel, FieldDescriptor } from "../../static/airtable-model";
import { RecordId, AirtableButton } from "../../static/special-types";
import { LinkedRecord, LinkedRecords } from "../../static/linked-record";
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

/** Model for `Secondary` (tblPPScS3XMuFkDYN) */
export class SecondaryModel extends AirtableModel<SecondaryFieldSet, ISecondary, SecondaryField> {
    protected static schema = SecondarySchema;
    public static f = SecondaryFormulas
    protected static nameToIdMap = SecondaryFieldNameIdMapping;
    protected static idToNameMap = SecondaryFieldIdNameMapping;
    protected static nameToPropertyMap = SecondaryFieldNamePropertyMapping;

    /** Table name (Secondary) */
    public static tableName: string = 'Secondary';
    /** Table name (Secondary) */
    public get tableName(): string { return SecondaryModel.tableName; }

    /** Table ID (tblPPScS3XMuFkDYN) */
    public static tableId: string = 'tblPPScS3XMuFkDYN';
    /** Table ID (tblPPScS3XMuFkDYN) */
    public get tableId(): string { return SecondaryModel.tableId; }

    protected static fieldDescriptors: FieldDescriptor[] = [
        { propertyName: "linkToTertiary", fieldId: "fldKR6tdbnOBRCtdQ", fieldName: "Link to Tertiary", isComputed: false, fieldType: "generic" },
        { propertyName: "name", fieldId: "fld1RagdJ09mpWhzM", fieldName: "Name", isComputed: false, fieldType: "generic" },
        { propertyName: "primary", fieldId: "fldl0nB9WRFSdqlii", fieldName: "Primary", isComputed: false, fieldType: "linkedRecords", linkedModelFromId: (id, config) => PrimaryModel.fromId(id, config) },
        { propertyName: "primary2", fieldId: "fldgoE2oZmXmKkQca", fieldName: "Primary 2", isComputed: false, fieldType: "linkedRecords", linkedModelFromId: (id, config) => PrimaryModel.fromId(id, config) },
        { propertyName: "value", fieldId: "fldi6Mxh5H1gPGxFX", fieldName: "Value", isComputed: false, fieldType: "generic" },
    ];

    /** `Link to Tertiary` (fldKR6tdbnOBRCtdQ) */
    public get linkToTertiary(): string | undefined { return this._fields["linkToTertiary"] as string; }
    public set linkToTertiary(value: string | undefined) { this._fields["linkToTertiary"] = value; this.markDirty('linkToTertiary'); }
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
