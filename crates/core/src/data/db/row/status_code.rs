use diesel::*;
use crate::model::{StatusCode, StatusCodeType};

#[derive(Debug, Clone)]
#[derive(Queryable, QueryableByName, Selectable, Insertable)]
#[diesel(table_name = crate::data::db::schema::status_codes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StatusCodeRow {
    pub code: i16,
    pub c_type: Option<String>,
    pub long_text: String,
}

impl From<&StatusCode> for StatusCodeRow {
    fn from(status_code: &StatusCode) -> Self {
        StatusCodeRow {
            code: status_code.code,
            c_type: status_code.c_type.as_ref().map(|c| c.as_string()),
            long_text: status_code.long_text.clone(),
        }
    }
}

impl From<&StatusCodeRow> for StatusCode {
    fn from(status_code_row: &StatusCodeRow) -> Self {
        StatusCode {
            code: status_code_row.code,
            c_type: status_code_row.c_type.as_deref().map(StatusCodeType::from).clone(),
            long_text: status_code_row.long_text.clone(),
        }
    }
}

impl From<StatusCode> for StatusCodeRow {
    fn from(status_code: StatusCode) -> Self {
        StatusCodeRow {
            code: status_code.code,
            c_type: status_code.c_type.as_ref().map(|c| c.as_string()),
            long_text: status_code.long_text.clone(),
        }
    }
}

impl From<StatusCodeRow> for StatusCode {
    fn from(status_code_row: StatusCodeRow) -> Self {
        StatusCode {
            code: status_code_row.code,
            c_type: status_code_row.c_type.as_deref().map(StatusCodeType::from).clone(),
            long_text: status_code_row.long_text.clone(),
        }
    }
}
