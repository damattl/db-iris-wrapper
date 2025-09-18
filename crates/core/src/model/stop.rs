use chrono::{NaiveDateTime};

use super::train::Train;
use super::station::Station;

pub trait HasStopGetter {
    fn get_stop(&self) -> &Stop;
}

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

impl HasStopGetter for Stop {
    fn get_stop(&self) -> &Stop {
        self
    }
}

#[derive(Debug, Clone)]
pub struct StopWithStation {
    pub stop: Stop,
    pub station: Station,
}

impl HasStopGetter for StopWithStation {
    fn get_stop(&self) -> &Stop {
        &self.stop
    }
}


#[derive(Debug, Clone)]
pub struct Movement {
    pub platform: Option<String>,
    pub planned: Option<NaiveDateTime>,
    pub current: Option<NaiveDateTime>,
    pub planned_path: Option<Vec<String>>,
    pub changed_path: Option<Vec<String>>,
}

impl Movement {
    pub fn from_iris_movement(movement: &iris::dto::Movement) -> Self {
        Movement {
            platform: movement.platform.clone(),
            planned: movement.planned,
            current: movement.current,
            planned_path: movement.ppth.clone(),
            changed_path: movement.cpth.clone(),
        }
    }
}


pub fn split_stops_by_time<S, M>(stops: &[S], now: &NaiveDateTime, mapper: fn(stop: &S) -> M) -> (Option<M>, Vec<M>, Vec<M>)
where
    S: HasStopGetter,
    M: Clone
{
    let mut earliest: Option<NaiveDateTime> = None;
    let mut next_stop: Option<M> = None;

    let mut next_stops = Vec::<M>::new();
    let mut past_stops = Vec::<M>::new();


    // TODO: Update when current (belated) time is available
    // TODO: Maybe move to dedicated function
    // TODO: Test this logic!
    for stop in stops {
        let planned_arrival = stop.get_stop().arrival.as_ref().and_then(|a| a.planned);
        let planned_departure = stop.get_stop().departure.as_ref().and_then(|d| d.planned);

        let Some(relevant_time) = planned_arrival.or(planned_departure) else {
            continue;
        };

        if relevant_time > *now {
            let mapped_stop = mapper(stop);

            if earliest.is_none_or(|t| t > relevant_time) {
                earliest = Some(relevant_time);
                next_stop = Some(mapped_stop.clone());
            }

            next_stops.push(mapped_stop);
        } else {
            past_stops.push(mapper(stop));
        }
    }

    (next_stop, next_stops, past_stops)
}
