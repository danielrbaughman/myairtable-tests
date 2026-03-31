// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use crate::types::RecordId;
use serde::{Deserialize, Serialize};

/// Record fields for `Secondary`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Secondary {
    /// Link to Tertiary `fldKR6tdbnOBRCtdQ`
    #[serde(rename = "fldKR6tdbnOBRCtdQ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_to_tertiary: Option<Vec<RecordId>>,
    /// Name `fld1RagdJ09mpWhzM` - `Primary Key`
    #[serde(rename = "fld1RagdJ09mpWhzM")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Primary `fldl0nB9WRFSdqlii`
    #[serde(rename = "fldl0nB9WRFSdqlii")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<Vec<RecordId>>,
    /// Primary 2 `fldgoE2oZmXmKkQca`
    #[serde(rename = "fldgoE2oZmXmKkQca")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_2: Option<Vec<RecordId>>,
    /// Value `fldi6Mxh5H1gPGxFX`
    #[serde(rename = "fldi6Mxh5H1gPGxFX")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Writable fields for creating/updating `Secondary` records.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSecondary {
    #[serde(rename = "fldKR6tdbnOBRCtdQ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_to_tertiary: Option<Vec<RecordId>>,
    #[serde(rename = "fld1RagdJ09mpWhzM")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "fldl0nB9WRFSdqlii")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<Vec<RecordId>>,
    #[serde(rename = "fldgoE2oZmXmKkQca")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_2: Option<Vec<RecordId>>,
    #[serde(rename = "fldi6Mxh5H1gPGxFX")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Alias for `CreateSecondary`.
pub type UpdateSecondary = CreateSecondary;
