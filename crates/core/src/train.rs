use chrono::NaiveDate;
use iris::{fetch::get_timetable_for_station};

use super::model::train::Train;

// Date, Number, Category, Line, Operator
// TODO: Move this to domain

#[derive(thiserror::Error, Debug)]
pub enum GetTrainsError {
    #[error("failed to fetch timetable for station {0} at {1}:{2}, Reason: {3}")]
    TimetableError(String, String, u16, String),
    #[error("missing train number")]
    MissingNumber,
}

pub fn get_trains_for_station(eva: i32, date: &NaiveDate) -> Result<Vec<Train>, GetTrainsError> {
    let mut trains = Vec::<Train>::new();
    for n in 1..24 {
        let tt = get_timetable_for_station(eva, date, n).map_err(|e| {
            error!("Error fetching trains {}", e);
            GetTrainsError::TimetableError(eva.to_string(), date.to_string(), n, e.to_string())
        })?;

        for stop in tt.stops {
            match Train::from_stop(&stop, date) {
                Ok(train) => trains.push(train),
                Err(e) => println!("TrainBuildError: {}", e),
            };
        }
    }
    Ok(trains)
}
