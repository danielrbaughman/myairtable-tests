// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

#[path = "../static/client.rs"]
pub mod client;
#[path = "../static/error.rs"]
pub mod error;
#[path = "../static/pagination.rs"]
pub mod pagination;
/// Auto-generated Airtable SDK.

#[path = "../static/types.rs"]
pub mod types;

pub mod airtable;
pub mod models;
pub mod options;
pub mod tables;

pub use airtable::Airtable;
pub use client::AirtableClient;
pub use error::AirtableError;
pub use models::{CreateFormulas, Formulas, UpdateFormulas};
pub use models::{CreatePrimary, Primary, UpdatePrimary};
pub use models::{CreateSecondary, Secondary, UpdateSecondary};
pub use models::{CreateTertiary, Tertiary, UpdateTertiary};
pub use options::primary::*;
pub use pagination::PaginatedResponse;
pub use types::*;
