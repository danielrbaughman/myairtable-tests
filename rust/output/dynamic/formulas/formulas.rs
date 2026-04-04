// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use crate::formula::*;

/// Formula builder for `Formulas`
pub struct FormulasFormulas {
    /// Record ID formula.
    pub id: FormulaId,
    /// `Date Formula`
    pub date_formula: FormulaTextField,
    /// `First Date`
    pub first_date: FormulaDateField,
    /// `First Number`
    pub first_number: FormulaNumberField,
    /// `First Text`
    pub first_text: FormulaTextField,
    /// `Math Formula`
    pub math_formula: FormulaTextField,
    /// `Primary Key`
    pub primary_key: FormulaTextField,
    /// `Second Date`
    pub second_date: FormulaDateField,
    /// `Second Number`
    pub second_number: FormulaNumberField,
    /// `Second Text`
    pub second_text: FormulaTextField,
    /// `Text Formula`
    pub text_formula: FormulaTextField,
    /// `Third Date`
    pub third_date: FormulaDateField,
    /// `Third Number`
    pub third_number: FormulaNumberField,
    /// `Third Text`
    pub third_text: FormulaTextField,
}

impl FormulasFormulas {
    pub const fn new() -> Self {
        Self {
            id: FormulaId,
            date_formula: FormulaTextField::new("fldY7kjaklLeoSgGd"),
            first_date: FormulaDateField::new("fldlZT521Iy0FFXFL"),
            first_number: FormulaNumberField::new("fldA04pqfjMkGXcZU"),
            first_text: FormulaTextField::new("fldHJuw6pAujnHvkP"),
            math_formula: FormulaTextField::new("fldlSuvoeWGokSz8Z"),
            primary_key: FormulaTextField::new("fldLZFrZKvSCS4dKb"),
            second_date: FormulaDateField::new("fld1LZ3Ebpt0LaMmu"),
            second_number: FormulaNumberField::new("fldj5nAkal5y8OOZg"),
            second_text: FormulaTextField::new("fldA2boNwwsiuvXw1"),
            text_formula: FormulaTextField::new("flddvzeqt7FJpQ9NX"),
            third_date: FormulaDateField::new("fldxSQRRn8W879aiU"),
            third_number: FormulaNumberField::new("fld5NBdekrAUzu4Fi"),
            third_text: FormulaTextField::new("fldfruPf8V9K6qIAN"),
        }
    }
}
