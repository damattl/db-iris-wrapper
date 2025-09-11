use rocket::{get, response::status, routes, serde::json::Json, Route, State};
use rocket::http::Status;
use wrapper_core::{model::{message::Message}};

use crate::{common::{error::ErrorBody, params::DateParam}, service::AppService};
type JsonErr = status::Custom<Json<ErrorBody>>;

#[get("/")] // Make private later on
fn messages(st: &State<AppService>) -> Result<Json<Vec<Message>>, JsonErr> {
    let messages = st.message_repo.get_all().map_err(|e| {
        status::Custom(Status::InternalServerError, Json(ErrorBody {
            error: "Failed to fetch messages",
            message: e.to_string(),
        }))
    })?;

    Ok(Json(messages))
}

#[get("/<date>/<code>")]
fn messages_for_date_and_code(date: DateParam, code: i32, st: &State<AppService>) -> Result<Json<Vec<Message>>, JsonErr> {
    let messages = st.message_repo.get_by_date_and_code(&date.0, code).map_err(|e| {
        status::Custom(Status::InternalServerError, Json(ErrorBody {
            error: "Failed to fetch messages",
            message: e.to_string(),
        }))
    })?;

    Ok(Json(messages))
}

#[get("/train/<train_id>")]
fn messages_for_train(train_id: &str, st: &State<AppService>) -> Result<Json<Vec<Message>>, JsonErr> {
    let messages = st.message_repo.get_by_train_id(train_id);

    match messages {
        Ok(messages) => Ok(Json(messages)),
        Err(err) => Err(status::Custom(Status::NotFound, Json(ErrorBody {
            error: "Messages not found",
            message: err.to_string(),
        }))), // TODO: Better error message
    }
}


pub fn routes() -> Vec<Route> {
    routes![
        messages, messages_for_date_and_code, messages_for_train
    ]
}
