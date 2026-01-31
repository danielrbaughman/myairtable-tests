// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

// #region IMPORTS
import { Attachment, Collaborator, FieldSet } from "airtable";
import { RecordId } from "../../static/special-types";
// #endregion


// #region FIELD OPTIONS
    /** Select options for `Multiple Select` */
export type PrimaryMultipleSelectOption = 
    "Option 1" |
    "Option 2" |
    "Option 3"
    /** Select options for `Multiple Select` */
export const PrimaryMultipleSelectOptions: PrimaryMultipleSelectOption[] = [
    "Option 1",
    "Option 2",
    "Option 3",
]

    /** Select options for `Single Select` */
export type PrimarySingleSelectOption = 
    "Choice 1" |
    "Choice 2" |
    "Choice 3"
    /** Select options for `Single Select` */
export const PrimarySingleSelectOptions: PrimarySingleSelectOption[] = [
    "Choice 1",
    "Choice 2",
    "Choice 3",
]

// #endregion

// #region PRIMARY
    /** Field names for `Primary` */
export type PrimaryField = 
    "Attachment" |
    "Auto Number" |
    "Button" |
    "Checkbox" |
    "Created By" |
    "Created Time" |
    "Currency (float)" |
    "Currency (int)" |
    "Date" |
    "Date (with time)" |
    "Duration" |
    "Email" |
    "Formula (Complex)" |
    "Formula (ID)" |
    "Formula (Nested)" |
    "Formula (Simple)" |
    "Last Modified By" |
    "Last Modified Time" |
    "Link (multiple)" |
    "Link (single)" |
    "Long Text" |
    "Long Text with Rich Text" |
    "Lookup" |
    "Multiple Select" |
    "Number (float)" |
    "Number (int)" |
    "Percent (float)" |
    "Percent (int)" |
    "Phone Number" |
    "Primary Key" |
    "Rating" |
    "Rollup" |
    "Single Line Text" |
    "Single Select" |
    "URL" |
    "User" |
    "User (allow multiple)"
    /** Field names for `Primary` */
export const PrimaryFields: PrimaryField[] = [
    "Attachment",
    "Auto Number",
    "Button",
    "Checkbox",
    "Created By",
    "Created Time",
    "Currency (float)",
    "Currency (int)",
    "Date",
    "Date (with time)",
    "Duration",
    "Email",
    "Formula (Complex)",
    "Formula (ID)",
    "Formula (Nested)",
    "Formula (Simple)",
    "Last Modified By",
    "Last Modified Time",
    "Link (multiple)",
    "Link (single)",
    "Long Text",
    "Long Text with Rich Text",
    "Lookup",
    "Multiple Select",
    "Number (float)",
    "Number (int)",
    "Percent (float)",
    "Percent (int)",
    "Phone Number",
    "Primary Key",
    "Rating",
    "Rollup",
    "Single Line Text",
    "Single Select",
    "URL",
    "User",
    "User (allow multiple)",
]

    /** Field IDs for `Primary` */
export type PrimaryFieldId = 
    "fldhF2AEuSC1haCZd" |
    "fldizvTkxgIn0mC3L" |
    "fldY48yKPG16AajtU" |
    "fldjQIaAZVegb1FUa" |
    "fldGLQhDz2UjjiHG6" |
    "fld2YgW382Kt9xltA" |
    "fldyh8pzDXiy5abEr" |
    "fldBfo74z9hD78hP8" |
    "fldC6LfNVvVIxKyQH" |
    "fldizYmjpXABGDLTG" |
    "fldLTyf6ljS0rhur8" |
    "fldHCJoYBiFVsNvP4" |
    "fld2vnFc0Bl5IOFUQ" |
    "fldcf62YFeIIDHElt" |
    "fldXFeHRPBLz6AiWh" |
    "fldy1axxaoUToLVC6" |
    "fldF8iDttqP0AgzWC" |
    "fldMinKh4pa3YX86g" |
    "fldFyFheQWczd8oux" |
    "fld7F5onkDo6mkmbN" |
    "fld8ulc6J0W29M6La" |
    "fldHJkxCMC0xo343u" |
    "fldbmFmrzYKBktJvE" |
    "fld6GTabFmu1xKPvZ" |
    "fldmU0X2l4RWd21dd" |
    "fldOfPKGmnRPv94QH" |
    "fldiGui9ll69N7WOj" |
    "fldbAAyWboGulpb4s" |
    "fld38tnNpHmoks8C8" |
    "fldol5Q4wmQJQvPRy" |
    "fldRsmwFwQNZkKLp4" |
    "fldGaFgBsDC3IBUdV" |
    "fld0BL2lFo9fqcKv3" |
    "fldn0GFFtMFpCXUNU" |
    "fldLYloz2oP4ymf3B" |
    "fldU6SbLp8CSkLcA4" |
    "fldBwCDbAVxRj9yg7"
    /** Field IDs for `Primary` */
export const PrimaryFieldIds: PrimaryFieldId[] = [
    "fldhF2AEuSC1haCZd",
    "fldizvTkxgIn0mC3L",
    "fldY48yKPG16AajtU",
    "fldjQIaAZVegb1FUa",
    "fldGLQhDz2UjjiHG6",
    "fld2YgW382Kt9xltA",
    "fldyh8pzDXiy5abEr",
    "fldBfo74z9hD78hP8",
    "fldC6LfNVvVIxKyQH",
    "fldizYmjpXABGDLTG",
    "fldLTyf6ljS0rhur8",
    "fldHCJoYBiFVsNvP4",
    "fld2vnFc0Bl5IOFUQ",
    "fldcf62YFeIIDHElt",
    "fldXFeHRPBLz6AiWh",
    "fldy1axxaoUToLVC6",
    "fldF8iDttqP0AgzWC",
    "fldMinKh4pa3YX86g",
    "fldFyFheQWczd8oux",
    "fld7F5onkDo6mkmbN",
    "fld8ulc6J0W29M6La",
    "fldHJkxCMC0xo343u",
    "fldbmFmrzYKBktJvE",
    "fld6GTabFmu1xKPvZ",
    "fldmU0X2l4RWd21dd",
    "fldOfPKGmnRPv94QH",
    "fldiGui9ll69N7WOj",
    "fldbAAyWboGulpb4s",
    "fld38tnNpHmoks8C8",
    "fldol5Q4wmQJQvPRy",
    "fldRsmwFwQNZkKLp4",
    "fldGaFgBsDC3IBUdV",
    "fld0BL2lFo9fqcKv3",
    "fldn0GFFtMFpCXUNU",
    "fldLYloz2oP4ymf3B",
    "fldU6SbLp8CSkLcA4",
    "fldBwCDbAVxRj9yg7",
]

    /** Property names for `Primary` */
export type PrimaryFieldProperty = 
    "attachment" |
    "autoNumber" |
    "button" |
    "checkbox" |
    "createdBy" |
    "createdAtTime" |
    "currencyFloat" |
    "currencyInt" |
    "date" |
    "dateWithTime" |
    "duration" |
    "email" |
    "formulaComplex" |
    "formulaId" |
    "formulaNested" |
    "formulaSimple" |
    "lastModifiedBy" |
    "lastModifiedTime" |
    "linkMultiple" |
    "linkSingle" |
    "longText" |
    "longTextWithRichText" |
    "lookup" |
    "multipleSelect" |
    "numberFloat" |
    "numberInt" |
    "percentFloat" |
    "percentInt" |
    "phoneNumber" |
    "primaryKey" |
    "rating" |
    "rollup" |
    "singleLineText" |
    "singleSelect" |
    "url" |
    "user" |
    "userAllowMultiple"
    /** Property names for `Primary` */
export const PrimaryFieldPropertys: PrimaryFieldProperty[] = [
    "attachment",
    "autoNumber",
    "button",
    "checkbox",
    "createdBy",
    "createdAtTime",
    "currencyFloat",
    "currencyInt",
    "date",
    "dateWithTime",
    "duration",
    "email",
    "formulaComplex",
    "formulaId",
    "formulaNested",
    "formulaSimple",
    "lastModifiedBy",
    "lastModifiedTime",
    "linkMultiple",
    "linkSingle",
    "longText",
    "longTextWithRichText",
    "lookup",
    "multipleSelect",
    "numberFloat",
    "numberInt",
    "percentFloat",
    "percentInt",
    "phoneNumber",
    "primaryKey",
    "rating",
    "rollup",
    "singleLineText",
    "singleSelect",
    "url",
    "user",
    "userAllowMultiple",
]

    /** Calculated fields for `Primary` */
export const PrimaryCalculatedFields: string[] = [
    "Button",
    "Created By",
    "Created Time",
    "Formula (Complex)",
    "Formula (ID)",
    "Formula (Nested)",
    "Formula (Simple)",
    "Last Modified By",
    "Last Modified Time",
    "Lookup",
    "Rollup",
]
    /** Calculated fields for `Primary` */
export const PrimaryCalculatedFieldIds: string[] = [
    "fldY48yKPG16AajtU",
    "fldGLQhDz2UjjiHG6",
    "fld2YgW382Kt9xltA",
    "fld2vnFc0Bl5IOFUQ",
    "fldcf62YFeIIDHElt",
    "fldXFeHRPBLz6AiWh",
    "fldy1axxaoUToLVC6",
    "fldF8iDttqP0AgzWC",
    "fldMinKh4pa3YX86g",
    "fldbmFmrzYKBktJvE",
    "fldGaFgBsDC3IBUdV",
]

    /** Writable fields for `Primary` */
export const PrimaryWritableFields: string[] = [
    "Attachment",
    "Auto Number",
    "Checkbox",
    "Currency (float)",
    "Currency (int)",
    "Date",
    "Date (with time)",
    "Duration",
    "Email",
    "Link (multiple)",
    "Link (single)",
    "Long Text",
    "Long Text with Rich Text",
    "Multiple Select",
    "Number (float)",
    "Number (int)",
    "Percent (float)",
    "Percent (int)",
    "Phone Number",
    "Primary Key",
    "Rating",
    "Single Line Text",
    "Single Select",
    "URL",
    "User",
    "User (allow multiple)",
]
    /** Writable fields for `Primary` */
export const PrimaryWritableFieldIds: string[] = [
    "fldhF2AEuSC1haCZd",
    "fldizvTkxgIn0mC3L",
    "fldjQIaAZVegb1FUa",
    "fldyh8pzDXiy5abEr",
    "fldBfo74z9hD78hP8",
    "fldC6LfNVvVIxKyQH",
    "fldizYmjpXABGDLTG",
    "fldLTyf6ljS0rhur8",
    "fldHCJoYBiFVsNvP4",
    "fldFyFheQWczd8oux",
    "fld7F5onkDo6mkmbN",
    "fld8ulc6J0W29M6La",
    "fldHJkxCMC0xo343u",
    "fld6GTabFmu1xKPvZ",
    "fldmU0X2l4RWd21dd",
    "fldOfPKGmnRPv94QH",
    "fldiGui9ll69N7WOj",
    "fldbAAyWboGulpb4s",
    "fld38tnNpHmoks8C8",
    "fldol5Q4wmQJQvPRy",
    "fldRsmwFwQNZkKLp4",
    "fld0BL2lFo9fqcKv3",
    "fldn0GFFtMFpCXUNU",
    "fldLYloz2oP4ymf3B",
    "fldU6SbLp8CSkLcA4",
    "fldBwCDbAVxRj9yg7",
]
export const PrimaryFieldNameIdMapping: Record<PrimaryField, PrimaryFieldId> = {
    "Attachment": "fldhF2AEuSC1haCZd",
    "Auto Number": "fldizvTkxgIn0mC3L",
    "Button": "fldY48yKPG16AajtU",
    "Checkbox": "fldjQIaAZVegb1FUa",
    "Created By": "fldGLQhDz2UjjiHG6",
    "Created Time": "fld2YgW382Kt9xltA",
    "Currency (float)": "fldyh8pzDXiy5abEr",
    "Currency (int)": "fldBfo74z9hD78hP8",
    "Date": "fldC6LfNVvVIxKyQH",
    "Date (with time)": "fldizYmjpXABGDLTG",
    "Duration": "fldLTyf6ljS0rhur8",
    "Email": "fldHCJoYBiFVsNvP4",
    "Formula (Complex)": "fld2vnFc0Bl5IOFUQ",
    "Formula (ID)": "fldcf62YFeIIDHElt",
    "Formula (Nested)": "fldXFeHRPBLz6AiWh",
    "Formula (Simple)": "fldy1axxaoUToLVC6",
    "Last Modified By": "fldF8iDttqP0AgzWC",
    "Last Modified Time": "fldMinKh4pa3YX86g",
    "Link (multiple)": "fldFyFheQWczd8oux",
    "Link (single)": "fld7F5onkDo6mkmbN",
    "Long Text": "fld8ulc6J0W29M6La",
    "Long Text with Rich Text": "fldHJkxCMC0xo343u",
    "Lookup": "fldbmFmrzYKBktJvE",
    "Multiple Select": "fld6GTabFmu1xKPvZ",
    "Number (float)": "fldmU0X2l4RWd21dd",
    "Number (int)": "fldOfPKGmnRPv94QH",
    "Percent (float)": "fldiGui9ll69N7WOj",
    "Percent (int)": "fldbAAyWboGulpb4s",
    "Phone Number": "fld38tnNpHmoks8C8",
    "Primary Key": "fldol5Q4wmQJQvPRy",
    "Rating": "fldRsmwFwQNZkKLp4",
    "Rollup": "fldGaFgBsDC3IBUdV",
    "Single Line Text": "fld0BL2lFo9fqcKv3",
    "Single Select": "fldn0GFFtMFpCXUNU",
    "URL": "fldLYloz2oP4ymf3B",
    "User": "fldU6SbLp8CSkLcA4",
    "User (allow multiple)": "fldBwCDbAVxRj9yg7",
}

export const PrimaryFieldIdNameMapping: Record<PrimaryFieldId, PrimaryField> = {
    "fldhF2AEuSC1haCZd": "Attachment",
    "fldizvTkxgIn0mC3L": "Auto Number",
    "fldY48yKPG16AajtU": "Button",
    "fldjQIaAZVegb1FUa": "Checkbox",
    "fldGLQhDz2UjjiHG6": "Created By",
    "fld2YgW382Kt9xltA": "Created Time",
    "fldyh8pzDXiy5abEr": "Currency (float)",
    "fldBfo74z9hD78hP8": "Currency (int)",
    "fldC6LfNVvVIxKyQH": "Date",
    "fldizYmjpXABGDLTG": "Date (with time)",
    "fldLTyf6ljS0rhur8": "Duration",
    "fldHCJoYBiFVsNvP4": "Email",
    "fld2vnFc0Bl5IOFUQ": "Formula (Complex)",
    "fldcf62YFeIIDHElt": "Formula (ID)",
    "fldXFeHRPBLz6AiWh": "Formula (Nested)",
    "fldy1axxaoUToLVC6": "Formula (Simple)",
    "fldF8iDttqP0AgzWC": "Last Modified By",
    "fldMinKh4pa3YX86g": "Last Modified Time",
    "fldFyFheQWczd8oux": "Link (multiple)",
    "fld7F5onkDo6mkmbN": "Link (single)",
    "fld8ulc6J0W29M6La": "Long Text",
    "fldHJkxCMC0xo343u": "Long Text with Rich Text",
    "fldbmFmrzYKBktJvE": "Lookup",
    "fld6GTabFmu1xKPvZ": "Multiple Select",
    "fldmU0X2l4RWd21dd": "Number (float)",
    "fldOfPKGmnRPv94QH": "Number (int)",
    "fldiGui9ll69N7WOj": "Percent (float)",
    "fldbAAyWboGulpb4s": "Percent (int)",
    "fld38tnNpHmoks8C8": "Phone Number",
    "fldol5Q4wmQJQvPRy": "Primary Key",
    "fldRsmwFwQNZkKLp4": "Rating",
    "fldGaFgBsDC3IBUdV": "Rollup",
    "fld0BL2lFo9fqcKv3": "Single Line Text",
    "fldn0GFFtMFpCXUNU": "Single Select",
    "fldLYloz2oP4ymf3B": "URL",
    "fldU6SbLp8CSkLcA4": "User",
    "fldBwCDbAVxRj9yg7": "User (allow multiple)",
}

export const PrimaryFieldIdPropertyMapping: Record<PrimaryFieldId, PrimaryFieldProperty> = {
    "fldhF2AEuSC1haCZd": "attachment",
    "fldizvTkxgIn0mC3L": "autoNumber",
    "fldY48yKPG16AajtU": "button",
    "fldjQIaAZVegb1FUa": "checkbox",
    "fldGLQhDz2UjjiHG6": "createdBy",
    "fld2YgW382Kt9xltA": "createdAtTime",
    "fldyh8pzDXiy5abEr": "currencyFloat",
    "fldBfo74z9hD78hP8": "currencyInt",
    "fldC6LfNVvVIxKyQH": "date",
    "fldizYmjpXABGDLTG": "dateWithTime",
    "fldLTyf6ljS0rhur8": "duration",
    "fldHCJoYBiFVsNvP4": "email",
    "fld2vnFc0Bl5IOFUQ": "formulaComplex",
    "fldcf62YFeIIDHElt": "formulaId",
    "fldXFeHRPBLz6AiWh": "formulaNested",
    "fldy1axxaoUToLVC6": "formulaSimple",
    "fldF8iDttqP0AgzWC": "lastModifiedBy",
    "fldMinKh4pa3YX86g": "lastModifiedTime",
    "fldFyFheQWczd8oux": "linkMultiple",
    "fld7F5onkDo6mkmbN": "linkSingle",
    "fld8ulc6J0W29M6La": "longText",
    "fldHJkxCMC0xo343u": "longTextWithRichText",
    "fldbmFmrzYKBktJvE": "lookup",
    "fld6GTabFmu1xKPvZ": "multipleSelect",
    "fldmU0X2l4RWd21dd": "numberFloat",
    "fldOfPKGmnRPv94QH": "numberInt",
    "fldiGui9ll69N7WOj": "percentFloat",
    "fldbAAyWboGulpb4s": "percentInt",
    "fld38tnNpHmoks8C8": "phoneNumber",
    "fldol5Q4wmQJQvPRy": "primaryKey",
    "fldRsmwFwQNZkKLp4": "rating",
    "fldGaFgBsDC3IBUdV": "rollup",
    "fld0BL2lFo9fqcKv3": "singleLineText",
    "fldn0GFFtMFpCXUNU": "singleSelect",
    "fldLYloz2oP4ymf3B": "url",
    "fldU6SbLp8CSkLcA4": "user",
    "fldBwCDbAVxRj9yg7": "userAllowMultiple",
}

export const PrimaryFieldPropertyIdMapping: Record<PrimaryFieldProperty, PrimaryFieldId> = {
    "attachment": "fldhF2AEuSC1haCZd",
    "autoNumber": "fldizvTkxgIn0mC3L",
    "button": "fldY48yKPG16AajtU",
    "checkbox": "fldjQIaAZVegb1FUa",
    "createdBy": "fldGLQhDz2UjjiHG6",
    "createdAtTime": "fld2YgW382Kt9xltA",
    "currencyFloat": "fldyh8pzDXiy5abEr",
    "currencyInt": "fldBfo74z9hD78hP8",
    "date": "fldC6LfNVvVIxKyQH",
    "dateWithTime": "fldizYmjpXABGDLTG",
    "duration": "fldLTyf6ljS0rhur8",
    "email": "fldHCJoYBiFVsNvP4",
    "formulaComplex": "fld2vnFc0Bl5IOFUQ",
    "formulaId": "fldcf62YFeIIDHElt",
    "formulaNested": "fldXFeHRPBLz6AiWh",
    "formulaSimple": "fldy1axxaoUToLVC6",
    "lastModifiedBy": "fldF8iDttqP0AgzWC",
    "lastModifiedTime": "fldMinKh4pa3YX86g",
    "linkMultiple": "fldFyFheQWczd8oux",
    "linkSingle": "fld7F5onkDo6mkmbN",
    "longText": "fld8ulc6J0W29M6La",
    "longTextWithRichText": "fldHJkxCMC0xo343u",
    "lookup": "fldbmFmrzYKBktJvE",
    "multipleSelect": "fld6GTabFmu1xKPvZ",
    "numberFloat": "fldmU0X2l4RWd21dd",
    "numberInt": "fldOfPKGmnRPv94QH",
    "percentFloat": "fldiGui9ll69N7WOj",
    "percentInt": "fldbAAyWboGulpb4s",
    "phoneNumber": "fld38tnNpHmoks8C8",
    "primaryKey": "fldol5Q4wmQJQvPRy",
    "rating": "fldRsmwFwQNZkKLp4",
    "rollup": "fldGaFgBsDC3IBUdV",
    "singleLineText": "fld0BL2lFo9fqcKv3",
    "singleSelect": "fldn0GFFtMFpCXUNU",
    "url": "fldLYloz2oP4ymf3B",
    "user": "fldU6SbLp8CSkLcA4",
    "userAllowMultiple": "fldBwCDbAVxRj9yg7",
}

export const PrimaryFieldNamePropertyMapping: Record<PrimaryField, PrimaryFieldProperty> = {
    "Attachment": "attachment",
    "Auto Number": "autoNumber",
    "Button": "button",
    "Checkbox": "checkbox",
    "Created By": "createdBy",
    "Created Time": "createdAtTime",
    "Currency (float)": "currencyFloat",
    "Currency (int)": "currencyInt",
    "Date": "date",
    "Date (with time)": "dateWithTime",
    "Duration": "duration",
    "Email": "email",
    "Formula (Complex)": "formulaComplex",
    "Formula (ID)": "formulaId",
    "Formula (Nested)": "formulaNested",
    "Formula (Simple)": "formulaSimple",
    "Last Modified By": "lastModifiedBy",
    "Last Modified Time": "lastModifiedTime",
    "Link (multiple)": "linkMultiple",
    "Link (single)": "linkSingle",
    "Long Text": "longText",
    "Long Text with Rich Text": "longTextWithRichText",
    "Lookup": "lookup",
    "Multiple Select": "multipleSelect",
    "Number (float)": "numberFloat",
    "Number (int)": "numberInt",
    "Percent (float)": "percentFloat",
    "Percent (int)": "percentInt",
    "Phone Number": "phoneNumber",
    "Primary Key": "primaryKey",
    "Rating": "rating",
    "Rollup": "rollup",
    "Single Line Text": "singleLineText",
    "Single Select": "singleSelect",
    "URL": "url",
    "User": "user",
    "User (allow multiple)": "userAllowMultiple",
}

export const PrimaryFieldPropertyNameMapping: Record<PrimaryFieldProperty, PrimaryField> = {
    "attachment": "Attachment",
    "autoNumber": "Auto Number",
    "button": "Button",
    "checkbox": "Checkbox",
    "createdBy": "Created By",
    "createdAtTime": "Created Time",
    "currencyFloat": "Currency (float)",
    "currencyInt": "Currency (int)",
    "date": "Date",
    "dateWithTime": "Date (with time)",
    "duration": "Duration",
    "email": "Email",
    "formulaComplex": "Formula (Complex)",
    "formulaId": "Formula (ID)",
    "formulaNested": "Formula (Nested)",
    "formulaSimple": "Formula (Simple)",
    "lastModifiedBy": "Last Modified By",
    "lastModifiedTime": "Last Modified Time",
    "linkMultiple": "Link (multiple)",
    "linkSingle": "Link (single)",
    "longText": "Long Text",
    "longTextWithRichText": "Long Text with Rich Text",
    "lookup": "Lookup",
    "multipleSelect": "Multiple Select",
    "numberFloat": "Number (float)",
    "numberInt": "Number (int)",
    "percentFloat": "Percent (float)",
    "percentInt": "Percent (int)",
    "phoneNumber": "Phone Number",
    "primaryKey": "Primary Key",
    "rating": "Rating",
    "rollup": "Rollup",
    "singleLineText": "Single Line Text",
    "singleSelect": "Single Select",
    "url": "URL",
    "user": "User",
    "userAllowMultiple": "User (allow multiple)",
}

export interface PrimaryFieldSetIds extends FieldSet {
    //@ts-ignore
    fldhF2AEuSC1haCZd?: Attachment[]
    //@ts-ignore
    fldizvTkxgIn0mC3L?: number
    //@ts-ignore
    fldY48yKPG16AajtU?: AirtableButton
    //@ts-ignore
    fldjQIaAZVegb1FUa?: boolean
    //@ts-ignore
    fldGLQhDz2UjjiHG6?: Collaborator
    //@ts-ignore
    fld2YgW382Kt9xltA?: string
    //@ts-ignore
    fldyh8pzDXiy5abEr?: number
    //@ts-ignore
    fldBfo74z9hD78hP8?: number
    //@ts-ignore
    fldC6LfNVvVIxKyQH?: string
    //@ts-ignore
    fldizYmjpXABGDLTG?: string
    //@ts-ignore
    fldLTyf6ljS0rhur8?: number
    //@ts-ignore
    fldHCJoYBiFVsNvP4?: string
    //@ts-ignore
    fld2vnFc0Bl5IOFUQ?: string
    //@ts-ignore
    fldcf62YFeIIDHElt?: string
    //@ts-ignore
    fldXFeHRPBLz6AiWh?: string
    //@ts-ignore
    fldy1axxaoUToLVC6?: number
    //@ts-ignore
    fldF8iDttqP0AgzWC?: Collaborator
    //@ts-ignore
    fldMinKh4pa3YX86g?: string
    //@ts-ignore
    fldFyFheQWczd8oux?: RecordId[]
    //@ts-ignore
    fld7F5onkDo6mkmbN?: RecordId
    //@ts-ignore
    fld8ulc6J0W29M6La?: string
    //@ts-ignore
    fldHJkxCMC0xo343u?: string
    //@ts-ignore
    fldbmFmrzYKBktJvE?: string[]
    //@ts-ignore
    fld6GTabFmu1xKPvZ?: PrimaryMultipleSelectOption[]
    //@ts-ignore
    fldmU0X2l4RWd21dd?: number
    //@ts-ignore
    fldOfPKGmnRPv94QH?: number
    //@ts-ignore
    fldiGui9ll69N7WOj?: number
    //@ts-ignore
    fldbAAyWboGulpb4s?: number
    //@ts-ignore
    fld38tnNpHmoks8C8?: string
    //@ts-ignore
    fldol5Q4wmQJQvPRy?: string
    //@ts-ignore
    fldRsmwFwQNZkKLp4?: any
    //@ts-ignore
    fldGaFgBsDC3IBUdV?: string[]
    //@ts-ignore
    fld0BL2lFo9fqcKv3?: string
    //@ts-ignore
    fldn0GFFtMFpCXUNU?: PrimarySingleSelectOption
    //@ts-ignore
    fldLYloz2oP4ymf3B?: string
    //@ts-ignore
    fldU6SbLp8CSkLcA4?: Collaborator
    //@ts-ignore
    fldBwCDbAVxRj9yg7?: Collaborator[]
}

export interface PrimaryFieldSet extends FieldSet {
    //@ts-ignore
    "Attachment"?: Attachment[],
    //@ts-ignore
    "Auto Number"?: number,
    //@ts-ignore
    "Button"?: AirtableButton,
    //@ts-ignore
    "Checkbox"?: boolean,
    //@ts-ignore
    "Created By"?: Collaborator,
    //@ts-ignore
    "Created Time"?: string,
    //@ts-ignore
    "Currency (float)"?: number,
    //@ts-ignore
    "Currency (int)"?: number,
    //@ts-ignore
    "Date"?: string,
    //@ts-ignore
    "Date (with time)"?: string,
    //@ts-ignore
    "Duration"?: number,
    //@ts-ignore
    "Email"?: string,
    //@ts-ignore
    "Formula (Complex)"?: string,
    //@ts-ignore
    "Formula (ID)"?: string,
    //@ts-ignore
    "Formula (Nested)"?: string,
    //@ts-ignore
    "Formula (Simple)"?: number,
    //@ts-ignore
    "Last Modified By"?: Collaborator,
    //@ts-ignore
    "Last Modified Time"?: string,
    //@ts-ignore
    "Link (multiple)"?: RecordId[],
    //@ts-ignore
    "Link (single)"?: RecordId,
    //@ts-ignore
    "Long Text"?: string,
    //@ts-ignore
    "Long Text with Rich Text"?: string,
    //@ts-ignore
    "Lookup"?: string[],
    //@ts-ignore
    "Multiple Select"?: PrimaryMultipleSelectOption[],
    //@ts-ignore
    "Number (float)"?: number,
    //@ts-ignore
    "Number (int)"?: number,
    //@ts-ignore
    "Percent (float)"?: number,
    //@ts-ignore
    "Percent (int)"?: number,
    //@ts-ignore
    "Phone Number"?: string,
    //@ts-ignore
    "Primary Key"?: string,
    //@ts-ignore
    "Rating"?: any,
    //@ts-ignore
    "Rollup"?: string[],
    //@ts-ignore
    "Single Line Text"?: string,
    //@ts-ignore
    "Single Select"?: PrimarySingleSelectOption,
    //@ts-ignore
    "URL"?: string,
    //@ts-ignore
    "User"?: Collaborator,
    //@ts-ignore
    "User (allow multiple)"?: Collaborator[],
}

    /** View names for `Primary` */
export type PrimaryView = 
    "Grid view"
    /** View names for `Primary` */
export const PrimaryViews: PrimaryView[] = [
    "Grid view",
]

    /** View IDs for `Primary` */
export type PrimaryViewId = 
    "viwvPRDMaHyldUpmd"
    /** View IDs for `Primary` */
export const PrimaryViewIds: PrimaryViewId[] = [
    "viwvPRDMaHyldUpmd",
]

export const PrimaryViewNameIdMapping: Record<PrimaryView, PrimaryViewId> = {
    "Grid view": "viwvPRDMaHyldUpmd",
}

export const PrimaryViewIdNameMapping: Record<PrimaryViewId, PrimaryView> = {
    "viwvPRDMaHyldUpmd": "Grid view",
}

// #endregion

