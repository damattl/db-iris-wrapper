use std::collections::HashMap;

use iris::dto::Timetable;

use crate::model::{Message, Train, Station, Stop};

pub fn ingest_timetable(tt: &iris::dto::Timetable, station: &Station) -> (Vec<Train>, Vec<Stop>) {
    let mut trains: Vec<Train> = Vec::with_capacity(tt.stops.len());
    let mut stops: Vec<Stop> = Vec::with_capacity(tt.stops.len());
    for stop in tt.stops.iter() {
        if stop.tl.as_ref().and_then(|tl| tl.category.as_ref()).is_none_or(|c| c == "Bus") {
            continue;
        }

        let train = match Train::from_stop(stop) {
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
    let mut messages_set: HashMap<String, Message> = HashMap::new();

    let mut stop_changes: Vec<Stop> = Vec::with_capacity(tt_changes.stops.len());

    let station_id = tt_changes.eva.as_deref().unwrap().parse::<i32>().unwrap(); // Changes Timetable always has an eva

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
            let message = match Message::from_iris_msg(msg, &train_id, station_id) {
                Ok(message) => message,
                Err(err) => {
                    error!("Error building message from iris message: {}", err);
                    continue;
                }
            };
            messages_set.insert(message.id.clone(), message);
        }

        if iris_stop_change.arrival.is_some() {
            for msg in iris_stop_change.arrival.as_ref().unwrap().msgs.iter() {
                let message = match Message::from_iris_msg(msg, &train_id, station_id) {
                    Ok(message) => message,
                    Err(err) => {
                        error!("Error building message from iris message: {}", err);
                        continue;
                    }
                };
                messages_set.insert(message.id.clone(), message);
            }
        }

        if iris_stop_change.departure.is_some() {
            for msg in iris_stop_change.departure.as_ref().unwrap().msgs.iter() {
                let message = match Message::from_iris_msg(msg, &train_id, station_id) {
                    Ok(message) => message,
                    Err(err) => {
                        error!("Error building message from iris message: {}", err);
                        continue;
                    }
                };
                messages_set.insert(message.id.clone(), message);
            }
        }
    }

    let messages = messages_set.into_values().collect();

    (messages, stop_changes)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Station, Stop as DomainStop, Train};
    use chrono::{NaiveDate, NaiveDateTime};
    use iris::dto::{Movement as IrisMovement, Msg, Stop as IrisStop, Timetable, TrainLine};
    use std::collections::HashMap;

    fn sample_station() -> Station {
        Station {
            id: 8002549,
            lat: Some(53.5511),
            lon: Some(9.9937),
            name: "Hamburg Hbf".to_string(),
            ds100: "AH".to_string(),
        }
    }

    fn sample_train_line() -> TrainLine {
        TrainLine {
            f: None,
            t: None,
            operator: Some("DB".to_string()),
            category: Some("ICE".to_string()),
            number: Some("123".to_string()),
        }
    }

    fn sample_movement(planned: &str, current: Option<&str>) -> IrisMovement {
        let planned_dt = NaiveDateTime::parse_from_str(planned, "%Y-%m-%d %H:%M:%S").unwrap();
        IrisMovement {
            planned: Some(planned_dt),
            current: current.map(|ts| NaiveDateTime::parse_from_str(ts, "%Y-%m-%d %H:%M:%S").unwrap()),
            platform: None,
            line: None,
            hi: None,
            ppth: None,
            cpth: None,
            cs: None,
            clt: None,
            wings: None,
            msgs: Vec::new(),
        }
    }

    fn base_iris_stop(id: &str) -> IrisStop {
        IrisStop {
            id: id.to_string(),
            eva: Some("8002549".to_string()),
            tl: Some(sample_train_line()),
            msgs: Vec::new(),
            arrival: Some(sample_movement("2025-09-10 08:00:00", None)),
            departure: None,
        }
    }

    #[test]
    fn ingest_timetable_skips_stops_that_cannot_build_trains() {
        let station = sample_station();
        let valid_stop = base_iris_stop("test-stop-2509100800-1");
        let mut invalid_stop = base_iris_stop("invalid-2509100900-1");
        invalid_stop.tl = None;

        let timetable = Timetable {
            station: station.ds100.clone(),
            eva: Some(station.id.to_string()),
            stops: vec![valid_stop.clone(), invalid_stop],
        };

        let (trains, stops) = ingest_timetable(&timetable, &station);

        assert_eq!(1, trains.len());
        assert_eq!(1, stops.len());
        assert_eq!("123-250910", trains[0].id);
        assert_eq!(trains[0].id, stops[0].train_id);
    }

    #[test]
    fn ingest_timetable_changes_deduplicates_messages_and_updates_stop() {
        let station = sample_station();
        let iris_stop = base_iris_stop("test-stop-2509100800-1");
        let train_id = Train::new_id("123", &NaiveDate::from_ymd_opt(2025, 9, 10).unwrap());
        let existing_stop: DomainStop = DomainStop::from_iris_stop(&iris_stop, &train_id, station.id);

        let mut stops_map: HashMap<String, &DomainStop> = HashMap::new();
        stops_map.insert(existing_stop.id.clone(), &existing_stop);

        let message_ts = NaiveDateTime::parse_from_str("2025-09-10 08:15:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let duplicate_msg = Msg {
            id: Some("duplicate".to_string()),
            kind: Some("h".to_string()),
            from: None,
            to: None,
            cat: Some("Info".to_string()),
            pr: Some(1),
            code: Some(99),
            ts: Some(message_ts),
            ts_tts: None,
        };

        let change_stop = IrisStop {
            id: existing_stop.id.clone(),
            eva: Some("8002549".to_string()),
            tl: Some(sample_train_line()),
            msgs: vec![duplicate_msg.clone()],
            arrival: Some(IrisMovement {
                planned: Some(NaiveDateTime::parse_from_str("2025-09-10 08:00:00", "%Y-%m-%d %H:%M:%S").unwrap()),
                current: Some(NaiveDateTime::parse_from_str("2025-09-10 08:05:00", "%Y-%m-%d %H:%M:%S").unwrap()),
                platform: None,
                line: None,
                hi: None,
                ppth: None,
                cpth: None,
                cs: None,
                clt: None,
                wings: None,
                msgs: vec![duplicate_msg.clone()],
            }),
            departure: None,
        };

        let changes = Timetable {
            station: station.ds100.clone(),
            eva: Some(station.id.to_string()),
            stops: vec![change_stop],
        };

        let (messages, stop_updates) = ingest_timetable_changes(&changes, stops_map);

        assert_eq!(1, messages.len());
        assert_eq!("duplicate", messages[0].id);

        assert_eq!(1, stop_updates.len());
        let updated_arrival = stop_updates[0].arrival.as_ref().expect("arrival should be present");
        assert_eq!(Some(NaiveDateTime::parse_from_str("2025-09-10 08:05:00", "%Y-%m-%d %H:%M:%S").unwrap()), updated_arrival.current);
    }
}
