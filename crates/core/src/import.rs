use std::{env};

use chrono::{NaiveDate, NaiveDateTime};
use iris::{dto::{IRISStationError, IRISTimetableError, StationInfo}, fetch::get_station_infos};

use crate::{
    ingest::{ingest_timetable, ingest_timetable_messages}, io::get_status_codes, model::{message::Message, station::Station, stop::Stop, train::Train}, ports::{MessagePort, PortError, StationPort, StatusCodePort, StopPort, TrainPort}, utils::HourIter
};

/// Literal identifying long-distance trains in `StationInfo.available_transports`.
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

/// Discover **IRIS-active** stations and persist them.
///
/// This function queries the external IRIS metadata (via [`get_station_infos`])
/// and applies the following filters:
///
/// 1. `ds100` must be present and **must not** start with `'X'` (typically
///    non-public or technical points).
/// 2. `is_active_iris` must be `true`.
/// 3. `available_transports` must contain [`INTERCITY_TRAIN`].
///
/// Eligible stations are converted to domain [`Station`]s via
/// `Station::from_info` and persisted in bulk using the provided
/// [`StationPort`]. The function returns **only the entities that were newly
/// inserted** (as reported by the port), enabling clients to reason about
/// incremental growth.
///
/// # Returns
/// A `Vec<Station>` with the **newly persisted** stations.
///
/// # Errors
/// - Propagates network, parsing, and repository errors via the boxed error.
///
/// # Side Effects
/// - Emits `info!` logs about persistence counts.
///
/// # Examples
/// ```ignore
/// let new_stations = import_station_data(&my_station_repo)?;
/// println!("Inserted {} stations", new_stations.len());
/// ```
pub fn import_station_data(
    port: &dyn StationPort,
) -> Result<Vec<Station>, ImportError> {
    let stations_src = env::var("STATIONS_SRC")
        .unwrap_or("API:https://bahnvorhersage.de/api/stations.json".to_string());

    let parts: Vec<String> = stations_src.splitn(2, ':').map(|p| p.to_string()).collect();
    let src_type = parts.first().ok_or(ImportError::InvalidSourceFormat(stations_src.clone()))?;
    let src = parts.get(1).ok_or(ImportError::InvalidSourceFormat(stations_src.clone()))?;

    let station_infos;
    println!("{}", stations_src);

    match src_type.as_str() {
        "API" => {
            station_infos = get_station_infos(src, true);
        },
        "JSON" => {
            station_infos = get_station_infos(src, false);
        },
        "SQL" => {
            println!("{}", src);
            let results = port.import_from_sql(src)?;
            return Ok(results)
        },
        _ => {
            return Err(ImportError::InvalidSourceFormat(stations_src.clone()))
        }
    };

    // TODO: const
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

/// Shorthand for the import tuple returned by timetable ingestion:
/// `(trains, stops, messages)`.
///
/// > **Testing note:** The import functions that return this alias
/// > deliberately return the **fetched** items, not the ones that were
/// > inserted by the repository, to make assertions independent of deduplication.
type ImportResult = (Vec<Train>, Vec<Stop>, Vec<Message>);

/// Import **timetable** (trains, stops) and **messages** for a single station
/// over a window of hourly slices.
///
/// The function iterates over **12 successive hourly windows** (starting at
/// `datetime`, at hourly granularity) using [`HourIter`]. For each hour it:
///
/// 1. Fetches the timetable via `iris::fetch::get_timetable_for_station`.
///    - If the timetable is empty (`GetTimetableError::EmptyTimetable`), that
///      hour is skipped without error.
/// 2. Ingests trains and stops via [`ingest_timetable`], appending to the
///    in-memory collections.
///
/// After the hourly loop finishes, it fetches **timetable messages** for the
/// station, ingests them with [`ingest_timetable_messages`] using the
/// accumulated stops as context, and finally persists **all** collected trains,
/// stops, and messages using the respective ports.
///
///
/// # Returns
/// **Returns the fetched domain values** `(trains, stops, messages)` —
/// **not** the subset that happened to be newly inserted.
/// *This makes the function easier to test*, because test assertions can
/// target the semantic ingestion result directly, independent of repository
/// deduplication or conflict handling.
///
/// # Errors
/// - Any non-empty-timetable fetch error is propagated.
/// - Repository errors from `persist_all` calls are propagated.
///
/// # Side Effects
/// - Emits `info!` logs for progress and counts.
/// - Persists data through the provided ports.
///
/// # Examples
/// ```ignore
/// let (trains, stops, messages) = import_iris_data_for_station(
///     &station,
///     &start,
///     &message_repo,
///     &train_repo,
///     &stop_repo,
/// )?;
/// assert!(!trains.is_empty());
/// ```
pub fn import_iris_data_for_station(
    station: &Station,
    datetime: &NaiveDateTime,
    message_port: &dyn MessagePort,
    train_port: &dyn TrainPort,
    stop_port: &dyn StopPort,
) -> Result<ImportResult, Box<dyn std::error::Error>> {
    let mut trains: Vec<Train> = Vec::new();
    let mut stops: Vec<Stop> = Vec::new();

    for (date, hour) in HourIter::new(*datetime, 12) {
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

    let messages = match iris::fetch::get_timetable_messages_for_station(station.id) {
        Ok(tt) => ingest_timetable_messages(&tt, stops.iter().map(|s| (s.id.clone(), s)).collect()),
        Err(IRISTimetableError::EmptyTimetable(_)) => Vec::new(),
        Err(err) => return Err(err.into()),
    };

    info!("Ingested {} messages", messages.len());

    let new_trains = train_port.persist_all(&trains)?.len();
    let new_stops = stop_port.persist_all(&stops)?.len();
    let new_messages = message_port.persist_all(&messages)?.len();

    info!(
        "{} new messages, {} new stops, {} new_trains",
        new_messages, new_stops, new_trains
    );

    info!("Import finshed");

    Ok((trains, stops, messages))
}

/// Import timetable and messages for a station identified by **DS100 code**.
///
/// This is a convenience wrapper that **fetches the station first** via
/// `iris::fetch::get_station(ds100)` and converts it with `Station::from_iris`,
/// then delegates to [`import_iris_data_for_station`].
///
/// # Returns
/// **Returns the fetched domain values** `(trains, stops, messages)` — not the
/// inserted subset. *This makes it easier to test.*
///
/// # Errors
/// - Propagates lookup, mapping, and import errors.
///
/// # Examples
/// ```ignore
/// let (trains, stops, messages) = import_iris_data_for_station_by_ds100(
///     "HH",
///     &start,
///     &message_repo,
///     &train_repo,
///     &stop_repo,
/// )?;
/// ```
pub fn import_iris_data_for_station_by_ds100(
    ds100: &str,
    datetime: &NaiveDateTime,
    message_port: &dyn MessagePort,
    train_port: &dyn TrainPort,
    stop_port: &dyn StopPort,
) -> Result<ImportResult, Box<dyn std::error::Error>> {
    let station = iris::fetch::get_station(ds100).map(Station::from_iris)??;
    import_iris_data_for_station(&station, datetime, message_port, train_port, stop_port)
}

/// Import timetables and messages for **all** stations known to the repository.
///
/// The function retrieves the full station list from [`StationPort::get_all`]
/// and calls [`import_iris_data_for_station`] for each entry.
///
/// # Returns
/// `Ok(())` on success.
///
/// # Errors
/// - Propagates repository and import errors from the per-station calls.
///
/// # Side Effects
/// - Emits `info!` logs for each station.
/// - Performs persistence via the provided ports.
pub fn import_iris_data(
    datetime: &NaiveDateTime,
    station_port: &dyn StationPort,
    message_port: &dyn MessagePort,
    train_port: &dyn TrainPort,
    stop_port: &dyn StopPort,
) -> Result<(), Box<dyn std::error::Error>> {
    let stations = station_port.get_all()?;
    for station in stations {
        import_iris_data_for_station(&station, datetime, message_port, train_port, stop_port)?;
    }
    Ok(())
}

/// Import **messages only** for a single station and date.
///
/// This function fetches timetable messages via
/// `iris::fetch::get_timetable_messages_for_station` and ingests them with
/// [`ingest_timetable_messages`]. It uses the existing stops for the given
/// `date` (retrieved via [`StopPort::get_for_date`]) to provide the necessary
/// context for message association.
///
/// # Returns
/// **Returns the fetched messages** (not only the newly inserted subset).
/// *This makes it easier to test.*
///
/// # Errors
/// - Propagates network, mapping, and repository errors.
///
/// # Side Effects
/// - Emits `info!` logs and persists through the provided ports.
///
/// # Examples
/// ```ignore
/// let messages = import_iris_messages_for_station(&station, &date, &message_repo, &stop_repo)?;
/// assert!(!messages.is_empty());
/// ```
pub fn import_iris_messages_for_station(
    station: &Station,
    date: &NaiveDate,
    message_port: &dyn MessagePort,
    stop_port: &dyn StopPort,
) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
    let tt_with_messages = iris::fetch::get_timetable_messages_for_station(station.id)?;

    let stops = stop_port.get_for_date(date)?;

    let messages =
        ingest_timetable_messages(&tt_with_messages, stops.iter().map(|s| (s.id.clone(), s)).collect());
    info!("Ingested {} messages", messages.len());

    let new_messages = message_port.persist_all(&messages)?.len();

    info!("{} new messages", new_messages);

    info!("Import finshed");

    Ok(messages)
}

/// Import **messages only** for a station identified by **DS100 code**.
///
/// This wrapper **fetches the station first** and then delegates to
/// [`import_iris_messages_for_station`].
///
/// # Returns
/// **Returns the fetched messages** (not only those newly inserted).
/// *This makes it easier to test.*
///
/// # Errors
/// - Propagates lookup, mapping, and repository errors.
///
/// # Examples
/// ```ignore
/// let msgs = import_iris_messages_for_station_by_ds100("HH", &date, &message_repo, &stop_repo)?;
/// ```
pub fn import_iris_messages_for_station_by_ds100(
    ds100: &str,
    date: &NaiveDate,
    message_port: &dyn MessagePort,
    stop_port: &dyn StopPort,
) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
    let station = iris::fetch::get_station(ds100).map(Station::from_iris)??;
    import_iris_messages_for_station(&station, date, message_port, stop_port)
}

/// Import **messages only** for **all** stations known to the repository on a
/// given date.
///
/// The function enumerates stations via [`StationPort::get_all`] and calls
/// [`import_iris_messages_for_station`] for each.
///
/// # Returns
/// `Ok(())` on success.
///
/// # Errors
/// - Propagates repository and import errors from the per-station calls.
///
/// # Side Effects
/// - Emits `info!` logs and persists through the provided ports.
pub fn import_iris_messages(
    date: &NaiveDate,
    station_port: &dyn StationPort,
    message_port: &dyn MessagePort,
    stop_port: &dyn StopPort,
) -> Result<(), Box<dyn std::error::Error>> {
    let stations = station_port.get_all()?;
    for station in stations {
        let _ = import_iris_messages_for_station(&station, date, message_port, stop_port).inspect_err(
            |e| error!("Error while importing iris_messages for station {}: {}", station.id, e)
        );
    }
    Ok(())
}



pub fn import_status_codes(
    status_code_port: &dyn StatusCodePort,
) -> Result<(), ImportError> {
    let codes = get_status_codes().map_err(|e| {
        match e {
            crate::io::IOError::InvalidSourceFormat(err) => ImportError::InvalidSourceFormat(err),
            crate::io::IOError::ExcelError(err) => ImportError::Custom(Box::new(err)),
        }
    })?;

    status_code_port.persist_all(&codes)?;

    Ok(())
}
