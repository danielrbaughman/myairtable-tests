// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use crate::airtable_model::{ModelMeta, OrmModel};
use crate::types::RecordId;
use serde::{Deserialize, Serialize};

/// ORM model for `Secondary`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecondaryModel {
    #[serde(skip)]
    pub id: Option<RecordId>,
    #[serde(skip)]
    pub created_time: Option<String>,
    #[serde(skip)]
    pub _meta: ModelMeta,
    /// Link to Tertiary `fldKR6tdbnOBRCtdQ`
    #[serde(rename = "fldKR6tdbnOBRCtdQ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_to_tertiary: Option<Vec<RecordId>>,
    /// Name `fld1RagdJ09mpWhzM` - `Primary Key`
    #[serde(rename = "fld1RagdJ09mpWhzM")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Primary `fldl0nB9WRFSdqlii`
    #[serde(rename = "fldl0nB9WRFSdqlii")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<Vec<RecordId>>,
    /// Primary 2 `fldgoE2oZmXmKkQca`
    #[serde(rename = "fldgoE2oZmXmKkQca")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_2: Option<Vec<RecordId>>,
    /// Value `fldi6Mxh5H1gPGxFX`
    #[serde(rename = "fldi6Mxh5H1gPGxFX")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl SecondaryModel {
    /// Formula builder for this table.
    pub const F: crate::formulas::SecondaryFormulas = crate::formulas::SecondaryFormulas::new();
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
        crate::types::build_url(base_id, "tblPPScS3XMuFkDYN", view_id.as_ref(), record_id)
    }
}

impl OrmModel for SecondaryModel {
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
    fn set_created_time(&mut self, ct: Option<String>) {
        self.created_time = ct;
    }
}

/// Writable fields for creating/updating `Secondary` records.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSecondaryModel {
    #[serde(rename = "fldKR6tdbnOBRCtdQ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_to_tertiary: Option<Vec<RecordId>>,
    #[serde(rename = "fld1RagdJ09mpWhzM")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "fldl0nB9WRFSdqlii")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<Vec<RecordId>>,
    #[serde(rename = "fldgoE2oZmXmKkQca")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_2: Option<Vec<RecordId>>,
    #[serde(rename = "fldi6Mxh5H1gPGxFX")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
