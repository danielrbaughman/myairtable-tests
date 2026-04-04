// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use serde::{Deserialize, Serialize};

/// Field constants for `Tertiary`
pub struct TertiaryFields;

impl TertiaryFields {
    /// `Name`
    pub const NAME: &'static str = "Name";
    /// `Name` (field ID)
    pub const NAME_ID: &'static str = "fldwzqKxsRnPZJ2Ll";
    /// `Secondary`
    pub const SECONDARY: &'static str = "Secondary";
    /// `Secondary` (field ID)
    pub const SECONDARY_ID: &'static str = "fld8lCuUXpEXkIeYv";
    /// `Value`
    pub const VALUE: &'static str = "Value";
    /// `Value` (field ID)
    pub const VALUE_ID: &'static str = "fldjNLBh2UccM64h5";

    /// Look up a field ID by its Airtable field name.
    pub fn id_by_name(name: &str) -> Option<&'static str> {
        match name {
            "Name" => Some("fldwzqKxsRnPZJ2Ll"),
            "Secondary" => Some("fld8lCuUXpEXkIeYv"),
            "Value" => Some("fldjNLBh2UccM64h5"),
            _ => None,
        }
    }

    /// Look up an Airtable field name by its field ID.
    pub fn name_by_id(id: &str) -> Option<&'static str> {
        match id {
            "fldwzqKxsRnPZJ2Ll" => Some("Name"),
            "fld8lCuUXpEXkIeYv" => Some("Secondary"),
            "fldjNLBh2UccM64h5" => Some("Value"),
            _ => None,
        }
    }

    /// Look up a Rust property name by field ID.
    pub fn property_by_id(id: &str) -> Option<&'static str> {
        match id {
            "fldwzqKxsRnPZJ2Ll" => Some("name"),
            "fld8lCuUXpEXkIeYv" => Some("secondary"),
            "fldjNLBh2UccM64h5" => Some("value"),
            _ => None,
        }
    }

    /// Look up a field ID by Rust property name.
    pub fn id_by_property(property: &str) -> Option<&'static str> {
        match property {
            "name" => Some("fldwzqKxsRnPZJ2Ll"),
            "secondary" => Some("fld8lCuUXpEXkIeYv"),
            "value" => Some("fldjNLBh2UccM64h5"),
            _ => None,
        }
    }
}

/// Views for `Tertiary`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TertiaryView {
    /// `Grid view` (grid)
    #[serde(rename = "viwdp3tOB8ooOCvP4")]
    GridView,
}

impl AsRef<str> for TertiaryView {
    fn as_ref(&self) -> &str {
        match self {
            Self::GridView => "viwdp3tOB8ooOCvP4",
        }
    }
}

impl From<TertiaryView> for String {
    fn from(v: TertiaryView) -> String {
        v.as_ref().to_string()
    }
}

/// Writable field constants for `Tertiary`
pub struct CreateTertiaryFields;

impl CreateTertiaryFields {
    /// `Name`
    pub const NAME: &'static str = "Name";
    /// `Name` (field ID)
    pub const NAME_ID: &'static str = "fldwzqKxsRnPZJ2Ll";
    /// `Secondary`
    pub const SECONDARY: &'static str = "Secondary";
    /// `Secondary` (field ID)
    pub const SECONDARY_ID: &'static str = "fld8lCuUXpEXkIeYv";
    /// `Value`
    pub const VALUE: &'static str = "Value";
    /// `Value` (field ID)
    pub const VALUE_ID: &'static str = "fldjNLBh2UccM64h5";
}
