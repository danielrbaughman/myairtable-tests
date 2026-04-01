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
}

/// Views for `Tertiary`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TertiaryView {
    /// `Grid view` (grid)
    #[serde(rename = "viwdp3tOB8ooOCvP4")]
    GridView,
}

impl From<TertiaryView> for String {
    fn from(v: TertiaryView) -> String {
        match v {
            TertiaryView::GridView => "viwdp3tOB8ooOCvP4".to_string(),
        }
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
