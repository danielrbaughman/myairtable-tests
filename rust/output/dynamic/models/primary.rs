// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use crate::airtable_model::{ModelMeta, OrmModel};
use crate::options::{PrimaryMultipleSelectOption, PrimarySingleSelectOption};
use crate::types::{AirtableButton, Attachment, Collaborator, RecordId};
use serde::{Deserialize, Serialize};

/// ORM model for `Primary`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrimaryModel {
    #[serde(skip)]
    pub id: Option<RecordId>,
    #[serde(skip)]
    pub created_time: Option<String>,
    #[serde(skip)]
    pub _meta: ModelMeta,
    /// Attachment `fldhF2AEuSC1haCZd`
    #[serde(rename = "fldhF2AEuSC1haCZd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<Attachment>>,
    /// Auto Number `fldizvTkxgIn0mC3L` - `Read-Only`
    #[serde(rename = "fldizvTkxgIn0mC3L")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_number: Option<i64>,
    /// Button `fldY48yKPG16AajtU` - `Read-Only`
    #[serde(rename = "fldY48yKPG16AajtU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<AirtableButton>,
    /// Checkbox `fldjQIaAZVegb1FUa`
    #[serde(rename = "fldjQIaAZVegb1FUa")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkbox: Option<bool>,
    /// Created By `fldGLQhDz2UjjiHG6` - `Read-Only`
    #[serde(rename = "fldGLQhDz2UjjiHG6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Collaborator>,
    /// Created Time `fld2YgW382Kt9xltA` - `Read-Only`
    #[serde(rename = "fld2YgW382Kt9xltA")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_time: Option<String>,
    /// Currency (float) `fldyh8pzDXiy5abEr`
    #[serde(rename = "fldyh8pzDXiy5abEr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_float: Option<f64>,
    /// Currency (int) `fldBfo74z9hD78hP8`
    #[serde(rename = "fldBfo74z9hD78hP8")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_int: Option<f64>,
    /// Date `fldC6LfNVvVIxKyQH`
    #[serde(rename = "fldC6LfNVvVIxKyQH")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Date (with time) `fldizYmjpXABGDLTG`
    #[serde(rename = "fldizYmjpXABGDLTG")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_with_time: Option<String>,
    /// Duration `fldLTyf6ljS0rhur8`
    #[serde(rename = "fldLTyf6ljS0rhur8")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Email `fldHCJoYBiFVsNvP4`
    #[serde(rename = "fldHCJoYBiFVsNvP4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Formula (Complex) `fld2vnFc0Bl5IOFUQ` - `Read-Only`
    ///
    /// ```text
    /// CONCATENATE(
    ///   "Primary Key: ",
    ///   {Primary Key},
    ///   "\n",
    ///   "Single Line Text: ",
    ///   {Single Line Text},
    ///   "\n",
    ///   "Long Text: ",
    ///   {Long Text},
    ///   "\n",
    ///   "Long Text with Rich Text: ",
    ///   {Long Text with Rich Text},
    ///   "\n",
    ///   "Attachment: ",
    ///   IF(
    ///     {Attachment},
    ///     {Attachment},
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Checkbox: ",
    ///   IF(
    ///     {Checkbox},
    ///     "Checked",
    ///     "Unchecked"
    ///   ),
    ///   "\n",
    ///   "Multiple Select: ",
    ///   IF(
    ///     {Multiple Select},
    ///     {Multiple Select},
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Single Select: ",
    ///   IF(
    ///     {Single Select},
    ///     {Single Select},
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "User: ",
    ///   IF(
    ///     {User},
    ///     {User},
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "User (allow multiple): ",
    ///   IF(
    ///     {User (allow multiple)},
    ///     {User (allow multiple)},
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Date: ",
    ///   IF(
    ///     {Date},
    ///     DATETIME_FORMAT({Date}, 'YYYY-MM-DD'),
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Date (with time): ",
    ///   IF(
    ///     {Date (with time)},
    ///     DATETIME_FORMAT(
    ///       {Date (with time)},
    ///       'YYYY-MM-DD HH:mm'
    ///     ),
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Phone Number: ",
    ///   IF(
    ///     {Phone Number},
    ///     {Phone Number},
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Email: ",
    ///   IF(
    ///     {Email},
    ///     {Email},
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "URL: ",
    ///   IF(
    ///     {URL},
    ///     {URL},
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Number (int): ",
    ///   IF(
    ///     {Number (int)},
    ///     {Number(int) } & "",
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Number (float): ",
    ///   IF(
    ///     {Number (float)},
    ///     {Number(float) } & "",
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Currency (int): ",
    ///   IF(
    ///     {Currency (int)},
    ///     {Currency(int) } & "",
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Currency (float): ",
    ///   IF(
    ///     {Currency (float)},
    ///     {Currency(float) } & "",
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Percent (int): ",
    ///   IF(
    ///     {Percent (int)},
    ///     {Percent(int) } & "",
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Percent (float): ",
    ///   IF(
    ///     {Percent (float)},
    ///     {Percent(float) } & "",
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Duration: ",
    ///   IF(
    ///     {Duration},
    ///     {Duration} & "",
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Rating: ",
    ///   IF(
    ///     {Rating},
    ///     {Rating} & "",
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Created Time: ",
    ///   IF(
    ///     {Created Time},
    ///     DATETIME_FORMAT(
    ///       {Created Time},
    ///       'YYYY-MM-DD HH:mm'
    ///     ),
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Last Modified Time: ",
    ///   IF(
    ///     {Last Modified Time},
    ///     DATETIME_FORMAT(
    ///       {Last Modified Time},
    ///       'YYYY-MM-DD HH:mm'
    ///     ),
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Created By: ",
    ///   IF(
    ///     {Created By},
    ///     {Created By},
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Last Modified By: ",
    ///   IF(
    ///     {Last Modified By},
    ///     {Last Modified By},
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Auto Number: ",
    ///   IF(
    ///     {Auto Number},
    ///     {Auto Number} & "",
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Button: ",
    ///   IF(
    ///     {Button},
    ///     {Button},
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Link (single): ",
    ///   IF(
    ///     {Link (single)},
    ///     {Link (single)},
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Link (multiple): ",
    ///   IF(
    ///     {Link (multiple)},
    ///     {Link (multiple)},
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Lookup: ",
    ///   IF(
    ///     {Lookup},
    ///     {Lookup},
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Rollup: ",
    ///   IF(
    ///     {Rollup},
    ///     {Rollup},
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Formula (ID): ",
    ///   IF(
    ///     {Formula (ID)},
    ///     {Formula (ID)},
    ///     "None"
    ///   ),
    ///   "\n",
    ///   "Formula (Simple): ",
    ///   IF(
    ///     {Formula (Simple)},
    ///     {Formula (Simple)},
    ///     "None"
    ///   )
    /// )
    /// ```
    #[serde(rename = "fld2vnFc0Bl5IOFUQ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formula_complex: Option<String>,
    /// Formula (ID) `fldcf62YFeIIDHElt` - `Read-Only`
    ///
    /// ```text
    /// RECORD_ID()
    /// ```
    #[serde(rename = "fldcf62YFeIIDHElt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formula_id: Option<String>,
    /// Formula (Nested) `fldXFeHRPBLz6AiWh` - `Read-Only`
    ///
    /// ```text
    /// {Formula (ID)} & {Formula (Simple)} & {Formula (Complex)}
    /// ```
    #[serde(rename = "fldXFeHRPBLz6AiWh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formula_nested: Option<String>,
    /// Formula (Simple) `fldy1axxaoUToLVC6` - `Read-Only`
    ///
    /// ```text
    /// {Number (int)} + {Number (float)}
    /// ```
    #[serde(rename = "fldy1axxaoUToLVC6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formula_simple: Option<f64>,
    /// Last Modified By `fldF8iDttqP0AgzWC` - `Read-Only`
    #[serde(rename = "fldF8iDttqP0AgzWC")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<Collaborator>,
    /// Last Modified Time `fldMinKh4pa3YX86g` - `Read-Only`
    #[serde(rename = "fldMinKh4pa3YX86g")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    /// Link (multiple) `fldFyFheQWczd8oux`
    #[serde(rename = "fldFyFheQWczd8oux")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_multiple: Option<Vec<RecordId>>,
    /// Link (single) `fld7F5onkDo6mkmbN`
    #[serde(rename = "fld7F5onkDo6mkmbN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_single: Option<Vec<RecordId>>,
    /// Long Text `fld8ulc6J0W29M6La`
    #[serde(rename = "fld8ulc6J0W29M6La")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_text: Option<String>,
    /// Long Text with Rich Text `fldHJkxCMC0xo343u`
    #[serde(rename = "fldHJkxCMC0xo343u")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_text_with_rich_text: Option<String>,
    /// Lookup `fldbmFmrzYKBktJvE` - `Read-Only`
    #[serde(rename = "fldbmFmrzYKBktJvE")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup: Option<Vec<String>>,
    /// Multiple Select `fld6GTabFmu1xKPvZ`
    #[serde(rename = "fld6GTabFmu1xKPvZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_select: Option<Vec<PrimaryMultipleSelectOption>>,
    /// Number (float) `fldmU0X2l4RWd21dd`
    #[serde(rename = "fldmU0X2l4RWd21dd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_float: Option<f64>,
    /// Number (int) `fldOfPKGmnRPv94QH`
    #[serde(rename = "fldOfPKGmnRPv94QH")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_int: Option<i64>,
    /// Percent (float) `fldiGui9ll69N7WOj`
    #[serde(rename = "fldiGui9ll69N7WOj")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_float: Option<f64>,
    /// Percent (int) `fldbAAyWboGulpb4s`
    #[serde(rename = "fldbAAyWboGulpb4s")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_int: Option<f64>,
    /// Phone Number `fld38tnNpHmoks8C8`
    #[serde(rename = "fld38tnNpHmoks8C8")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Primary Key `fldol5Q4wmQJQvPRy` - `Primary Key`
    #[serde(rename = "fldol5Q4wmQJQvPRy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    /// Rating `fldRsmwFwQNZkKLp4`
    #[serde(rename = "fldRsmwFwQNZkKLp4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<serde_json::Value>,
    /// Rollup `fldGaFgBsDC3IBUdV` - `Read-Only`
    #[serde(rename = "fldGaFgBsDC3IBUdV")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollup: Option<Vec<String>>,
    /// Single Line Text `fld0BL2lFo9fqcKv3`
    #[serde(rename = "fld0BL2lFo9fqcKv3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_line_text: Option<String>,
    /// Single Select `fldn0GFFtMFpCXUNU`
    #[serde(rename = "fldn0GFFtMFpCXUNU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_select: Option<PrimarySingleSelectOption>,
    /// URL `fldLYloz2oP4ymf3B`
    #[serde(rename = "fldLYloz2oP4ymf3B")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// User `fldU6SbLp8CSkLcA4`
    #[serde(rename = "fldU6SbLp8CSkLcA4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Collaborator>,
    /// User (allow multiple) `fldBwCDbAVxRj9yg7`
    #[serde(rename = "fldBwCDbAVxRj9yg7")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_allow_multiple: Option<Vec<Collaborator>>,
}

impl PrimaryModel {
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
}

impl OrmModel for PrimaryModel {
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

/// Writable fields for creating/updating `Primary` records.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePrimaryModel {
    #[serde(rename = "fldhF2AEuSC1haCZd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<Attachment>>,
    #[serde(rename = "fldjQIaAZVegb1FUa")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkbox: Option<bool>,
    #[serde(rename = "fldyh8pzDXiy5abEr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_float: Option<f64>,
    #[serde(rename = "fldBfo74z9hD78hP8")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_int: Option<f64>,
    #[serde(rename = "fldC6LfNVvVIxKyQH")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "fldizYmjpXABGDLTG")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_with_time: Option<String>,
    #[serde(rename = "fldLTyf6ljS0rhur8")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "fldHCJoYBiFVsNvP4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "fldFyFheQWczd8oux")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_multiple: Option<Vec<RecordId>>,
    #[serde(rename = "fld7F5onkDo6mkmbN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_single: Option<Vec<RecordId>>,
    #[serde(rename = "fld8ulc6J0W29M6La")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_text: Option<String>,
    #[serde(rename = "fldHJkxCMC0xo343u")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_text_with_rich_text: Option<String>,
    #[serde(rename = "fld6GTabFmu1xKPvZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_select: Option<Vec<PrimaryMultipleSelectOption>>,
    #[serde(rename = "fldmU0X2l4RWd21dd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_float: Option<f64>,
    #[serde(rename = "fldOfPKGmnRPv94QH")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_int: Option<i64>,
    #[serde(rename = "fldiGui9ll69N7WOj")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_float: Option<f64>,
    #[serde(rename = "fldbAAyWboGulpb4s")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_int: Option<f64>,
    #[serde(rename = "fld38tnNpHmoks8C8")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "fldol5Q4wmQJQvPRy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[serde(rename = "fldRsmwFwQNZkKLp4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<serde_json::Value>,
    #[serde(rename = "fld0BL2lFo9fqcKv3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_line_text: Option<String>,
    #[serde(rename = "fldn0GFFtMFpCXUNU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_select: Option<PrimarySingleSelectOption>,
    #[serde(rename = "fldLYloz2oP4ymf3B")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "fldU6SbLp8CSkLcA4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Collaborator>,
    #[serde(rename = "fldBwCDbAVxRj9yg7")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_allow_multiple: Option<Vec<Collaborator>>,
}
