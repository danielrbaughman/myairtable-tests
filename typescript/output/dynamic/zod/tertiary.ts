// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

import * as z from "zod";
import { recordIdSchema, AirtableAttachmentSchema, AirtableCollaboratorSchema, AirtableButtonSchema, SpecialNumberSchema, ErrorValueSchema } from "../../static/special-types";

// #region TERTIARY
export const TertiarySchema = z.object({
    id: recordIdSchema.optional(),
    name: z.string().optional(),
    value: z.string().optional(),
});

export type ITertiary = z.infer<typeof TertiarySchema>;

// #endregion

