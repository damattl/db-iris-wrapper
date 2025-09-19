use std::env;

use chrono::{NaiveDate, NaiveDateTime};
use iris::{
    dto::{IRISStationError, IRISTimetableError, StationInfo},
    fetch::get_station_infos,
};

use crate::{
    ingest::{ingest_timetable, ingest_timetable_changes},
    io::get_status_codes,
    model::{Message, Station, Stop, StopUpdate, Train},
    ports::{MessagePort, PortError, StationPort, StatusCodePort, StopPort, TrainPort},
    utils::HourIter,
};

/// Identifier used in `StationInfo.available_transports` for long-distance trains.
const INTERCITY_TRAIN: &str = "INTERCITY_TRAIN";

#[derive(thiserror::Error, Debug)]
pub enum ImportError {
    #[error("invalid src format {0}")]
    InvalidSourceFormat(String),
    #[error(transparent)]
    StationError(#[from] IRISStationError),
    #[error(transparent)]
    TimetableError(#[from] IRISTimetableError),
    #[error(transparent)]
    PersistanceError(#[from] PortError),
    #[error(transparent)]
    Custom(#[from] Box<dyn std::error::Error>),
}

/// Import IRIS-active stations and persist **only newly inserted** ones.
///
/// Source is taken from `STATIONS_SRC` as `API:<url>`, `JSON:<path>`, or `SQL:<file>`.
/// Filters: `ds100` present, not starting with `X`; `is_active_iris == true`;
/// `available_transports` contains `INTERCITY_TRAIN`.
///
/// Returns: newly persisted `Station`s.
/// Errors: network/parse/repo errors are propagated.
pub fn import_station_data(port: &dyn StationPort) -> Result<Vec<Station>, ImportError> {
    let stations_src = env::var("STATIONS_SRC")
        .unwrap_or("API:https://bahnvorhersage.de/api/stations.json".to_string());

    let parts: Vec<String> = stations_src.splitn(2, ':').map(|p| p.to_string()).collect();
    let src_type =
        parts.first().ok_or(ImportError::InvalidSourceFormat(stations_src.clone()))?;
    let src = parts
        .get(1)
        .ok_or(ImportError::InvalidSourceFormat(stations_src.clone()))?;

    println!("{}", stations_src);

    let station_infos = match src_type.as_str() {
        "API" => {
            get_station_infos(src, true)
        }
        "JSON" => {
            get_station_infos(src, false)
        }
        "SQL" => {
            println!("{}", src);
            let results = port.import_from_sql(src)?;
            return Ok(results);
        }
        _ => {
            return Err(ImportError::InvalidSourceFormat(stations_src.clone()));
        }
    };

    let iris_stations: Vec<StationInfo> = station_infos?
        .into_iter()
        .filter(|s| {
            if s.ds100.is_none() {
                return false;
            }

            s.is_active_iris
                && !s.ds100.as_deref().unwrap().starts_with('X')
                && s.available_transports.contains(&INTERCITY_TRAIN.to_string())
        })
        .collect();

    let stations: Vec<Station> = iris_stations
        .into_iter()
        .filter_map(|s| Station::from_info(s).ok())
        .collect();

    info!("Persisting stations");
    let result = port.persist_all(&stations)?;

    info!("Imported {} stations, {} new", stations.len(), result.len());

    Ok(result)
}

/// Shorthand for `(trains, stops, messages)` returned by timetable ingestion.
type ImportResult = (Vec<Train>, Vec<Stop>, Vec<Message>);

/// Import timetable (trains, stops) and messages for one station over hourly windows.
///
/// Iterates hours from `start` for `hours_in_advance`. Skips empty timetables.
/// Persists all collected entities; returns the **fetched** domain values
/// (independent of deduplication).
///
/// Returns: `(trains, stops, messages)`.
/// Errors: fetch/persistence errors are propagated.
pub fn import_iris_data_for_station(
    station: &Station,
    start: &NaiveDateTime,
    hours_in_advance: u16,
    message_port: &dyn MessagePort,
    train_port: &dyn TrainPort,
    stop_port: &dyn StopPort,
) -> Result<ImportResult, Box<dyn std::error::Error>> {
    let mut trains: Vec<Train> = Vec::new();
    let mut stops: Vec<Stop> = Vec::new();

    for (date, hour) in HourIter::new(*start, hours_in_advance) {
        info!(
            "Importing timetable for {} at {} {:02}",
            station.ds100,
            date.format("%Y-%m-%d"),
            hour
        );
        let tt = match iris::fetch::get_timetable_for_station(station.id, &date, hour) {
            Ok(tt) => tt,
            Err(iris::dto::IRISTimetableError::EmptyTimetable(_)) => continue,
            Err(e) => return Err(e.into()),
        };

        info!("Ingesting timetable");

        let (mut new_trains, mut new_stops) = ingest_timetable(&tt, station, &date);
        trains.append(&mut new_trains);
        stops.append(&mut new_stops);
    }

    let (messages, stop_changes) =
        match iris::fetch::get_timetable_changes_for_station(station.id) {
            Ok(tt) => ingest_timetable_changes(&tt, stops.iter().map(|s| (s.id.clone(), s)).collect()),
            Err(IRISTimetableError::EmptyTimetable(_)) => (Vec::new(), Vec::new()),
            Err(err) => return Err(err.into()),
        };

    info!("Ingested {} messages", messages.len());

    let new_trains = train_port.persist_all(&trains)?.len();
    let new_stops = stop_port.persist_all(&stops)?.len();
    let new_messages = message_port.persist_all(&messages)?.len();

    let stop_updates = stop_changes
        .iter()
        .map(StopUpdate::from)
        .collect::<Vec<StopUpdate>>();
    let updated_stops_count = stop_port.update_many(&stop_updates)?.len();

    info!(
        "{} new messages, {} new stops, {} new_trains, {} updated_stops",
        new_messages, new_stops, new_trains, updated_stops_count
    );

    info!("Import finished");

    Ok((trains, stops, messages))
}

/// Convenience wrapper: import by DS100 code (fetch station first).
///
/// Returns fetched `(trains, stops, messages)`.
/// Errors: lookup/mapping/import errors are propagated.
pub fn import_iris_data_for_station_by_ds100(
    ds100: &str,
    start: &NaiveDateTime,
    hours_in_advance: u16,
    message_port: &dyn MessagePort,
    train_port: &dyn TrainPort,
    stop_port: &dyn StopPort,
) -> Result<ImportResult, Box<dyn std::error::Error>> {
    let station = iris::fetch::get_station(ds100).map(Station::from_iris)??;
    import_iris_data_for_station(
        &station,
        start,
        hours_in_advance,
        message_port,
        train_port,
        stop_port,
    )
}

/// Import timetables and messages for **all** persisted stations.
///
/// Returns `Ok(())` on success.
/// Errors: per-station import errors are propagated.
pub fn import_iris_data(
    start: &NaiveDateTime,
    hours_in_advance: u16,
    station_port: &dyn StationPort,
    message_port: &dyn MessagePort,
    train_port: &dyn TrainPort,
    stop_port: &dyn StopPort,
) -> Result<(), Box<dyn std::error::Error>> {
    let stations = station_port.get_all()?;
    for station in stations {
        import_iris_data_for_station(
            &station,
            start,
            hours_in_advance,
            message_port,
            train_port,
            stop_port,
        )?;
    }
    Ok(())
}

/// Import **timetable changes/messages** for a station and update affected stops.
///
/// Uses existing stops for `date` as context. Returns fetched `Message`s.
/// Errors: fetch/mapping/persistence errors are propagated.
pub fn import_iris_changes_for_station(
    station: &Station,
    date: &NaiveDate,
    message_port: &dyn MessagePort,
    stop_port: &dyn StopPort,
) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
    let tt_changes = iris::fetch::get_timetable_changes_for_station(station.id)?;

    let stops = stop_port.get_for_date(date)?;

    let (messages, stop_changes) =
        ingest_timetable_changes(&tt_changes, stops.iter().map(|s| (s.id.clone(), s)).collect());
    info!("Ingested {} messages", messages.len());

    let updates = stop_changes
        .iter()
        .map(StopUpdate::from)
        .collect::<Vec<StopUpdate>>();
    let updated_stops_count = stop_port.update_many(&updates)?.len();

    let new_messages = message_port.persist_all(&messages)?.len();

    info!("{} new messages, {} updated stops", new_messages, updated_stops_count);
    info!("Import finished");

    Ok(messages)
}

/// Convenience wrapper: import **changes/messages** by DS100 code.
///
/// Returns fetched `Message`s.
/// Errors: lookup/mapping/import errors are propagated.
pub fn import_iris_changes_for_station_by_ds100(
    ds100: &str,
    date: &NaiveDate,
    message_port: &dyn MessagePort,
    stop_port: &dyn StopPort,
) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
    let station = iris::fetch::get_station(ds100).map(Station::from_iris)??;
    import_iris_changes_for_station(&station, date, message_port, stop_port)
}

/// Import **changes/messages** for **all** stations on a given date.
///
/// Logs per-station results; continues on per-station errors.
/// Returns `Ok(())` on success.
pub fn import_iris_changes(
    date: &NaiveDate,
    station_port: &dyn StationPort,
    message_port: &dyn MessagePort,
    stop_port: &dyn StopPort,
) -> Result<(), Box<dyn std::error::Error>> {
    let stations = station_port.get_all()?;
    for station in stations {
        let _ = import_iris_changes_for_station(&station, date, message_port, stop_port)
            .inspect_err(|e| {
                error!(
                    "Error while importing iris_messages for station {}: {}",
                    station.id, e
                )
            });
    }
    Ok(())
}

/// Import status codes from the configured source and persist them.
///
/// Returns `Ok(())` on success.
/// Errors: source/Excel/persistence errors are mapped to `ImportError`.
pub fn import_status_codes(
    status_code_port: &dyn StatusCodePort,
) -> Result<(), ImportError> {
    let codes = get_status_codes().map_err(|e| match e {
        crate::io::IOError::InvalidSourceFormat(err) => ImportError::InvalidSourceFormat(err),
        crate::io::IOError::ExcelError(err) => ImportError::Custom(Box::new(err)),
    })?;

    status_code_port.persist_all(&codes)?;
    Ok(())
}
