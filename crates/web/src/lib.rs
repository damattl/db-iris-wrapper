use rocket::{Build, Rocket};
use rocket_okapi::{mount_endpoints_and_merged_docs, settings::OpenApiSettings, swagger_ui::{make_swagger_ui, SwaggerUIConfig}};

use crate::service::AppService;

pub mod service;

mod common;

// Route modules
mod index;
mod stations;
mod trains;
mod messages;



pub fn build(service: AppService) -> Rocket<Build> {
    let mut builder = rocket::build().manage(service);
    let settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        builder, "/v1".to_owned(), settings,
        "/stations" => stations::routes(),
        "/trains" =>  trains::routes(),
        "/messages" => messages::routes()
    };
    builder
        .mount("/", index::routes())
        .mount(
        "/v1/swagger",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../openapi.json".to_owned(),
            ..Default::default()
        }),
    )
}
