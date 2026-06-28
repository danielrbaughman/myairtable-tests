// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use crate::airtable_model::OrmModel;
use crate::client::AirtableClient;
use crate::error::AirtableError;
use crate::models::FormulasModel;
use crate::orm_table::OrmTable;
use crate::table::StructTable;
use crate::types::{AirtableQuery, RecordId};
use std::sync::Arc;

/// Table accessor for `Formulas`. ORM by default, `.dict` for raw records.
///
/// # Example
///
/// ```ignore
/// // ORM access (typed models)
/// let record = airtable.formulas.get_one("rec123").await?;
///
/// // Dict access (raw JSON fields)
/// let record = airtable.formulas.dict.get_one("rec123", true).await?;
/// ```
pub struct FormulasTable {
    /// Raw record (dict) access.
    pub dict: StructTable,
    orm: OrmTable<FormulasModel>,
}

impl FormulasTable {
    /// Create a new table accessor from a shared client.
    pub fn new(client: Arc<AirtableClient>) -> Self {
        Self {
            dict: StructTable::new(Arc::clone(&client), "tblnuYBsMdXNDsuRc", "Formulas"),
            orm: OrmTable::new(Arc::clone(&client), "tblnuYBsMdXNDsuRc", "Formulas"),
        }
    }

    /// Get a single record by ID.
    pub async fn get_one(&self, record_id: &RecordId) -> Result<FormulasModel, AirtableError> {
        self.orm.get_one(record_id).await
    }
    /// Get multiple records.
    pub async fn get_many(
        &self,
        params: &AirtableQuery,
    ) -> Result<Vec<FormulasModel>, AirtableError> {
        self.orm.get_many(params).await
    }
    /// Create a new record.
    pub async fn create_one(
        &self,
        model: &FormulasModel,
        typecast: bool,
    ) -> Result<FormulasModel, AirtableError> {
        self.orm.create_one(model, typecast).await
    }
    /// Create multiple records.
    pub async fn create_many(
        &self,
        models: &[FormulasModel],
        typecast: bool,
    ) -> Result<Vec<FormulasModel>, AirtableError> {
        self.orm.create_many(models, typecast).await
    }
    /// Update an existing record.
    pub async fn update_one(
        &self,
        record_id: &RecordId,
        model: &FormulasModel,
        typecast: bool,
    ) -> Result<FormulasModel, AirtableError> {
        self.orm.update_one(record_id, model, typecast).await
    }
    /// Update multiple records.
    pub async fn update_many(
        &self,
        records: &[(&RecordId, &FormulasModel)],
        typecast: bool,
    ) -> Result<Vec<FormulasModel>, AirtableError> {
        self.orm.update_many(records, typecast).await
    }
    /// Upsert a model, updated in place. With `fields_to_merge_on`, Airtable matches an existing record by those field values (server-side performUpsert); otherwise it creates if no ID / updates by ID.
    pub async fn upsert(
        &self,
        model: &mut FormulasModel,
        fields_to_merge_on: Option<&[&str]>,
        typecast: bool,
    ) -> Result<(), AirtableError> {
        self.orm.upsert(model, fields_to_merge_on, typecast).await
    }
    /// Upsert multiple models in one batched server-side performUpsert, matching by `fields_to_merge_on`. Returns the upserted models.
    pub async fn upsert_many(
        &self,
        models: &[FormulasModel],
        fields_to_merge_on: &[&str],
        typecast: bool,
    ) -> Result<Vec<FormulasModel>, AirtableError> {
        self.orm
            .upsert_many(models, fields_to_merge_on, typecast)
            .await
    }
    /// Delete a record.
    pub async fn delete_one(&self, record_id: &RecordId) -> Result<(), AirtableError> {
        self.orm.delete_one(record_id).await
    }
    /// Delete multiple records.
    pub async fn delete_many(&self, record_ids: &[RecordId]) -> Result<(), AirtableError> {
        self.orm.delete_many(record_ids).await
    }
    /// Get the Airtable web URL for this table.
    pub fn url(&self) -> String {
        self.orm.url()
    }
    /// Set cache TTL in seconds for both ORM and dict layers. 0 = disabled.
    pub fn set_cache_seconds(&mut self, seconds: u64) {
        self.orm.set_cache_seconds(seconds);
        self.dict.set_cache_seconds(seconds);
    }
    /// Clear the response cache for both ORM and dict layers.
    pub fn invalidate_cache(&self) {
        self.orm.invalidate_cache();
        self.dict.invalidate_cache();
    }
}
