use rocket::{Build, Rocket};

mod index;
mod station;
mod common;

pub fn build() -> Rocket<Build> {
    rocket::build().mount("/", index::routes()).mount("/station", station::routes())
}
