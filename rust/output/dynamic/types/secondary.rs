// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use serde::{Deserialize, Serialize};

/// Field constants for `Secondary`
pub struct SecondaryFields;

impl SecondaryFields {
    /// `Link to Tertiary`
    pub const LINK_TO_TERTIARY: &'static str = "Link to Tertiary";
    /// `Link to Tertiary` (field ID)
    pub const LINK_TO_TERTIARY_ID: &'static str = "fldKR6tdbnOBRCtdQ";
    /// `Name`
    pub const NAME: &'static str = "Name";
    /// `Name` (field ID)
    pub const NAME_ID: &'static str = "fld1RagdJ09mpWhzM";
    /// `Primary`
    pub const PRIMARY: &'static str = "Primary";
    /// `Primary` (field ID)
    pub const PRIMARY_ID: &'static str = "fldl0nB9WRFSdqlii";
    /// `Primary 2`
    pub const PRIMARY_2: &'static str = "Primary 2";
    /// `Primary 2` (field ID)
    pub const PRIMARY_2_ID: &'static str = "fldgoE2oZmXmKkQca";
    /// `Value`
    pub const VALUE: &'static str = "Value";
    /// `Value` (field ID)
    pub const VALUE_ID: &'static str = "fldi6Mxh5H1gPGxFX";
}

/// Views for `Secondary`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecondaryView {
    /// `Grid view` (grid)
    #[serde(rename = "viwTml4ZHkNi8kJbD")]
    GridView,
}

impl AsRef<str> for SecondaryView {
    fn as_ref(&self) -> &str {
        match self {
            Self::GridView => "viwTml4ZHkNi8kJbD",
        }
    }
}

impl From<SecondaryView> for String {
    fn from(v: SecondaryView) -> String {
        v.as_ref().to_string()
    }
}

/// Writable field constants for `Secondary`
pub struct CreateSecondaryFields;

impl CreateSecondaryFields {
    /// `Link to Tertiary`
    pub const LINK_TO_TERTIARY: &'static str = "Link to Tertiary";
    /// `Link to Tertiary` (field ID)
    pub const LINK_TO_TERTIARY_ID: &'static str = "fldKR6tdbnOBRCtdQ";
    /// `Name`
    pub const NAME: &'static str = "Name";
    /// `Name` (field ID)
    pub const NAME_ID: &'static str = "fld1RagdJ09mpWhzM";
    /// `Primary`
    pub const PRIMARY: &'static str = "Primary";
    /// `Primary` (field ID)
    pub const PRIMARY_ID: &'static str = "fldl0nB9WRFSdqlii";
    /// `Primary 2`
    pub const PRIMARY_2: &'static str = "Primary 2";
    /// `Primary 2` (field ID)
    pub const PRIMARY_2_ID: &'static str = "fldgoE2oZmXmKkQca";
    /// `Value`
    pub const VALUE: &'static str = "Value";
    /// `Value` (field ID)
    pub const VALUE_ID: &'static str = "fldi6Mxh5H1gPGxFX";
}
