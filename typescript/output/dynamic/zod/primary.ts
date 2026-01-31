// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

import * as z from "zod";
import { recordIdSchema, AirtableAttachmentSchema, AirtableCollaboratorSchema, AirtableButtonSchema, SpecialNumberSchema, ErrorValueSchema } from "../../static/special-types";

// #region PRIMARY
export const PrimarySchema = z.object({
    id: recordIdSchema.optional(),
    primaryKey: z.string().optional(),
    singleLineText: z.string().optional(),
});

export type IPrimary = z.infer<typeof PrimarySchema>;

// #endregion

