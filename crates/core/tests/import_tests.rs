mod common;

use std::collections::HashSet;

use wrapper_core::{db::{establish_pg_pool, run_migrations}, repos::{MessageRepo, StationRepo, StopRepo, TrainRepo}, model::train::Train, ports::Port, usecases::{import_iris_data_for_station_by_ds100, import_station_data}};

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
fn import_iris_data_succeeds() {
    // Setup
    let (_container, db_url) = setup_test_postgres(); // _container needs to be kept in scope
    println!("Postgres URL: {}", db_url);
    let pool = establish_pg_pool(&db_url);
    run_migrations(pool.clone());

    let train_repo = TrainRepo::new(pool.clone());
    let stop_repo = StopRepo::new(pool.clone());
    let message_repo = MessageRepo::new(pool.clone());
    let station_repo = StationRepo::new(pool.clone());

    let stations = import_station_data(&station_repo).unwrap();
    println!("Stations: {:#?}", stations);

    // Test

    let date = Local::now().naive_local();
    let (trains, stops, messages) = import_iris_data_for_station_by_ds100("AH", &date, &message_repo, &train_repo, &stop_repo).unwrap();

    let station_id = stops.first().unwrap().station_id;


    let station = station_repo.get_by_id(station_id).unwrap();
    println!("Station {:#?}", station);

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
    println!("Trains: {:#?}", trains);
}

// TODO: Test wrong station code
// TODO: Test wrong date code
