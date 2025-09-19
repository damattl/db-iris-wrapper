use std::{
    env,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};
use std::time::Duration;

use chrono::{Utc, TimeDelta};
use chrono_tz::Europe::Berlin;

use crate::{
    import::{
        import_iris_changes, import_iris_changes_for_station_by_ds100, import_iris_data,
        import_iris_data_for_station_by_ds100, import_station_data, import_status_codes,
    },
    ports::{MessagePort, StationPort, StatusCodePort, StopPort, TrainPort},
};

/// Periodic importer orchestrating station discovery, timetables, and messages.
pub struct ImportService {
    pub station_repo: Arc<dyn StationPort>,
    pub message_repo: Arc<dyn MessagePort>,
    pub train_repo: Arc<dyn TrainPort>,
    pub stop_repo: Arc<dyn StopPort>,
    pub status_code_repo: Arc<dyn StatusCodePort>,

    /// Cooperative shutdown flag for the background loop.
    stop_ch: Arc<AtomicBool>,
}

impl ImportService {
    /// Create a new service. All repos must be `Send + Sync + 'static`.
    pub fn new(
        station_repo: Arc<dyn StationPort>,
        message_repo: Arc<dyn MessagePort>,
        train_repo: Arc<dyn TrainPort>,
        stop_repo: Arc<dyn StopPort>,
        status_code_repo: Arc<dyn StatusCodePort>,
    ) -> Self {
        Self {
            station_repo,
            message_repo,
            train_repo,
            stop_repo,
            status_code_repo,
            stop_ch: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Start a detached 20-minute loop:
    /// - One-off: `import_station_data` and `import_status_codes` (startup).
    /// - Every ~8 h: full timetable import (12 h on first run, then 8 h windows).
    /// - Otherwise: messages-only import for the current local date.
    ///
    /// Errors are logged and do not stop the loop.
    pub fn start(&self) {
        import_station_data(self.station_repo.as_ref()).unwrap(); // TODO: Make this daily.
        import_status_codes(self.status_code_repo.as_ref()).unwrap();

        let stop_ch_clone = Arc::clone(&self.stop_ch);
        let station_repo = Arc::clone(&self.station_repo);
        let message_repo = Arc::clone(&self.message_repo);
        let train_repo = Arc::clone(&self.train_repo);
        let stop_repo = Arc::clone(&self.stop_repo);

        thread::spawn(move || {
            // 3 iterations ≈ 1 hour (20 min sleeps). Threshold 8*3 = 24 iterations ≈ 8 h.
            let mut loop_count = 0;
            let mut first_run = true;
            let single_station = env::var("SINGLE_STATION").ok();

            while !stop_ch_clone.load(Ordering::Relaxed) {
                // Local wall time for IRIS calls/logging.
                let now = Utc::now().with_timezone(&Berlin).naive_local();

                if loop_count >= 8 * 3 || first_run {
                    loop_count = 0;

                    // First run: import 12 h from now. Afterwards: 8 h window shifted by 8 h.
                    let (start, hours_in_advance) = if first_run {
                        first_run = false;
                        (now, 12)
                    } else {
                        (now + TimeDelta::hours(8), 8)
                    };

                    if let Some(ds100) = &single_station {
                        if let Err(err) = import_iris_data_for_station_by_ds100(
                            ds100,
                            &start,
                            hours_in_advance,
                            message_repo.as_ref(),
                            train_repo.as_ref(),
                            stop_repo.as_ref(),
                        ) {
                            error!("Error importing iris data: {}", err);
                        }
                    } else if let Err(err) = import_iris_data(
                        &start,
                        hours_in_advance,
                        station_repo.as_ref(),
                        message_repo.as_ref(),
                        train_repo.as_ref(),
                        stop_repo.as_ref(),
                    ) {
                        error!("Error importing iris data: {}", err);
                    }
                } else {
                    // Messages-only import for today.
                    if let Some(ds100) = &single_station {
                        if let Err(err) = import_iris_changes_for_station_by_ds100(
                            ds100,
                            &now.date(),
                            message_repo.as_ref(),
                            stop_repo.as_ref(),
                        ) {
                            error!("Error importing iris messages: {}", err);
                        }
                    } else if let Err(err) = import_iris_changes(
                        &now.date(),
                        station_repo.as_ref(),
                        message_repo.as_ref(),
                        stop_repo.as_ref(),
                    ) {
                        error!("Error importing iris messages: {}", err);
                    }
                }

                loop_count += 1;
                thread::sleep(Duration::from_secs(20 * 60));
            }

            println!("Thread stopping gracefully.");
        });
    }

    /// Request cooperative shutdown (takes effect after the current sleep).
    pub fn stop(&self) {
        self.stop_ch.store(true, Ordering::Relaxed);
    }
}
