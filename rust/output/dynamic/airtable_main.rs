// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use std::sync::Arc;

use crate::client::AirtableClient;
use crate::error::AirtableError;
use crate::tables::FormulasTable;
use crate::tables::PrimaryTable;
use crate::tables::SecondaryTable;
use crate::tables::TertiaryTable;
use crate::types::build_url;

/// Main entry point for the Airtable base.
///
/// # Example
///
/// ```ignore
/// let airtable = Airtable::new("api_key", "base_id");
/// let records = airtable.formulas.get_many(&AirtableQuery::new()).await?;
/// ```
pub struct Airtable {
    client: Arc<AirtableClient>,
    /// `Formulas`
    pub formulas: FormulasTable,
    /// `Primary`
    pub primary: PrimaryTable,
    /// `Secondary`
    pub secondary: SecondaryTable,
    /// `Tertiary`
    pub tertiary: TertiaryTable,
}

impl Airtable {
    /// Create a new Airtable instance.
    pub fn new(api_key: &str, base_id: &str) -> Self {
        Self::with_cache(api_key, base_id, 0)
    }

    /// Create a new Airtable instance with response caching.
    pub fn with_cache(api_key: &str, base_id: &str, cache_seconds: u64) -> Self {
        let client = Arc::new(AirtableClient::new(api_key, base_id));
        let mut instance = Self {
            client: Arc::clone(&client),
            formulas: FormulasTable::new(Arc::clone(&client)),
            primary: PrimaryTable::new(Arc::clone(&client)),
            secondary: SecondaryTable::new(Arc::clone(&client)),
            tertiary: TertiaryTable::new(Arc::clone(&client)),
        };
        if cache_seconds > 0 {
            instance.formulas.set_cache_seconds(cache_seconds);
            instance.primary.set_cache_seconds(cache_seconds);
            instance.secondary.set_cache_seconds(cache_seconds);
            instance.tertiary.set_cache_seconds(cache_seconds);
        }
        instance
    }

    /// Get the Airtable web URL for this base.
    pub fn url(&self) -> String {
        build_url(self.client.base_id(), "", "", "")
    }

    /// Fetch a live version of the schema from Airtable's metadata API.
    pub async fn get_schema(&self) -> Result<serde_json::Value, AirtableError> {
        self.client.get_schema().await
    }
}
