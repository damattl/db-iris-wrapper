use rocket::{get, Route};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec};

#[openapi]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![
        index,
    ]
}
