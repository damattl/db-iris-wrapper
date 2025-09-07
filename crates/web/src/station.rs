use domain::train::{get_trains_for_station, Train};
use infrastructure::iris::station::{get_station_infos, StationInfo};
use infrastructure::iris::timetable::{get_timetable_for_station, get_timetable_messages_for_station};
use infrastructure::iris::timetable_dto::{GetTimetableError, Timetable};
use infrastructure::iris::{station::get_station, station_dto::GetStationError};
use infrastructure::iris::station_dto::IRISStation;
use rocket::{get, routes, Route, response::status, serde::{json::Json}};
use rocket::http::Status;
use std::str::FromStr;

use crate::common::{error::ErrorBody, params::DateParam};



type JsonErr = status::Custom<Json<ErrorBody>>;

#[get("/station/<id>")]
fn station(id: &str) -> Result<Json<IRISStation>, JsonErr> {
    let station = get_station(id);

    match station {
        Ok(station) => Ok(Json(station)),
        Err(err) => Err(status::Custom(Status::NotFound, Json(ErrorBody {
            error: "Station not found",
            message: err.to_string(),
        }))), // TODO: Better error message
    }
}

#[get("/station/<id>/trains/<date>")]
fn trains_for_station(id: &str, date: DateParam) -> Result<Json<Vec<Train>>, JsonErr> {
    let station = get_station(id).map_err(station_error_to_json_err)?;

    let eva = station.eva; // TODO: Name id and eva differently
    let trains = get_trains_for_station(&eva, &date.0).map_err(|e| {
        status::Custom(Status::InternalServerError, Json(ErrorBody {
            error: "Failed to fetch trains",
            message: e.to_string(),
        }))
    })?;

    Ok(Json(trains))
}

#[get("/stations")]
fn stations() -> Result<Json<Vec<StationInfo>>, JsonErr> {
    let stations = get_station_infos().map_err(|e| {
        status::Custom(Status::InternalServerError, Json(ErrorBody {
            error: "Failed to fetch station infos",
            message: e.to_string(),
        }))
    })?;
    let intercity_train = "INTERCITY_TRAIN".to_string();

    let iris_stations: Vec<StationInfo> = stations
            .into_iter()
            .filter(|s| {
                if s.ds100.is_none() {
                    return false;
                }

                s.is_active_iris
                && s.ds100.as_deref().unwrap().starts_with('X')
                    && s.available_transports.contains(&intercity_train)// treat None as not matching
            })
            .collect();

    Ok(Json(iris_stations))
}

#[get("/station/<id>/timetable/<date>/<time>?<include_messages>&<until>")]
fn timetable_for_station(id: &str, date: DateParam, time: &str, include_messages: bool, until: Option<String>) -> Result<Json<Timetable>, JsonErr> {
    let station = get_station(id).map_err(station_error_to_json_err)?;

    let eva = station.eva; // TODO: Name id and eva differently
    let mut timetable = get_timetable_for_station(&eva, &date.0, time).map_err(timetable_error_to_json_err)?;

    if until.is_some() {
        let time_i32: i32 = i32::from_str(time).map_err(|e| {
            status::Custom(Status::BadRequest, Json(ErrorBody {
                error: "Failed to parse time into an int",
                message: e.to_string(),
            }))
        })?;


        let until_i32: i32 = i32::from_str(&until.unwrap()).map_err(|e| {
            status::Custom(Status::BadRequest, Json(ErrorBody {
                error: "Failed to parse until into an int",
                message: e.to_string(),
            }))
        })?;

        if time_i32 < until_i32 {
            for n in time_i32+1..until_i32 {
                let tt = get_timetable_for_station(&eva, &date.0, &format!("{:02}", n)).map_err(timetable_error_to_json_err)?;

                timetable.stops.extend_from_slice(&tt.stops);
            }
        }
    }

    if include_messages {
        let mut messages = get_timetable_messages_for_station(&eva).map_err(timetable_error_to_json_err)?;
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


pub fn routes() -> Vec<Route> {
    routes![
        station, timetable_for_station, trains_for_station, stations
    ]
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
    }
}
