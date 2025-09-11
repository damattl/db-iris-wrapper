use rocket::{get, response::status, routes, serde::json::Json, Route, State};
use rocket::http::Status;
use wrapper_core::{model::{station::Station, train::Train}};

use crate::{common::{error::ErrorBody, params::DateParam}, service::AppService};
type JsonErr = status::Custom<Json<ErrorBody>>;

#[get("/")]
fn stations(st: &State<AppService>) -> Result<Json<Vec<Station>>, JsonErr> {
    let stations = st.station_repo.get_all().map_err(|e| {
        status::Custom(Status::InternalServerError, Json(ErrorBody {
            error: "Failed to fetch station infos",
            message: e.to_string(),
        }))
    })?;

    Ok(Json(stations))
}

#[get("/<ds100>")]
fn station(ds100: &str, st: &State<AppService>) -> Result<Json<Station>, JsonErr> {
    let station = st.station_repo.get_by_ds100(ds100);

    match station {
        Ok(station) => Ok(Json(station)),
        Err(err) => Err(status::Custom(Status::NotFound, Json(ErrorBody {
            error: "Station not found",
            message: err.to_string(),
        }))), // TODO: Better error message
    }
}

#[get("/<ds100>/trains/<date>")]
fn trains_for_station(ds100: &str, date: DateParam, st: &State<AppService>) -> Result<Json<Vec<Train>>, JsonErr> {
    let station = st.station_repo.get_by_ds100(ds100).map_err(|e| {
        status::Custom(Status::InternalServerError, Json(ErrorBody {
            error: "Failed to fetch station infos",
            message: e.to_string(),
        }))
    })?;

    let trains = st.train_repo.get_by_station_and_date(&station, &date.0).map_err(|e| {
        status::Custom(Status::InternalServerError, Json(ErrorBody {
            error: "Failed to fetch trains",
            message: e.to_string(),
        }))
    })?; // TODO: for station and date

    Ok(Json(trains))
}


pub fn routes() -> Vec<Route> {
    routes![
        station, trains_for_station, stations
    ]
}

/* Old direct IRIS Routes
 *
#[get("/station/<ds100>/timetable/<date>/<time>?<include_messages>&<until>")]
fn timetable_for_station(
    ds100: &str,
    date: DateParam,
    time: &str,
    include_messages: bool,
    until: Option<String>,
    st: &State<AppService>
) -> Result<Json<Timetable>, JsonErr> {
    let station = st.station_repo.get_by_ds100(ds100).map_err(station_error_to_json_err)?;

    let mut timetable = get_timetable_for_station(station.id, &date.0, time_u16).map_err(timetable_error_to_json_err)?;

    if until.is_some() {



        let until_u16: u16 = u16::from_str(&until.unwrap()).map_err(|e| {
            status::Custom(Status::BadRequest, Json(ErrorBody {
                error: "Failed to parse until into an int",
                message: e.to_string(),
            }))
        })?;

        if time_u16 < until_u16 {
            for n in time_u16+1..until_u16 {
                let tt = get_timetable_for_station(eva, &date.0, n).map_err(timetable_error_to_json_err)?;

                timetable.stops.extend_from_slice(&tt.stops);
            }
        }
    }

    if include_messages {
        let mut messages = get_timetable_messages_for_station(eva).map_err(timetable_error_to_json_err)?;
        for stop in messages.stops.iter_mut() {
            let tt_stop = timetable.stops.iter_mut().find(|s| s.id == stop.id);
            match tt_stop {
                Some(tt_stop) => {
                    tt_stop.msgs.extend_from_slice(&stop.msgs);
                    if stop.arrival.is_some() && tt_stop.arrival.is_some() {
                        tt_stop.arrival.as_mut().unwrap().msgs.extend_from_slice(&stop.arrival.as_ref().unwrap().msgs);
                    }
                    if stop.departure.is_some() && tt_stop.departure.is_some() {
                        tt_stop.departure.as_mut().unwrap().msgs.extend_from_slice(&stop.departure.as_ref().unwrap().msgs);
                    }
                }
                None => continue
            }
        }
    }

    Ok(Json(timetable))
}


fn station_error_to_json_err(err: GetStationError) -> JsonErr {
    match err {
        GetStationError::Network(e) => status::Custom(Status::InternalServerError, Json(ErrorBody {
                    error: "Network error",
                    message: e.to_string(),
                })),
        GetStationError::Io(e) => status::Custom(Status::InternalServerError, Json(ErrorBody {
                    error: "Failed to connect to IRIS API",
                    message: e.to_string(),
                })),
        GetStationError::Xml(e) => status::Custom(Status::InternalServerError, Json(ErrorBody {
                    error: "Failed to parse XML",
                    message: e.to_string(),
                })),
        GetStationError::NotFound(e) => status::Custom(Status::NotFound, Json(ErrorBody {
                    error: "Station not found",
                    message: e.to_string(),
                })),
    }
}

fn timetable_error_to_json_err(err: GetTimetableError) -> JsonErr {
    match err {
        GetTimetableError::Network(e) => status::Custom(Status::InternalServerError, Json(ErrorBody {
                error: "Network error",
                message: e.to_string(),
            })),
        GetTimetableError::Io(e) => status::Custom(Status::InternalServerError, Json(ErrorBody {
                error: "Failed to connect to IRIS API",
                message: e.to_string(),
            })),
        GetTimetableError::Xml(e) => status::Custom(Status::InternalServerError, Json(ErrorBody {
                error: "Failed to parse XML",
                message: e.to_string(),
            })),
        GetTimetableError::RequestFailed(code, body) => status::Custom(Status::InternalServerError, Json(ErrorBody {
                error: "Failed to fetch timetable",
                message: format!("HTTP {}: {}", code, body),
            })),
        GetTimetableError::EmptyTimetable(_) => status::Custom(Status::NotFound, Json(ErrorBody {
                error: "No timetable available",
                message: "No timetable available for this time".to_string(),
            })),
    }
}
*/
