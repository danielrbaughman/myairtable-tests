// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

#![allow(unused_imports)]

#[path = "../static/airtable_model.rs"]
pub mod airtable_model;
#[path = "../static/airtable_runtime.rs"]
pub mod airtable_runtime;
#[path = "../static/client.rs"]
pub mod client;
#[path = "../static/error.rs"]
pub mod error;
#[path = "../static/formula.rs"]
pub mod formula;
#[path = "../static/orm_table.rs"]
pub mod orm_table;
#[path = "../static/pagination.rs"]
pub mod pagination;
#[path = "../static/struct_table.rs"]
pub mod table;
/// Auto-generated Airtable SDK.

#[path = "../static/types.rs"]
pub mod types;

pub mod airtable;
#[path = "types/mod.rs"]
pub mod field_types;
pub mod formulas;
pub mod models;
pub mod options;

pub use airtable::Airtable;
pub use airtable::FormulasTable;
pub use airtable::PrimaryTable;
pub use airtable::SecondaryTable;
pub use airtable::TertiaryTable;
pub use airtable_model::{ModelMeta, OrmModel};
pub use client::AirtableClient;
pub use error::AirtableError;
pub use field_types::{CreateFormulasFields, FormulasFields, FormulasView};
pub use field_types::{CreatePrimaryFields, PrimaryFields, PrimaryView};
pub use field_types::{CreateSecondaryFields, SecondaryFields, SecondaryView};
pub use field_types::{CreateTertiaryFields, TertiaryFields, TertiaryView};
pub use formula::{FormulaField, FormulaTextOps};
pub use formulas::FormulasFormulas;
pub use formulas::PrimaryFormulas;
pub use formulas::SecondaryFormulas;
pub use formulas::TertiaryFormulas;
pub use models::FormulasModel;
pub use models::PrimaryModel;
pub use models::SecondaryModel;
pub use models::TertiaryModel;
pub use options::primary::*;
pub use orm_table::OrmTable;
pub use pagination::PaginatedResponse;
pub use table::StructTable;
pub use types::*;
