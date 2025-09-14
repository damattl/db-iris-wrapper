use rocket::{get, response::status, serde::json::Json, Route, State};
use rocket::http::Status;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec};


use crate::common::JsonResult;
use crate::views::MessageView;
use crate::{common::{error::ErrorBody, params::DateParam}, service::AppService};


#[openapi(tag = "Messages")]
#[get("/<date>/<code>")]
fn messages_for_date_and_code(date: DateParam, code: i32, st: &State<AppService>) -> JsonResult<Vec<MessageView>> {
    let messages = st.message_repo.get_by_date_and_code(&date.0, code).map_err(|e| {
        status::Custom(Status::InternalServerError, Json(ErrorBody {
            code: 500,
            error: "Failed to fetch messages".to_string(),
            message: e.to_string(),
        }))
    })?;

    Ok(Json(messages.iter().map(|m| MessageView::from_model(m, &st.api_base)).collect()))
}

#[openapi(tag = "Messages")]
#[get("/train/<train_id>")]
fn messages_for_train(train_id: &str, st: &State<AppService>) -> JsonResult<Vec<MessageView>> {
    let messages = st.message_repo
        .get_by_train_id(train_id)
        .map_err(|e| {
            status::Custom(Status::NotFound, Json(ErrorBody {
                code: 404,
                error: "Messages not found".to_string(),
                message: e.to_string(),
            }))
        })?;

    Ok(Json(messages.iter().map(|m| MessageView::from_model(m, &st.api_base)).collect()))

}


pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![
        messages_for_date_and_code, messages_for_train
    ]
}
