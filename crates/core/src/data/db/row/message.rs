use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{prelude::{Insertable, Queryable}, Selectable};


use crate::model::Message;

#[derive(Debug, Clone)]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::data::db::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MessageRow {
    pub id: String,
    pub train_id: String,
    pub valid_from: Option<NaiveDateTime>,
    pub valid_to: Option<NaiveDateTime>,
    pub priority: Option<i16>,
    pub category: Option<String>,
    pub code: Option<i32>,
    pub timestamp: NaiveDateTime,
    pub m_type: Option<String>,
    pub last_updated: Option<DateTime<Utc>>,
}

impl From<Message> for MessageRow {
    fn from(msg: Message) -> Self {
        MessageRow::from(&msg)
    }
}

impl From<&Message> for MessageRow {
    fn from(msg: &Message) -> Self {
        MessageRow {
            id: msg.id.clone(),
            train_id: msg.train_id.clone(),
            valid_from: msg.valid_from,
            valid_to: msg.valid_to,
            priority: msg.priority,
            category: msg.category.clone(),
            code: msg.code,
            timestamp: msg.timestamp,
            m_type: msg.m_type.clone(),
            last_updated: msg.last_updated,
        }
    }
}


impl From<MessageRow> for Message {
    fn from(msg: MessageRow) -> Self {
        Message::from(&msg)
    }
}

impl From<&MessageRow> for Message {
    fn from(msg: &MessageRow) -> Self {
        Message {
            id: msg.id.clone(),
            train_id: msg.train_id.clone(),
            valid_from: msg.valid_from,
            valid_to: msg.valid_to,
            priority: msg.priority,
            category: msg.category.clone(),
            code: msg.code,
            timestamp: msg.timestamp,
            m_type: msg.m_type.clone(),
            last_updated: msg.last_updated,
        }
    }
}
