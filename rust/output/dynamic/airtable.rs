// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use std::sync::Arc;

use crate::client::AirtableClient;
use crate::models::{CreateFormulasModel, FormulasModel};
use crate::models::{CreatePrimaryModel, PrimaryModel};
use crate::models::{CreateSecondaryModel, SecondaryModel};
use crate::models::{CreateTertiaryModel, TertiaryModel};
use crate::orm_table::OrmTable;
use crate::table::StructTable;

/// Main entry point for the Airtable base.
pub struct Airtable {
    /// `Formulas` (dict)
    pub formulas: StructTable,
    /// `Formulas` (ORM)
    pub formulas_orm: OrmTable<FormulasModel, CreateFormulasModel>,
    /// `Primary` (dict)
    pub primary: StructTable,
    /// `Primary` (ORM)
    pub primary_orm: OrmTable<PrimaryModel, CreatePrimaryModel>,
    /// `Secondary` (dict)
    pub secondary: StructTable,
    /// `Secondary` (ORM)
    pub secondary_orm: OrmTable<SecondaryModel, CreateSecondaryModel>,
    /// `Tertiary` (dict)
    pub tertiary: StructTable,
    /// `Tertiary` (ORM)
    pub tertiary_orm: OrmTable<TertiaryModel, CreateTertiaryModel>,
}

impl Airtable {
    /// Create a new Airtable instance.
    pub fn new(api_key: &str, base_id: &str) -> Self {
        let client = Arc::new(AirtableClient::new(api_key, base_id));
        Self {
            formulas: StructTable::new(Arc::clone(&client), "tblnuYBsMdXNDsuRc", "Formulas"),
            formulas_orm: OrmTable::new(Arc::clone(&client), "tblnuYBsMdXNDsuRc", "Formulas"),
            primary: StructTable::new(Arc::clone(&client), "tblmb3iqgpNS1ysV2", "Primary"),
            primary_orm: OrmTable::new(Arc::clone(&client), "tblmb3iqgpNS1ysV2", "Primary"),
            secondary: StructTable::new(Arc::clone(&client), "tblPPScS3XMuFkDYN", "Secondary"),
            secondary_orm: OrmTable::new(Arc::clone(&client), "tblPPScS3XMuFkDYN", "Secondary"),
            tertiary: StructTable::new(Arc::clone(&client), "tblLFoLxEdWlxjmLP", "Tertiary"),
            tertiary_orm: OrmTable::new(Arc::clone(&client), "tblLFoLxEdWlxjmLP", "Tertiary"),
        }
    }
}
