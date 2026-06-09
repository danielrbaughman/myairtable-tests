// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

const z = require("zod");
const {
    recordIdSchema,
    ErrorValueSchema,
} = require("../../static/special-types");

// #region FORMULAS
const FormulasSchema = z.object({
    id: recordIdSchema.optional(),
    dateFormula: z.union([z.string(), ErrorValueSchema]).optional(),
    firstDate: z.string().optional(),
    firstNumber: z.number().optional(),
    firstText: z.string().optional(),
    mathFormula: z.union([z.string(), ErrorValueSchema]).optional(),
    primaryKey: z.string().optional(),
    secondDate: z.string().optional(),
    secondNumber: z.number().optional(),
    secondText: z.string().optional(),
    textFormula: z.union([z.string(), ErrorValueSchema]).optional(),
    thirdDate: z.string().optional(),
    thirdNumber: z.number().optional(),
    thirdText: z.string().optional(),
});

// #endregion

module.exports = { FormulasSchema };
