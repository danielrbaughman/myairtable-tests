// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use crate::client::AirtableClient;
use crate::error::AirtableError;
use crate::models::{CreatePrimary, Primary};
use crate::pagination::PaginatedResponse;
use crate::types::{Record, RecordId};

/// Table wrapper for `Primary`
pub struct PrimaryTable<'a> {
    client: &'a AirtableClient,
}

impl<'a> PrimaryTable<'a> {
    /// Airtable table ID.
    pub const TABLE_ID: &'static str = "tblmb3iqgpNS1ysV2";
    /// Airtable table name.
    pub const TABLE_NAME: &'static str = "Primary";

    /// Create a new table wrapper.
    pub fn new(client: &'a AirtableClient) -> Self {
        Self { client }
    }

    /// List records from this table.
    pub async fn list(
        &self,
        offset: Option<&str>,
    ) -> Result<PaginatedResponse<Primary>, AirtableError> {
        self.client.list_records(Self::TABLE_ID, offset).await
    }

    /// Get a single record by ID.
    pub async fn get(&self, record_id: &RecordId) -> Result<Record<Primary>, AirtableError> {
        self.client.get_record(Self::TABLE_ID, record_id).await
    }

    /// Create a new record.
    pub async fn create(&self, fields: &CreatePrimary) -> Result<Record<Primary>, AirtableError> {
        self.client.create_record(Self::TABLE_ID, fields).await
    }

    /// Create multiple records (batched in groups of 10).
    pub async fn create_many(
        &self,
        records: &[CreatePrimary],
    ) -> Result<Vec<Record<Primary>>, AirtableError> {
        self.client.create_records(Self::TABLE_ID, records).await
    }

    /// Update an existing record.
    pub async fn update(
        &self,
        record_id: &RecordId,
        fields: &CreatePrimary,
    ) -> Result<Record<Primary>, AirtableError> {
        self.client
            .update_record(Self::TABLE_ID, record_id, fields)
            .await
    }

    /// Update multiple records (batched in groups of 10).
    pub async fn update_many(
        &self,
        records: &[(&RecordId, &CreatePrimary)],
    ) -> Result<Vec<Record<Primary>>, AirtableError> {
        self.client.update_records(Self::TABLE_ID, records).await
    }

    /// Delete a record.
    pub async fn delete(&self, record_id: &RecordId) -> Result<(), AirtableError> {
        self.client.delete_record(Self::TABLE_ID, record_id).await
    }

    /// Delete multiple records (batched in groups of 10).
    pub async fn delete_many(&self, record_ids: &[RecordId]) -> Result<(), AirtableError> {
        self.client.delete_records(Self::TABLE_ID, record_ids).await
    }
}
