use std::{env, sync::Arc};

use dotenvy::dotenv;
use log::info;
use web::build;
use web::service::AppService;
use wrapper_core::{data::{establish_default_pg_pool, run_migrations}, data::repos::{MessageRepo, StationRepo, StatusCodeRepo, StopRepo, TrainRepo}, service::ImportService};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    pretty_env_logger::init();
    info!("Logger init");
    // Setup Database
    let pool = establish_default_pg_pool();
    run_migrations(pool.clone());

    let service = AppService {
        api_base: env::var("API_BASE").unwrap_or(String::from("http://localhost:8080/v1")), // TODO: Maybe the service should panic if this is not set
        station_repo: Arc::new(StationRepo::new(pool.clone())),
        message_repo: Arc::new(MessageRepo::new(pool.clone())),
        train_repo: Arc::new(TrainRepo::new(pool.clone())),
        stop_repo: Arc::new(StopRepo::new(pool.clone())),
        status_code_repo: Arc::new(StatusCodeRepo::new(pool.clone())),
    };

    let import_service = ImportService::new(
        service.station_repo.clone(),
        service.message_repo.clone(),
        service.train_repo.clone(),
        service.stop_repo.clone(),
        service.status_code_repo.clone()
    );

    import_service.start();

    build(service).launch().await?;

    import_service.stop();
    Ok(())
}
