use rocket::{Build, Rocket};

use crate::service::AppService;

mod index;
mod stations;
mod common;
pub mod service;

pub fn build(service: AppService) -> Rocket<Build> {
    rocket::build().manage(service).mount("/", index::routes()).mount("/stations", stations::routes())
}
