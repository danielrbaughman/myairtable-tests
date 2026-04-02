// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use serde::{Deserialize, Serialize};

/// Field constants for `Primary`
pub struct PrimaryFields;

impl PrimaryFields {
    /// `Attachment`
    pub const ATTACHMENT: &'static str = "Attachment";
    /// `Attachment` (field ID)
    pub const ATTACHMENT_ID: &'static str = "fldhF2AEuSC1haCZd";
    /// `Auto Number`
    pub const AUTO_NUMBER: &'static str = "Auto Number";
    /// `Auto Number` (field ID)
    pub const AUTO_NUMBER_ID: &'static str = "fldizvTkxgIn0mC3L";
    /// `Button`
    pub const BUTTON: &'static str = "Button";
    /// `Button` (field ID)
    pub const BUTTON_ID: &'static str = "fldY48yKPG16AajtU";
    /// `Checkbox`
    pub const CHECKBOX: &'static str = "Checkbox";
    /// `Checkbox` (field ID)
    pub const CHECKBOX_ID: &'static str = "fldjQIaAZVegb1FUa";
    /// `Created By`
    pub const CREATED_BY: &'static str = "Created By";
    /// `Created By` (field ID)
    pub const CREATED_BY_ID: &'static str = "fldGLQhDz2UjjiHG6";
    /// `Created Time`
    pub const CREATED_AT_TIME: &'static str = "Created Time";
    /// `Created Time` (field ID)
    pub const CREATED_AT_TIME_ID: &'static str = "fld2YgW382Kt9xltA";
    /// `Currency (float)`
    pub const CURRENCY_FLOAT: &'static str = "Currency (float)";
    /// `Currency (float)` (field ID)
    pub const CURRENCY_FLOAT_ID: &'static str = "fldyh8pzDXiy5abEr";
    /// `Currency (int)`
    pub const CURRENCY_INT: &'static str = "Currency (int)";
    /// `Currency (int)` (field ID)
    pub const CURRENCY_INT_ID: &'static str = "fldBfo74z9hD78hP8";
    /// `Date`
    pub const DATE: &'static str = "Date";
    /// `Date` (field ID)
    pub const DATE_ID: &'static str = "fldC6LfNVvVIxKyQH";
    /// `Date (with time)`
    pub const DATE_WITH_TIME: &'static str = "Date (with time)";
    /// `Date (with time)` (field ID)
    pub const DATE_WITH_TIME_ID: &'static str = "fldizYmjpXABGDLTG";
    /// `Duration`
    pub const DURATION: &'static str = "Duration";
    /// `Duration` (field ID)
    pub const DURATION_ID: &'static str = "fldLTyf6ljS0rhur8";
    /// `Email`
    pub const EMAIL: &'static str = "Email";
    /// `Email` (field ID)
    pub const EMAIL_ID: &'static str = "fldHCJoYBiFVsNvP4";
    /// `Formula (Complex)`
    pub const FORMULA_COMPLEX: &'static str = "Formula (Complex)";
    /// `Formula (Complex)` (field ID)
    pub const FORMULA_COMPLEX_ID: &'static str = "fld2vnFc0Bl5IOFUQ";
    /// `Formula (ID)`
    pub const FORMULA_ID: &'static str = "Formula (ID)";
    /// `Formula (ID)` (field ID)
    pub const FORMULA_ID_ID: &'static str = "fldcf62YFeIIDHElt";
    /// `Formula (Nested)`
    pub const FORMULA_NESTED: &'static str = "Formula (Nested)";
    /// `Formula (Nested)` (field ID)
    pub const FORMULA_NESTED_ID: &'static str = "fldXFeHRPBLz6AiWh";
    /// `Formula (Simple)`
    pub const FORMULA_SIMPLE: &'static str = "Formula (Simple)";
    /// `Formula (Simple)` (field ID)
    pub const FORMULA_SIMPLE_ID: &'static str = "fldy1axxaoUToLVC6";
    /// `Last Modified By`
    pub const LAST_MODIFIED_BY: &'static str = "Last Modified By";
    /// `Last Modified By` (field ID)
    pub const LAST_MODIFIED_BY_ID: &'static str = "fldF8iDttqP0AgzWC";
    /// `Last Modified Time`
    pub const LAST_MODIFIED_TIME: &'static str = "Last Modified Time";
    /// `Last Modified Time` (field ID)
    pub const LAST_MODIFIED_TIME_ID: &'static str = "fldMinKh4pa3YX86g";
    /// `Link (multiple)`
    pub const LINK_MULTIPLE: &'static str = "Link (multiple)";
    /// `Link (multiple)` (field ID)
    pub const LINK_MULTIPLE_ID: &'static str = "fldFyFheQWczd8oux";
    /// `Link (single)`
    pub const LINK_SINGLE: &'static str = "Link (single)";
    /// `Link (single)` (field ID)
    pub const LINK_SINGLE_ID: &'static str = "fld7F5onkDo6mkmbN";
    /// `Long Text`
    pub const LONG_TEXT: &'static str = "Long Text";
    /// `Long Text` (field ID)
    pub const LONG_TEXT_ID: &'static str = "fld8ulc6J0W29M6La";
    /// `Long Text with Rich Text`
    pub const LONG_TEXT_WITH_RICH_TEXT: &'static str = "Long Text with Rich Text";
    /// `Long Text with Rich Text` (field ID)
    pub const LONG_TEXT_WITH_RICH_TEXT_ID: &'static str = "fldHJkxCMC0xo343u";
    /// `Lookup`
    pub const LOOKUP: &'static str = "Lookup";
    /// `Lookup` (field ID)
    pub const LOOKUP_ID: &'static str = "fldbmFmrzYKBktJvE";
    /// `Multiple Select`
    pub const MULTIPLE_SELECT: &'static str = "Multiple Select";
    /// `Multiple Select` (field ID)
    pub const MULTIPLE_SELECT_ID: &'static str = "fld6GTabFmu1xKPvZ";
    /// `Number (float)`
    pub const NUMBER_FLOAT: &'static str = "Number (float)";
    /// `Number (float)` (field ID)
    pub const NUMBER_FLOAT_ID: &'static str = "fldmU0X2l4RWd21dd";
    /// `Number (int)`
    pub const NUMBER_INT: &'static str = "Number (int)";
    /// `Number (int)` (field ID)
    pub const NUMBER_INT_ID: &'static str = "fldOfPKGmnRPv94QH";
    /// `Percent (float)`
    pub const PERCENT_FLOAT: &'static str = "Percent (float)";
    /// `Percent (float)` (field ID)
    pub const PERCENT_FLOAT_ID: &'static str = "fldiGui9ll69N7WOj";
    /// `Percent (int)`
    pub const PERCENT_INT: &'static str = "Percent (int)";
    /// `Percent (int)` (field ID)
    pub const PERCENT_INT_ID: &'static str = "fldbAAyWboGulpb4s";
    /// `Phone Number`
    pub const PHONE_NUMBER: &'static str = "Phone Number";
    /// `Phone Number` (field ID)
    pub const PHONE_NUMBER_ID: &'static str = "fld38tnNpHmoks8C8";
    /// `Primary Key`
    pub const PRIMARY_KEY: &'static str = "Primary Key";
    /// `Primary Key` (field ID)
    pub const PRIMARY_KEY_ID: &'static str = "fldol5Q4wmQJQvPRy";
    /// `Rating`
    pub const RATING: &'static str = "Rating";
    /// `Rating` (field ID)
    pub const RATING_ID: &'static str = "fldRsmwFwQNZkKLp4";
    /// `Rollup`
    pub const ROLLUP: &'static str = "Rollup";
    /// `Rollup` (field ID)
    pub const ROLLUP_ID: &'static str = "fldGaFgBsDC3IBUdV";
    /// `Single Line Text`
    pub const SINGLE_LINE_TEXT: &'static str = "Single Line Text";
    /// `Single Line Text` (field ID)
    pub const SINGLE_LINE_TEXT_ID: &'static str = "fld0BL2lFo9fqcKv3";
    /// `Single Select`
    pub const SINGLE_SELECT: &'static str = "Single Select";
    /// `Single Select` (field ID)
    pub const SINGLE_SELECT_ID: &'static str = "fldn0GFFtMFpCXUNU";
    /// `URL`
    pub const URL: &'static str = "URL";
    /// `URL` (field ID)
    pub const URL_ID: &'static str = "fldLYloz2oP4ymf3B";
    /// `User`
    pub const USER: &'static str = "User";
    /// `User` (field ID)
    pub const USER_ID: &'static str = "fldU6SbLp8CSkLcA4";
    /// `User (allow multiple)`
    pub const USER_ALLOW_MULTIPLE: &'static str = "User (allow multiple)";
    /// `User (allow multiple)` (field ID)
    pub const USER_ALLOW_MULTIPLE_ID: &'static str = "fldBwCDbAVxRj9yg7";
}

/// Views for `Primary`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimaryView {
    /// `Grid view` (grid)
    #[serde(rename = "viwvPRDMaHyldUpmd")]
    GridView,
    /// `Filter by View` (grid)
    #[serde(rename = "viwHlcwGu4xthX1gf")]
    FilterByView,
}

impl AsRef<str> for PrimaryView {
    fn as_ref(&self) -> &str {
        match self {
            Self::GridView => "viwvPRDMaHyldUpmd",
            Self::FilterByView => "viwHlcwGu4xthX1gf",
        }
    }
}

impl From<PrimaryView> for String {
    fn from(v: PrimaryView) -> String {
        v.as_ref().to_string()
    }
}

/// Writable field constants for `Primary`
pub struct CreatePrimaryFields;

impl CreatePrimaryFields {
    /// `Attachment`
    pub const ATTACHMENT: &'static str = "Attachment";
    /// `Attachment` (field ID)
    pub const ATTACHMENT_ID: &'static str = "fldhF2AEuSC1haCZd";
    /// `Checkbox`
    pub const CHECKBOX: &'static str = "Checkbox";
    /// `Checkbox` (field ID)
    pub const CHECKBOX_ID: &'static str = "fldjQIaAZVegb1FUa";
    /// `Currency (float)`
    pub const CURRENCY_FLOAT: &'static str = "Currency (float)";
    /// `Currency (float)` (field ID)
    pub const CURRENCY_FLOAT_ID: &'static str = "fldyh8pzDXiy5abEr";
    /// `Currency (int)`
    pub const CURRENCY_INT: &'static str = "Currency (int)";
    /// `Currency (int)` (field ID)
    pub const CURRENCY_INT_ID: &'static str = "fldBfo74z9hD78hP8";
    /// `Date`
    pub const DATE: &'static str = "Date";
    /// `Date` (field ID)
    pub const DATE_ID: &'static str = "fldC6LfNVvVIxKyQH";
    /// `Date (with time)`
    pub const DATE_WITH_TIME: &'static str = "Date (with time)";
    /// `Date (with time)` (field ID)
    pub const DATE_WITH_TIME_ID: &'static str = "fldizYmjpXABGDLTG";
    /// `Duration`
    pub const DURATION: &'static str = "Duration";
    /// `Duration` (field ID)
    pub const DURATION_ID: &'static str = "fldLTyf6ljS0rhur8";
    /// `Email`
    pub const EMAIL: &'static str = "Email";
    /// `Email` (field ID)
    pub const EMAIL_ID: &'static str = "fldHCJoYBiFVsNvP4";
    /// `Link (multiple)`
    pub const LINK_MULTIPLE: &'static str = "Link (multiple)";
    /// `Link (multiple)` (field ID)
    pub const LINK_MULTIPLE_ID: &'static str = "fldFyFheQWczd8oux";
    /// `Link (single)`
    pub const LINK_SINGLE: &'static str = "Link (single)";
    /// `Link (single)` (field ID)
    pub const LINK_SINGLE_ID: &'static str = "fld7F5onkDo6mkmbN";
    /// `Long Text`
    pub const LONG_TEXT: &'static str = "Long Text";
    /// `Long Text` (field ID)
    pub const LONG_TEXT_ID: &'static str = "fld8ulc6J0W29M6La";
    /// `Long Text with Rich Text`
    pub const LONG_TEXT_WITH_RICH_TEXT: &'static str = "Long Text with Rich Text";
    /// `Long Text with Rich Text` (field ID)
    pub const LONG_TEXT_WITH_RICH_TEXT_ID: &'static str = "fldHJkxCMC0xo343u";
    /// `Multiple Select`
    pub const MULTIPLE_SELECT: &'static str = "Multiple Select";
    /// `Multiple Select` (field ID)
    pub const MULTIPLE_SELECT_ID: &'static str = "fld6GTabFmu1xKPvZ";
    /// `Number (float)`
    pub const NUMBER_FLOAT: &'static str = "Number (float)";
    /// `Number (float)` (field ID)
    pub const NUMBER_FLOAT_ID: &'static str = "fldmU0X2l4RWd21dd";
    /// `Number (int)`
    pub const NUMBER_INT: &'static str = "Number (int)";
    /// `Number (int)` (field ID)
    pub const NUMBER_INT_ID: &'static str = "fldOfPKGmnRPv94QH";
    /// `Percent (float)`
    pub const PERCENT_FLOAT: &'static str = "Percent (float)";
    /// `Percent (float)` (field ID)
    pub const PERCENT_FLOAT_ID: &'static str = "fldiGui9ll69N7WOj";
    /// `Percent (int)`
    pub const PERCENT_INT: &'static str = "Percent (int)";
    /// `Percent (int)` (field ID)
    pub const PERCENT_INT_ID: &'static str = "fldbAAyWboGulpb4s";
    /// `Phone Number`
    pub const PHONE_NUMBER: &'static str = "Phone Number";
    /// `Phone Number` (field ID)
    pub const PHONE_NUMBER_ID: &'static str = "fld38tnNpHmoks8C8";
    /// `Primary Key`
    pub const PRIMARY_KEY: &'static str = "Primary Key";
    /// `Primary Key` (field ID)
    pub const PRIMARY_KEY_ID: &'static str = "fldol5Q4wmQJQvPRy";
    /// `Rating`
    pub const RATING: &'static str = "Rating";
    /// `Rating` (field ID)
    pub const RATING_ID: &'static str = "fldRsmwFwQNZkKLp4";
    /// `Single Line Text`
    pub const SINGLE_LINE_TEXT: &'static str = "Single Line Text";
    /// `Single Line Text` (field ID)
    pub const SINGLE_LINE_TEXT_ID: &'static str = "fld0BL2lFo9fqcKv3";
    /// `Single Select`
    pub const SINGLE_SELECT: &'static str = "Single Select";
    /// `Single Select` (field ID)
    pub const SINGLE_SELECT_ID: &'static str = "fldn0GFFtMFpCXUNU";
    /// `URL`
    pub const URL: &'static str = "URL";
    /// `URL` (field ID)
    pub const URL_ID: &'static str = "fldLYloz2oP4ymf3B";
    /// `User`
    pub const USER: &'static str = "User";
    /// `User` (field ID)
    pub const USER_ID: &'static str = "fldU6SbLp8CSkLcA4";
    /// `User (allow multiple)`
    pub const USER_ALLOW_MULTIPLE: &'static str = "User (allow multiple)";
    /// `User (allow multiple)` (field ID)
    pub const USER_ALLOW_MULTIPLE_ID: &'static str = "fldBwCDbAVxRj9yg7";
}
