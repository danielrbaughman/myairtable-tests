// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

import { AirtableOptions, Attachment, Collaborator, FieldSet, Record } from "airtable";
import { AirtableModel, FieldDescriptor } from "../../static/airtable-model";
import { RecordId, AirtableButton } from "../../static/special-types";
import { LinkedRecord, LinkedRecords } from "../../static/linked-record";
import {
    TertiaryFieldSet,
    TertiaryField,
    TertiaryFieldNameIdMapping,
    TertiaryFieldIdNameMapping,
    TertiaryFieldNamePropertyMapping,
} from "../types/tertiary";
import { TertiaryFormulas } from '../formulas/tertiary';
import { TertiaryTable } from '../tables/tertiary';
import { TertiarySchema, ITertiary } from '../zod/tertiary';

/** Model for `Tertiary` (tblLFoLxEdWlxjmLP) */
export class TertiaryModel extends AirtableModel<TertiaryFieldSet, ITertiary, TertiaryField> {
    protected static schema = TertiarySchema;
    public static f = TertiaryFormulas
    protected static nameToIdMap = TertiaryFieldNameIdMapping;
    protected static idToNameMap = TertiaryFieldIdNameMapping;
    protected static nameToPropertyMap = TertiaryFieldNamePropertyMapping;

    /** Table name (Tertiary) */
    public static tableName: string = 'Tertiary';
    /** Table name (Tertiary) */
    public get tableName(): string { return TertiaryModel.tableName; }

    /** Table ID (tblLFoLxEdWlxjmLP) */
    public static tableId: string = 'tblLFoLxEdWlxjmLP';
    /** Table ID (tblLFoLxEdWlxjmLP) */
    public get tableId(): string { return TertiaryModel.tableId; }

    protected static fieldDescriptors: FieldDescriptor[] = [
        { propertyName: "name", fieldId: "fldwzqKxsRnPZJ2Ll", fieldName: "Name", isComputed: false, fieldType: "generic" },
        { propertyName: "value", fieldId: "fldjNLBh2UccM64h5", fieldName: "Value", isComputed: false, fieldType: "generic" },
    ];

    /** `Name` (fldwzqKxsRnPZJ2Ll) */
    public get name(): string | undefined { return this._fields["name"] as string; }
    public set name(value: string | undefined) { this._fields["name"] = value; this.markDirty('name'); }
    /** `Value` (fldjNLBh2UccM64h5) */
    public get value(): string | undefined { return this._fields["value"] as string; }
    public set value(value: string | undefined) { this._fields["value"] = value; this.markDirty('value'); }

    constructor(data: ITertiary = {}) {
        super(data.id ?? '');
        this.initializeFields(data);
        this.record = new Record<TertiaryFieldSet>(new TertiaryTable(this.getInstanceBaseId(), this.getInstanceOptions())._table, this.id, {});
        this.updateRecord();
    }

}
