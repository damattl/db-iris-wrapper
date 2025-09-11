use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ErrorBody {
    pub error: &'static str,
    pub message: String,
}
