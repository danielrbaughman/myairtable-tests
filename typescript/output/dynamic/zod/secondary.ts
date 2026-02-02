// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

import * as z from "zod";
import { recordIdSchema, AirtableAttachmentSchema, AirtableCollaboratorSchema, AirtableButtonSchema, SpecialNumberSchema, ErrorValueSchema } from "../../static/special-types";

// #region SECONDARY
export const SecondarySchema = z.object({
    id: recordIdSchema.optional(),
    linkToTertiary: z.array(recordIdSchema).optional(),
    name: z.string().optional(),
    primary: z.array(recordIdSchema).optional(),
    primary2: z.array(recordIdSchema).optional(),
    value: z.string().optional(),
});

export type ISecondary = z.infer<typeof SecondarySchema>;

// #endregion

