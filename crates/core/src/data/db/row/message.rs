use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{prelude::{Identifiable, Insertable, Queryable}, Selectable};


use crate::{data::db::row::MessageToStationRow, model::Message};

#[derive(Debug, Clone)]
#[derive(Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = crate::data::db::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MessageRow {
    pub id: String,
    pub iris_id: String,
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
            iris_id: msg.iris_id.clone(),
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

impl MessageRow {
    pub fn to_message(&self, stations: &[MessageToStationRow]) -> Message {
        Message {
            id: self.id.clone(),
            iris_id: self.iris_id.clone(),
            train_id: self.train_id.clone(),
            valid_from: self.valid_from,
            valid_to: self.valid_to,
            priority: self.priority,
            category: self.category.clone(),
            code: self.code,
            timestamp: self.timestamp,
            m_type: self.m_type.clone(),
            last_updated: self.last_updated,
            stations: stations.iter().map(|s| s.station_id).collect::<Vec<i32>>()
        }
    }
}
