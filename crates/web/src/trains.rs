use rocket::{get, response::status, routes, serde::json::Json, Route, State};
use rocket::http::Status;
use wrapper_core::{model::{train::Train}};

use crate::{common::{error::ErrorBody, params::DateParam}, service::AppService};
type JsonErr = status::Custom<Json<ErrorBody>>;

#[get("/?<date>")] // TODO: Maybe disable this route the more data is available
fn trains(date: Option<DateParam>, st: &State<AppService>) -> Result<Json<Vec<Train>>, JsonErr> {
    let trains = match date {
        Some(date) => {
            st.train_repo.get_by_date(&date.0).map_err(|e| {
                status::Custom(Status::InternalServerError, Json(ErrorBody {
                    error: "Failed to fetch trains",
                    message: e.to_string(),
                }))
            })?
        },
        None => {
            st.train_repo.get_all().map_err(|e| {
                status::Custom(Status::InternalServerError, Json(ErrorBody {
                    error: "Failed to fetch trains",
                    message: e.to_string(),
                }))
            })?
        },
    };


    Ok(Json(trains))
}

#[get("/<number>/<date>")]
fn train(number: &str, date: DateParam, st: &State<AppService>) -> Result<Json<Train>, JsonErr> {
    let id = Train::new_id(number, &date.0);
    let train = st.train_repo.get_by_id(id);

    match train {
        Ok(train) => Ok(Json(train)),
        Err(err) => Err(status::Custom(Status::NotFound, Json(ErrorBody {
            error: "Train not found",
            message: err.to_string(),
        }))), // TODO: Better error message
    }
}


pub fn routes() -> Vec<Route> {
    routes![
        train, trains
    ]
}
