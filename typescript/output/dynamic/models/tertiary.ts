// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

import { AirtableOptions, FieldSet, Record } from "airtable";
import { AirtableModel, FieldDescriptor } from "../../static/airtable-model";
import { RecordId } from "../../static/special-types";
import { LinkedRecords } from "../../static/linked-record";
import { buildUrl } from "../../static/helpers";
import {
    TertiaryFieldSet,
    TertiaryField,
    TertiaryView,
    TertiaryViewNameIdMapping,
    TertiaryFieldNameIdMapping,
    TertiaryFieldIdNameMapping,
    TertiaryFieldNamePropertyMapping,
} from "../types/tertiary";
import { TertiaryFormulas } from "../formulas/tertiary";
import { SecondaryModel } from "../models";
import { TertiaryTable } from "../tables/tertiary";
import { TertiarySchema, ITertiary } from "../zod/tertiary";

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
        { propertyName: "secondary", fieldId: "fld8lCuUXpEXkIeYv", fieldName: "Secondary", isComputed: false, fieldType: "linkedRecords", linkedModelFromId: (id, config) => SecondaryModel.fromId(id, config), linkedModelClass: SecondaryModel as any },
        { propertyName: "value", fieldId: "fldjNLBh2UccM64h5", fieldName: "Value", isComputed: false, fieldType: "generic" },
    ];

    /** `Name` (fldwzqKxsRnPZJ2Ll) */
    public get name(): string | undefined { return this._fields["name"] as string; }
    public set name(value: string | undefined) { this._fields["name"] = value; this.markDirty('name'); }
    /** `Secondary` (fld8lCuUXpEXkIeYv) */
    public get secondary(): LinkedRecords<SecondaryModel> { return this._fields["secondary"] as LinkedRecords<SecondaryModel>; }
    public set secondary(value: SecondaryModel[] | LinkedRecords<SecondaryModel> | RecordId[] | undefined) { this._setLinkedRecordsField('secondary', value); }
    /** `Value` (fldjNLBh2UccM64h5) */
    public get value(): string | undefined { return this._fields["value"] as string; }
    public set value(value: string | undefined) { this._fields["value"] = value; this.markDirty('value'); }

    constructor(data: ITertiary = {}) {
        super(data.id ?? '');
        this.initializeFields(data);
        this.record = new Record<TertiaryFieldSet>(new TertiaryTable(this.getInstanceBaseId(), this.getInstanceOptions())._table, this.id, {});
        this.updateRecord();
    }

    /** Get the URL for this record in Airtable, with optional view. */
    public url(view?: TertiaryView): string {
        if (view) {
            return buildUrl(this.getInstanceBaseId(), 'tblLFoLxEdWlxjmLP', this.id, TertiaryViewNameIdMapping[view]);
        } else {
            return buildUrl(this.getInstanceBaseId(), 'tblLFoLxEdWlxjmLP', this.id);
        }
    }

}
