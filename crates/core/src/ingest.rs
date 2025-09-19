use std::collections::HashMap;

use chrono::{NaiveDate};
use iris::dto::Timetable;

use crate::model::{Message, Train, Station, Stop};

pub fn ingest_timetable(tt: &iris::dto::Timetable, station: &Station, date: &NaiveDate) -> (Vec<Train>, Vec<Stop>) {
    let mut trains: Vec<Train> = Vec::with_capacity(tt.stops.len());
    let mut stops: Vec<Stop> = Vec::with_capacity(tt.stops.len());
    for stop in tt.stops.iter() {
        let train = match Train::from_stop(stop, date) {
            Ok(train) => {
                train
            }
            Err(err) => {
                error!("Error building train from stop: {}", err);
                continue;
            }
        };
        let stop = Stop::from_iris_stop(stop, &train.id, station.id);

        stops.push(stop);
        trains.push(train);
    }

    (trains, stops)
}


pub fn ingest_timetable_changes(tt_changes: &Timetable, stops: HashMap<String, &Stop>) -> (Vec<Message>, Vec<Stop>) {
    let mut messages: Vec<Message> = Vec::with_capacity(tt_changes.stops.len());
    let mut stop_changes: Vec<Stop> = Vec::with_capacity(tt_changes.stops.len());

    for iris_stop_change in tt_changes.stops.iter() {
        let known_stop = match stops.get(&iris_stop_change.id) {
            Some(st) => *st,
            None => { // This can happen if the message is for a stop thats outside the time window the stops are fetched for
                debug!("{:?} not found in stops", iris_stop_change);
                continue;
            }
        };
        let train_id = known_stop.train_id.clone();
        stop_changes.push(Stop::from_iris_stop(iris_stop_change, &known_stop.train_id, known_stop.station_id));

        for msg in iris_stop_change.msgs.iter() {
            let message = match Message::from_iris_msg(msg, &train_id) {
                Ok(message) => message,
                Err(err) => {
                    error!("Error building message from iris message: {}", err);
                    continue;
                }
            };
            messages.push(message);
        }

        if iris_stop_change.arrival.is_some() {
            for msg in iris_stop_change.arrival.as_ref().unwrap().msgs.iter() {
                let message = match Message::from_iris_msg(msg, &train_id) {
                    Ok(message) => message,
                    Err(err) => {
                        error!("Error building message from iris message: {}", err);
                        continue;
                    }
                };
                messages.push(message);
            }
        }

        if iris_stop_change.departure.is_some() {
            for msg in iris_stop_change.departure.as_ref().unwrap().msgs.iter() {
                let message = match Message::from_iris_msg(msg, &train_id) {
                    Ok(message) => message,
                    Err(err) => {
                        error!("Error building message from iris message: {}", err);
                        continue;
                    }
                };
                messages.push(message);
            }
        }
    }
    (messages, stop_changes)
}
