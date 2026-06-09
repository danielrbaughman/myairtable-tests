// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

import { AirtableOptions, FieldSet, Record } from "airtable";
import { AirtableModel, FieldDescriptor } from "../../static/airtable-model";
import { RecordId } from "../../static/special-types";
import { LinkedRecords } from "../../static/linked-record";
import { buildUrl } from "../../static/helpers";
import {
    SecondaryFieldSet,
    SecondaryField,
    SecondaryView,
    SecondaryViewNameIdMapping,
    SecondaryFieldNameIdMapping,
    SecondaryFieldIdNameMapping,
    SecondaryFieldNamePropertyMapping,
} from "../types/secondary";
import { SecondaryFormulas } from "../formulas/secondary";
import { TertiaryModel, PrimaryModel } from "../models";
import { SecondaryTable } from "../tables/secondary";
import { SecondarySchema, ISecondary } from "../zod/secondary";

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
        { propertyName: "linkToTertiary", fieldId: "fldKR6tdbnOBRCtdQ", fieldName: "Link to Tertiary", isComputed: false, fieldType: "linkedRecords", linkedModelFromId: (id, config) => TertiaryModel.fromId(id, config), linkedModelClass: TertiaryModel as any },
        { propertyName: "name", fieldId: "fld1RagdJ09mpWhzM", fieldName: "Name", isComputed: false, fieldType: "generic" },
        { propertyName: "primary", fieldId: "fldl0nB9WRFSdqlii", fieldName: "Primary", isComputed: false, fieldType: "linkedRecords", linkedModelFromId: (id, config) => PrimaryModel.fromId(id, config), linkedModelClass: PrimaryModel as any },
        { propertyName: "primary2", fieldId: "fldgoE2oZmXmKkQca", fieldName: "Primary 2", isComputed: false, fieldType: "linkedRecords", linkedModelFromId: (id, config) => PrimaryModel.fromId(id, config), linkedModelClass: PrimaryModel as any },
        { propertyName: "value", fieldId: "fldi6Mxh5H1gPGxFX", fieldName: "Value", isComputed: false, fieldType: "generic" },
    ];

    /** `Link to Tertiary` (fldKR6tdbnOBRCtdQ) */
    public get linkToTertiary(): LinkedRecords<TertiaryModel> { return this._fields["linkToTertiary"] as LinkedRecords<TertiaryModel>; }
    public set linkToTertiary(value: TertiaryModel[] | LinkedRecords<TertiaryModel> | RecordId[] | undefined) { this._setLinkedRecordsField('linkToTertiary', value); }
    /** `Name` (fld1RagdJ09mpWhzM) */
    public get name(): string | undefined { return this._fields["name"] as string; }
    public set name(value: string | undefined) { this._fields["name"] = value; this.markDirty('name'); }
    /** `Primary` (fldl0nB9WRFSdqlii) */
    public get primary(): LinkedRecords<PrimaryModel> { return this._fields["primary"] as LinkedRecords<PrimaryModel>; }
    public set primary(value: PrimaryModel[] | LinkedRecords<PrimaryModel> | RecordId[] | undefined) { this._setLinkedRecordsField('primary', value); }
    /** `Primary 2` (fldgoE2oZmXmKkQca) */
    public get primary2(): LinkedRecords<PrimaryModel> { return this._fields["primary2"] as LinkedRecords<PrimaryModel>; }
    public set primary2(value: PrimaryModel[] | LinkedRecords<PrimaryModel> | RecordId[] | undefined) { this._setLinkedRecordsField('primary2', value); }
    /** `Value` (fldi6Mxh5H1gPGxFX) */
    public get value(): string | undefined { return this._fields["value"] as string; }
    public set value(value: string | undefined) { this._fields["value"] = value; this.markDirty('value'); }

    constructor(data: ISecondary = {}) {
        super(data.id ?? '');
        this.initializeFields(data);
        this.record = new Record<SecondaryFieldSet>(new SecondaryTable(this.getInstanceBaseId(), this.getInstanceOptions())._table, this.id, {});
        this.updateRecord();
    }

    /** Get the URL for this record in Airtable, with optional view. */
    public url(view?: SecondaryView): string {
        if (view) {
            return buildUrl(this.getInstanceBaseId(), 'tblPPScS3XMuFkDYN', this.id, SecondaryViewNameIdMapping[view]);
        } else {
            return buildUrl(this.getInstanceBaseId(), 'tblPPScS3XMuFkDYN', this.id);
        }
    }

}
