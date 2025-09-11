use rocket::{response::status, serde::json::Json};

use crate::common::error::ErrorBody;

pub mod error;
pub mod params;


pub type JsonResult<T> = Result<Json<T>, status::Custom<Json<ErrorBody>>>;
