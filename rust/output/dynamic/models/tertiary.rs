// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use crate::types::{OrmModel, RecordId};
use serde::{Deserialize, Serialize};

/// ORM model for `Tertiary`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TertiaryModel {
    #[serde(default)]
    #[serde(skip_serializing)]
    pub id: Option<RecordId>,
    #[serde(default)]
    #[serde(skip_serializing)]
    pub created_time: Option<String>,
    /// Name `fldwzqKxsRnPZJ2Ll` - `Primary Key`
    #[serde(rename = "fldwzqKxsRnPZJ2Ll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Secondary `fld8lCuUXpEXkIeYv`
    #[serde(rename = "fld8lCuUXpEXkIeYv")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<Vec<RecordId>>,
    /// Value `fldjNLBh2UccM64h5`
    #[serde(rename = "fldjNLBh2UccM64h5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl OrmModel for TertiaryModel {
    fn set_record_meta(&mut self, id: RecordId, created_time: Option<String>) {
        self.id = Some(id);
        self.created_time = created_time;
    }
}

/// Writable fields for creating/updating `Tertiary` records.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTertiaryModel {
    #[serde(rename = "fldwzqKxsRnPZJ2Ll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "fld8lCuUXpEXkIeYv")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<Vec<RecordId>>,
    #[serde(rename = "fldjNLBh2UccM64h5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
