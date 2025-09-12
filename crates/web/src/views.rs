use chrono::{NaiveDate, NaiveDateTime, Utc};
use chrono_tz::Europe::Berlin;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wrapper_core::model::{message::Message, station::Station, stop::{split_stops_by_time, Movement, Stop}, train::Train};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct TrainView {
    pub id: String,
    pub operator: Option<String>,
    pub category: String,
    pub number: String,
    pub line: Option<String>,
    pub date: NaiveDate,
    pub next_stop: Option<StopView>,
    pub past_stops: Vec<StopView>,
    pub next_stops: Vec<StopView>,
}

impl TrainView {
    pub fn from_model(train: &Train, stops: &[Stop]) -> Self {
        // TODO: Sort stops by time
        let now = Utc::now().with_timezone(&Berlin).naive_local();
        let (next_stop, past_stops, next_stops) = split_stops_by_time(stops, &now, |stop| StopView::from_model(stop, true));

        TrainView {
            id: train.id.clone(),
            operator: train.operator.clone(),
            category: train.category.clone(),
            number: train.number.clone(),
            line: train.line.clone(),
            date: train.date,
            next_stop,
            next_stops,
            past_stops,
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct StopView {
    pub id: String,
    pub train_id: String,
    pub station_id: i32,

    pub arrival: Option<MovementView>,
    pub departure: Option<MovementView>,
}

impl StopView {
    pub fn from_model(stop: &Stop, simple: bool) -> Self {
        let movement_builder = match simple {
            true => MovementView::from_model_simple,
            false => MovementView::from_model,
        };

        StopView {
            id: stop.id.clone(),
            train_id: stop.train_id.clone(),
            station_id: stop.station_id,
            arrival: stop.arrival.as_ref().map(movement_builder),
            departure: stop.departure.as_ref().map(movement_builder),
        }
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MovementView {
    pub platform: Option<String>,
    pub planned: Option<NaiveDateTime>,
    pub planned_path: Option<Vec<String>>,
    pub changed_path: Option<Vec<String>>,
}

impl MovementView {
    pub fn from_model_simple(movement: &Movement) -> Self {
        MovementView {
            platform: movement.platform.clone(),
            planned: movement.planned,
            planned_path: None,
            changed_path: None,
        }
    }
    pub fn from_model(movement: &Movement) -> Self {
        MovementView {
            platform: movement.platform.clone(),
            planned: movement.planned,
            planned_path: movement.planned_path.clone(),
            changed_path: movement.changed_path.clone(),
        }
    }
}


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct StationView {
    pub id: i32,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: String,
    pub ds100: String,
}


impl StationView {
    pub fn from_model(station: &Station) -> Self {
        StationView {
            id: station.id,
            lat: station.lat,
            lon: station.lon,
            name: station.name.clone(),
            ds100: station.ds100.clone(),
        }
    }
}


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MessageView {
    pub id: String,
    pub train_id: String,
    pub train: String,

    pub valid_from: Option<NaiveDateTime>,
    pub valid_to: Option<NaiveDateTime>,
    pub priority: Option<i16>,
    pub category: Option<String>,
    pub code: Option<i32>,
    pub timestamp: NaiveDateTime,
    pub m_type: Option<String>,
}

impl MessageView {
    pub fn from_model(message: &Message, api_base_path: &str) -> Self {
        MessageView {
            id: message.id.clone(),
            train_id: message.train_id.clone(),
            train: format!("{}/trains/{}?include_stops=true", api_base_path, message.train_id),
            valid_from: message.valid_from,
            valid_to: message.valid_to,
            priority: message.priority,
            category: message.category.clone(),
            code: message.code,
            timestamp: message.timestamp,
            m_type: message.m_type.clone(),
        }
    }
}
