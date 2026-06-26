// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use std::sync::Arc;

use crate::airtable_model::OrmModel;
use crate::client::AirtableClient;
use crate::error::AirtableError;
use crate::models::FormulasModel;
use crate::models::PrimaryModel;
use crate::models::SecondaryModel;
use crate::models::TertiaryModel;
use crate::orm_table::OrmTable;
use crate::table::StructTable;
use crate::types::{build_url, AirtableQuery, RecordId};

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

/// Table accessor for `Primary`. ORM by default, `.dict` for raw records.
///
/// # Example
///
/// ```ignore
/// // ORM access (typed models)
/// let record = airtable.primary.get_one("rec123").await?;
///
/// // Dict access (raw JSON fields)
/// let record = airtable.primary.dict.get_one("rec123", true).await?;
/// ```
pub struct PrimaryTable {
    /// Raw record (dict) access.
    pub dict: StructTable,
    orm: OrmTable<PrimaryModel>,
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
        model: &PrimaryModel,
        typecast: bool,
    ) -> Result<PrimaryModel, AirtableError> {
        self.orm.create_one(model, typecast).await
    }
    /// Create multiple records.
    pub async fn create_many(
        &self,
        models: &[PrimaryModel],
        typecast: bool,
    ) -> Result<Vec<PrimaryModel>, AirtableError> {
        self.orm.create_many(models, typecast).await
    }
    /// Update an existing record.
    pub async fn update_one(
        &self,
        record_id: &RecordId,
        model: &PrimaryModel,
        typecast: bool,
    ) -> Result<PrimaryModel, AirtableError> {
        self.orm.update_one(record_id, model, typecast).await
    }
    /// Update multiple records.
    pub async fn update_many(
        &self,
        records: &[(&RecordId, &PrimaryModel)],
        typecast: bool,
    ) -> Result<Vec<PrimaryModel>, AirtableError> {
        self.orm.update_many(records, typecast).await
    }
    /// Upsert a model, updated in place. With `fields_to_merge_on`, Airtable matches an existing record by those field values (server-side performUpsert); otherwise it creates if no ID / updates by ID.
    pub async fn upsert(
        &self,
        model: &mut PrimaryModel,
        fields_to_merge_on: Option<&[&str]>,
        typecast: bool,
    ) -> Result<(), AirtableError> {
        self.orm.upsert(model, fields_to_merge_on, typecast).await
    }
    /// Upsert multiple models in one batched server-side performUpsert, matching by `fields_to_merge_on`. Returns the upserted models.
    pub async fn upsert_many(
        &self,
        models: &[PrimaryModel],
        fields_to_merge_on: &[&str],
        typecast: bool,
    ) -> Result<Vec<PrimaryModel>, AirtableError> {
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

/// Table accessor for `Secondary`. ORM by default, `.dict` for raw records.
///
/// # Example
///
/// ```ignore
/// // ORM access (typed models)
/// let record = airtable.secondary.get_one("rec123").await?;
///
/// // Dict access (raw JSON fields)
/// let record = airtable.secondary.dict.get_one("rec123", true).await?;
/// ```
pub struct SecondaryTable {
    /// Raw record (dict) access.
    pub dict: StructTable,
    orm: OrmTable<SecondaryModel>,
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
        model: &SecondaryModel,
        typecast: bool,
    ) -> Result<SecondaryModel, AirtableError> {
        self.orm.create_one(model, typecast).await
    }
    /// Create multiple records.
    pub async fn create_many(
        &self,
        models: &[SecondaryModel],
        typecast: bool,
    ) -> Result<Vec<SecondaryModel>, AirtableError> {
        self.orm.create_many(models, typecast).await
    }
    /// Update an existing record.
    pub async fn update_one(
        &self,
        record_id: &RecordId,
        model: &SecondaryModel,
        typecast: bool,
    ) -> Result<SecondaryModel, AirtableError> {
        self.orm.update_one(record_id, model, typecast).await
    }
    /// Update multiple records.
    pub async fn update_many(
        &self,
        records: &[(&RecordId, &SecondaryModel)],
        typecast: bool,
    ) -> Result<Vec<SecondaryModel>, AirtableError> {
        self.orm.update_many(records, typecast).await
    }
    /// Upsert a model, updated in place. With `fields_to_merge_on`, Airtable matches an existing record by those field values (server-side performUpsert); otherwise it creates if no ID / updates by ID.
    pub async fn upsert(
        &self,
        model: &mut SecondaryModel,
        fields_to_merge_on: Option<&[&str]>,
        typecast: bool,
    ) -> Result<(), AirtableError> {
        self.orm.upsert(model, fields_to_merge_on, typecast).await
    }
    /// Upsert multiple models in one batched server-side performUpsert, matching by `fields_to_merge_on`. Returns the upserted models.
    pub async fn upsert_many(
        &self,
        models: &[SecondaryModel],
        fields_to_merge_on: &[&str],
        typecast: bool,
    ) -> Result<Vec<SecondaryModel>, AirtableError> {
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

/// Table accessor for `Tertiary`. ORM by default, `.dict` for raw records.
///
/// # Example
///
/// ```ignore
/// // ORM access (typed models)
/// let record = airtable.tertiary.get_one("rec123").await?;
///
/// // Dict access (raw JSON fields)
/// let record = airtable.tertiary.dict.get_one("rec123", true).await?;
/// ```
pub struct TertiaryTable {
    /// Raw record (dict) access.
    pub dict: StructTable,
    orm: OrmTable<TertiaryModel>,
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
        model: &TertiaryModel,
        typecast: bool,
    ) -> Result<TertiaryModel, AirtableError> {
        self.orm.create_one(model, typecast).await
    }
    /// Create multiple records.
    pub async fn create_many(
        &self,
        models: &[TertiaryModel],
        typecast: bool,
    ) -> Result<Vec<TertiaryModel>, AirtableError> {
        self.orm.create_many(models, typecast).await
    }
    /// Update an existing record.
    pub async fn update_one(
        &self,
        record_id: &RecordId,
        model: &TertiaryModel,
        typecast: bool,
    ) -> Result<TertiaryModel, AirtableError> {
        self.orm.update_one(record_id, model, typecast).await
    }
    /// Update multiple records.
    pub async fn update_many(
        &self,
        records: &[(&RecordId, &TertiaryModel)],
        typecast: bool,
    ) -> Result<Vec<TertiaryModel>, AirtableError> {
        self.orm.update_many(records, typecast).await
    }
    /// Upsert a model, updated in place. With `fields_to_merge_on`, Airtable matches an existing record by those field values (server-side performUpsert); otherwise it creates if no ID / updates by ID.
    pub async fn upsert(
        &self,
        model: &mut TertiaryModel,
        fields_to_merge_on: Option<&[&str]>,
        typecast: bool,
    ) -> Result<(), AirtableError> {
        self.orm.upsert(model, fields_to_merge_on, typecast).await
    }
    /// Upsert multiple models in one batched server-side performUpsert, matching by `fields_to_merge_on`. Returns the upserted models.
    pub async fn upsert_many(
        &self,
        models: &[TertiaryModel],
        fields_to_merge_on: &[&str],
        typecast: bool,
    ) -> Result<Vec<TertiaryModel>, AirtableError> {
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

    /// The base schema, embedded at generation time.
    pub fn schema() -> serde_json::Value {
        serde_json::from_str("{\"tables\":[{\"id\":\"tblnuYBsMdXNDsuRc\",\"name\":\"Formulas\",\"primaryFieldId\":\"fldLZFrZKvSCS4dKb\",\"fields\":[{\"type\":\"formula\",\"options\":{\"isValid\":true,\"formula\":\"\\\"YEAR: \\\" & YEAR({fldlZT521Iy0FFXFL}) & \\\", MONTH: \\\" & MONTH({fldlZT521Iy0FFXFL}) & \\\", DAY: \\\" & DAY({fldlZT521Iy0FFXFL}) &\\n\\\", HOUR: \\\" & HOUR({fld1LZ3Ebpt0LaMmu}) & \\\", MINUTE: \\\" & MINUTE({fld1LZ3Ebpt0LaMmu}) & \\\", SECOND: \\\" & SECOND({fld1LZ3Ebpt0LaMmu}) &\\n\\\", TODAY: \\\" & DATETIME_FORMAT(TODAY(), 'YYYY-MM-DD') &\\n\\\", WORKDAY+5: \\\" & DATETIME_FORMAT(WORKDAY({fldlZT521Iy0FFXFL}, 5), 'YYYY-MM-DD') &\\n\\\", WORKDAY_DIFF: \\\" & WORKDAY_DIFF({fldlZT521Iy0FFXFL}, {fld1LZ3Ebpt0LaMmu}) &\\n\\\", PARSE: \\\" & DATETIME_FORMAT(DATETIME_PARSE(DATESTR({fldlZT521Iy0FFXFL})), 'YYYY-MM-DD') &\\n\\\", FORMAT: \\\" & DATETIME_FORMAT({fld1LZ3Ebpt0LaMmu}, 'YYYY-MM-DD HH:mm') &\\n\\\", LOCALE: \\\" & DATETIME_FORMAT(SET_LOCALE({fld1LZ3Ebpt0LaMmu}, 'en'), 'YYYY-MM-DD') &\\n\\\", TIMEZONE: \\\" & DATETIME_FORMAT(SET_TIMEZONE({fld1LZ3Ebpt0LaMmu}, 'America/New_York'), 'YYYY-MM-DD HH:mm') &\\n\\\", DATESTR: \\\" & DATESTR({fld1LZ3Ebpt0LaMmu}) & \\\", TIMESTR: \\\" & TIMESTR({fld1LZ3Ebpt0LaMmu}) &\\n\\\", TONOW: \\\" & TONOW({fldlZT521Iy0FFXFL}) & \\\", FROMNOW: \\\" & FROMNOW({fld1LZ3Ebpt0LaMmu}) &\\n\\\", DATEADD+2d: \\\" & DATETIME_FORMAT(DATEADD({fldlZT521Iy0FFXFL}, 2, 'days'), 'YYYY-MM-DD') &\\n\\\", WEEKNUM: \\\" & WEEKNUM({fldlZT521Iy0FFXFL}, 'Monday') &\\n\\\", DIFF days: \\\" & DATETIME_DIFF({fldlZT521Iy0FFXFL}, {fld1LZ3Ebpt0LaMmu}, 'days') &\\n\\\", IS_BEFORE: \\\" & IF(IS_BEFORE({fldlZT521Iy0FFXFL}, {fld1LZ3Ebpt0LaMmu}), 'Yes', 'No') &\\n\\\", IS_SAME day: \\\" & IF(IS_SAME({fldlZT521Iy0FFXFL}, {fld1LZ3Ebpt0LaMmu}, 'day'), 'Yes', 'No') &\\n\\\", IS_AFTER: \\\" & IF(IS_AFTER({fldxSQRRn8W879aiU}, {fld1LZ3Ebpt0LaMmu}), 'Yes', 'No')\",\"referencedFieldIds\":[\"fldlZT521Iy0FFXFL\",\"fld1LZ3Ebpt0LaMmu\",\"fldxSQRRn8W879aiU\"],\"result\":{\"type\":\"singleLineText\"}},\"id\":\"fldY7kjaklLeoSgGd\",\"name\":\"Date Formula\",\"description\":\"Demonstrates all date-related Airtable functions using First Date, Second Date, and Third Date fields.\"},{\"type\":\"date\",\"options\":{\"dateFormat\":{\"name\":\"local\",\"format\":\"l\"}},\"id\":\"fldlZT521Iy0FFXFL\",\"name\":\"First Date\"},{\"type\":\"number\",\"options\":{\"precision\":1},\"id\":\"fldA04pqfjMkGXcZU\",\"name\":\"First Number\"},{\"type\":\"singleLineText\",\"id\":\"fldHJuw6pAujnHvkP\",\"name\":\"First Text\"},{\"type\":\"formula\",\"options\":{\"isValid\":true,\"formula\":\"IF(\\n  OR({fldA04pqfjMkGXcZU} = BLANK(), {fldj5nAkal5y8OOZg} = BLANK()),\\n  BLANK(),\\n  \\\"SUM: \\\" & SUM({fldA04pqfjMkGXcZU}, {fldj5nAkal5y8OOZg}) &\\n  \\\", MIN: \\\" & MIN({fldA04pqfjMkGXcZU}, {fldj5nAkal5y8OOZg}) &\\n  \\\", MAX: \\\" & MAX({fldA04pqfjMkGXcZU}, {fldj5nAkal5y8OOZg}) &\\n  \\\", AVG: \\\" & AVERAGE({fldA04pqfjMkGXcZU}, {fldj5nAkal5y8OOZg}) &\\n  \\\", COUNT: \\\" & COUNT({fldA04pqfjMkGXcZU}, {fldj5nAkal5y8OOZg}) &\\n  \\\", CEIL: \\\" & CEILING({fldA04pqfjMkGXcZU}) &\\n  \\\", FLOOR: \\\" & FLOOR({fldj5nAkal5y8OOZg}) &\\n  \\\", ROUND: \\\" & ROUND({fldA04pqfjMkGXcZU}/2, 1) &\\n  \\\", ROUNDUP: \\\" & ROUNDUP({fldj5nAkal5y8OOZg}/2, 0) &\\n  \\\", ROUNDDOWN: \\\" & ROUNDDOWN({fldA04pqfjMkGXcZU}/2, 0) &\\n  \\\", INT: \\\" & INT({fldj5nAkal5y8OOZg}/2) &\\n  \\\", EVEN: \\\" & EVEN({fldA04pqfjMkGXcZU}) &\\n  \\\", ODD: \\\" & ODD({fldj5nAkal5y8OOZg}) &\\n  \\\", MOD: \\\" & MOD({fldA04pqfjMkGXcZU}, {fldj5nAkal5y8OOZg}) &\\n  \\\", LOG: \\\" & LOG({fldA04pqfjMkGXcZU}) &\\n  \\\", EXP: \\\" & EXP(1) &\\n  \\\", POWER: \\\" & POWER({fldA04pqfjMkGXcZU}, 2) &\\n  \\\", SQRT: \\\" & SQRT(ABS({fldj5nAkal5y8OOZg})) &\\n  \\\", ABS: \\\" & ABS({fldj5nAkal5y8OOZg})\\n)\",\"referencedFieldIds\":[\"fldA04pqfjMkGXcZU\",\"fldj5nAkal5y8OOZg\"],\"result\":{\"type\":\"singleLineText\"}},\"id\":\"fldlSuvoeWGokSz8Z\",\"name\":\"Math Formula\",\"description\":\"Demonstrates all math-related functions using First Number and Second Number fields.\"},{\"type\":\"singleLineText\",\"id\":\"fldLZFrZKvSCS4dKb\",\"name\":\"Primary Key\"},{\"type\":\"dateTime\",\"options\":{\"dateFormat\":{\"name\":\"local\",\"format\":\"l\"},\"timeFormat\":{\"name\":\"12hour\",\"format\":\"h:mma\"},\"timeZone\":\"client\"},\"id\":\"fld1LZ3Ebpt0LaMmu\",\"name\":\"Second Date\"},{\"type\":\"number\",\"options\":{\"precision\":1},\"id\":\"fldj5nAkal5y8OOZg\",\"name\":\"Second Number\"},{\"type\":\"singleLineText\",\"id\":\"fldA2boNwwsiuvXw1\",\"name\":\"Second Text\"},{\"type\":\"formula\",\"options\":{\"isValid\":true,\"formula\":\"CONCATENATE(\\n  \\\"LEN: \\\", LEN({fldHJuw6pAujnHvkP}), \\\"; \\\",\\n  \\\"MID: \\\", MID({fldHJuw6pAujnHvkP}, 2, 3), \\\"; \\\",\\n  \\\"LEFT: \\\", LEFT({fldA2boNwwsiuvXw1}, 2), \\\"; \\\",\\n  \\\"RIGHT: \\\", RIGHT({fldA2boNwwsiuvXw1}, 2), \\\"; \\\",\\n  \\\"FIND: \\\", FIND(\\\"e\\\", {fldHJuw6pAujnHvkP}), \\\"; \\\",\\n  \\\"SEARCH: \\\", SEARCH(\\\"e\\\", {fldHJuw6pAujnHvkP}), \\\"; \\\",\\n  \\\"REPLACE: \\\", REPLACE({fldHJuw6pAujnHvkP}, 2, 2, \\\"XX\\\"), \\\"; \\\",\\n  \\\"REPT: \\\", REPT({fldHJuw6pAujnHvkP}, 2), \\\"; \\\",\\n  \\\"LOWER: \\\", LOWER({fldA2boNwwsiuvXw1}), \\\"; \\\",\\n  \\\"UPPER: \\\", UPPER({fldA2boNwwsiuvXw1}), \\\"; \\\",\\n  \\\"TRIM: \\\", TRIM(\\\"   \\\" & {fldHJuw6pAujnHvkP} & \\\"   \\\"), \\\"; \\\",\\n  \\\"SUBSTITUTE: \\\", SUBSTITUTE({fldHJuw6pAujnHvkP}, \\\"e\\\", \\\"@\\\"), \\\"; \\\",\\n  \\\"CONCATENATE: \\\", CONCATENATE({fldHJuw6pAujnHvkP}, \\\"-\\\", {fldA2boNwwsiuvXw1}, \\\"-\\\", {fldfruPf8V9K6qIAN}), \\\"; \\\",\\n  \\\"T: \\\", T({fldA04pqfjMkGXcZU}), \\\"; \\\",\\n  \\\"REGEX_EXTRACT: \\\", REGEX_EXTRACT({fldHJuw6pAujnHvkP}, \\\"[aeiou]\\\"), \\\"; \\\",\\n  \\\"REGEX_MATCH: \\\", IF(REGEX_MATCH({fldHJuw6pAujnHvkP}, \\\"^.e\\\"), \\\"1\\\", \\\"0\\\"), \\\"; \\\",\\n  \\\"REGEX_REPLACE: \\\", REGEX_REPLACE({fldHJuw6pAujnHvkP}, \\\"[aeiou]\\\", \\\"*\\\"), \\\"; \\\",\\n  \\\"ENCODE_URL_COMPONENT: \\\", ENCODE_URL_COMPONENT({fldHJuw6pAujnHvkP})\\n)\",\"referencedFieldIds\":[\"fldHJuw6pAujnHvkP\",\"fldA2boNwwsiuvXw1\",\"fldfruPf8V9K6qIAN\",\"fldA04pqfjMkGXcZU\"],\"result\":{\"type\":\"singleLineText\"}},\"id\":\"flddvzeqt7FJpQ9NX\",\"name\":\"Text Formula\",\"description\":\"Demonstrates all string-related Airtable functions using First Text, Second Text, and Third Text fields.\"},{\"type\":\"dateTime\",\"options\":{\"dateFormat\":{\"name\":\"local\",\"format\":\"l\"},\"timeFormat\":{\"name\":\"24hour\",\"format\":\"HH:mm\"},\"timeZone\":\"client\"},\"id\":\"fldxSQRRn8W879aiU\",\"name\":\"Third Date\"},{\"type\":\"number\",\"options\":{\"precision\":1},\"id\":\"fld5NBdekrAUzu4Fi\",\"name\":\"Third Number\"},{\"type\":\"singleLineText\",\"id\":\"fldfruPf8V9K6qIAN\",\"name\":\"Third Text\"}],\"views\":[{\"id\":\"viw7gdr4uJSpnHjR7\",\"name\":\"Grid view\",\"type\":\"grid\"}]},{\"id\":\"tblmb3iqgpNS1ysV2\",\"name\":\"Primary\",\"primaryFieldId\":\"fldol5Q4wmQJQvPRy\",\"fields\":[{\"type\":\"multipleAttachments\",\"options\":{\"isReversed\":false},\"id\":\"fldhF2AEuSC1haCZd\",\"name\":\"Attachment\"},{\"type\":\"autoNumber\",\"id\":\"fldizvTkxgIn0mC3L\",\"name\":\"Auto Number\"},{\"type\":\"button\",\"id\":\"fldY48yKPG16AajtU\",\"name\":\"Button\"},{\"type\":\"checkbox\",\"options\":{\"icon\":\"check\",\"color\":\"greenBright\"},\"id\":\"fldjQIaAZVegb1FUa\",\"name\":\"Checkbox\"},{\"type\":\"createdBy\",\"id\":\"fldGLQhDz2UjjiHG6\",\"name\":\"Created By\"},{\"type\":\"createdTime\",\"options\":{\"result\":{\"type\":\"dateTime\",\"options\":{\"dateFormat\":{\"name\":\"local\",\"format\":\"l\"},\"timeFormat\":{\"name\":\"12hour\",\"format\":\"h:mma\"},\"timeZone\":\"client\"}}},\"id\":\"fld2YgW382Kt9xltA\",\"name\":\"Created Time\"},{\"type\":\"currency\",\"options\":{\"precision\":2,\"symbol\":\"$\"},\"id\":\"fldyh8pzDXiy5abEr\",\"name\":\"Currency (float)\"},{\"type\":\"currency\",\"options\":{\"precision\":0,\"symbol\":\"$\"},\"id\":\"fldBfo74z9hD78hP8\",\"name\":\"Currency (int)\"},{\"type\":\"date\",\"options\":{\"dateFormat\":{\"name\":\"local\",\"format\":\"l\"}},\"id\":\"fldC6LfNVvVIxKyQH\",\"name\":\"Date\"},{\"type\":\"dateTime\",\"options\":{\"dateFormat\":{\"name\":\"local\",\"format\":\"l\"},\"timeFormat\":{\"name\":\"12hour\",\"format\":\"h:mma\"},\"timeZone\":\"client\"},\"id\":\"fldizYmjpXABGDLTG\",\"name\":\"Date (with time)\"},{\"type\":\"duration\",\"options\":{\"durationFormat\":\"h:mm\"},\"id\":\"fldLTyf6ljS0rhur8\",\"name\":\"Duration\"},{\"type\":\"email\",\"id\":\"fldHCJoYBiFVsNvP4\",\"name\":\"Email\"},{\"type\":\"formula\",\"options\":{\"isValid\":true,\"formula\":\"CONCATENATE(\\n\\\"Primary Key: \\\", {fldol5Q4wmQJQvPRy}, \\\"\\\\n\\\",\\n\\\"Single Line Text: \\\", {fld0BL2lFo9fqcKv3}, \\\"\\\\n\\\",\\n\\\"Long Text: \\\", {fld8ulc6J0W29M6La}, \\\"\\\\n\\\",\\n\\\"Long Text with Rich Text: \\\", {fldHJkxCMC0xo343u}, \\\"\\\\n\\\",\\n\\\"Attachment: \\\", IF({fldhF2AEuSC1haCZd}, {fldhF2AEuSC1haCZd}, \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Checkbox: \\\", IF({fldjQIaAZVegb1FUa}, \\\"Checked\\\", \\\"Unchecked\\\"), \\\"\\\\n\\\",\\n\\\"Multiple Select: \\\", IF({fld6GTabFmu1xKPvZ}, {fld6GTabFmu1xKPvZ}, \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Single Select: \\\", IF({fldn0GFFtMFpCXUNU}, {fldn0GFFtMFpCXUNU}, \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"User: \\\", IF({fldU6SbLp8CSkLcA4}, {fldU6SbLp8CSkLcA4}, \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"User (allow multiple): \\\", IF({fldBwCDbAVxRj9yg7}, {fldBwCDbAVxRj9yg7}, \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Date: \\\", IF({fldC6LfNVvVIxKyQH}, DATETIME_FORMAT({fldC6LfNVvVIxKyQH}, 'YYYY-MM-DD'), \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Date (with time): \\\", IF({fldizYmjpXABGDLTG}, DATETIME_FORMAT({fldizYmjpXABGDLTG}, 'YYYY-MM-DD HH:mm'), \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Phone Number: \\\", IF({fld38tnNpHmoks8C8}, {fld38tnNpHmoks8C8}, \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Email: \\\", IF({fldHCJoYBiFVsNvP4}, {fldHCJoYBiFVsNvP4}, \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"URL: \\\", IF({fldLYloz2oP4ymf3B}, {fldLYloz2oP4ymf3B}, \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Number (int): \\\", IF({fldOfPKGmnRPv94QH}, {fldOfPKGmnRPv94QH} & \\\"\\\", \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Number (float): \\\", IF({fldmU0X2l4RWd21dd}, {fldmU0X2l4RWd21dd} & \\\"\\\", \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Currency (int): \\\", IF({fldBfo74z9hD78hP8}, {fldBfo74z9hD78hP8} & \\\"\\\", \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Currency (float): \\\", IF({fldyh8pzDXiy5abEr}, {fldyh8pzDXiy5abEr} & \\\"\\\", \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Percent (int): \\\", IF({fldbAAyWboGulpb4s}, {fldbAAyWboGulpb4s} & \\\"\\\", \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Percent (float): \\\", IF({fldiGui9ll69N7WOj}, {fldiGui9ll69N7WOj} & \\\"\\\", \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Duration: \\\", IF({fldLTyf6ljS0rhur8}, {fldLTyf6ljS0rhur8} & \\\"\\\", \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Rating: \\\", IF({fldRsmwFwQNZkKLp4}, {fldRsmwFwQNZkKLp4} & \\\"\\\", \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Created Time: \\\", IF({fld2YgW382Kt9xltA}, DATETIME_FORMAT({fld2YgW382Kt9xltA}, 'YYYY-MM-DD HH:mm'), \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Last Modified Time: \\\", IF({fldMinKh4pa3YX86g}, DATETIME_FORMAT({fldMinKh4pa3YX86g}, 'YYYY-MM-DD HH:mm'), \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Created By: \\\", IF({fldGLQhDz2UjjiHG6}, {fldGLQhDz2UjjiHG6}, \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Last Modified By: \\\", IF({fldF8iDttqP0AgzWC}, {fldF8iDttqP0AgzWC}, \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Auto Number: \\\", IF({fldizvTkxgIn0mC3L}, {fldizvTkxgIn0mC3L} & \\\"\\\", \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Button: \\\", IF({fldY48yKPG16AajtU}, {fldY48yKPG16AajtU}, \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Link (single): \\\", IF({fld7F5onkDo6mkmbN}, {fld7F5onkDo6mkmbN}, \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Link (multiple): \\\", IF({fldFyFheQWczd8oux}, {fldFyFheQWczd8oux}, \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Lookup: \\\", IF({fldbmFmrzYKBktJvE}, {fldbmFmrzYKBktJvE}, \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Rollup: \\\", IF({fldGaFgBsDC3IBUdV}, {fldGaFgBsDC3IBUdV}, \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Formula (ID): \\\", IF({fldcf62YFeIIDHElt}, {fldcf62YFeIIDHElt}, \\\"None\\\"), \\\"\\\\n\\\",\\n\\\"Formula (Simple): \\\", IF({fldy1axxaoUToLVC6}, {fldy1axxaoUToLVC6}, \\\"None\\\")\\n)\",\"referencedFieldIds\":[\"fldol5Q4wmQJQvPRy\",\"fld0BL2lFo9fqcKv3\",\"fld8ulc6J0W29M6La\",\"fldHJkxCMC0xo343u\",\"fldhF2AEuSC1haCZd\",\"fldjQIaAZVegb1FUa\",\"fld6GTabFmu1xKPvZ\",\"fldn0GFFtMFpCXUNU\",\"fldU6SbLp8CSkLcA4\",\"fldBwCDbAVxRj9yg7\",\"fldC6LfNVvVIxKyQH\",\"fldizYmjpXABGDLTG\",\"fld38tnNpHmoks8C8\",\"fldHCJoYBiFVsNvP4\",\"fldLYloz2oP4ymf3B\",\"fldOfPKGmnRPv94QH\",\"fldmU0X2l4RWd21dd\",\"fldBfo74z9hD78hP8\",\"fldyh8pzDXiy5abEr\",\"fldbAAyWboGulpb4s\",\"fldiGui9ll69N7WOj\",\"fldLTyf6ljS0rhur8\",\"fldRsmwFwQNZkKLp4\",\"fld2YgW382Kt9xltA\",\"fldMinKh4pa3YX86g\",\"fldGLQhDz2UjjiHG6\",\"fldF8iDttqP0AgzWC\",\"fldizvTkxgIn0mC3L\",\"fldY48yKPG16AajtU\",\"fld7F5onkDo6mkmbN\",\"fldFyFheQWczd8oux\",\"fldbmFmrzYKBktJvE\",\"fldGaFgBsDC3IBUdV\",\"fldcf62YFeIIDHElt\",\"fldy1axxaoUToLVC6\"],\"result\":{\"type\":\"singleLineText\"}},\"id\":\"fld2vnFc0Bl5IOFUQ\",\"name\":\"Formula (Complex)\",\"description\":\"Concatenates all field values into a readable summary, using correct field references.\"},{\"type\":\"formula\",\"options\":{\"isValid\":true,\"formula\":\"RECORD_ID()\",\"referencedFieldIds\":[],\"result\":{\"type\":\"singleLineText\"}},\"id\":\"fldcf62YFeIIDHElt\",\"name\":\"Formula (ID)\"},{\"type\":\"formula\",\"options\":{\"isValid\":true,\"formula\":\"{fldcf62YFeIIDHElt} & {fldy1axxaoUToLVC6} & {fld2vnFc0Bl5IOFUQ}\",\"referencedFieldIds\":[\"fldcf62YFeIIDHElt\",\"fldy1axxaoUToLVC6\",\"fld2vnFc0Bl5IOFUQ\"],\"result\":{\"type\":\"singleLineText\"}},\"id\":\"fldXFeHRPBLz6AiWh\",\"name\":\"Formula (Nested)\"},{\"type\":\"formula\",\"options\":{\"isValid\":true,\"formula\":\"{fldOfPKGmnRPv94QH} + {fldmU0X2l4RWd21dd}\",\"referencedFieldIds\":[\"fldOfPKGmnRPv94QH\",\"fldmU0X2l4RWd21dd\"],\"result\":{\"type\":\"number\",\"options\":{\"precision\":0}}},\"id\":\"fldy1axxaoUToLVC6\",\"name\":\"Formula (Simple)\"},{\"type\":\"lastModifiedBy\",\"id\":\"fldF8iDttqP0AgzWC\",\"name\":\"Last Modified By\"},{\"type\":\"lastModifiedTime\",\"options\":{\"isValid\":true,\"referencedFieldIds\":[],\"result\":{\"type\":\"dateTime\",\"options\":{\"dateFormat\":{\"name\":\"local\",\"format\":\"l\"},\"timeFormat\":{\"name\":\"12hour\",\"format\":\"h:mma\"},\"timeZone\":\"client\"}}},\"id\":\"fldMinKh4pa3YX86g\",\"name\":\"Last Modified Time\"},{\"type\":\"multipleRecordLinks\",\"options\":{\"linkedTableId\":\"tblPPScS3XMuFkDYN\",\"isReversed\":false,\"prefersSingleRecordLink\":false,\"inverseLinkFieldId\":\"fldgoE2oZmXmKkQca\"},\"id\":\"fldFyFheQWczd8oux\",\"name\":\"Link (multiple)\"},{\"type\":\"multipleRecordLinks\",\"options\":{\"linkedTableId\":\"tblPPScS3XMuFkDYN\",\"isReversed\":false,\"prefersSingleRecordLink\":true,\"inverseLinkFieldId\":\"fldl0nB9WRFSdqlii\"},\"id\":\"fld7F5onkDo6mkmbN\",\"name\":\"Link (single)\"},{\"type\":\"multilineText\",\"id\":\"fld8ulc6J0W29M6La\",\"name\":\"Long Text\"},{\"type\":\"multilineText\",\"id\":\"fldHJkxCMC0xo343u\",\"name\":\"Long Text with Rich Text\"},{\"type\":\"multipleLookupValues\",\"options\":{\"isValid\":true,\"recordLinkFieldId\":\"fld7F5onkDo6mkmbN\",\"fieldIdInLinkedTable\":\"fldi6Mxh5H1gPGxFX\",\"result\":{\"type\":\"singleLineText\"}},\"id\":\"fldbmFmrzYKBktJvE\",\"name\":\"Lookup\"},{\"type\":\"multipleSelects\",\"options\":{\"choices\":[{\"id\":\"selKcXTyNnNHd3nSK\",\"name\":\"Option 1\",\"color\":\"blueLight2\"},{\"id\":\"selBUFMeKPuDUvn0a\",\"name\":\"Option 2\",\"color\":\"cyanLight2\"},{\"id\":\"selmD0luyDWhS6AyO\",\"name\":\"Option 3\",\"color\":\"tealLight2\"}]},\"id\":\"fld6GTabFmu1xKPvZ\",\"name\":\"Multiple Select\"},{\"type\":\"number\",\"options\":{\"precision\":2},\"id\":\"fldmU0X2l4RWd21dd\",\"name\":\"Number (float)\"},{\"type\":\"number\",\"options\":{\"precision\":0},\"id\":\"fldOfPKGmnRPv94QH\",\"name\":\"Number (int)\"},{\"type\":\"percent\",\"options\":{\"precision\":2},\"id\":\"fldiGui9ll69N7WOj\",\"name\":\"Percent (float)\"},{\"type\":\"percent\",\"options\":{\"precision\":0},\"id\":\"fldbAAyWboGulpb4s\",\"name\":\"Percent (int)\"},{\"type\":\"phoneNumber\",\"id\":\"fld38tnNpHmoks8C8\",\"name\":\"Phone Number\"},{\"type\":\"singleLineText\",\"id\":\"fldol5Q4wmQJQvPRy\",\"name\":\"Primary Key\"},{\"type\":\"rating\",\"options\":{\"icon\":\"star\",\"max\":5,\"color\":\"yellowBright\"},\"id\":\"fldRsmwFwQNZkKLp4\",\"name\":\"Rating\"},{\"type\":\"rollup\",\"options\":{\"isValid\":true,\"recordLinkFieldId\":\"fld7F5onkDo6mkmbN\",\"fieldIdInLinkedTable\":\"fldi6Mxh5H1gPGxFX\",\"referencedFieldIds\":[],\"result\":{\"type\":\"singleLineText\"}},\"id\":\"fldGaFgBsDC3IBUdV\",\"name\":\"Rollup\"},{\"type\":\"singleLineText\",\"id\":\"fld0BL2lFo9fqcKv3\",\"name\":\"Single Line Text\"},{\"type\":\"singleSelect\",\"options\":{\"choices\":[{\"id\":\"sellmCNG85TCgc7Pu\",\"name\":\"Choice 1\",\"color\":\"blueLight2\"},{\"id\":\"sel4oPckj6NHWKn2u\",\"name\":\"Choice 2\",\"color\":\"cyanLight2\"},{\"id\":\"selYfbfAsr7X75S2V\",\"name\":\"Choice 3\",\"color\":\"tealLight2\"}]},\"id\":\"fldn0GFFtMFpCXUNU\",\"name\":\"Single Select\"},{\"type\":\"url\",\"id\":\"fldLYloz2oP4ymf3B\",\"name\":\"URL\"},{\"type\":\"singleCollaborator\",\"id\":\"fldU6SbLp8CSkLcA4\",\"name\":\"User\"},{\"type\":\"multipleCollaborators\",\"id\":\"fldBwCDbAVxRj9yg7\",\"name\":\"User (allow multiple)\"}],\"views\":[{\"id\":\"viwvPRDMaHyldUpmd\",\"name\":\"Grid view\",\"type\":\"grid\"},{\"id\":\"viwHlcwGu4xthX1gf\",\"name\":\"Filter by View\",\"type\":\"grid\"}]},{\"id\":\"tblPPScS3XMuFkDYN\",\"name\":\"Secondary\",\"primaryFieldId\":\"fld1RagdJ09mpWhzM\",\"fields\":[{\"type\":\"multipleRecordLinks\",\"options\":{\"linkedTableId\":\"tblLFoLxEdWlxjmLP\",\"isReversed\":false,\"prefersSingleRecordLink\":false,\"inverseLinkFieldId\":\"fld8lCuUXpEXkIeYv\"},\"id\":\"fldKR6tdbnOBRCtdQ\",\"name\":\"Link to Tertiary\"},{\"type\":\"singleLineText\",\"id\":\"fld1RagdJ09mpWhzM\",\"name\":\"Name\"},{\"type\":\"multipleRecordLinks\",\"options\":{\"linkedTableId\":\"tblmb3iqgpNS1ysV2\",\"isReversed\":false,\"prefersSingleRecordLink\":false,\"inverseLinkFieldId\":\"fld7F5onkDo6mkmbN\"},\"id\":\"fldl0nB9WRFSdqlii\",\"name\":\"Primary\"},{\"type\":\"multipleRecordLinks\",\"options\":{\"linkedTableId\":\"tblmb3iqgpNS1ysV2\",\"isReversed\":false,\"prefersSingleRecordLink\":false,\"inverseLinkFieldId\":\"fldFyFheQWczd8oux\"},\"id\":\"fldgoE2oZmXmKkQca\",\"name\":\"Primary 2\"},{\"type\":\"singleLineText\",\"id\":\"fldi6Mxh5H1gPGxFX\",\"name\":\"Value\"}],\"views\":[{\"id\":\"viwTml4ZHkNi8kJbD\",\"name\":\"Grid view\",\"type\":\"grid\"}]},{\"id\":\"tblLFoLxEdWlxjmLP\",\"name\":\"Tertiary\",\"primaryFieldId\":\"fldwzqKxsRnPZJ2Ll\",\"fields\":[{\"type\":\"singleLineText\",\"id\":\"fldwzqKxsRnPZJ2Ll\",\"name\":\"Name\"},{\"type\":\"multipleRecordLinks\",\"options\":{\"linkedTableId\":\"tblPPScS3XMuFkDYN\",\"isReversed\":false,\"prefersSingleRecordLink\":false,\"inverseLinkFieldId\":\"fldKR6tdbnOBRCtdQ\"},\"id\":\"fld8lCuUXpEXkIeYv\",\"name\":\"Secondary\"},{\"type\":\"singleLineText\",\"id\":\"fldjNLBh2UccM64h5\",\"name\":\"Value\"}],\"views\":[{\"id\":\"viwdp3tOB8ooOCvP4\",\"name\":\"Grid view\",\"type\":\"grid\"}]}]}").unwrap()
    }

    /// Fetch a live version of the schema from Airtable's metadata API.
    pub async fn get_schema(&self) -> Result<serde_json::Value, AirtableError> {
        self.client.get_schema().await
    }
}
