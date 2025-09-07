use chrono::NaiveDate;
use infrastructure::iris::{timetable::get_timetable_for_station, timetable_dto::Stop};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Train {
    pub id: String,
    pub number: String,
    pub category: String,
    pub line: String,
    pub operator: String,
    pub date: NaiveDate,
}

#[derive(thiserror::Error, Debug)]
pub enum TrainBuildError {
    #[error("missing <tl> element")]
    MissingTL,
    #[error("missing train number")]
    MissingNumber,
}

impl Train {
    pub fn from_stop(stop: &Stop, date: &NaiveDate) -> Result<Self, TrainBuildError> {
        let tl = stop.tl.as_ref().ok_or(TrainBuildError::MissingTL)?;
        let number = tl.number.as_ref().ok_or(TrainBuildError::MissingNumber)?;

        let id = format!("{}-{}", number, date.format("%y%m%d"));

        let arr = &stop.arrival;
        let dep = &stop.departure;

        let line = if arr.is_some() {
            arr.as_ref().unwrap().line.as_deref().unwrap_or("unknown")
        } else if dep.is_some() {
            dep.as_ref().unwrap().line.as_deref().unwrap_or("unknown")
        } else {
            "unknown"
        };

        Ok(
            Train {
                id,
                number: number.clone(),
                category: tl.category.as_deref().unwrap_or("unknown").to_owned(),
                line: line.to_owned(),
                operator: tl.operator.as_deref().unwrap_or("unknown").to_owned(),
                date: *date,
            }
        )
    }
}

// Date, Number, Category, Line, Operator
// TODO: Move this to domain

#[derive(thiserror::Error, Debug)]
pub enum GetTrainsError {
    #[error("failed to fetch timetable for station {0} at {1}:{2}, Reason: {3}")]
    TimetableError(String, String, String, String),
    #[error("missing train number")]
    MissingNumber,
}

pub fn get_trains_for_station(eva: &str, date: &NaiveDate) -> Result<Vec<Train>, GetTrainsError> {

    let mut trains = Vec::<Train>::new();
    for n in 1..24 {
        let time = format!("{:02}", n);
        let tt = get_timetable_for_station(eva, date, &time).map_err(|e| {
            GetTrainsError::TimetableError(eva.to_string(), date.to_string(), time.to_string(), e.to_string())
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
