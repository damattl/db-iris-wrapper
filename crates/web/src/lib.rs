use rocket::{Build, Rocket};

use crate::service::AppService;

pub mod service;

mod common;

// Route modules
mod index;
mod stations;
mod trains;
mod messages;



pub fn build(service: AppService) -> Rocket<Build> {
    rocket::build()
        .manage(service)
        .mount("/", index::routes())
        .mount("/stations", stations::routes())
        .mount("/trains", trains::routes())
        .mount("/messages", messages::routes())
}
