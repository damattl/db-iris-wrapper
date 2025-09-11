use chrono::NaiveDate;

use crate::model::{message::Message, station::Station, stop::Stop, train::Train};

pub trait Port<T, ID> {
    fn persist(&self, value: &T) -> Result<T, Box<dyn std::error::Error>>;
    fn persist_all(&self, values: &[T]) -> Result<Vec<T>, Box<dyn std::error::Error>>;
    fn get_by_id(&self, id: ID) -> Result<T, Box<dyn std::error::Error>>;
    fn get_all(&self) -> Result<Vec<T>, Box<dyn std::error::Error>>;
}

pub trait StationPort: Port<Station, i32> + Send + Sync {
    fn get_by_ds100(&self, ds100: &str) -> Result<Station, Box<dyn std::error::Error>>;
    fn from_sql(&self, path: &str) -> Result<Vec<Station>, Box<dyn std::error::Error>>;
}

pub trait TrainPort: Port<Train, String> + Send + Sync {
    fn get_by_station_and_date(&self, station: &Station, date: &NaiveDate) -> Result<Vec<Train>, Box<dyn std::error::Error>>;
    fn get_by_date(&self, date: &NaiveDate) -> Result<Vec<Train>, Box<dyn std::error::Error>>;
}


pub trait StopPort: Port<Stop, String> + Send + Sync {
    fn get_for_date(&self, date: &NaiveDate) -> Result<Vec<Stop>, Box<dyn std::error::Error>>;
}


pub trait MessagePort: Port<Message, String> + Send + Sync {
    fn get_by_date_and_code(&self, date: &NaiveDate, code: i32) -> Result<Vec<Message>, Box<dyn std::error::Error>>;
    fn get_by_train_id(&self, train_id: &str) -> Result<Vec<Message>, Box<dyn std::error::Error>>;
}
