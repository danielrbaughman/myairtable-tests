// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

use crate::airtable_model::{ModelMeta, OrmModel};
use crate::types::RecordId;
use serde::{Deserialize, Serialize};

/// ORM model for `Formulas`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FormulasModel {
    #[serde(skip)]
    pub id: Option<RecordId>,
    #[serde(skip)]
    pub created_time: Option<String>,
    #[serde(skip)]
    pub _meta: ModelMeta,
    /// Date Formula `fldY7kjaklLeoSgGd` - `Read-Only`
    ///
    /// ```text
    /// "YEAR: " & YEAR({First Date})
    /// & ", MONTH: " & MONTH({First Date})
    /// & ", DAY: " & DAY({First Date})
    /// & ", HOUR: " & HOUR({Second Date})
    /// & ", MINUTE: " & MINUTE({Second Date})
    /// & ", SECOND: " & SECOND({Second Date})
    /// & ", TODAY: " & DATETIME_FORMAT(
    ///   TODAY(),
    ///   'YYYY-MM-DD'
    /// )
    /// & ", WORKDAY+5: " & DATETIME_FORMAT(
    ///   WORKDAY({First Date}, 5),
    ///   'YYYY-MM-DD'
    /// )
    /// & ", WORKDAY_DIFF: " & WORKDAY_DIFF({First Date}, {Second Date})
    /// & ", PARSE: " & DATETIME_FORMAT(
    ///   DATETIME_PARSE(
    ///     DATESTR({First Date})
    ///   ),
    ///   'YYYY-MM-DD'
    /// )
    /// & ", FORMAT: " & DATETIME_FORMAT({Second Date}, 'YYYY-MM-DD HH:mm')
    /// & ", LOCALE: " & DATETIME_FORMAT(
    ///   SET_LOCALE({Second Date}, 'en'),
    ///   'YYYY-MM-DD'
    /// )
    /// & ", TIMEZONE: " & DATETIME_FORMAT(
    ///   SET_TIMEZONE({Second Date}, 'America/New_York'),
    ///   'YYYY-MM-DD HH:mm'
    /// )
    /// & ", DATESTR: " & DATESTR({Second Date})
    /// & ", TIMESTR: " & TIMESTR({Second Date})
    /// & ", TONOW: " & TONOW({First Date})
    /// & ", FROMNOW: " & FROMNOW({Second Date})
    /// & ", DATEADD+2d: " & DATETIME_FORMAT(
    ///   DATEADD({First Date}, 2, 'days'),
    ///   'YYYY-MM-DD'
    /// )
    /// & ", WEEKNUM: " & WEEKNUM({First Date}, 'Monday')
    /// & ", DIFF days: " & DATETIME_DIFF({First Date}, {Second Date}, 'days')
    /// & ", IS_BEFORE: " & IF(
    ///   IS_BEFORE({First Date}, {Second Date}),
    ///   'Yes',
    ///   'No'
    /// )
    /// & ", IS_SAME day: " & IF(
    ///   IS_SAME({First Date}, {Second Date}, 'day'),
    ///   'Yes',
    ///   'No'
    /// )
    /// & ", IS_AFTER: " & IF(
    ///   IS_AFTER({Third Date}, {Second Date}),
    ///   'Yes',
    ///   'No'
    /// )
    /// ```
    #[serde(rename = "fldY7kjaklLeoSgGd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_formula: Option<String>,
    /// First Date `fldlZT521Iy0FFXFL`
    #[serde(rename = "fldlZT521Iy0FFXFL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_date: Option<String>,
    /// First Number `fldA04pqfjMkGXcZU`
    #[serde(rename = "fldA04pqfjMkGXcZU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_number: Option<f64>,
    /// First Text `fldHJuw6pAujnHvkP`
    #[serde(rename = "fldHJuw6pAujnHvkP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_text: Option<String>,
    /// Math Formula `fldlSuvoeWGokSz8Z` - `Read-Only`
    ///
    /// ```text
    /// IF(
    ///   OR(
    ///     {First Number} = BLANK(),
    ///     {Second Number} = BLANK()
    ///   ),
    ///   BLANK(),
    ///   "SUM: " & SUM({First Number}, {Second Number})
    ///   & ", MIN: " & MIN({First Number}, {Second Number})
    ///   & ", MAX: " & MAX({First Number}, {Second Number})
    ///   & ", AVG: " & AVERAGE({First Number}, {Second Number})
    ///   & ", COUNT: " & COUNT({First Number}, {Second Number})
    ///   & ", CEIL: " & CEILING({First Number})
    ///   & ", FLOOR: " & FLOOR({Second Number})
    ///   & ", ROUND: " & ROUND({First Number}/2, 1)
    ///   & ", ROUNDUP: " & ROUNDUP({Second Number}/2, 0)
    ///   & ", ROUNDDOWN: " & ROUNDDOWN({First Number}/2, 0)
    ///   & ", INT: " & INT({Second Number}/2)
    ///   & ", EVEN: " & EVEN({First Number})
    ///   & ", ODD: " & ODD({Second Number})
    ///   & ", MOD: " & MOD({First Number}, {Second Number})
    ///   & ", LOG: " & LOG({First Number})
    ///   & ", EXP: " & EXP(1)
    ///   & ", POWER: " & POWER({First Number}, 2)
    ///   & ", SQRT: " & SQRT(
    ///     ABS({Second Number})
    ///   ) & ", ABS: " & ABS({Second Number})
    /// )
    /// ```
    #[serde(rename = "fldlSuvoeWGokSz8Z")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub math_formula: Option<String>,
    /// Primary Key `fldLZFrZKvSCS4dKb` - `Primary Key`
    #[serde(rename = "fldLZFrZKvSCS4dKb")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    /// Second Date `fld1LZ3Ebpt0LaMmu`
    #[serde(rename = "fld1LZ3Ebpt0LaMmu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_date: Option<String>,
    /// Second Number `fldj5nAkal5y8OOZg`
    #[serde(rename = "fldj5nAkal5y8OOZg")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_number: Option<f64>,
    /// Second Text `fldA2boNwwsiuvXw1`
    #[serde(rename = "fldA2boNwwsiuvXw1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_text: Option<String>,
    /// Text Formula `flddvzeqt7FJpQ9NX` - `Read-Only`
    ///
    /// ```text
    /// CONCATENATE(
    ///   "LEN: ",
    ///   LEN({First Text}),
    ///   "; ",
    ///   "MID: ",
    ///   MID({First Text}, 2, 3),
    ///   "; ",
    ///   "LEFT: ",
    ///   LEFT({Second Text}, 2),
    ///   "; ",
    ///   "RIGHT: ",
    ///   RIGHT({Second Text}, 2),
    ///   "; ",
    ///   "FIND: ",
    ///   FIND("e", {First Text}),
    ///   "; ",
    ///   "SEARCH: ",
    ///   SEARCH("e", {First Text}),
    ///   "; ",
    ///   "REPLACE: ",
    ///   REPLACE({First Text}, 2, 2, "XX"),
    ///   "; ",
    ///   "REPT: ",
    ///   REPT({First Text}, 2),
    ///   "; ",
    ///   "LOWER: ",
    ///   LOWER({Second Text}),
    ///   "; ",
    ///   "UPPER: ",
    ///   UPPER({Second Text}),
    ///   "; ",
    ///   "TRIM: ",
    ///   TRIM("   " & {First Text} & "   "),
    ///   "; ",
    ///   "SUBSTITUTE: ",
    ///   SUBSTITUTE({First Text}, "e", "@"),
    ///   "; ",
    ///   "CONCATENATE: ",
    ///   CONCATENATE(
    ///     {First Text},
    ///     "-",
    ///     {Second Text},
    ///     "-",
    ///     {Third Text}
    ///   ),
    ///   "; ",
    ///   "T: ",
    ///   T({First Number}),
    ///   "; ",
    ///   "REGEX_EXTRACT: ",
    ///   REGEX_EXTRACT({First Text}, "[aeiou]"),
    ///   "; ",
    ///   "REGEX_MATCH: ",
    ///   IF(
    ///     REGEX_MATCH({First Text}, "^.e"),
    ///     "1",
    ///     "0"
    ///   ),
    ///   "; ",
    ///   "REGEX_REPLACE: ",
    ///   REGEX_REPLACE({First Text}, "[aeiou]", "*"),
    ///   "; ",
    ///   "ENCODE_URL_COMPONENT: ",
    ///   ENCODE_URL_COMPONENT({First Text})
    /// )
    /// ```
    #[serde(rename = "flddvzeqt7FJpQ9NX")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_formula: Option<String>,
    /// Third Date `fldxSQRRn8W879aiU`
    #[serde(rename = "fldxSQRRn8W879aiU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_date: Option<String>,
    /// Third Number `fld5NBdekrAUzu4Fi`
    #[serde(rename = "fld5NBdekrAUzu4Fi")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_number: Option<f64>,
    /// Third Text `fldfruPf8V9K6qIAN`
    #[serde(rename = "fldfruPf8V9K6qIAN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_text: Option<String>,
}

impl FormulasModel {
    /// Formula builder for this table.
    pub const F: crate::formulas::FormulasFormulas = crate::formulas::FormulasFormulas::new();
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
        crate::types::build_url(base_id, "tblnuYBsMdXNDsuRc", view_id.as_ref(), record_id)
    }
}

impl OrmModel for FormulasModel {
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

/// Writable fields for creating/updating `Formulas` records.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateFormulasModel {
    #[serde(rename = "fldlZT521Iy0FFXFL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_date: Option<String>,
    #[serde(rename = "fldA04pqfjMkGXcZU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_number: Option<f64>,
    #[serde(rename = "fldHJuw6pAujnHvkP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_text: Option<String>,
    #[serde(rename = "fldLZFrZKvSCS4dKb")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[serde(rename = "fld1LZ3Ebpt0LaMmu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_date: Option<String>,
    #[serde(rename = "fldj5nAkal5y8OOZg")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_number: Option<f64>,
    #[serde(rename = "fldA2boNwwsiuvXw1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_text: Option<String>,
    #[serde(rename = "fldxSQRRn8W879aiU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_date: Option<String>,
    #[serde(rename = "fld5NBdekrAUzu4Fi")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_number: Option<f64>,
    #[serde(rename = "fldfruPf8V9K6qIAN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_text: Option<String>,
}
