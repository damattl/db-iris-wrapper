use chrono::{NaiveDate};
use iris;

use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::db::schema::trains)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize, JsonSchema)]
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
}

impl Train {
    pub fn new_id(number: &str, date: &NaiveDate) -> String {
        format!("{}-{}", number, date.format("%y%m%d"))
    }
    pub fn from_stop(stop: &iris::dto::Stop, date: &NaiveDate) -> Result<Self, TrainBuildError> {
        let tl = stop.tl.as_ref().ok_or(TrainBuildError::MissingTL)?;
        let number = tl.number.as_ref().ok_or(TrainBuildError::MissingNumber)?;

        let id = Self::new_id(number, date);

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
                date: *date,
            }
        )
    }
}
