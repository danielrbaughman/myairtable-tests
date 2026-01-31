// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

import { AirtableOptions, Attachment, Collaborator, FieldSet, Record } from "airtable";
import { AirtableModel, FieldDescriptor } from "../../static/airtable-model";
import { RecordId, AirtableButton } from "../../static/special-types";
import { LinkedRecord, LinkedRecords } from "../../static/linked-record";
import {
    PrimaryFieldSet,
    PrimaryField,
    PrimaryFieldNameIdMapping,
    PrimaryFieldIdNameMapping,
    PrimaryFieldNamePropertyMapping,
    PrimaryMultipleSelectOption,
    PrimarySingleSelectOption,
} from "../types/primary";
import { PrimaryFormulas } from '../formulas/primary';
import {
    SecondaryModel,
} from "../models";
import { PrimaryTable } from '../tables/primary';
import { PrimarySchema, IPrimary } from '../zod/primary';

/** Model for `Primary` (tblmb3iqgpNS1ysV2) */
export class PrimaryModel extends AirtableModel<PrimaryFieldSet, IPrimary, PrimaryField> {
    protected static schema = PrimarySchema;
    public static f = PrimaryFormulas
    protected static nameToIdMap = PrimaryFieldNameIdMapping;
    protected static idToNameMap = PrimaryFieldIdNameMapping;
    protected static nameToPropertyMap = PrimaryFieldNamePropertyMapping;

    /** Table name (Primary) */
    public static tableName: string = 'Primary';
    /** Table name (Primary) */
    public get tableName(): string { return PrimaryModel.tableName; }

    /** Table ID (tblmb3iqgpNS1ysV2) */
    public static tableId: string = 'tblmb3iqgpNS1ysV2';
    /** Table ID (tblmb3iqgpNS1ysV2) */
    public get tableId(): string { return PrimaryModel.tableId; }

    protected static fieldDescriptors: FieldDescriptor[] = [
        { propertyName: "attachment", fieldId: "fldhF2AEuSC1haCZd", fieldName: "Attachment", isComputed: false, fieldType: "attachment" },
        { propertyName: "autoNumber", fieldId: "fldizvTkxgIn0mC3L", fieldName: "Auto Number", isComputed: true, fieldType: "generic" },
        { propertyName: "button", fieldId: "fldY48yKPG16AajtU", fieldName: "Button", isComputed: true, fieldType: "generic" },
        { propertyName: "checkbox", fieldId: "fldjQIaAZVegb1FUa", fieldName: "Checkbox", isComputed: false, fieldType: "generic" },
        { propertyName: "createdBy", fieldId: "fldGLQhDz2UjjiHG6", fieldName: "Created By", isComputed: true, fieldType: "generic" },
        { propertyName: "createdAtTime", fieldId: "fld2YgW382Kt9xltA", fieldName: "Created Time", isComputed: true, fieldType: "generic" },
        { propertyName: "currencyFloat", fieldId: "fldyh8pzDXiy5abEr", fieldName: "Currency (float)", isComputed: false, fieldType: "generic" },
        { propertyName: "currencyInt", fieldId: "fldBfo74z9hD78hP8", fieldName: "Currency (int)", isComputed: false, fieldType: "generic" },
        { propertyName: "date", fieldId: "fldC6LfNVvVIxKyQH", fieldName: "Date", isComputed: false, fieldType: "generic" },
        { propertyName: "dateWithTime", fieldId: "fldizYmjpXABGDLTG", fieldName: "Date (with time)", isComputed: false, fieldType: "generic" },
        { propertyName: "duration", fieldId: "fldLTyf6ljS0rhur8", fieldName: "Duration", isComputed: false, fieldType: "generic" },
        { propertyName: "email", fieldId: "fldHCJoYBiFVsNvP4", fieldName: "Email", isComputed: false, fieldType: "generic" },
        { propertyName: "formulaComplex", fieldId: "fld2vnFc0Bl5IOFUQ", fieldName: "Formula (Complex)", isComputed: true, fieldType: "generic" },
        { propertyName: "formulaId", fieldId: "fldcf62YFeIIDHElt", fieldName: "Formula (ID)", isComputed: true, fieldType: "generic" },
        { propertyName: "formulaNested", fieldId: "fldXFeHRPBLz6AiWh", fieldName: "Formula (Nested)", isComputed: true, fieldType: "generic" },
        { propertyName: "formulaSimple", fieldId: "fldy1axxaoUToLVC6", fieldName: "Formula (Simple)", isComputed: true, fieldType: "generic" },
        { propertyName: "lastModifiedBy", fieldId: "fldF8iDttqP0AgzWC", fieldName: "Last Modified By", isComputed: true, fieldType: "generic" },
        { propertyName: "lastModifiedTime", fieldId: "fldMinKh4pa3YX86g", fieldName: "Last Modified Time", isComputed: true, fieldType: "generic" },
        { propertyName: "linkMultiple", fieldId: "fldFyFheQWczd8oux", fieldName: "Link (multiple)", isComputed: false, fieldType: "linkedRecords", linkedModelFromId: (id, config) => SecondaryModel.fromId(id, config) },
        { propertyName: "linkSingle", fieldId: "fld7F5onkDo6mkmbN", fieldName: "Link (single)", isComputed: false, fieldType: "linkedRecord", linkedModelFromId: (id, config) => SecondaryModel.fromId(id, config) },
        { propertyName: "longText", fieldId: "fld8ulc6J0W29M6La", fieldName: "Long Text", isComputed: false, fieldType: "generic" },
        { propertyName: "longTextWithRichText", fieldId: "fldHJkxCMC0xo343u", fieldName: "Long Text with Rich Text", isComputed: false, fieldType: "generic" },
        { propertyName: "lookup", fieldId: "fldbmFmrzYKBktJvE", fieldName: "Lookup", isComputed: true, fieldType: "generic" },
        { propertyName: "multipleSelect", fieldId: "fld6GTabFmu1xKPvZ", fieldName: "Multiple Select", isComputed: false, fieldType: "generic" },
        { propertyName: "numberFloat", fieldId: "fldmU0X2l4RWd21dd", fieldName: "Number (float)", isComputed: false, fieldType: "generic" },
        { propertyName: "numberInt", fieldId: "fldOfPKGmnRPv94QH", fieldName: "Number (int)", isComputed: false, fieldType: "generic" },
        { propertyName: "percentFloat", fieldId: "fldiGui9ll69N7WOj", fieldName: "Percent (float)", isComputed: false, fieldType: "generic" },
        { propertyName: "percentInt", fieldId: "fldbAAyWboGulpb4s", fieldName: "Percent (int)", isComputed: false, fieldType: "generic" },
        { propertyName: "phoneNumber", fieldId: "fld38tnNpHmoks8C8", fieldName: "Phone Number", isComputed: false, fieldType: "generic" },
        { propertyName: "primaryKey", fieldId: "fldol5Q4wmQJQvPRy", fieldName: "Primary Key", isComputed: false, fieldType: "generic" },
        { propertyName: "rating", fieldId: "fldRsmwFwQNZkKLp4", fieldName: "Rating", isComputed: false, fieldType: "generic" },
        { propertyName: "rollup", fieldId: "fldGaFgBsDC3IBUdV", fieldName: "Rollup", isComputed: true, fieldType: "generic" },
        { propertyName: "singleLineText", fieldId: "fld0BL2lFo9fqcKv3", fieldName: "Single Line Text", isComputed: false, fieldType: "generic" },
        { propertyName: "singleSelect", fieldId: "fldn0GFFtMFpCXUNU", fieldName: "Single Select", isComputed: false, fieldType: "generic" },
        { propertyName: "url", fieldId: "fldLYloz2oP4ymf3B", fieldName: "URL", isComputed: false, fieldType: "generic" },
        { propertyName: "user", fieldId: "fldU6SbLp8CSkLcA4", fieldName: "User", isComputed: false, fieldType: "generic" },
        { propertyName: "userAllowMultiple", fieldId: "fldBwCDbAVxRj9yg7", fieldName: "User (allow multiple)", isComputed: false, fieldType: "generic" },
    ];

    /** `Attachment` (fldhF2AEuSC1haCZd) */
    public get attachment(): Attachment[] | undefined { return this._fields["attachment"] as Attachment[]; }
    public set attachment(value: Attachment[] | undefined) { this._fields["attachment"] = value; this.markDirty('attachment'); }
    /** `Auto Number` (fldizvTkxgIn0mC3L) */
    public get autoNumber(): number | undefined { return this._fields["autoNumber"] as number; }
    public set autoNumber(value: number | undefined) { this._fields["autoNumber"] = value; this.markDirty('autoNumber'); }
    /** `Button` (fldY48yKPG16AajtU) */
    public get button(): AirtableButton | undefined { return this._fields["button"] as AirtableButton; }
    public set button(value: AirtableButton | undefined) { this._fields["button"] = value; this.markDirty('button'); }
    /** `Checkbox` (fldjQIaAZVegb1FUa) */
    public get checkbox(): boolean | undefined { return this._fields["checkbox"] as boolean; }
    public set checkbox(value: boolean | undefined) { this._fields["checkbox"] = value; this.markDirty('checkbox'); }
    /** `Created By` (fldGLQhDz2UjjiHG6) */
    public get createdBy(): Collaborator | undefined { return this._fields["createdBy"] as Collaborator; }
    public set createdBy(value: Collaborator | undefined) { this._fields["createdBy"] = value; this.markDirty('createdBy'); }
    /** `Created Time` (fld2YgW382Kt9xltA) */
    public get createdAtTime(): string | undefined { return this._fields["createdAtTime"] as string; }
    public set createdAtTime(value: string | undefined) { this._fields["createdAtTime"] = value; this.markDirty('createdAtTime'); }
    /** `Currency (float)` (fldyh8pzDXiy5abEr) */
    public get currencyFloat(): number | undefined { return this._fields["currencyFloat"] as number; }
    public set currencyFloat(value: number | undefined) { this._fields["currencyFloat"] = value; this.markDirty('currencyFloat'); }
    /** `Currency (int)` (fldBfo74z9hD78hP8) */
    public get currencyInt(): number | undefined { return this._fields["currencyInt"] as number; }
    public set currencyInt(value: number | undefined) { this._fields["currencyInt"] = value; this.markDirty('currencyInt'); }
    /** `Date` (fldC6LfNVvVIxKyQH) */
    public get date(): string | undefined { return this._fields["date"] as string; }
    public set date(value: string | undefined) { this._fields["date"] = value; this.markDirty('date'); }
    /** `Date (with time)` (fldizYmjpXABGDLTG) */
    public get dateWithTime(): string | undefined { return this._fields["dateWithTime"] as string; }
    public set dateWithTime(value: string | undefined) { this._fields["dateWithTime"] = value; this.markDirty('dateWithTime'); }
    /** `Duration` (fldLTyf6ljS0rhur8) */
    public get duration(): number | undefined { return this._fields["duration"] as number; }
    public set duration(value: number | undefined) { this._fields["duration"] = value; this.markDirty('duration'); }
    /** `Email` (fldHCJoYBiFVsNvP4) */
    public get email(): string | undefined { return this._fields["email"] as string; }
    public set email(value: string | undefined) { this._fields["email"] = value; this.markDirty('email'); }
    /**
     * `Formula (Complex)` (fld2vnFc0Bl5IOFUQ)
     * 
     * ```
     * CONCATENATE(
     *   "Primary Key: ",
     *   {Primary Key},
     *   "\n",
     *   "Single Line Text: ",
     *   {Single Line Text},
     *   "\n",
     *   "Long Text: ",
     *   {Long Text},
     *   "\n",
     *   "Long Text with Rich Text: ",
     *   {Long Text with Rich Text},
     *   "\n",
     *   "Attachment: ",
     *   IF(
     *     {Attachment},
     *     {Attachment},
     *     "None"
     *   ),
     *   "\n",
     *   "Checkbox: ",
     *   IF(
     *     {Checkbox},
     *     "Checked",
     *     "Unchecked"
     *   ),
     *   "\n",
     *   "Multiple Select: ",
     *   IF(
     *     {Multiple Select},
     *     {Multiple Select},
     *     "None"
     *   ),
     *   "\n",
     *   "Single Select: ",
     *   IF(
     *     {Single Select},
     *     {Single Select},
     *     "None"
     *   ),
     *   "\n",
     *   "User: ",
     *   IF(
     *     {User},
     *     {User},
     *     "None"
     *   ),
     *   "\n",
     *   "User (allow multiple): ",
     *   IF(
     *     {User (allow multiple)},
     *     {User (allow multiple)},
     *     "None"
     *   ),
     *   "\n",
     *   "Date: ",
     *   IF(
     *     {Date},
     *     DATETIME_FORMAT({Date}, 'YYYY-MM-DD'),
     *     "None"
     *   ),
     *   "\n",
     *   "Date (with time): ",
     *   IF(
     *     {Date (with time)},
     *     DATETIME_FORMAT(
     *       {Date (with time)},
     *       'YYYY-MM-DD HH:mm'
     *     ),
     *     "None"
     *   ),
     *   "\n",
     *   "Phone Number: ",
     *   IF(
     *     {Phone Number},
     *     {Phone Number},
     *     "None"
     *   ),
     *   "\n",
     *   "Email: ",
     *   IF(
     *     {Email},
     *     {Email},
     *     "None"
     *   ),
     *   "\n",
     *   "URL: ",
     *   IF(
     *     {URL},
     *     {URL},
     *     "None"
     *   ),
     *   "\n",
     *   "Number (int): ",
     *   IF(
     *     {Number (int)},
     *     {Number(int) } & "",
     *     "None"
     *   ),
     *   "\n",
     *   "Number (float): ",
     *   IF(
     *     {Number (float)},
     *     {Number(float) } & "",
     *     "None"
     *   ),
     *   "\n",
     *   "Currency (int): ",
     *   IF(
     *     {Currency (int)},
     *     {Currency(int) } & "",
     *     "None"
     *   ),
     *   "\n",
     *   "Currency (float): ",
     *   IF(
     *     {Currency (float)},
     *     {Currency(float) } & "",
     *     "None"
     *   ),
     *   "\n",
     *   "Percent (int): ",
     *   IF(
     *     {Percent (int)},
     *     {Percent(int) } & "",
     *     "None"
     *   ),
     *   "\n",
     *   "Percent (float): ",
     *   IF(
     *     {Percent (float)},
     *     {Percent(float) } & "",
     *     "None"
     *   ),
     *   "\n",
     *   "Duration: ",
     *   IF(
     *     {Duration},
     *     {Duration} & "",
     *     "None"
     *   ),
     *   "\n",
     *   "Rating: ",
     *   IF(
     *     {Rating},
     *     {Rating} & "",
     *     "None"
     *   ),
     *   "\n",
     *   "Created Time: ",
     *   IF(
     *     {Created Time},
     *     DATETIME_FORMAT(
     *       {Created Time},
     *       'YYYY-MM-DD HH:mm'
     *     ),
     *     "None"
     *   ),
     *   "\n",
     *   "Last Modified Time: ",
     *   IF(
     *     {Last Modified Time},
     *     DATETIME_FORMAT(
     *       {Last Modified Time},
     *       'YYYY-MM-DD HH:mm'
     *     ),
     *     "None"
     *   ),
     *   "\n",
     *   "Created By: ",
     *   IF(
     *     {Created By},
     *     {Created By},
     *     "None"
     *   ),
     *   "\n",
     *   "Last Modified By: ",
     *   IF(
     *     {Last Modified By},
     *     {Last Modified By},
     *     "None"
     *   ),
     *   "\n",
     *   "Auto Number: ",
     *   IF(
     *     {Auto Number},
     *     {Auto Number} & "",
     *     "None"
     *   ),
     *   "\n",
     *   "Button: ",
     *   IF(
     *     {Button},
     *     {Button},
     *     "None"
     *   ),
     *   "\n",
     *   "Link (single): ",
     *   IF(
     *     {Link (single)},
     *     {Link (single)},
     *     "None"
     *   ),
     *   "\n",
     *   "Link (multiple): ",
     *   IF(
     *     {Link (multiple)},
     *     {Link (multiple)},
     *     "None"
     *   ),
     *   "\n",
     *   "Lookup: ",
     *   IF(
     *     {Lookup},
     *     {Lookup},
     *     "None"
     *   ),
     *   "\n",
     *   "Rollup: ",
     *   IF(
     *     {Rollup},
     *     {Rollup},
     *     "None"
     *   ),
     *   "\n",
     *   "Formula (ID): ",
     *   IF(
     *     {Formula (ID)},
     *     {Formula (ID)},
     *     "None"
     *   ),
     *   "\n",
     *   "Formula (Simple): ",
     *   IF(
     *     {Formula (Simple)},
     *     {Formula (Simple)},
     *     "None"
     *   )
     * )
     * ```
     */
    public get formulaComplex(): string | undefined { return this._fields["formulaComplex"] as string; }
    public set formulaComplex(value: string | undefined) { this._fields["formulaComplex"] = value; this.markDirty('formulaComplex'); }
    /**
     * `Formula (ID)` (fldcf62YFeIIDHElt)
     * 
     * ```
     * RECORD_ID()
     * ```
     */
    public get formulaId(): string | undefined { return this._fields["formulaId"] as string; }
    public set formulaId(value: string | undefined) { this._fields["formulaId"] = value; this.markDirty('formulaId'); }
    /**
     * `Formula (Nested)` (fldXFeHRPBLz6AiWh)
     * 
     * ```
     * {Formula (ID)} & {Formula (Simple)} & {Formula (Complex)}
     * ```
     */
    public get formulaNested(): string | undefined { return this._fields["formulaNested"] as string; }
    public set formulaNested(value: string | undefined) { this._fields["formulaNested"] = value; this.markDirty('formulaNested'); }
    /**
     * `Formula (Simple)` (fldy1axxaoUToLVC6)
     * 
     * ```
     * {Number (int)} + {Number (float)}
     * ```
     */
    public get formulaSimple(): number | undefined { return this._fields["formulaSimple"] as number; }
    public set formulaSimple(value: number | undefined) { this._fields["formulaSimple"] = value; this.markDirty('formulaSimple'); }
    /** `Last Modified By` (fldF8iDttqP0AgzWC) */
    public get lastModifiedBy(): Collaborator | undefined { return this._fields["lastModifiedBy"] as Collaborator; }
    public set lastModifiedBy(value: Collaborator | undefined) { this._fields["lastModifiedBy"] = value; this.markDirty('lastModifiedBy'); }
    /** `Last Modified Time` (fldMinKh4pa3YX86g) */
    public get lastModifiedTime(): string | undefined { return this._fields["lastModifiedTime"] as string; }
    public set lastModifiedTime(value: string | undefined) { this._fields["lastModifiedTime"] = value; this.markDirty('lastModifiedTime'); }
    /** `Link (multiple)` (fldFyFheQWczd8oux) */
    public get linkMultiple(): LinkedRecords<SecondaryModel> { return this._fields["linkMultiple"] as LinkedRecords<SecondaryModel>; }
    public set linkMultiple(value: LinkedRecords<SecondaryModel> | undefined) { this._fields["linkMultiple"] = value!; this.markDirty('linkMultiple'); }
    /** `Link (single)` (fld7F5onkDo6mkmbN) */
    public get linkSingle(): LinkedRecord<SecondaryModel> { return this._fields["linkSingle"] as LinkedRecord<SecondaryModel>; }
    public set linkSingle(value: LinkedRecord<SecondaryModel> | undefined) { this._fields["linkSingle"] = value!; this.markDirty('linkSingle'); }
    /** `Long Text` (fld8ulc6J0W29M6La) */
    public get longText(): string | undefined { return this._fields["longText"] as string; }
    public set longText(value: string | undefined) { this._fields["longText"] = value; this.markDirty('longText'); }
    /** `Long Text with Rich Text` (fldHJkxCMC0xo343u) */
    public get longTextWithRichText(): string | undefined { return this._fields["longTextWithRichText"] as string; }
    public set longTextWithRichText(value: string | undefined) { this._fields["longTextWithRichText"] = value; this.markDirty('longTextWithRichText'); }
    /** `Lookup` (fldbmFmrzYKBktJvE) */
    public get lookup(): string[] | undefined { return this._fields["lookup"] as string[]; }
    public set lookup(value: string[] | undefined) { this._fields["lookup"] = value; this.markDirty('lookup'); }
    /** `Multiple Select` (fld6GTabFmu1xKPvZ) */
    public get multipleSelect(): PrimaryMultipleSelectOption[] | undefined { return this._fields["multipleSelect"] as PrimaryMultipleSelectOption[]; }
    public set multipleSelect(value: PrimaryMultipleSelectOption[] | undefined) { this._fields["multipleSelect"] = value; this.markDirty('multipleSelect'); }
    /** `Number (float)` (fldmU0X2l4RWd21dd) */
    public get numberFloat(): number | undefined { return this._fields["numberFloat"] as number; }
    public set numberFloat(value: number | undefined) { this._fields["numberFloat"] = value; this.markDirty('numberFloat'); }
    /** `Number (int)` (fldOfPKGmnRPv94QH) */
    public get numberInt(): number | undefined { return this._fields["numberInt"] as number; }
    public set numberInt(value: number | undefined) { this._fields["numberInt"] = value; this.markDirty('numberInt'); }
    /** `Percent (float)` (fldiGui9ll69N7WOj) */
    public get percentFloat(): number | undefined { return this._fields["percentFloat"] as number; }
    public set percentFloat(value: number | undefined) { this._fields["percentFloat"] = value; this.markDirty('percentFloat'); }
    /** `Percent (int)` (fldbAAyWboGulpb4s) */
    public get percentInt(): number | undefined { return this._fields["percentInt"] as number; }
    public set percentInt(value: number | undefined) { this._fields["percentInt"] = value; this.markDirty('percentInt'); }
    /** `Phone Number` (fld38tnNpHmoks8C8) */
    public get phoneNumber(): string | undefined { return this._fields["phoneNumber"] as string; }
    public set phoneNumber(value: string | undefined) { this._fields["phoneNumber"] = value; this.markDirty('phoneNumber'); }
    /** `Primary Key` (fldol5Q4wmQJQvPRy) */
    public get primaryKey(): string | undefined { return this._fields["primaryKey"] as string; }
    public set primaryKey(value: string | undefined) { this._fields["primaryKey"] = value; this.markDirty('primaryKey'); }
    /** `Rating` (fldRsmwFwQNZkKLp4) */
    public get rating(): any | undefined { return this._fields["rating"] as any; }
    public set rating(value: any | undefined) { this._fields["rating"] = value; this.markDirty('rating'); }
    /** `Rollup` (fldGaFgBsDC3IBUdV) */
    public get rollup(): string[] | undefined { return this._fields["rollup"] as string[]; }
    public set rollup(value: string[] | undefined) { this._fields["rollup"] = value; this.markDirty('rollup'); }
    /** `Single Line Text` (fld0BL2lFo9fqcKv3) */
    public get singleLineText(): string | undefined { return this._fields["singleLineText"] as string; }
    public set singleLineText(value: string | undefined) { this._fields["singleLineText"] = value; this.markDirty('singleLineText'); }
    /** `Single Select` (fldn0GFFtMFpCXUNU) */
    public get singleSelect(): PrimarySingleSelectOption | undefined { return this._fields["singleSelect"] as PrimarySingleSelectOption; }
    public set singleSelect(value: PrimarySingleSelectOption | undefined) { this._fields["singleSelect"] = value; this.markDirty('singleSelect'); }
    /** `URL` (fldLYloz2oP4ymf3B) */
    public get url(): string | undefined { return this._fields["url"] as string; }
    public set url(value: string | undefined) { this._fields["url"] = value; this.markDirty('url'); }
    /** `User` (fldU6SbLp8CSkLcA4) */
    public get user(): Collaborator | undefined { return this._fields["user"] as Collaborator; }
    public set user(value: Collaborator | undefined) { this._fields["user"] = value; this.markDirty('user'); }
    /** `User (allow multiple)` (fldBwCDbAVxRj9yg7) */
    public get userAllowMultiple(): Collaborator[] | undefined { return this._fields["userAllowMultiple"] as Collaborator[]; }
    public set userAllowMultiple(value: Collaborator[] | undefined) { this._fields["userAllowMultiple"] = value; this.markDirty('userAllowMultiple'); }

    constructor(data: IPrimary = {}) {
        super(data.id ?? '');
        this.initializeFields(data);
        this.record = new Record<PrimaryFieldSet>(new PrimaryTable(this.getInstanceBaseId(), this.getInstanceOptions())._table, this.id, {});
        this.updateRecord();
    }

}
