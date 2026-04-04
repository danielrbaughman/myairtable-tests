// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use crate::airtable_model::{ModelMeta, OrmModel};
use crate::types::RecordId;
use serde::{Deserialize, Serialize};

/// ORM model for `Tertiary`.
///
/// # Example
///
/// ```ignore
/// let record = airtable.tertiary.get_one("rec123").await?;
/// println!("{:?}", record);
///
/// let new = CreateTertiaryModel { ..Default::default() };
/// let created = airtable.tertiary.create_one(&new).await?;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TertiaryModel {
    #[serde(skip)]
    pub id: Option<RecordId>,
    #[serde(skip)]
    pub created_time: Option<String>,
    #[serde(skip)]
    pub _meta: ModelMeta,
    /// Name `fldwzqKxsRnPZJ2Ll` - `Primary Key`
    #[serde(rename = "fldwzqKxsRnPZJ2Ll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Secondary `fld8lCuUXpEXkIeYv`
    #[serde(rename = "fld8lCuUXpEXkIeYv")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<Vec<RecordId>>,
    /// Value `fldjNLBh2UccM64h5`
    #[serde(rename = "fldjNLBh2UccM64h5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl TertiaryModel {
    /// Formula builder for this table.
    pub const F: crate::formulas::TertiaryFormulas = crate::formulas::TertiaryFormulas::new();
    /// Create a model from just a record ID (for later fetch).
    pub fn from_id(
        client: std::sync::Arc<crate::client::AirtableClient>,
        table_id: &'static str,
        id: &str,
    ) -> Self {
        let mut model = Self::default();
        model.id = Some(id.to_string());
        model._meta.client = Some(client);
        model._meta.table_id = Some(table_id);
        model
    }

    /// Get the Airtable web URL for this record.
    pub fn url(&self, view_id: impl AsRef<str>) -> String {
        let base_id = self
            ._meta
            .client
            .as_ref()
            .map(|c| c.base_id())
            .unwrap_or("");
        let record_id = self.id.as_deref().unwrap_or("");
        crate::types::build_url(base_id, "tblLFoLxEdWlxjmLP", view_id.as_ref(), record_id)
    }
}

impl OrmModel for TertiaryModel {
    fn meta(&self) -> &ModelMeta {
        &self._meta
    }
    fn meta_mut(&mut self) -> &mut ModelMeta {
        &mut self._meta
    }
    fn get_id(&self) -> &Option<RecordId> {
        &self.id
    }
    fn set_id(&mut self, id: Option<RecordId>) {
        self.id = id;
    }
    fn get_created_time(&self) -> &Option<String> {
        &self.created_time
    }
    fn set_created_time(&mut self, ct: Option<String>) {
        self.created_time = ct;
    }
}

/// Writable fields for creating/updating `Tertiary` records.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTertiaryModel {
    #[serde(rename = "fldwzqKxsRnPZJ2Ll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "fld8lCuUXpEXkIeYv")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<Vec<RecordId>>,
    #[serde(rename = "fldjNLBh2UccM64h5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
