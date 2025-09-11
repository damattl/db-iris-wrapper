mod common;

use std::collections::HashSet;

use dotenvy::dotenv;
use wrapper_core::{db::{establish_pg_pool, run_migrations}, model::train::Train, ports::Port, repos::{MessageRepo, StationRepo, StopRepo, TrainRepo}, usecases::{import_iris_data, import_iris_data_for_station_by_ds100, import_station_data}};

use chrono::{Local};

use crate::common::{setup_test_postgres};

fn find_duplicates<T: Eq + std::hash::Hash + Clone>(items: &[T]) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();

    for item in items {
        if !seen.insert(item) {
            duplicates.insert(item.clone());
        }
    }
    duplicates.into_iter().collect()
}

#[test]
fn import_iris_data_for_single_station_succeeds() {
    dotenv().ok();
    let _ = pretty_env_logger::try_init();
    // Setup
    let (_container, db_url) = setup_test_postgres(); // _container needs to be kept in scope
    println!("Postgres URL: {}", db_url);
    let pool = establish_pg_pool(&db_url);
    run_migrations(pool.clone());

    let train_repo = TrainRepo::new(pool.clone());
    let stop_repo = StopRepo::new(pool.clone());
    let message_repo = MessageRepo::new(pool.clone());
    let station_repo = StationRepo::new(pool.clone());

    let _ = import_station_data(&station_repo).unwrap();

    // Test

    let date = Local::now().naive_local();
    let (trains, stops, messages) = import_iris_data_for_station_by_ds100("AH", &date, &message_repo, &train_repo, &stop_repo).unwrap();

    let station_id = stops.first().unwrap().station_id;

    assert!(trains.len() > 0);
    assert!(stops.len() > 0);
    assert!(messages.len() > 0);


    let train_ids: Vec<String> = trains.iter().map(|t| t.id.clone()).collect();
    let duplicates = find_duplicates(&train_ids);
    println!("TrainIds: {:?}", duplicates);

    let filtered_trains: Vec<&Train> = trains.iter().filter(|t| duplicates.contains(&t.id)).collect();
    println!("Duplicate Trains:{:#?}", filtered_trains);

    let result = train_repo.get_all().unwrap();

    assert_eq!(trains.len() - duplicates.len(), result.len());

    // Insert stops and test length, account for duplicates
    let stop_ids: Vec<String> = stops.iter().map(|t| t.id.clone()).collect();
    let duplicates = find_duplicates(&stop_ids);

    let result = stop_repo.get_all().unwrap();

    assert_eq!(stops.len() - duplicates.len(), result.len());

    let _ = message_repo.get_all().unwrap();

    // No sense in comparing messages, as there are many duplicates in the data

    // TODO: Think about other ways to validate the data
}


/*  #[test] // Testing all stations is insanity, this takes minutes
fn import_iris_data_succeeds() { // More like a ratelimit tester for the iris endpoint
    dotenv().ok();
    let _ = pretty_env_logger::try_init();
    // Setup
    let (_container, db_url) = setup_test_postgres(); // _container needs to be kept in scope
    println!("Postgres URL: {}", db_url);
    let pool = establish_pg_pool(&db_url);
    run_migrations(pool.clone());

    let train_repo = TrainRepo::new(pool.clone());
    let stop_repo = StopRepo::new(pool.clone());
    let message_repo = MessageRepo::new(pool.clone());
    let station_repo = StationRepo::new(pool.clone());

    let _ = import_station_data(&station_repo).unwrap();

    // Test

    let date = Local::now().naive_local();
    import_iris_data(&date, &station_repo, &message_repo, &train_repo, &stop_repo).unwrap();

    let trains = train_repo.get_all().unwrap();
    let stops = stop_repo.get_all().unwrap();
    let messages = message_repo.get_all().unwrap();

    assert!(trains.len() > 0);
    assert!(stops.len() > 0);
    assert!(messages.len() > 0);
}*/
// TODO: Test wrong station code
// TODO: Test wrong date code
