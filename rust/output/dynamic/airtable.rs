// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use std::sync::Arc;

use crate::client::AirtableClient;
use crate::table::StructTable;

/// Main entry point for the Airtable base.
pub struct Airtable {
    /// `Formulas`
    pub formulas: StructTable,
    /// `Primary`
    pub primary: StructTable,
    /// `Secondary`
    pub secondary: StructTable,
    /// `Tertiary`
    pub tertiary: StructTable,
}

impl Airtable {
    /// Create a new Airtable instance.
    pub fn new(api_key: &str, base_id: &str) -> Self {
        let client = Arc::new(AirtableClient::new(api_key, base_id));
        Self {
            formulas: StructTable::new(Arc::clone(&client), "tblnuYBsMdXNDsuRc", "Formulas"),
            primary: StructTable::new(Arc::clone(&client), "tblmb3iqgpNS1ysV2", "Primary"),
            secondary: StructTable::new(Arc::clone(&client), "tblPPScS3XMuFkDYN", "Secondary"),
            tertiary: StructTable::new(Arc::clone(&client), "tblLFoLxEdWlxjmLP", "Tertiary"),
        }
    }
}
