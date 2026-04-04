// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use serde::{Deserialize, Serialize};

/// Field constants for `Formulas`
pub struct FormulasFields;

impl FormulasFields {
    /// `Date Formula`
    pub const DATE_FORMULA: &'static str = "Date Formula";
    /// `Date Formula` (field ID)
    pub const DATE_FORMULA_ID: &'static str = "fldY7kjaklLeoSgGd";
    /// `First Date`
    pub const FIRST_DATE: &'static str = "First Date";
    /// `First Date` (field ID)
    pub const FIRST_DATE_ID: &'static str = "fldlZT521Iy0FFXFL";
    /// `First Number`
    pub const FIRST_NUMBER: &'static str = "First Number";
    /// `First Number` (field ID)
    pub const FIRST_NUMBER_ID: &'static str = "fldA04pqfjMkGXcZU";
    /// `First Text`
    pub const FIRST_TEXT: &'static str = "First Text";
    /// `First Text` (field ID)
    pub const FIRST_TEXT_ID: &'static str = "fldHJuw6pAujnHvkP";
    /// `Math Formula`
    pub const MATH_FORMULA: &'static str = "Math Formula";
    /// `Math Formula` (field ID)
    pub const MATH_FORMULA_ID: &'static str = "fldlSuvoeWGokSz8Z";
    /// `Primary Key`
    pub const PRIMARY_KEY: &'static str = "Primary Key";
    /// `Primary Key` (field ID)
    pub const PRIMARY_KEY_ID: &'static str = "fldLZFrZKvSCS4dKb";
    /// `Second Date`
    pub const SECOND_DATE: &'static str = "Second Date";
    /// `Second Date` (field ID)
    pub const SECOND_DATE_ID: &'static str = "fld1LZ3Ebpt0LaMmu";
    /// `Second Number`
    pub const SECOND_NUMBER: &'static str = "Second Number";
    /// `Second Number` (field ID)
    pub const SECOND_NUMBER_ID: &'static str = "fldj5nAkal5y8OOZg";
    /// `Second Text`
    pub const SECOND_TEXT: &'static str = "Second Text";
    /// `Second Text` (field ID)
    pub const SECOND_TEXT_ID: &'static str = "fldA2boNwwsiuvXw1";
    /// `Text Formula`
    pub const TEXT_FORMULA: &'static str = "Text Formula";
    /// `Text Formula` (field ID)
    pub const TEXT_FORMULA_ID: &'static str = "flddvzeqt7FJpQ9NX";
    /// `Third Date`
    pub const THIRD_DATE: &'static str = "Third Date";
    /// `Third Date` (field ID)
    pub const THIRD_DATE_ID: &'static str = "fldxSQRRn8W879aiU";
    /// `Third Number`
    pub const THIRD_NUMBER: &'static str = "Third Number";
    /// `Third Number` (field ID)
    pub const THIRD_NUMBER_ID: &'static str = "fld5NBdekrAUzu4Fi";
    /// `Third Text`
    pub const THIRD_TEXT: &'static str = "Third Text";
    /// `Third Text` (field ID)
    pub const THIRD_TEXT_ID: &'static str = "fldfruPf8V9K6qIAN";

    /// Look up a field ID by its Airtable field name.
    pub fn id_by_name(name: &str) -> Option<&'static str> {
        match name {
            "Date Formula" => Some("fldY7kjaklLeoSgGd"),
            "First Date" => Some("fldlZT521Iy0FFXFL"),
            "First Number" => Some("fldA04pqfjMkGXcZU"),
            "First Text" => Some("fldHJuw6pAujnHvkP"),
            "Math Formula" => Some("fldlSuvoeWGokSz8Z"),
            "Primary Key" => Some("fldLZFrZKvSCS4dKb"),
            "Second Date" => Some("fld1LZ3Ebpt0LaMmu"),
            "Second Number" => Some("fldj5nAkal5y8OOZg"),
            "Second Text" => Some("fldA2boNwwsiuvXw1"),
            "Text Formula" => Some("flddvzeqt7FJpQ9NX"),
            "Third Date" => Some("fldxSQRRn8W879aiU"),
            "Third Number" => Some("fld5NBdekrAUzu4Fi"),
            "Third Text" => Some("fldfruPf8V9K6qIAN"),
            _ => None,
        }
    }

    /// Look up an Airtable field name by its field ID.
    pub fn name_by_id(id: &str) -> Option<&'static str> {
        match id {
            "fldY7kjaklLeoSgGd" => Some("Date Formula"),
            "fldlZT521Iy0FFXFL" => Some("First Date"),
            "fldA04pqfjMkGXcZU" => Some("First Number"),
            "fldHJuw6pAujnHvkP" => Some("First Text"),
            "fldlSuvoeWGokSz8Z" => Some("Math Formula"),
            "fldLZFrZKvSCS4dKb" => Some("Primary Key"),
            "fld1LZ3Ebpt0LaMmu" => Some("Second Date"),
            "fldj5nAkal5y8OOZg" => Some("Second Number"),
            "fldA2boNwwsiuvXw1" => Some("Second Text"),
            "flddvzeqt7FJpQ9NX" => Some("Text Formula"),
            "fldxSQRRn8W879aiU" => Some("Third Date"),
            "fld5NBdekrAUzu4Fi" => Some("Third Number"),
            "fldfruPf8V9K6qIAN" => Some("Third Text"),
            _ => None,
        }
    }

    /// Look up a Rust property name by field ID.
    pub fn property_by_id(id: &str) -> Option<&'static str> {
        match id {
            "fldY7kjaklLeoSgGd" => Some("date_formula"),
            "fldlZT521Iy0FFXFL" => Some("first_date"),
            "fldA04pqfjMkGXcZU" => Some("first_number"),
            "fldHJuw6pAujnHvkP" => Some("first_text"),
            "fldlSuvoeWGokSz8Z" => Some("math_formula"),
            "fldLZFrZKvSCS4dKb" => Some("primary_key"),
            "fld1LZ3Ebpt0LaMmu" => Some("second_date"),
            "fldj5nAkal5y8OOZg" => Some("second_number"),
            "fldA2boNwwsiuvXw1" => Some("second_text"),
            "flddvzeqt7FJpQ9NX" => Some("text_formula"),
            "fldxSQRRn8W879aiU" => Some("third_date"),
            "fld5NBdekrAUzu4Fi" => Some("third_number"),
            "fldfruPf8V9K6qIAN" => Some("third_text"),
            _ => None,
        }
    }

    /// Look up a field ID by Rust property name.
    pub fn id_by_property(property: &str) -> Option<&'static str> {
        match property {
            "date_formula" => Some("fldY7kjaklLeoSgGd"),
            "first_date" => Some("fldlZT521Iy0FFXFL"),
            "first_number" => Some("fldA04pqfjMkGXcZU"),
            "first_text" => Some("fldHJuw6pAujnHvkP"),
            "math_formula" => Some("fldlSuvoeWGokSz8Z"),
            "primary_key" => Some("fldLZFrZKvSCS4dKb"),
            "second_date" => Some("fld1LZ3Ebpt0LaMmu"),
            "second_number" => Some("fldj5nAkal5y8OOZg"),
            "second_text" => Some("fldA2boNwwsiuvXw1"),
            "text_formula" => Some("flddvzeqt7FJpQ9NX"),
            "third_date" => Some("fldxSQRRn8W879aiU"),
            "third_number" => Some("fld5NBdekrAUzu4Fi"),
            "third_text" => Some("fldfruPf8V9K6qIAN"),
            _ => None,
        }
    }
}

/// Views for `Formulas`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormulasView {
    /// `Grid view` (grid)
    #[serde(rename = "viw7gdr4uJSpnHjR7")]
    GridView,
}

impl AsRef<str> for FormulasView {
    fn as_ref(&self) -> &str {
        match self {
            Self::GridView => "viw7gdr4uJSpnHjR7",
        }
    }
}

impl From<FormulasView> for String {
    fn from(v: FormulasView) -> String {
        v.as_ref().to_string()
    }
}

/// Writable field constants for `Formulas`
pub struct CreateFormulasFields;

impl CreateFormulasFields {
    /// `First Date`
    pub const FIRST_DATE: &'static str = "First Date";
    /// `First Date` (field ID)
    pub const FIRST_DATE_ID: &'static str = "fldlZT521Iy0FFXFL";
    /// `First Number`
    pub const FIRST_NUMBER: &'static str = "First Number";
    /// `First Number` (field ID)
    pub const FIRST_NUMBER_ID: &'static str = "fldA04pqfjMkGXcZU";
    /// `First Text`
    pub const FIRST_TEXT: &'static str = "First Text";
    /// `First Text` (field ID)
    pub const FIRST_TEXT_ID: &'static str = "fldHJuw6pAujnHvkP";
    /// `Primary Key`
    pub const PRIMARY_KEY: &'static str = "Primary Key";
    /// `Primary Key` (field ID)
    pub const PRIMARY_KEY_ID: &'static str = "fldLZFrZKvSCS4dKb";
    /// `Second Date`
    pub const SECOND_DATE: &'static str = "Second Date";
    /// `Second Date` (field ID)
    pub const SECOND_DATE_ID: &'static str = "fld1LZ3Ebpt0LaMmu";
    /// `Second Number`
    pub const SECOND_NUMBER: &'static str = "Second Number";
    /// `Second Number` (field ID)
    pub const SECOND_NUMBER_ID: &'static str = "fldj5nAkal5y8OOZg";
    /// `Second Text`
    pub const SECOND_TEXT: &'static str = "Second Text";
    /// `Second Text` (field ID)
    pub const SECOND_TEXT_ID: &'static str = "fldA2boNwwsiuvXw1";
    /// `Third Date`
    pub const THIRD_DATE: &'static str = "Third Date";
    /// `Third Date` (field ID)
    pub const THIRD_DATE_ID: &'static str = "fldxSQRRn8W879aiU";
    /// `Third Number`
    pub const THIRD_NUMBER: &'static str = "Third Number";
    /// `Third Number` (field ID)
    pub const THIRD_NUMBER_ID: &'static str = "fld5NBdekrAUzu4Fi";
    /// `Third Text`
    pub const THIRD_TEXT: &'static str = "Third Text";
    /// `Third Text` (field ID)
    pub const THIRD_TEXT_ID: &'static str = "fldfruPf8V9K6qIAN";
}
