// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use serde::{Deserialize, Serialize};

/// Options for `Multiple Select`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimaryMultipleSelectOption {
    #[serde(rename = "Option 1")]
    Option1,
    #[serde(rename = "Option 2")]
    Option2,
    #[serde(rename = "Option 3")]
    Option3,
    #[serde(other)]
    Unknown,
}

/// Options for `Single Select`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimarySingleSelectOption {
    #[serde(rename = "Choice 1")]
    Choice1,
    #[serde(rename = "Choice 2")]
    Choice2,
    #[serde(rename = "Choice 3")]
    Choice3,
    #[serde(other)]
    Unknown,
}
