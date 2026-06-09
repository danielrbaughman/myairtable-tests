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

impl PrimaryMultipleSelectOption {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Option1 => "selKcXTyNnNHd3nSK",
            Self::Option2 => "selBUFMeKPuDUvn0a",
            Self::Option3 => "selmD0luyDWhS6AyO",
            Self::Unknown => "",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "selKcXTyNnNHd3nSK" => Some(Self::Option1),
            "selBUFMeKPuDUvn0a" => Some(Self::Option2),
            "selmD0luyDWhS6AyO" => Some(Self::Option3),
            _ => None,
        }
    }
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

impl PrimarySingleSelectOption {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Choice1 => "sellmCNG85TCgc7Pu",
            Self::Choice2 => "sel4oPckj6NHWKn2u",
            Self::Choice3 => "selYfbfAsr7X75S2V",
            Self::Unknown => "",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "sellmCNG85TCgc7Pu" => Some(Self::Choice1),
            "sel4oPckj6NHWKn2u" => Some(Self::Choice2),
            "selYfbfAsr7X75S2V" => Some(Self::Choice3),
            _ => None,
        }
    }
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
