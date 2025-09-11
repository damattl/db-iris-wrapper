use chrono::{NaiveDateTime};
use iris::{dto::StationInfo, fetch::get_station_infos};

use crate::{ingest::{ingest_timetable, ingest_timetable_messages}, model::{message::Message, station::Station, stop::Stop, train::Train}, ports::{MessagePort, StationPort, StopPort, TrainPort}, utils::HourIter};

pub fn import_station_data(port: &dyn StationPort) -> Result<Vec<Station>, Box<dyn std::error::Error>> {
    let station_infos = get_station_infos()?;
    let intercity_train = "INTERCITY_TRAIN".to_string(); // TODO: const
    let iris_stations: Vec<StationInfo> = station_infos
            .into_iter()
            .filter(|s| {
                if s.ds100.is_none() {
                    return false;
                }

                s.is_active_iris
                && !s.ds100.as_deref().unwrap().starts_with('X')
                    && s.available_transports.contains(&intercity_train)// treat None as not matching
            })
            .collect();

    let stations: Vec<Station> = iris_stations.into_iter().filter_map(|s| Station::from_info(s).ok()).collect();
    info!("Persisting stations");
    let result = port.persist_all(&stations)?;

    info!("Imported {} stations, {} new", stations.len(), result.len());

    Ok(result)
}

type ImportResult = (Vec<Train>, Vec<Stop>, Vec<Message>);

// Document: Returns not the inserted values, but the fetched ones this makes it easier to test

pub fn import_iris_data_for_station(
    station: &Station,
    datetime: &NaiveDateTime,
    message_port: &dyn MessagePort,
    train_port: &dyn TrainPort,
    stop_port: &dyn StopPort
) -> Result<ImportResult, Box<dyn std::error::Error>> {
    let mut trains: Vec<Train> = Vec::new();
    let mut stops: Vec<Stop> = Vec::new();

    for (date, hour) in HourIter::new(*datetime, 12) {
        info!("Importing timetable for {} at {} {:02}", station.ds100, date.format("%Y-%m-%d"), hour);
        let tt = match iris::fetch::get_timetable_for_station(station.id, &date, hour) {
            Ok(tt) => tt,
            Err(iris::dto::GetTimetableError::EmptyTimetable(_)) => continue,
            Err(e) => return Err(e.into()),
        };

        let (mut new_trains, mut new_stops) = ingest_timetable(&tt, &station, &date);
        trains.append(&mut new_trains);
        stops.append(&mut new_stops);
    }

    let tt_with_messages = iris::fetch::get_timetable_messages_for_station(station.id)?;
    let messages = ingest_timetable_messages(&tt_with_messages, stops.iter().map(|s| (s.id.clone(), s)).collect());
    info!("Ingested {} messages", messages.len());

    let new_trains = train_port.persist_all(&trains)?.len();
    let new_stops = stop_port.persist_all(&stops)?.len();
    let new_messages = message_port.persist_all(&messages)?.len();

    info!("{} new messages, {} new stops, {} new_trains", new_messages, new_stops, new_trains);

    info!("Import finshed");

    Ok((trains, stops, messages))
}

// Document: Returns not the inserted values, but the fetched ones this makes it easier to test
// // Document: fetches the station first
pub fn import_iris_data_for_station_by_ds100(
    ds100: &str,
    datetime: &NaiveDateTime,
    message_port: &dyn MessagePort,
    train_port: &dyn TrainPort,
    stop_port: &dyn StopPort
) -> Result<ImportResult, Box<dyn std::error::Error>> {
    let station = iris::fetch::get_station(ds100).map(Station::from_iris)??;
    import_iris_data_for_station(&station, datetime, message_port, train_port, stop_port)
}

pub fn import_iris_data(
    datetime: &NaiveDateTime,
    station_port: &dyn StationPort,
    message_port: &dyn MessagePort,
    train_port: &dyn TrainPort,
    stop_port: &dyn StopPort
) -> Result<(), Box<dyn std::error::Error>> {
    let stations = station_port.get_all()?;
    for station in stations {
        import_iris_data_for_station(&station, datetime, message_port, train_port, stop_port)?;
    }
    Ok(())
}
