use rocket::{get, response::status, serde::json::Json, Route, State};
use rocket::http::Status;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use wrapper_core::model::stop::Stop;
use wrapper_core::{model::{train::Train}};

use crate::common::JsonResult;
use crate::views::{TrainView};
use crate::{common::{error::ErrorBody, params::DateParam}, service::AppService};

#[openapi(tag = "Trains")]
#[get("/on/<date>")] // TODO: Maybe disable this route the more data is available
fn trains(date: DateParam, st: &State<AppService>) -> JsonResult<Vec<TrainView>> {
    let trains = st.train_repo.get_by_date(&date.0).map_err(|e| {
        status::Custom(Status::InternalServerError, Json(ErrorBody {
            error: "Failed to fetch trains".to_string(),
            message: e.to_string(),
        }))
    })?;

    let train_views = trains.iter().map(|t| TrainView::from_model(t, &[])).collect();

    Ok(Json(train_views))
}

#[openapi(tag = "Trains")]
#[get("/<id>?<include_stops>")]
fn train_by_id(id: &str, include_stops: Option<bool>, st: &State<AppService>) -> JsonResult<TrainView> {
    let train = st.train_repo.get_by_id(id.to_string()).map_err(|e| {
        status::Custom(Status::NotFound, Json(ErrorBody {
            error: "Train not found".to_string(),
            message: e.to_string(),
        }))
    })?;

    let stops = match include_stops.unwrap_or(false) {
        true => {
            st.stop_repo.get_for_train(&train.id).map_err(|e| {
                status::Custom(Status::InternalServerError, Json(ErrorBody {
                    error: format!("Failed to fetch stops for train {}", train.id),
                    message: e.to_string(),
                }))
            })?
        },
        false => {
            Vec::<Stop>::new()
        }
    };

    Ok(Json(TrainView::from_model(&train, &stops)))
}

#[openapi(tag = "Trains")]
#[get("/<number>/<date>?<include_stops>")]
fn train(number: &str, include_stops: Option<bool>, date: DateParam, st: &State<AppService>) -> JsonResult<TrainView> {
    let id = Train::new_id(number, &date.0);
    train_by_id(&id, include_stops, st)
}


pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![
        train, trains, train_by_id
    ]
}
