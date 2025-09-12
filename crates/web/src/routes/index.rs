use rocket::{get, routes, Route};
use rocket_okapi::{openapi};

#[openapi]
#[get("/")]
fn index() -> &'static str {
    "Wir wünschen eine gute Fahrt"
}

pub fn routes() -> Vec<Route> {
    routes![
        index,
    ]
}
