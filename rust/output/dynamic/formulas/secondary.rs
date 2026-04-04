// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use crate::formula::*;

/// Formula builder for `Secondary`
pub struct SecondaryFormulas {
    /// Record ID formula.
    pub id: FormulaId,
    /// `Link to Tertiary`
    pub link_to_tertiary: FormulaTextField,
    /// `Name`
    pub name: FormulaTextField,
    /// `Primary`
    pub primary: FormulaTextField,
    /// `Primary 2`
    pub primary_2: FormulaTextField,
    /// `Value`
    pub value: FormulaTextField,
}

impl SecondaryFormulas {
    pub const fn new() -> Self {
        Self {
            id: FormulaId,
            link_to_tertiary: FormulaTextField::new("fldKR6tdbnOBRCtdQ"),
            name: FormulaTextField::new("fld1RagdJ09mpWhzM"),
            primary: FormulaTextField::new("fldl0nB9WRFSdqlii"),
            primary_2: FormulaTextField::new("fldgoE2oZmXmKkQca"),
            value: FormulaTextField::new("fldi6Mxh5H1gPGxFX"),
        }
    }
}
