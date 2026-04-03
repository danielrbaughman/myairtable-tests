// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use std::sync::Arc;

use crate::airtable_model::OrmModel;
use crate::client::AirtableClient;
use crate::error::AirtableError;
use crate::models::{CreateFormulasModel, FormulasModel};
use crate::models::{CreatePrimaryModel, PrimaryModel};
use crate::models::{CreateSecondaryModel, SecondaryModel};
use crate::models::{CreateTertiaryModel, TertiaryModel};
use crate::orm_table::OrmTable;
use crate::table::StructTable;
use crate::types::{build_url, AirtableQuery, RecordId};

/// Table accessor for `Formulas`. ORM by default, `.dict` for raw records.
pub struct FormulasTable {
    /// Raw record (dict) access.
    pub dict: StructTable,
    orm: OrmTable<FormulasModel, CreateFormulasModel>,
}

impl FormulasTable {
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
        fields: &CreateFormulasModel,
    ) -> Result<FormulasModel, AirtableError> {
        self.orm.create_one(fields).await
    }
    /// Create multiple records.
    pub async fn create_many(
        &self,
        records: &[CreateFormulasModel],
    ) -> Result<Vec<FormulasModel>, AirtableError> {
        self.orm.create_many(records).await
    }
    /// Update an existing record.
    pub async fn update_one(
        &self,
        record_id: &RecordId,
        fields: &CreateFormulasModel,
    ) -> Result<FormulasModel, AirtableError> {
        self.orm.update_one(record_id, fields).await
    }
    /// Update multiple records.
    pub async fn update_many(
        &self,
        records: &[(&RecordId, &CreateFormulasModel)],
    ) -> Result<Vec<FormulasModel>, AirtableError> {
        self.orm.update_many(records).await
    }
    /// Upsert a model. Creates if no ID, updates if ID exists.
    pub async fn upsert(&self, model: &mut FormulasModel) -> Result<(), AirtableError> {
        self.orm.upsert(model).await
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

/// Table accessor for `Primary`. ORM by default, `.dict` for raw records.
pub struct PrimaryTable {
    /// Raw record (dict) access.
    pub dict: StructTable,
    orm: OrmTable<PrimaryModel, CreatePrimaryModel>,
}

impl PrimaryTable {
    /// Get a single record by ID.
    pub async fn get_one(&self, record_id: &RecordId) -> Result<PrimaryModel, AirtableError> {
        self.orm.get_one(record_id).await
    }
    /// Get multiple records.
    pub async fn get_many(
        &self,
        params: &AirtableQuery,
    ) -> Result<Vec<PrimaryModel>, AirtableError> {
        self.orm.get_many(params).await
    }
    /// Create a new record.
    pub async fn create_one(
        &self,
        fields: &CreatePrimaryModel,
    ) -> Result<PrimaryModel, AirtableError> {
        self.orm.create_one(fields).await
    }
    /// Create multiple records.
    pub async fn create_many(
        &self,
        records: &[CreatePrimaryModel],
    ) -> Result<Vec<PrimaryModel>, AirtableError> {
        self.orm.create_many(records).await
    }
    /// Update an existing record.
    pub async fn update_one(
        &self,
        record_id: &RecordId,
        fields: &CreatePrimaryModel,
    ) -> Result<PrimaryModel, AirtableError> {
        self.orm.update_one(record_id, fields).await
    }
    /// Update multiple records.
    pub async fn update_many(
        &self,
        records: &[(&RecordId, &CreatePrimaryModel)],
    ) -> Result<Vec<PrimaryModel>, AirtableError> {
        self.orm.update_many(records).await
    }
    /// Upsert a model. Creates if no ID, updates if ID exists.
    pub async fn upsert(&self, model: &mut PrimaryModel) -> Result<(), AirtableError> {
        self.orm.upsert(model).await
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

/// Table accessor for `Secondary`. ORM by default, `.dict` for raw records.
pub struct SecondaryTable {
    /// Raw record (dict) access.
    pub dict: StructTable,
    orm: OrmTable<SecondaryModel, CreateSecondaryModel>,
}

impl SecondaryTable {
    /// Get a single record by ID.
    pub async fn get_one(&self, record_id: &RecordId) -> Result<SecondaryModel, AirtableError> {
        self.orm.get_one(record_id).await
    }
    /// Get multiple records.
    pub async fn get_many(
        &self,
        params: &AirtableQuery,
    ) -> Result<Vec<SecondaryModel>, AirtableError> {
        self.orm.get_many(params).await
    }
    /// Create a new record.
    pub async fn create_one(
        &self,
        fields: &CreateSecondaryModel,
    ) -> Result<SecondaryModel, AirtableError> {
        self.orm.create_one(fields).await
    }
    /// Create multiple records.
    pub async fn create_many(
        &self,
        records: &[CreateSecondaryModel],
    ) -> Result<Vec<SecondaryModel>, AirtableError> {
        self.orm.create_many(records).await
    }
    /// Update an existing record.
    pub async fn update_one(
        &self,
        record_id: &RecordId,
        fields: &CreateSecondaryModel,
    ) -> Result<SecondaryModel, AirtableError> {
        self.orm.update_one(record_id, fields).await
    }
    /// Update multiple records.
    pub async fn update_many(
        &self,
        records: &[(&RecordId, &CreateSecondaryModel)],
    ) -> Result<Vec<SecondaryModel>, AirtableError> {
        self.orm.update_many(records).await
    }
    /// Upsert a model. Creates if no ID, updates if ID exists.
    pub async fn upsert(&self, model: &mut SecondaryModel) -> Result<(), AirtableError> {
        self.orm.upsert(model).await
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

/// Table accessor for `Tertiary`. ORM by default, `.dict` for raw records.
pub struct TertiaryTable {
    /// Raw record (dict) access.
    pub dict: StructTable,
    orm: OrmTable<TertiaryModel, CreateTertiaryModel>,
}

impl TertiaryTable {
    /// Get a single record by ID.
    pub async fn get_one(&self, record_id: &RecordId) -> Result<TertiaryModel, AirtableError> {
        self.orm.get_one(record_id).await
    }
    /// Get multiple records.
    pub async fn get_many(
        &self,
        params: &AirtableQuery,
    ) -> Result<Vec<TertiaryModel>, AirtableError> {
        self.orm.get_many(params).await
    }
    /// Create a new record.
    pub async fn create_one(
        &self,
        fields: &CreateTertiaryModel,
    ) -> Result<TertiaryModel, AirtableError> {
        self.orm.create_one(fields).await
    }
    /// Create multiple records.
    pub async fn create_many(
        &self,
        records: &[CreateTertiaryModel],
    ) -> Result<Vec<TertiaryModel>, AirtableError> {
        self.orm.create_many(records).await
    }
    /// Update an existing record.
    pub async fn update_one(
        &self,
        record_id: &RecordId,
        fields: &CreateTertiaryModel,
    ) -> Result<TertiaryModel, AirtableError> {
        self.orm.update_one(record_id, fields).await
    }
    /// Update multiple records.
    pub async fn update_many(
        &self,
        records: &[(&RecordId, &CreateTertiaryModel)],
    ) -> Result<Vec<TertiaryModel>, AirtableError> {
        self.orm.update_many(records).await
    }
    /// Upsert a model. Creates if no ID, updates if ID exists.
    pub async fn upsert(&self, model: &mut TertiaryModel) -> Result<(), AirtableError> {
        self.orm.upsert(model).await
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

/// Main entry point for the Airtable base.
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
            formulas: FormulasTable {
                dict: StructTable::new(Arc::clone(&client), "tblnuYBsMdXNDsuRc", "Formulas"),
                orm: OrmTable::new(Arc::clone(&client), "tblnuYBsMdXNDsuRc", "Formulas"),
            },
            primary: PrimaryTable {
                dict: StructTable::new(Arc::clone(&client), "tblmb3iqgpNS1ysV2", "Primary"),
                orm: OrmTable::new(Arc::clone(&client), "tblmb3iqgpNS1ysV2", "Primary"),
            },
            secondary: SecondaryTable {
                dict: StructTable::new(Arc::clone(&client), "tblPPScS3XMuFkDYN", "Secondary"),
                orm: OrmTable::new(Arc::clone(&client), "tblPPScS3XMuFkDYN", "Secondary"),
            },
            tertiary: TertiaryTable {
                dict: StructTable::new(Arc::clone(&client), "tblLFoLxEdWlxjmLP", "Tertiary"),
                orm: OrmTable::new(Arc::clone(&client), "tblLFoLxEdWlxjmLP", "Tertiary"),
            },
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
