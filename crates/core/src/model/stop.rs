use chrono::{NaiveDateTime};

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

pub fn split_stops_by_time<M>(stops: &[Stop], now: &NaiveDateTime, mapper: fn(stop: &Stop) -> M) -> (Option<M>, Vec<M>, Vec<M>) {
    let mut earliest = *now;
    let mut next_stop: Option<M> = None;

    let mut next_stops = Vec::<M>::new();
    let mut past_stops = Vec::<M>::new();


    // TODO: Update when current (belated) time is available
    // TODO: Maybe move to dedicated function
    // TODO: Test this logic!
    for stop in stops {
        let planned_arrival = stop.arrival.as_ref().and_then(|a| a.planned);
        let planned_departure = stop.departure.as_ref().and_then(|d| d.planned);

        let Some(relevant_time) = planned_arrival.or(planned_departure) else {
            continue;
        };

        if relevant_time > *now {
            if earliest > relevant_time {
                earliest = relevant_time;
                next_stop = Some(mapper(stop));
            };
            next_stops.push(mapper(stop));
        } else {
            past_stops.push(mapper(stop));
        }
    }

    (next_stop, next_stops, past_stops)
}
