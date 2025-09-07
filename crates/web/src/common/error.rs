use serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorBody {
    pub error: &'static str,
    pub message: String,
}
