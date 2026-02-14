// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

import { AirtableOptions, Attachment, Collaborator, FieldSet, Record } from "airtable";
import { AirtableModel, FieldDescriptor } from "../../static/airtable-model";
import { RecordId, AirtableButton } from "../../static/special-types";
import { LinkedRecord, LinkedRecords } from "../../static/linked-record";
import {
    FormulasFieldSet,
    FormulasField,
    FormulasFieldNameIdMapping,
    FormulasFieldIdNameMapping,
    FormulasFieldNamePropertyMapping,
} from "../types/formulas";
import { FormulasFormulas } from '../formulas/formulas';
import { FormulasTable } from '../tables/formulas';
import { FormulasSchema, IFormulas } from '../zod/formulas';
import { AirtableRuntime as F } from "../../static/airtable-runtime";

/** Model for `Formulas` (tblnuYBsMdXNDsuRc) */
export class FormulasModel extends AirtableModel<FormulasFieldSet, IFormulas, FormulasField> {
    protected static schema = FormulasSchema;
    public static f = FormulasFormulas
    protected static nameToIdMap = FormulasFieldNameIdMapping;
    protected static idToNameMap = FormulasFieldIdNameMapping;
    protected static nameToPropertyMap = FormulasFieldNamePropertyMapping;

    /** Table name (Formulas) */
    public static tableName: string = 'Formulas';
    /** Table name (Formulas) */
    public get tableName(): string { return FormulasModel.tableName; }

    /** Table ID (tblnuYBsMdXNDsuRc) */
    public static tableId: string = 'tblnuYBsMdXNDsuRc';
    /** Table ID (tblnuYBsMdXNDsuRc) */
    public get tableId(): string { return FormulasModel.tableId; }

    protected static fieldDescriptors: FieldDescriptor[] = [
        { propertyName: "dateFormula", fieldId: "fldY7kjaklLeoSgGd", fieldName: "Date Formula", isComputed: true, fieldType: "generic" },
        { propertyName: "firstDate", fieldId: "fldlZT521Iy0FFXFL", fieldName: "First Date", isComputed: false, fieldType: "generic" },
        { propertyName: "firstNumber", fieldId: "fldA04pqfjMkGXcZU", fieldName: "First Number", isComputed: false, fieldType: "generic" },
        { propertyName: "firstText", fieldId: "fldHJuw6pAujnHvkP", fieldName: "First Text", isComputed: false, fieldType: "generic" },
        { propertyName: "mathFormula", fieldId: "fldlSuvoeWGokSz8Z", fieldName: "Math Formula", isComputed: true, fieldType: "generic" },
        { propertyName: "primaryKey", fieldId: "fldLZFrZKvSCS4dKb", fieldName: "Primary Key", isComputed: false, fieldType: "generic" },
        { propertyName: "secondDate", fieldId: "fld1LZ3Ebpt0LaMmu", fieldName: "Second Date", isComputed: false, fieldType: "generic" },
        { propertyName: "secondNumber", fieldId: "fldj5nAkal5y8OOZg", fieldName: "Second Number", isComputed: false, fieldType: "generic" },
        { propertyName: "secondText", fieldId: "fldA2boNwwsiuvXw1", fieldName: "Second Text", isComputed: false, fieldType: "generic" },
        { propertyName: "textFormula", fieldId: "flddvzeqt7FJpQ9NX", fieldName: "Text Formula", isComputed: true, fieldType: "generic" },
        { propertyName: "thirdDate", fieldId: "fldxSQRRn8W879aiU", fieldName: "Third Date", isComputed: false, fieldType: "generic" },
        { propertyName: "thirdNumber", fieldId: "fld5NBdekrAUzu4Fi", fieldName: "Third Number", isComputed: false, fieldType: "generic" },
        { propertyName: "thirdText", fieldId: "fldfruPf8V9K6qIAN", fieldName: "Third Text", isComputed: false, fieldType: "generic" },
    ];

    /**
     * `Date Formula` (fldY7kjaklLeoSgGd)
     * 
     * ```
     * "YEAR: " & YEAR({First Date})
     * & ", MONTH: " & MONTH({First Date})
     * & ", DAY: " & DAY({First Date})
     * & ", HOUR: " & HOUR({Second Date})
     * & ", MINUTE: " & MINUTE({Second Date})
     * & ", SECOND: " & SECOND({Second Date})
     * & ", TODAY: " & DATETIME_FORMAT(
     *   TODAY(),
     *   'YYYY-MM-DD'
     * )
     * & ", WORKDAY+5: " & DATETIME_FORMAT(
     *   WORKDAY({First Date}, 5),
     *   'YYYY-MM-DD'
     * )
     * & ", WORKDAY_DIFF: " & WORKDAY_DIFF({First Date}, {Second Date})
     * & ", PARSE: " & DATETIME_FORMAT(
     *   DATETIME_PARSE(
     *     DATESTR({First Date})
     *   ),
     *   'YYYY-MM-DD'
     * )
     * & ", FORMAT: " & DATETIME_FORMAT({Second Date}, 'YYYY-MM-DD HH:mm')
     * & ", LOCALE: " & DATETIME_FORMAT(
     *   SET_LOCALE({Second Date}, 'en'),
     *   'YYYY-MM-DD'
     * )
     * & ", TIMEZONE: " & DATETIME_FORMAT(
     *   SET_TIMEZONE({Second Date}, 'America/New_York'),
     *   'YYYY-MM-DD HH:mm'
     * )
     * & ", DATESTR: " & DATESTR({Second Date})
     * & ", TIMESTR: " & TIMESTR({Second Date})
     * & ", TONOW: " & TONOW({First Date})
     * & ", FROMNOW: " & FROMNOW({Second Date})
     * & ", DATEADD+2d: " & DATETIME_FORMAT(
     *   DATEADD({First Date}, 2, 'days'),
     *   'YYYY-MM-DD'
     * )
     * & ", WEEKNUM: " & WEEKNUM({First Date}, 'Monday')
     * & ", DIFF days: " & DATETIME_DIFF({First Date}, {Second Date}, 'days')
     * & ", IS_BEFORE: " & IF(
     *   IS_BEFORE({First Date}, {Second Date}),
     *   'Yes',
     *   'No'
     * )
     * & ", IS_SAME day: " & IF(
     *   IS_SAME({First Date}, {Second Date}, 'day'),
     *   'Yes',
     *   'No'
     * )
     * & ", IS_AFTER: " & IF(
     *   IS_AFTER({Third Date}, {Second Date}),
     *   'Yes',
     *   'No'
     * )
     * ```
     */
    public dateFormula(recalculate: boolean = false): string | undefined {
        if (recalculate) this._fields["dateFormula"] = ((((((((((((((((((((((((((((((((((((((((((((("YEAR: " + F.S(F.YEAR(this.firstDate))) + ", MONTH: ") + F.S(F.MONTH(this.firstDate))) + ", DAY: ") + F.S(F.DAY(this.firstDate))) + ", HOUR: ") + F.S(F.HOUR(this.secondDate))) + ", MINUTE: ") + F.S(F.MINUTE(this.secondDate))) + ", SECOND: ") + F.S(F.SECOND(this.secondDate))) + ", TODAY: ") + F.S(F.DATETIME_FORMAT(F.TODAY(), 'YYYY-MM-DD'))) + ", WORKDAY+5: ") + F.S(F.DATETIME_FORMAT(F.WORKDAY(this.firstDate, 5), 'YYYY-MM-DD'))) + ", WORKDAY_DIFF: ") + F.S(F.WORKDAY_DIFF(this.firstDate, this.secondDate))) + ", PARSE: ") + F.S(F.DATETIME_FORMAT(F.DATETIME_PARSE(F.DATESTR(this.firstDate)), 'YYYY-MM-DD'))) + ", FORMAT: ") + F.S(F.DATETIME_FORMAT(this.secondDate, 'YYYY-MM-DD HH:mm'))) + ", LOCALE: ") + F.S(F.DATETIME_FORMAT(F.SET_LOCALE(this.secondDate, 'en'), 'YYYY-MM-DD'))) + ", TIMEZONE: ") + F.S(F.DATETIME_FORMAT(F.SET_TIMEZONE(this.secondDate, 'America/New_York'), 'YYYY-MM-DD HH:mm'))) + ", DATESTR: ") + F.S(F.DATESTR(this.secondDate))) + ", TIMESTR: ") + F.S(F.TIMESTR(this.secondDate))) + ", TONOW: ") + F.S(F.TONOW(this.firstDate))) + ", FROMNOW: ") + F.S(F.FROMNOW(this.secondDate))) + ", DATEADD+2d: ") + F.S(F.DATETIME_FORMAT(F.DATEADD(this.firstDate, 2, 'days'), 'YYYY-MM-DD'))) + ", WEEKNUM: ") + F.S(F.WEEKNUM(this.firstDate, 'Monday'))) + ", DIFF days: ") + F.S(F.DATETIME_DIFF(this.firstDate, this.secondDate, 'days'))) + ", IS_BEFORE: ") + F.S((F.IS_BEFORE(this.firstDate, this.secondDate) ? 'Yes' : 'No'))) + ", IS_SAME day: ") + F.S((F.IS_SAME(this.firstDate, this.secondDate, 'day') ? 'Yes' : 'No'))) + ", IS_AFTER: ") + F.S((F.IS_AFTER(this.thirdDate, this.secondDate) ? 'Yes' : 'No')));
        return this._fields["dateFormula"] as string;
    }
    /** `First Date` (fldlZT521Iy0FFXFL) */
    public get firstDate(): string | undefined { return this._fields["firstDate"] as string; }
    public set firstDate(value: string | undefined) { this._fields["firstDate"] = value; this.markDirty('firstDate'); }
    /** `First Number` (fldA04pqfjMkGXcZU) */
    public get firstNumber(): number | undefined { return this._fields["firstNumber"] as number; }
    public set firstNumber(value: number | undefined) { this._fields["firstNumber"] = value; this.markDirty('firstNumber'); }
    /** `First Text` (fldHJuw6pAujnHvkP) */
    public get firstText(): string | undefined { return this._fields["firstText"] as string; }
    public set firstText(value: string | undefined) { this._fields["firstText"] = value; this.markDirty('firstText'); }
    /**
     * `Math Formula` (fldlSuvoeWGokSz8Z)
     * 
     * ```
     * IF(
     *   OR(
     *     {First Number} = BLANK(),
     *     {Second Number} = BLANK()
     *   ),
     *   BLANK(),
     *   "SUM: " & SUM({First Number}, {Second Number})
     *   & ", MIN: " & MIN({First Number}, {Second Number})
     *   & ", MAX: " & MAX({First Number}, {Second Number})
     *   & ", AVG: " & AVERAGE({First Number}, {Second Number})
     *   & ", COUNT: " & COUNT({First Number}, {Second Number})
     *   & ", CEIL: " & CEILING({First Number})
     *   & ", FLOOR: " & FLOOR({Second Number})
     *   & ", ROUND: " & ROUND({First Number}/2, 1)
     *   & ", ROUNDUP: " & ROUNDUP({Second Number}/2, 0)
     *   & ", ROUNDDOWN: " & ROUNDDOWN({First Number}/2, 0)
     *   & ", INT: " & INT({Second Number}/2)
     *   & ", EVEN: " & EVEN({First Number})
     *   & ", ODD: " & ODD({Second Number})
     *   & ", MOD: " & MOD({First Number}, {Second Number})
     *   & ", LOG: " & LOG({First Number})
     *   & ", EXP: " & EXP(1)
     *   & ", POWER: " & POWER({First Number}, 2)
     *   & ", SQRT: " & SQRT(
     *     ABS({Second Number})
     *   ) & ", ABS: " & ABS({Second Number})
     * )
     * ```
     */
    public mathFormula(recalculate: boolean = false): string | undefined {
        if (recalculate) this._fields["mathFormula"] = (((this.firstNumber == null) || (this.secondNumber == null)) ? null : ((((((((((((((((((((((((((((((((((((("SUM: " + F.S(F.SUM(this.firstNumber, this.secondNumber))) + ", MIN: ") + F.S(F.MIN(this.firstNumber, this.secondNumber))) + ", MAX: ") + F.S(F.MAX(this.firstNumber, this.secondNumber))) + ", AVG: ") + F.S(F.AVERAGE(this.firstNumber, this.secondNumber))) + ", COUNT: ") + F.S(F.COUNT(this.firstNumber, this.secondNumber))) + ", CEIL: ") + F.S(F.CEILING(this.firstNumber))) + ", FLOOR: ") + F.S(F.FLOOR(this.secondNumber))) + ", ROUND: ") + F.S(F.ROUND((F.N(this.firstNumber) / 2), 1))) + ", ROUNDUP: ") + F.S(F.ROUNDUP((F.N(this.secondNumber) / 2), 0))) + ", ROUNDDOWN: ") + F.S(F.ROUNDDOWN((F.N(this.firstNumber) / 2), 0))) + ", INT: ") + F.S(Math.floor((F.N(this.secondNumber) / 2)))) + ", EVEN: ") + F.S(F.EVEN(this.firstNumber))) + ", ODD: ") + F.S(F.ODD(this.secondNumber))) + ", MOD: ") + F.S((F.N(this.firstNumber) % F.N(this.secondNumber)))) + ", LOG: ") + F.S(F.LOG(this.firstNumber))) + ", EXP: ") + F.S(Math.exp(1))) + ", POWER: ") + F.S(Math.pow(F.N(this.firstNumber), 2))) + ", SQRT: ") + F.S(Math.sqrt(F.N(Math.abs(F.N(this.secondNumber)))))) + ", ABS: ") + F.S(Math.abs(F.N(this.secondNumber)))));
        return this._fields["mathFormula"] as string;
    }
    /** `Primary Key` (fldLZFrZKvSCS4dKb) */
    public get primaryKey(): string | undefined { return this._fields["primaryKey"] as string; }
    public set primaryKey(value: string | undefined) { this._fields["primaryKey"] = value; this.markDirty('primaryKey'); }
    /** `Second Date` (fld1LZ3Ebpt0LaMmu) */
    public get secondDate(): string | undefined { return this._fields["secondDate"] as string; }
    public set secondDate(value: string | undefined) { this._fields["secondDate"] = value; this.markDirty('secondDate'); }
    /** `Second Number` (fldj5nAkal5y8OOZg) */
    public get secondNumber(): number | undefined { return this._fields["secondNumber"] as number; }
    public set secondNumber(value: number | undefined) { this._fields["secondNumber"] = value; this.markDirty('secondNumber'); }
    /** `Second Text` (fldA2boNwwsiuvXw1) */
    public get secondText(): string | undefined { return this._fields["secondText"] as string; }
    public set secondText(value: string | undefined) { this._fields["secondText"] = value; this.markDirty('secondText'); }
    /**
     * `Text Formula` (flddvzeqt7FJpQ9NX)
     * 
     * ```
     * CONCATENATE(
     *   "LEN: ",
     *   LEN({First Text}),
     *   "; ",
     *   "MID: ",
     *   MID({First Text}, 2, 3),
     *   "; ",
     *   "LEFT: ",
     *   LEFT({Second Text}, 2),
     *   "; ",
     *   "RIGHT: ",
     *   RIGHT({Second Text}, 2),
     *   "; ",
     *   "FIND: ",
     *   FIND("e", {First Text}),
     *   "; ",
     *   "SEARCH: ",
     *   SEARCH("e", {First Text}),
     *   "; ",
     *   "REPLACE: ",
     *   REPLACE({First Text}, 2, 2, "XX"),
     *   "; ",
     *   "REPT: ",
     *   REPT({First Text}, 2),
     *   "; ",
     *   "LOWER: ",
     *   LOWER({Second Text}),
     *   "; ",
     *   "UPPER: ",
     *   UPPER({Second Text}),
     *   "; ",
     *   "TRIM: ",
     *   TRIM("   " & {First Text} & "   "),
     *   "; ",
     *   "SUBSTITUTE: ",
     *   SUBSTITUTE({First Text}, "e", "@"),
     *   "; ",
     *   "CONCATENATE: ",
     *   CONCATENATE(
     *     {First Text},
     *     "-",
     *     {Second Text},
     *     "-",
     *     {Third Text}
     *   ),
     *   "; ",
     *   "T: ",
     *   T({First Number}),
     *   "; ",
     *   "REGEX_EXTRACT: ",
     *   REGEX_EXTRACT({First Text}, "[aeiou]"),
     *   "; ",
     *   "REGEX_MATCH: ",
     *   IF(
     *     REGEX_MATCH({First Text}, "^.e"),
     *     "1",
     *     "0"
     *   ),
     *   "; ",
     *   "REGEX_REPLACE: ",
     *   REGEX_REPLACE({First Text}, "[aeiou]", "*"),
     *   "; ",
     *   "ENCODE_URL_COMPONENT: ",
     *   ENCODE_URL_COMPONENT({First Text})
     * )
     * ```
     */
    public textFormula(recalculate: boolean = false): string | undefined {
        if (recalculate) this._fields["textFormula"] = F.CONCATENATE("LEN: ", F.S(this.firstText).length, "; ", "MID: ", F.MID(this.firstText, 2, 3), "; ", "LEFT: ", F.LEFT(this.secondText, 2), "; ", "RIGHT: ", F.RIGHT(this.secondText, 2), "; ", "FIND: ", F.FIND("e", this.firstText), "; ", "SEARCH: ", F.SEARCH("e", this.firstText), "; ", "REPLACE: ", F.REPLACE(this.firstText, 2, 2, "XX"), "; ", "REPT: ", F.REPT(this.firstText, 2), "; ", "LOWER: ", F.S(this.secondText).toLowerCase(), "; ", "UPPER: ", F.S(this.secondText).toUpperCase(), "; ", "TRIM: ", (("   " + F.S(this.firstText)) + "   ").trim(), "; ", "SUBSTITUTE: ", F.SUBSTITUTE(this.firstText, "e", "@"), "; ", "CONCATENATE: ", F.CONCATENATE(this.firstText, "-", this.secondText, "-", this.thirdText), "; ", "T: ", F.T(this.firstNumber), "; ", "REGEX_EXTRACT: ", (F.S(this.firstText).match(new RegExp("[aeiou]"))?.[0] ?? null), "; ", "REGEX_MATCH: ", (F.REGEX_MATCH(this.firstText, "^.e") ? "1" : "0"), "; ", "REGEX_REPLACE: ", F.REGEX_REPLACE(this.firstText, "[aeiou]", "*"), "; ", "ENCODE_URL_COMPONENT: ", encodeURIComponent(F.S(this.firstText)));
        return this._fields["textFormula"] as string;
    }
    /** `Third Date` (fldxSQRRn8W879aiU) */
    public get thirdDate(): string | undefined { return this._fields["thirdDate"] as string; }
    public set thirdDate(value: string | undefined) { this._fields["thirdDate"] = value; this.markDirty('thirdDate'); }
    /** `Third Number` (fld5NBdekrAUzu4Fi) */
    public get thirdNumber(): number | undefined { return this._fields["thirdNumber"] as number; }
    public set thirdNumber(value: number | undefined) { this._fields["thirdNumber"] = value; this.markDirty('thirdNumber'); }
    /** `Third Text` (fldfruPf8V9K6qIAN) */
    public get thirdText(): string | undefined { return this._fields["thirdText"] as string; }
    public set thirdText(value: string | undefined) { this._fields["thirdText"] = value; this.markDirty('thirdText'); }

    constructor(data: IFormulas = {}) {
        super(data.id ?? '');
        this.initializeFields(data);
        this.record = new Record<FormulasFieldSet>(new FormulasTable(this.getInstanceBaseId(), this.getInstanceOptions())._table, this.id, {});
        this.updateRecord();
    }

}
