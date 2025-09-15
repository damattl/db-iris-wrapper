use rocket::{fs::{FileServer}, Build, Rocket};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use rocket_okapi::{mount_endpoints_and_merged_docs, settings::OpenApiSettings, swagger_ui::{make_swagger_ui, SwaggerUIConfig}};

use crate::{routes::index::get_static_path, service::AppService};

pub mod service;

mod common;

mod routes;
mod views;

pub fn build(service: AppService) -> Rocket<Build> {
    let cors = CorsOptions {
            allowed_origins: AllowedOrigins::all(), // or restrict with list
            allowed_headers: AllowedHeaders::all(),
            allow_credentials: true,
            ..Default::default()
        }
        .to_cors()
        .expect("error creating CORS fairing");

    let mut builder = rocket::build()
        .attach(cors)
        .manage(service);

    let settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        builder, "/v1".to_owned(), settings,
        "/stations" => routes::stations::routes(),
        "/trains" =>  routes::trains::routes(),
        "/messages" => routes::messages::routes(),
        "/status_codes" => routes::status_codes::routes()
    };
    builder
        .mount(
            "/v1/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount("/", FileServer::from(get_static_path()).rank(10))
        .mount("/", routes::index::routes())

}
