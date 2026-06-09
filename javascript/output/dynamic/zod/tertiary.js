// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

const z = require("zod");
const { recordIdSchema } = require("../../static/special-types");

// #region TERTIARY
const TertiarySchema = z.object({
    id: recordIdSchema.optional(),
    name: z.string().optional(),
    secondary: z.array(recordIdSchema).optional(),
    value: z.string().optional(),
});

// #endregion

module.exports = { TertiarySchema };
