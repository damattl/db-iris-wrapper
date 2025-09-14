use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ErrorBody {
    pub code: i32,
    pub error: String,
    pub message: String,
}
