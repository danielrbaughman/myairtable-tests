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

/// Select field options for `Primary`
pub struct PrimaryOptions {
    /// Valid options for `Multiple Select`
    pub multiple_select: &'static [PrimaryMultipleSelectOption],
    /// Valid options for `Single Select`
    pub single_select: &'static [PrimarySingleSelectOption],
}

impl PrimaryOptions {
    pub const fn new() -> Self {
        Self {
            multiple_select: &[
                PrimaryMultipleSelectOption::Option1,
                PrimaryMultipleSelectOption::Option2,
                PrimaryMultipleSelectOption::Option3,
            ],
            single_select: &[
                PrimarySingleSelectOption::Choice1,
                PrimarySingleSelectOption::Choice2,
                PrimarySingleSelectOption::Choice3,
            ],
        }
    }
}
