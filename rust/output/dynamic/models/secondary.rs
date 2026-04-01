// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use crate::types::{OrmModel, RecordId};
use serde::{Deserialize, Serialize};

/// ORM model for `Secondary`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecondaryModel {
    #[serde(default)]
    #[serde(skip_serializing)]
    pub id: Option<RecordId>,
    #[serde(default)]
    #[serde(skip_serializing)]
    pub created_time: Option<String>,
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

impl OrmModel for SecondaryModel {
    fn set_record_meta(&mut self, id: RecordId, created_time: Option<String>) {
        self.id = Some(id);
        self.created_time = created_time;
    }
}

/// Writable fields for creating/updating `Secondary` records.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSecondaryModel {
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
