use chrono::NaiveDate;

use crate::model::{Message, Station, StatusCode, Stop, StopUpdate, StopWithStation, Train};

#[derive(thiserror::Error, Debug)]
pub enum PortError {
    #[error("Not found")]
    NotFound,
    // #[error("conflict")]
    // Conflict,
    #[error("Invalid input")]
    InvalidInput,
    #[error("Connection error")]
    MalformedData,
    #[error("Malformed data")]
    Connection,
    #[error("Database error")]
    Database,
    #[error(transparent)]
    Custom(#[from] Box<dyn std::error::Error>),
}

pub trait Port<T, ID> {
    fn persist(&self, value: &T) -> Result<T, PortError>;
    fn persist_all(&self, values: &[T]) -> Result<Vec<T>, PortError>;
    fn get_by_id(&self, id: ID) -> Result<T, PortError>;
    fn get_all(&self) -> Result<Vec<T>, PortError>;
}

pub trait StationPort: Port<Station, i32> + Send + Sync {
    fn get_by_ds100(&self, ds100: &str) -> Result<Station, PortError>;
    fn import_from_sql(&self, path: &str) -> Result<Vec<Station>, PortError>;
}

pub trait TrainPort: Port<Train, String> + Send + Sync {
    fn get_by_station_and_date(&self, station: &Station, date: &NaiveDate) -> Result<Vec<Train>, PortError>;
    fn get_by_date(&self, date: &NaiveDate) -> Result<Vec<Train>, PortError>;
}


pub trait StopPort: Port<Stop, String> + Send + Sync {
    fn get_for_date(&self, date: &NaiveDate) -> Result<Vec<Stop>, PortError>;
    fn get_for_train(&self, train_id: &str) -> Result<Vec<Stop>, PortError>;
    fn get_for_train_with_station(&self, train_id: &str) -> Result<Vec<StopWithStation>, PortError>;

    fn get_by_station_and_date(&self, station: &Station, date: &NaiveDate) -> Result<Vec<Stop>, PortError>;

    fn update(&self, update: &StopUpdate) -> Result<Stop, PortError>;
    fn update_many(&self, updates: &[StopUpdate]) -> Result<Vec<Stop>, PortError>;
}


pub trait MessagePort: Port<Message, String> + Send + Sync {
    fn get_by_date_and_code(&self, date: &NaiveDate, code: i32) -> Result<Vec<Message>, PortError>;
    fn get_by_train_id(&self, train_id: &str) -> Result<Vec<Message>, PortError>;
}

pub trait StatusCodePort: Port<StatusCode, i16> + Send + Sync {}
