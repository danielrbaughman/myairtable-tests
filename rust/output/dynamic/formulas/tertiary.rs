// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use crate::formula::*;

/// Formula builder for `Tertiary`
pub struct TertiaryFormulas {
    /// Record ID formula.
    pub id: FormulaId,
    /// `Name`
    pub name: FormulaTextField,
    /// `Secondary`
    pub secondary: FormulaTextField,
    /// `Value`
    pub value: FormulaTextField,
}

impl TertiaryFormulas {
    pub const fn new() -> Self {
        Self {
            id: FormulaId,
            name: FormulaTextField::new("fldwzqKxsRnPZJ2Ll"),
            secondary: FormulaTextField::new("fld8lCuUXpEXkIeYv"),
            value: FormulaTextField::new("fldjNLBh2UccM64h5"),
        }
    }
}
