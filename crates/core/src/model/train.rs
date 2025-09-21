use chrono::{NaiveDate};
use iris::{self, dto::get_first_stop_departure_from_stop_id};


#[derive(Debug, Clone)]
pub struct Train {
    pub id: String, // Custom ID: format: number-date
    pub operator: Option<String>,
    pub category: String,
    pub number: String,
    pub line: Option<String>,
    // pub stops: Vec<Stop>,
    pub date: NaiveDate,
}

#[derive(thiserror::Error, Debug)]
pub enum TrainBuildError {
    #[error("missing <tl> element")]
    MissingTL,
    #[error("missing train number")]
    MissingNumber,
    #[error("missing category")]
    MissingCategory,
    #[error("invalid stop date {0}")]
    InvalidStopDate(String),
}

impl Train {
    pub fn new_id(number: &str, date: &NaiveDate) -> String {
        format!("{}-{}", number, date.format("%y%m%d"))
    }
    pub fn from_stop(stop: &iris::dto::Stop) -> Result<Self, TrainBuildError> {
        let tl = stop.tl.as_ref().ok_or(TrainBuildError::MissingTL)?;
        let number = tl.number.as_ref().ok_or(TrainBuildError::MissingNumber)?;

        let date = get_first_stop_departure_from_stop_id(stop)
            .ok_or(TrainBuildError::InvalidStopDate(stop.id.clone()))?
            .date();

        let id = Self::new_id(number, &date);

        let arr = &stop.arrival;
        let dep = &stop.departure;

        let line = if arr.is_some() {
            arr.as_ref().unwrap().line.to_owned()
        } else if dep.is_some() {
            dep.as_ref().unwrap().line.to_owned()
        } else {
            None
        };

        Ok(
            Train {
                id,
                number: number.clone(),
                category: tl.category.as_deref().ok_or(TrainBuildError::MissingCategory)?.to_owned(),
                line,
                operator: tl.operator.to_owned(),
                date,
            }
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    use iris::dto::{Movement as IrisMovement, Stop as IrisStop, TrainLine};

    fn base_stop() -> IrisStop {
        let planned = NaiveDateTime::parse_from_str("2025-09-10 08:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        IrisStop {
            id: "test-stop-2509100800-1".to_string(),
            eva: Some("8002549".to_string()),
            tl: Some(TrainLine {
                f: None,
                t: None,
                operator: Some("DB".to_string()),
                category: Some("ICE".to_string()),
                number: Some("123".to_string()),
            }),
            msgs: Vec::new(),
            arrival: Some(IrisMovement {
                planned: Some(planned),
                current: None,
                platform: None,
                line: None,
                hi: None,
                ppth: None,
                cpth: None,
                cs: None,
                clt: None,
                wings: None,
                msgs: Vec::new(),
            }),
            departure: None,
        }
    }

    #[test]
    fn train_from_stop_builds_expected_identifier() {
        let stop = base_stop();

        let train = Train::from_stop(&stop).expect("expected train to build");

        assert_eq!("123-250910", train.id);
        assert_eq!(Some("DB".to_string()), train.operator);
        assert_eq!("ICE", train.category);
        assert_eq!("123", train.number);
    }

    #[test]
    fn train_from_stop_requires_train_line() {
        let mut stop = base_stop();
        stop.tl = None;

        let err = Train::from_stop(&stop).unwrap_err();
        assert!(matches!(err, TrainBuildError::MissingTL));
    }

    #[test]
    fn train_from_stop_requires_number_and_category() {
        let mut stop_missing_number = base_stop();
        stop_missing_number.tl.as_mut().unwrap().number = None;
        let err = Train::from_stop(&stop_missing_number).unwrap_err();
        assert!(matches!(err, TrainBuildError::MissingNumber));

        let mut stop_missing_category = base_stop();
        stop_missing_category.tl.as_mut().unwrap().category = None;
        let err = Train::from_stop(&stop_missing_category).unwrap_err();
        assert!(matches!(err, TrainBuildError::MissingCategory));
    }
}
