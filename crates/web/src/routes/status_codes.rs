use rocket::{get, http::Status, response::status, serde::json::Json, Route, State};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec};

use crate::{common::{error::ErrorBody, JsonResult}, service::AppService, views::StatusCodeView};

#[openapi(tag = "Status Codes")]
#[get("/")]
fn status_codes(st: &State<AppService>) -> JsonResult<Vec<StatusCodeView>> {
    let trains = st.status_code_repo.get_all().map_err(|e| {
        status::Custom(Status::InternalServerError, Json(ErrorBody {
            code: 500,
            error: "Failed to fetch status codes".to_string(),
            message: e.to_string(),
        }))
    })?;

    let train_views = trains.iter().map(StatusCodeView::from_model).collect();

    Ok(Json(train_views))
}

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![
        status_codes
    ]
}
