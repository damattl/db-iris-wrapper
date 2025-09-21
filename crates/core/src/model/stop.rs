use chrono::{NaiveDateTime};

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
    pub fn from_iris_stop(stop: &iris::dto::Stop, train_id: &str, station_id: i32) -> Self {
        Stop {
            id: stop.id.clone(),
            train_id: train_id.to_string(),
            station_id,
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
pub struct StopUpdate {
    pub id: String,
    pub arrival: Option<Movement>,
    pub departure: Option<Movement>,
}

impl From<&Stop> for StopUpdate {
    fn from(value: &Stop) -> Self {
        StopUpdate {
            id: value.id.clone(),
            arrival: value.arrival.clone(),
            departure: value.departure.clone()
        }
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


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    fn map_stop_id(stop: &Stop) -> String {
        stop.id.clone()
    }

    fn build_stop(
        id: &str,
        arrival: Option<NaiveDateTime>,
        departure: Option<NaiveDateTime>,
    ) -> Stop {
        Stop {
            id: id.to_string(),
            train_id: "train".to_string(),
            station_id: 42,
            arrival: arrival.map(|planned| Movement {
                platform: None,
                planned: Some(planned),
                current: None,
                planned_path: None,
                changed_path: None,
            }),
            departure: departure.map(|planned| Movement {
                platform: None,
                planned: Some(planned),
                current: None,
                planned_path: None,
                changed_path: None,
            }),
        }
    }

    #[test]
    fn split_stops_by_time_returns_buckets_and_next() {
        let now = NaiveDateTime::parse_from_str("2025-09-10 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        let past = build_stop("past-2509101100-1", Some(NaiveDateTime::parse_from_str("2025-09-10 11:00:00", "%Y-%m-%d %H:%M:%S").unwrap()), None);
        let future_later = build_stop("future-b-2509101400-1", Some(NaiveDateTime::parse_from_str("2025-09-10 14:00:00", "%Y-%m-%d %H:%M:%S").unwrap()), None);
        let future_earliest = build_stop("future-a-2509101300-1", Some(NaiveDateTime::parse_from_str("2025-09-10 13:00:00", "%Y-%m-%d %H:%M:%S").unwrap()), None);

        let stops = vec![past, future_later.clone(), future_earliest.clone()];

        let (next_stop, next_stops, past_stops) = split_stops_by_time(&stops, &now, map_stop_id);

        assert_eq!(Some(future_earliest.id.clone()), next_stop);
        assert_eq!(vec![future_later.id.clone(), future_earliest.id.clone()], next_stops);
        assert_eq!(vec!["past-2509101100-1".to_string()], past_stops);
    }

    #[test]
    fn split_stops_by_time_ignores_stops_without_times() {
        let now = NaiveDateTime::parse_from_str("2025-09-10 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        let stops = vec![build_stop("no-times", None, None)];

        let (next_stop, next_stops, past_stops) = split_stops_by_time(&stops, &now, map_stop_id);

        assert!(next_stop.is_none());
        assert!(next_stops.is_empty());
        assert!(past_stops.is_empty());
    }
}
