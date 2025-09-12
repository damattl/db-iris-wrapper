use chrono::NaiveDateTime;

use super::train::Train;
use super::station::Station;

#[derive(Debug, Clone)]
pub struct Stop {
    pub id: String,
    pub train_id: String,
    pub station_id: i32,

    pub arrival: Option<Movement>,
    pub departure: Option<Movement>,
}

impl Stop {
    pub fn from_iris_stop(stop: &iris::dto::Stop, train: &Train, station: &Station) -> Self {
        Stop {
            id: stop.id.clone(),
            train_id: train.id.clone(),
            station_id: station.id,
            arrival: stop.arrival.as_ref().map(Movement::from_iris_movement),
            departure: stop.departure.as_ref().map(Movement::from_iris_movement),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Movement {
    pub platform: Option<String>,
    pub planned: Option<NaiveDateTime>,
    pub planned_path: Option<Vec<String>>,
    pub changed_path: Option<Vec<String>>,
}

impl Movement {
    pub fn from_iris_movement(movement: &iris::dto::Movement) -> Self {
        Movement {
            platform: movement.platform.clone(),
            planned: movement.planned,
            planned_path: movement.ppth.clone(),
            changed_path: movement.cpth.clone(),
        }
    }
}
