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
    /// `Duplicate (Name)`
    pub const DUPLICATE_NAME: &'static str = "Duplicate (Name)";
    /// `Duplicate (Name)` (field ID)
    pub const DUPLICATE_NAME_ID: &'static str = "fld99XdcwRa5WW6nw";
    /// `Duplicate Name`
    pub const DUPLICATE_NAME_V2: &'static str = "Duplicate Name";
    /// `Duplicate Name` (field ID)
    pub const DUPLICATE_NAME_V2_ID: &'static str = "fld95OnbzAUFUTTqZ";
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

    /// Look up a field ID by its Airtable field name.
    pub fn id_by_name(name: &str) -> Option<&'static str> {
        match name {
            "Attachment" => Some("fldhF2AEuSC1haCZd"),
            "Auto Number" => Some("fldizvTkxgIn0mC3L"),
            "Button" => Some("fldY48yKPG16AajtU"),
            "Checkbox" => Some("fldjQIaAZVegb1FUa"),
            "Created By" => Some("fldGLQhDz2UjjiHG6"),
            "Created Time" => Some("fld2YgW382Kt9xltA"),
            "Currency (float)" => Some("fldyh8pzDXiy5abEr"),
            "Currency (int)" => Some("fldBfo74z9hD78hP8"),
            "Date" => Some("fldC6LfNVvVIxKyQH"),
            "Date (with time)" => Some("fldizYmjpXABGDLTG"),
            "Duplicate (Name)" => Some("fld99XdcwRa5WW6nw"),
            "Duplicate Name" => Some("fld95OnbzAUFUTTqZ"),
            "Duration" => Some("fldLTyf6ljS0rhur8"),
            "Email" => Some("fldHCJoYBiFVsNvP4"),
            "Formula (Complex)" => Some("fld2vnFc0Bl5IOFUQ"),
            "Formula (ID)" => Some("fldcf62YFeIIDHElt"),
            "Formula (Nested)" => Some("fldXFeHRPBLz6AiWh"),
            "Formula (Simple)" => Some("fldy1axxaoUToLVC6"),
            "Last Modified By" => Some("fldF8iDttqP0AgzWC"),
            "Last Modified Time" => Some("fldMinKh4pa3YX86g"),
            "Link (multiple)" => Some("fldFyFheQWczd8oux"),
            "Link (single)" => Some("fld7F5onkDo6mkmbN"),
            "Long Text" => Some("fld8ulc6J0W29M6La"),
            "Long Text with Rich Text" => Some("fldHJkxCMC0xo343u"),
            "Lookup" => Some("fldbmFmrzYKBktJvE"),
            "Multiple Select" => Some("fld6GTabFmu1xKPvZ"),
            "Number (float)" => Some("fldmU0X2l4RWd21dd"),
            "Number (int)" => Some("fldOfPKGmnRPv94QH"),
            "Percent (float)" => Some("fldiGui9ll69N7WOj"),
            "Percent (int)" => Some("fldbAAyWboGulpb4s"),
            "Phone Number" => Some("fld38tnNpHmoks8C8"),
            "Primary Key" => Some("fldol5Q4wmQJQvPRy"),
            "Rating" => Some("fldRsmwFwQNZkKLp4"),
            "Rollup" => Some("fldGaFgBsDC3IBUdV"),
            "Single Line Text" => Some("fld0BL2lFo9fqcKv3"),
            "Single Select" => Some("fldn0GFFtMFpCXUNU"),
            "URL" => Some("fldLYloz2oP4ymf3B"),
            "User" => Some("fldU6SbLp8CSkLcA4"),
            "User (allow multiple)" => Some("fldBwCDbAVxRj9yg7"),
            _ => None,
        }
    }

    /// Look up an Airtable field name by its field ID.
    pub fn name_by_id(id: &str) -> Option<&'static str> {
        match id {
            "fldhF2AEuSC1haCZd" => Some("Attachment"),
            "fldizvTkxgIn0mC3L" => Some("Auto Number"),
            "fldY48yKPG16AajtU" => Some("Button"),
            "fldjQIaAZVegb1FUa" => Some("Checkbox"),
            "fldGLQhDz2UjjiHG6" => Some("Created By"),
            "fld2YgW382Kt9xltA" => Some("Created Time"),
            "fldyh8pzDXiy5abEr" => Some("Currency (float)"),
            "fldBfo74z9hD78hP8" => Some("Currency (int)"),
            "fldC6LfNVvVIxKyQH" => Some("Date"),
            "fldizYmjpXABGDLTG" => Some("Date (with time)"),
            "fld99XdcwRa5WW6nw" => Some("Duplicate (Name)"),
            "fld95OnbzAUFUTTqZ" => Some("Duplicate Name"),
            "fldLTyf6ljS0rhur8" => Some("Duration"),
            "fldHCJoYBiFVsNvP4" => Some("Email"),
            "fld2vnFc0Bl5IOFUQ" => Some("Formula (Complex)"),
            "fldcf62YFeIIDHElt" => Some("Formula (ID)"),
            "fldXFeHRPBLz6AiWh" => Some("Formula (Nested)"),
            "fldy1axxaoUToLVC6" => Some("Formula (Simple)"),
            "fldF8iDttqP0AgzWC" => Some("Last Modified By"),
            "fldMinKh4pa3YX86g" => Some("Last Modified Time"),
            "fldFyFheQWczd8oux" => Some("Link (multiple)"),
            "fld7F5onkDo6mkmbN" => Some("Link (single)"),
            "fld8ulc6J0W29M6La" => Some("Long Text"),
            "fldHJkxCMC0xo343u" => Some("Long Text with Rich Text"),
            "fldbmFmrzYKBktJvE" => Some("Lookup"),
            "fld6GTabFmu1xKPvZ" => Some("Multiple Select"),
            "fldmU0X2l4RWd21dd" => Some("Number (float)"),
            "fldOfPKGmnRPv94QH" => Some("Number (int)"),
            "fldiGui9ll69N7WOj" => Some("Percent (float)"),
            "fldbAAyWboGulpb4s" => Some("Percent (int)"),
            "fld38tnNpHmoks8C8" => Some("Phone Number"),
            "fldol5Q4wmQJQvPRy" => Some("Primary Key"),
            "fldRsmwFwQNZkKLp4" => Some("Rating"),
            "fldGaFgBsDC3IBUdV" => Some("Rollup"),
            "fld0BL2lFo9fqcKv3" => Some("Single Line Text"),
            "fldn0GFFtMFpCXUNU" => Some("Single Select"),
            "fldLYloz2oP4ymf3B" => Some("URL"),
            "fldU6SbLp8CSkLcA4" => Some("User"),
            "fldBwCDbAVxRj9yg7" => Some("User (allow multiple)"),
            _ => None,
        }
    }

    /// Look up a Rust property name by field ID.
    pub fn property_by_id(id: &str) -> Option<&'static str> {
        match id {
            "fldhF2AEuSC1haCZd" => Some("attachment"),
            "fldizvTkxgIn0mC3L" => Some("auto_number"),
            "fldY48yKPG16AajtU" => Some("button"),
            "fldjQIaAZVegb1FUa" => Some("checkbox"),
            "fldGLQhDz2UjjiHG6" => Some("created_by"),
            "fld2YgW382Kt9xltA" => Some("created_at_time"),
            "fldyh8pzDXiy5abEr" => Some("currency_float"),
            "fldBfo74z9hD78hP8" => Some("currency_int"),
            "fldC6LfNVvVIxKyQH" => Some("date"),
            "fldizYmjpXABGDLTG" => Some("date_with_time"),
            "fld99XdcwRa5WW6nw" => Some("duplicate_name"),
            "fld95OnbzAUFUTTqZ" => Some("duplicate_name_v2"),
            "fldLTyf6ljS0rhur8" => Some("duration"),
            "fldHCJoYBiFVsNvP4" => Some("email"),
            "fld2vnFc0Bl5IOFUQ" => Some("formula_complex"),
            "fldcf62YFeIIDHElt" => Some("formula_id"),
            "fldXFeHRPBLz6AiWh" => Some("formula_nested"),
            "fldy1axxaoUToLVC6" => Some("formula_simple"),
            "fldF8iDttqP0AgzWC" => Some("last_modified_by"),
            "fldMinKh4pa3YX86g" => Some("last_modified_time"),
            "fldFyFheQWczd8oux" => Some("link_multiple"),
            "fld7F5onkDo6mkmbN" => Some("link_single"),
            "fld8ulc6J0W29M6La" => Some("long_text"),
            "fldHJkxCMC0xo343u" => Some("long_text_with_rich_text"),
            "fldbmFmrzYKBktJvE" => Some("lookup"),
            "fld6GTabFmu1xKPvZ" => Some("multiple_select"),
            "fldmU0X2l4RWd21dd" => Some("number_float"),
            "fldOfPKGmnRPv94QH" => Some("number_int"),
            "fldiGui9ll69N7WOj" => Some("percent_float"),
            "fldbAAyWboGulpb4s" => Some("percent_int"),
            "fld38tnNpHmoks8C8" => Some("phone_number"),
            "fldol5Q4wmQJQvPRy" => Some("primary_key"),
            "fldRsmwFwQNZkKLp4" => Some("rating"),
            "fldGaFgBsDC3IBUdV" => Some("rollup"),
            "fld0BL2lFo9fqcKv3" => Some("single_line_text"),
            "fldn0GFFtMFpCXUNU" => Some("single_select"),
            "fldLYloz2oP4ymf3B" => Some("url"),
            "fldU6SbLp8CSkLcA4" => Some("user"),
            "fldBwCDbAVxRj9yg7" => Some("user_allow_multiple"),
            _ => None,
        }
    }

    /// Look up a field ID by Rust property name.
    pub fn id_by_property(property: &str) -> Option<&'static str> {
        match property {
            "attachment" => Some("fldhF2AEuSC1haCZd"),
            "auto_number" => Some("fldizvTkxgIn0mC3L"),
            "button" => Some("fldY48yKPG16AajtU"),
            "checkbox" => Some("fldjQIaAZVegb1FUa"),
            "created_by" => Some("fldGLQhDz2UjjiHG6"),
            "created_at_time" => Some("fld2YgW382Kt9xltA"),
            "currency_float" => Some("fldyh8pzDXiy5abEr"),
            "currency_int" => Some("fldBfo74z9hD78hP8"),
            "date" => Some("fldC6LfNVvVIxKyQH"),
            "date_with_time" => Some("fldizYmjpXABGDLTG"),
            "duplicate_name" => Some("fld99XdcwRa5WW6nw"),
            "duplicate_name_v2" => Some("fld95OnbzAUFUTTqZ"),
            "duration" => Some("fldLTyf6ljS0rhur8"),
            "email" => Some("fldHCJoYBiFVsNvP4"),
            "formula_complex" => Some("fld2vnFc0Bl5IOFUQ"),
            "formula_id" => Some("fldcf62YFeIIDHElt"),
            "formula_nested" => Some("fldXFeHRPBLz6AiWh"),
            "formula_simple" => Some("fldy1axxaoUToLVC6"),
            "last_modified_by" => Some("fldF8iDttqP0AgzWC"),
            "last_modified_time" => Some("fldMinKh4pa3YX86g"),
            "link_multiple" => Some("fldFyFheQWczd8oux"),
            "link_single" => Some("fld7F5onkDo6mkmbN"),
            "long_text" => Some("fld8ulc6J0W29M6La"),
            "long_text_with_rich_text" => Some("fldHJkxCMC0xo343u"),
            "lookup" => Some("fldbmFmrzYKBktJvE"),
            "multiple_select" => Some("fld6GTabFmu1xKPvZ"),
            "number_float" => Some("fldmU0X2l4RWd21dd"),
            "number_int" => Some("fldOfPKGmnRPv94QH"),
            "percent_float" => Some("fldiGui9ll69N7WOj"),
            "percent_int" => Some("fldbAAyWboGulpb4s"),
            "phone_number" => Some("fld38tnNpHmoks8C8"),
            "primary_key" => Some("fldol5Q4wmQJQvPRy"),
            "rating" => Some("fldRsmwFwQNZkKLp4"),
            "rollup" => Some("fldGaFgBsDC3IBUdV"),
            "single_line_text" => Some("fld0BL2lFo9fqcKv3"),
            "single_select" => Some("fldn0GFFtMFpCXUNU"),
            "url" => Some("fldLYloz2oP4ymf3B"),
            "user" => Some("fldU6SbLp8CSkLcA4"),
            "user_allow_multiple" => Some("fldBwCDbAVxRj9yg7"),
            _ => None,
        }
    }
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
    /// `Duplicate (Name)`
    pub const DUPLICATE_NAME: &'static str = "Duplicate (Name)";
    /// `Duplicate (Name)` (field ID)
    pub const DUPLICATE_NAME_ID: &'static str = "fld99XdcwRa5WW6nw";
    /// `Duplicate Name`
    pub const DUPLICATE_NAME_V2: &'static str = "Duplicate Name";
    /// `Duplicate Name` (field ID)
    pub const DUPLICATE_NAME_V2_ID: &'static str = "fld95OnbzAUFUTTqZ";
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
