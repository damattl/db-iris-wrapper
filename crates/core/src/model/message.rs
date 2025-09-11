use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: String,
    pub train_id: String,
    pub valid_from: Option<NaiveDateTime>,
    pub valid_to: Option<NaiveDateTime>,
    pub priority: Option<i16>,
    pub category: Option<String>,
    pub code: Option<i32>,
    pub timestamp: NaiveDateTime,
    pub m_type: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum MessageBuildError {
    #[error("missing id")]
    MissingId,
    #[error("missing train number")]
    MissingNumber,
    #[error("missing timestamp")]
    MissingTimestamp,
}

impl Message {
    pub fn from_iris_msg(msg: &iris::dto::Msg, train_id: &str) -> Result<Message, MessageBuildError> {
        Ok(Message {
            id: msg.id.as_ref().ok_or(MessageBuildError::MissingId)?.clone(),
            train_id: train_id.to_string(),
            valid_from: msg.from,
            valid_to: msg.to,
            priority: msg.pr.map(|p| p as i16), // TODO: Test overflows
            category: msg.cat.clone(),
            code: msg.code,
            timestamp: msg.ts.ok_or(MessageBuildError::MissingTimestamp)?,
            m_type: msg.kind.clone(),
        })
    }
}
