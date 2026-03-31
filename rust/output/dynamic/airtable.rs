// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use crate::client::AirtableClient;
use crate::tables::FormulasTable;
use crate::tables::PrimaryTable;
use crate::tables::SecondaryTable;
use crate::tables::TertiaryTable;

/// Main entry point for the Airtable base.
pub struct Airtable {
    client: AirtableClient,
}

impl Airtable {
    /// Create a new Airtable instance.
    pub fn new(api_key: &str, base_id: &str) -> Self {
        Self {
            client: AirtableClient::new(api_key, base_id),
        }
    }

    /// Access the `Formulas` table.
    pub fn formulas(&self) -> FormulasTable<'_> {
        FormulasTable::new(&self.client)
    }

    /// Access the `Primary` table.
    pub fn primary(&self) -> PrimaryTable<'_> {
        PrimaryTable::new(&self.client)
    }

    /// Access the `Secondary` table.
    pub fn secondary(&self) -> SecondaryTable<'_> {
        SecondaryTable::new(&self.client)
    }

    /// Access the `Tertiary` table.
    pub fn tertiary(&self) -> TertiaryTable<'_> {
        TertiaryTable::new(&self.client)
    }
}
