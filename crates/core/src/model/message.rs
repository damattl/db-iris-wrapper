use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Debug, Clone)]
pub struct Message {
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
    pub stations: Vec<i32>,
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
    pub fn from_iris_msg(msg: &iris::dto::Msg, train_id: &str, station_id: i32) -> Result<Message, MessageBuildError> {
        let iris_id = msg.id.as_ref().ok_or(MessageBuildError::MissingId)?.clone();
        let ts = msg.ts.ok_or(MessageBuildError::MissingTimestamp)?;

        let id = format!("{}-{}", iris_id, ts.format("%Y%m%d")); // In case there are duplicates over time

        Ok(Message {
            id,
            iris_id,
            train_id: train_id.to_string(),
            valid_from: msg.from,
            valid_to: msg.to,
            priority: msg.pr.map(|p| p as i16), // TODO: Test overflows
            category: msg.cat.clone(),
            code: msg.code,
            timestamp: ts,
            m_type: msg.kind.clone(),
            last_updated: Some(Utc::now()),
            stations: vec![station_id]
            // It probably makes sense to update the last_updated timestamp when a message is imported from Iris.
        })
    }
}



#[derive(Debug, Clone)]
pub struct StopInfo {
    pub id: String,
    pub station_name: String,
}
