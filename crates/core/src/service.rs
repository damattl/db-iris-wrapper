use std::{
    env, sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    }, thread
};
use std::time::Duration;

use chrono::Utc;
use chrono_tz::Europe::Berlin;

use crate::{
    ports::{MessagePort, StationPort, StopPort, TrainPort},
    usecases::{
        import_iris_data, import_iris_data_for_station_by_ds100, import_iris_messages, import_iris_messages_for_station_by_ds100, import_station_data
    },
};

/// Periodic importer orchestrating station discovery, timetables, and messages.
///
/// See the module-level docs for scheduling, shutdown, and time semantics.
pub struct ImportService {
    /// Repository for station persistence and lookup.
    pub station_repo: Arc<dyn StationPort>,
    /// Repository for message persistence and lookup.
    pub message_repo: Arc<dyn MessagePort>,
    /// Repository for train persistence and lookup.
    pub train_repo: Arc<dyn TrainPort>,
    /// Repository for stop persistence and lookup.
    pub stop_repo: Arc<dyn StopPort>,

    /// Cooperative shutdown flag checked by the background worker.
    ///
    /// When set to `true`, the worker exits after completing the next sleep
    /// cycle. The flag is **not** reset automatically.
    stop_ch: Arc<AtomicBool>,
}

impl ImportService {
    /// Construct a new [`ImportService`].
    ///
    /// # Requirements
    /// The trait objects behind the `Arc`s must be `Send + Sync + 'static`
    /// because they are used by a spawned background thread.
    pub fn new(
        station_repo: Arc<dyn StationPort>,
        message_repo: Arc<dyn MessagePort>,
        train_repo: Arc<dyn TrainPort>,
        stop_repo: Arc<dyn StopPort>,
    ) -> Self {
        Self {
            station_repo,
            message_repo,
            train_repo,
            stop_repo,
            stop_ch: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Start the background worker.
    ///
    /// Behavior:
    /// 1. Runs a **one-off** station discovery & persist via
    ///    [`import_station_data`]. *(Planned: make this daily.)*
    /// 2. Spawns a thread that executes a 20-minute loop:
    ///    - Every 33rd iteration (≈ 11 h): full 12-hour timetable import for
    ///      DS100 `"AH"`.
    ///    - Otherwise: messages-only import for the current local date.
    ///
    /// Errors from periodic tasks are **logged** and do not terminate the loop.
    ///
    /// # Panics
    /// Panics if the initial call to [`import_station_data`] fails, due to the
    /// `unwrap()` at startup.
    ///
    /// # Notes
    /// - The method returns immediately; the worker runs detached.
    /// - The thread `JoinHandle` is not exposed; use [`stop`](Self::stop) to
    ///   request a graceful stop.
    pub fn start(&self) {
        import_station_data(self.station_repo.as_ref()).unwrap(); // TODO: Do this once every day

        let stop_ch_clone = self.stop_ch.clone();
        let station_repo_clone = self.station_repo.clone();
        let message_repo_clone = self.message_repo.clone();
        let train_repo_clone = self.train_repo.clone();
        let stop_repo_clone = self.stop_repo.clone();
        thread::spawn(move || {
            let mut loop_count = 11 * 3 + 1;
            let single_station = env::var("SINGLE_STATION");

            while !stop_ch_clone.load(Ordering::Relaxed) {
                let datetime = Utc::now().with_timezone(&Berlin).naive_local();
                // Doesn't make much of a difference if UTC or Berlin, but makes the intention clearer

                if loop_count >= 11 * 3 {
                    loop_count = 0;
                    // Every 11 hours, but the loop sleeps for 20 minutes each, so 3 loops = 1 hour
                    // Combined with the time required for fetching the data this might result in some time drift

                    match single_station {
                        Ok(_) => {
                            let result = import_iris_data_for_station_by_ds100(
                                "AH", // TODO: Test for all stations
                                &datetime,
                                message_repo_clone.as_ref(),
                                train_repo_clone.as_ref(),
                                stop_repo_clone.as_ref(),
                            );
                            match result {
                                Ok(_) => (),
                                Err(err) => error!("Error importing iris data: {}", err),
                            }
                        },
                        Err(_) => {
                            let result = import_iris_data(
                                &datetime,
                                station_repo_clone.as_ref(),
                                message_repo_clone.as_ref(),
                                train_repo_clone.as_ref(),
                                stop_repo_clone.as_ref(),
                            );
                            match result {
                                Ok(_) => (),
                                Err(err) => error!("Error importing iris data: {}", err),
                            }
                        },
                    }
                } else {
                    match single_station {
                        Ok(_) => {
                            match import_iris_messages_for_station_by_ds100(
                                "AH", // TODO: Test for all stations
                                &datetime.date(),
                                message_repo_clone.as_ref(),
                                stop_repo_clone.as_ref(),
                            ) {
                                Ok(_) => (),
                                Err(err) => error!("Error importing iris messages: {}", err),
                            };
                        },
                        Err(_) => {
                            match import_iris_messages(
                                &datetime.date(),
                                station_repo_clone.as_ref(),
                                message_repo_clone.as_ref(),
                                stop_repo_clone.as_ref(),
                            ) {
                                Ok(_) => (),
                                Err(err) => error!("Error importing iris messages: {}", err),
                            };
                        },
                    }


                }
                loop_count += 1;

                thread::sleep(Duration::from_secs(60 * 20));
            }
            println!("Thread stopping gracefully.");
        });
    }

    /// Request the background worker to stop.
    ///
    /// Sets the atomic flag read by the loop in [`start`](Self::start). The
    /// worker exits after the current sleep concludes, so expect up to
    /// **20 minutes** of latency. The flag remains set; calling `start` again on
    /// the same instance will not restart the worker.
    pub fn stop(&self) {
        self.stop_ch.store(true, Ordering::Relaxed);
    }
}
