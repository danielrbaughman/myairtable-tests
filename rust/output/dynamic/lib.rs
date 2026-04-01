// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

#[path = "../static/airtable_model.rs"]
pub mod airtable_model;
#[path = "../static/client.rs"]
pub mod client;
#[path = "../static/error.rs"]
pub mod error;
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
pub mod models;
pub mod options;

pub use airtable::Airtable;
pub use airtable_model::{ModelMeta, OrmModel};
pub use client::AirtableClient;
pub use error::AirtableError;
pub use field_types::{CreateFormulasFields, FormulasFields, FormulasView};
pub use field_types::{CreatePrimaryFields, PrimaryFields, PrimaryView};
pub use field_types::{CreateSecondaryFields, SecondaryFields, SecondaryView};
pub use field_types::{CreateTertiaryFields, TertiaryFields, TertiaryView};
pub use models::{CreateFormulasModel, FormulasModel};
pub use models::{CreatePrimaryModel, PrimaryModel};
pub use models::{CreateSecondaryModel, SecondaryModel};
pub use models::{CreateTertiaryModel, TertiaryModel};
pub use options::primary::*;
pub use orm_table::OrmTable;
pub use pagination::PaginatedResponse;
pub use table::StructTable;
pub use types::*;
