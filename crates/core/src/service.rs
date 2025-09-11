use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread};

use std::time::Duration;

use chrono::Local;

use crate::{ports::{MessagePort, StationPort, StopPort, TrainPort}, usecases::{import_iris_data_for_station_by_ds100, import_station_data}};

pub struct ImportService {
    pub station_repo: Arc<dyn StationPort>,
    pub message_repo: Arc<dyn MessagePort<'static>>,
    pub train_repo: Arc<dyn TrainPort<'static>>,
    pub stop_repo: Arc<dyn StopPort<'static>>,

    stop_ch: Arc<AtomicBool>,
}

impl ImportService {
    pub fn new(
        station_repo: Arc<dyn StationPort>,
        message_repo: Arc<dyn MessagePort<'static>>,
        train_repo: Arc<dyn TrainPort<'static>>,
        stop_repo: Arc<dyn StopPort<'static>>,
    ) -> Self {
        Self {
            station_repo,
            message_repo,
            train_repo,
            stop_repo,
            stop_ch: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self) {
        import_station_data(self.station_repo.as_ref()).unwrap(); // TODO: Do this once every day

        let stop_ch_clone = self.stop_ch.clone();
        // let station_repo_clone = self.station_repo.clone();
        let message_repo_clone = self.message_repo.clone();
        let train_repo_clone = self.train_repo.clone();
        let stop_repo_clone = self.stop_repo.clone();
        thread::spawn(move || {
            while !stop_ch_clone.load(Ordering::Relaxed) {
                let datetime = Local::now().naive_local(); // TODO: Should this be utc or local? Decide
                match import_iris_data_for_station_by_ds100(
                    "AH", // TODO: Test for all station
                    &datetime,
                    message_repo_clone.as_ref(),
                    train_repo_clone.as_ref(),
                    stop_repo_clone.as_ref()
                ) {
                    Ok(_) => (),
                    Err(err) => error!("Error while importing iris data: {}", err),
                };


                thread::sleep(Duration::from_secs(3600 * 11)); // TODO: Should always be one less than the max amount fetched in advance
            }
            println!("Thread stopping gracefully.");
        });
    }

    pub fn stop(&self) {
        self.stop_ch.store(true, Ordering::Relaxed);
    }
}
