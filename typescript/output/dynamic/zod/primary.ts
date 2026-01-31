// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

import * as z from "zod";
import { recordIdSchema, AirtableAttachmentSchema, AirtableCollaboratorSchema, AirtableButtonSchema, SpecialNumberSchema, ErrorValueSchema } from "../../static/special-types";
import {
    PrimaryMultipleSelectOptions,
    PrimarySingleSelectOptions,
} from "../types/primary";

// #region PRIMARY
export const PrimarySchema = z.object({
    id: recordIdSchema.optional(),
    attachment: z.array(AirtableAttachmentSchema).optional(),
    autoNumber: z.union([z.number(), SpecialNumberSchema, ErrorValueSchema]).optional(),
    button: z.union([AirtableButtonSchema, ErrorValueSchema]).optional(),
    checkbox: z.boolean().optional(),
    createdBy: z.union([AirtableCollaboratorSchema, ErrorValueSchema]).optional(),
    createdAtTime: z.union([z.string(), ErrorValueSchema]).optional(),
    currencyFloat: z.number().optional(),
    currencyInt: z.number().optional(),
    date: z.string().optional(),
    dateWithTime: z.string().optional(),
    duration: z.number().optional(),
    email: z.string().optional(),
    formulaComplex: z.union([z.string(), SpecialNumberSchema, ErrorValueSchema, z.array(z.union([z.string(), SpecialNumberSchema, ErrorValueSchema]).nullable())]).optional(),
    formulaId: z.union([z.string(), ErrorValueSchema]).optional(),
    formulaNested: z.union([z.string(), SpecialNumberSchema, ErrorValueSchema, z.array(z.union([z.string(), SpecialNumberSchema, ErrorValueSchema]).nullable())]).optional(),
    formulaSimple: z.union([z.number(), SpecialNumberSchema, ErrorValueSchema]).optional(),
    lastModifiedBy: z.union([AirtableCollaboratorSchema, ErrorValueSchema]).optional(),
    lastModifiedTime: z.union([z.string(), ErrorValueSchema]).optional(),
    linkMultiple: z.array(recordIdSchema).optional(),
    linkSingle: recordIdSchema.optional(),
    longText: z.string().optional(),
    longTextWithRichText: z.string().optional(),
    lookup: z.union([z.string(), SpecialNumberSchema, ErrorValueSchema, z.array(z.union([z.string(), SpecialNumberSchema, ErrorValueSchema]).nullable())]).optional(),
    multipleSelect: z.array(z.enum(PrimaryMultipleSelectOptions)).optional(),
    numberFloat: z.number().optional(),
    numberInt: z.number().optional(),
    percentFloat: z.number().optional(),
    percentInt: z.number().optional(),
    phoneNumber: z.string().optional(),
    primaryKey: z.string().optional(),
    rating: z.any().optional(),
    rollup: z.union([z.string(), SpecialNumberSchema, ErrorValueSchema, z.array(z.union([z.string(), SpecialNumberSchema, ErrorValueSchema]).nullable())]).optional(),
    singleLineText: z.string().optional(),
    singleSelect: z.enum(PrimarySingleSelectOptions).optional(),
    url: z.string().optional(),
    user: AirtableCollaboratorSchema.optional(),
    userAllowMultiple: z.array(AirtableCollaboratorSchema).optional(),
});

export type IPrimary = z.infer<typeof PrimarySchema>;

// #endregion

