// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

const z = require("zod");
const {
    recordIdSchema,
    AirtableAttachmentSchema,
    AirtableCollaboratorSchema,
    AirtableButtonSchema,
    SpecialNumberSchema,
    ErrorValueSchema,
} = require("../../static/special-types");

// #region SECONDARY
const SecondarySchema = z.object({
    id: recordIdSchema.optional(),
    linkToTertiary: z.array(recordIdSchema).optional(),
    name: z.string().optional(),
    primary: z.array(recordIdSchema).optional(),
    primary2: z.array(recordIdSchema).optional(),
    value: z.string().optional(),
});

// #endregion

module.exports = { SecondarySchema };
