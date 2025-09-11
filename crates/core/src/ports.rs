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
}

pub trait TrainPort<'a>: Port<Train, &'a str> + Send + Sync {
    fn get_by_station_and_date(&self, station: &Station, date: &NaiveDate) -> Result<Vec<Train>, Box<dyn std::error::Error>>;
}


pub trait StopPort<'a>: Port<Stop, &'a str> + Send + Sync {}


pub trait MessagePort<'a>: Port<Message, &'a str> + Send + Sync {}
