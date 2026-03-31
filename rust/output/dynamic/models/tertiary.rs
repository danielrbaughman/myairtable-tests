// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use crate::types::RecordId;
use serde::{Deserialize, Serialize};

/// Record fields for `Tertiary`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tertiary {
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

/// Writable fields for creating/updating `Tertiary` records.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTertiary {
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

/// Alias for `CreateTertiary`.
pub type UpdateTertiary = CreateTertiary;
