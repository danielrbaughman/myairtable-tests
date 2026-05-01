// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use crate::formula::*;

/// Formula builder for `Primary`
pub struct PrimaryFormulas {
    /// Record ID formula.
    pub id: FormulaId,
    /// `Attachment`
    pub attachment: FormulaAttachmentsField,
    /// `Auto Number`
    pub auto_number: FormulaNumberField,
    /// `Button`
    pub button: FormulaTextField,
    /// `Checkbox`
    pub checkbox: FormulaBooleanField,
    /// `Created By`
    pub created_by: FormulaTextField,
    /// `Created Time`
    pub created_at_time: FormulaDateField,
    /// `Currency (float)`
    pub currency_float: FormulaNumberField,
    /// `Currency (int)`
    pub currency_int: FormulaNumberField,
    /// `Date`
    pub date: FormulaDateField,
    /// `Date (with time)`
    pub date_with_time: FormulaDateField,
    /// `Duration`
    pub duration: FormulaNumberField,
    /// `Email`
    pub email: FormulaTextField,
    /// `Formula (Complex)`
    pub formula_complex: FormulaTextField,
    /// `Formula (ID)`
    pub formula_id: FormulaTextField,
    /// `Formula (Nested)`
    pub formula_nested: FormulaTextField,
    /// `Formula (Simple)`
    pub formula_simple: FormulaNumberField,
    /// `Last Modified By`
    pub last_modified_by: FormulaTextField,
    /// `Last Modified Time`
    pub last_modified_time: FormulaDateField,
    /// `Link (multiple)`
    pub link_multiple: FormulaTextField,
    /// `Link (single)`
    pub link_single: FormulaTextField,
    /// `Long Text`
    pub long_text: FormulaTextField,
    /// `Long Text with Rich Text`
    pub long_text_with_rich_text: FormulaTextField,
    /// `Lookup`
    pub lookup: FormulaLookupField,
    /// `Multiple Select`
    pub multiple_select: FormulaMultiSelectField,
    /// `Number (float)`
    pub number_float: FormulaNumberField,
    /// `Number (int)`
    pub number_int: FormulaNumberField,
    /// `Percent (float)`
    pub percent_float: FormulaNumberField,
    /// `Percent (int)`
    pub percent_int: FormulaNumberField,
    /// `Phone Number`
    pub phone_number: FormulaTextField,
    /// `Primary Key`
    pub primary_key: FormulaTextField,
    /// `Rating`
    pub rating: FormulaTextField,
    /// `Rollup`
    pub rollup: FormulaTextField,
    /// `Single Line Text`
    pub single_line_text: FormulaTextField,
    /// `Single Select`
    pub single_select: FormulaSingleSelectField,
    /// `URL`
    pub url: FormulaTextField,
    /// `User`
    pub user: FormulaTextField,
    /// `User (allow multiple)`
    pub user_allow_multiple: FormulaTextField,
}

impl PrimaryFormulas {
    pub const fn new() -> Self {
        Self {
            id: FormulaId,
            attachment: FormulaAttachmentsField::new("fldhF2AEuSC1haCZd"),
            auto_number: FormulaNumberField::new("fldizvTkxgIn0mC3L"),
            button: FormulaTextField::new("fldY48yKPG16AajtU"),
            checkbox: FormulaBooleanField::new("fldjQIaAZVegb1FUa"),
            created_by: FormulaTextField::new("fldGLQhDz2UjjiHG6"),
            created_at_time: FormulaDateField::new("fld2YgW382Kt9xltA"),
            currency_float: FormulaNumberField::new("fldyh8pzDXiy5abEr"),
            currency_int: FormulaNumberField::new("fldBfo74z9hD78hP8"),
            date: FormulaDateField::new("fldC6LfNVvVIxKyQH"),
            date_with_time: FormulaDateField::new("fldizYmjpXABGDLTG"),
            duration: FormulaNumberField::new("fldLTyf6ljS0rhur8"),
            email: FormulaTextField::new("fldHCJoYBiFVsNvP4"),
            formula_complex: FormulaTextField::new("fld2vnFc0Bl5IOFUQ"),
            formula_id: FormulaTextField::new("fldcf62YFeIIDHElt"),
            formula_nested: FormulaTextField::new("fldXFeHRPBLz6AiWh"),
            formula_simple: FormulaNumberField::new("fldy1axxaoUToLVC6"),
            last_modified_by: FormulaTextField::new("fldF8iDttqP0AgzWC"),
            last_modified_time: FormulaDateField::new("fldMinKh4pa3YX86g"),
            link_multiple: FormulaTextField::new("fldFyFheQWczd8oux"),
            link_single: FormulaTextField::new("fld7F5onkDo6mkmbN"),
            long_text: FormulaTextField::new("fld8ulc6J0W29M6La"),
            long_text_with_rich_text: FormulaTextField::new("fldHJkxCMC0xo343u"),
            lookup: FormulaLookupField::new("fldbmFmrzYKBktJvE"),
            multiple_select: FormulaMultiSelectField::new("fld6GTabFmu1xKPvZ"),
            number_float: FormulaNumberField::new("fldmU0X2l4RWd21dd"),
            number_int: FormulaNumberField::new("fldOfPKGmnRPv94QH"),
            percent_float: FormulaNumberField::new("fldiGui9ll69N7WOj"),
            percent_int: FormulaNumberField::new("fldbAAyWboGulpb4s"),
            phone_number: FormulaTextField::new("fld38tnNpHmoks8C8"),
            primary_key: FormulaTextField::new("fldol5Q4wmQJQvPRy"),
            rating: FormulaTextField::new("fldRsmwFwQNZkKLp4"),
            rollup: FormulaTextField::new("fldGaFgBsDC3IBUdV"),
            single_line_text: FormulaTextField::new("fld0BL2lFo9fqcKv3"),
            single_select: FormulaSingleSelectField::new("fldn0GFFtMFpCXUNU"),
            url: FormulaTextField::new("fldLYloz2oP4ymf3B"),
            user: FormulaTextField::new("fldU6SbLp8CSkLcA4"),
            user_allow_multiple: FormulaTextField::new("fldBwCDbAVxRj9yg7"),
        }
    }
}
