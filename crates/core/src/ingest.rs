use std::collections::HashMap;

use chrono::{NaiveDate};
use iris::dto::Timetable;

use crate::model::{message::Message, station::Station, stop::Stop, train::Train};

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
        let stop = Stop::from_iris_stop(stop, &train, station);

        stops.push(stop);
        trains.push(train);
    }

    (trains, stops)
}

pub fn ingest_timetable_messages(tt_with_messages: &Timetable, stops: HashMap<String, &Stop>) -> Vec<Message> {
    let mut messages: Vec<Message> = Vec::with_capacity(tt_with_messages.stops.len());
    for iris_stop in tt_with_messages.stops.iter() {
        let train_id = match stops.get(&iris_stop.id) {
            Some(st) => st.train_id.to_string(),
            None => { // This can happen if the message is for a stop thats outside the time window the stops are fetched for
                debug!("{:?} not found in stops", iris_stop);
                continue;
            }
        };
        for msg in iris_stop.msgs.iter() {
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
    messages
}
