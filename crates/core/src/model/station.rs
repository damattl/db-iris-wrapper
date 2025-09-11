use std::num::ParseIntError;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Queryable, Selectable, Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::stations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Station {
    pub id: i32,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: String,
    pub ds100: String,
    //TODO: Perhaps include meta and platforms
}

#[derive(thiserror::Error, Debug)]
pub enum StationBuildError {
    #[error(transparent)]
    IdParsingError(ParseIntError),
    #[error("missing ds100")]
    MissingDS100,
}

impl Station {
    pub fn from_iris(station: iris::dto::IRISStation) -> Result<Self, StationBuildError> {
        Ok(Station {
            id: station.eva.parse::<i32>().map_err(StationBuildError::IdParsingError)?,
            lat: None,
            lon: None,
            name: station.name,
            ds100: station.ds100,
        })
    }

    pub fn from_info(station: iris::dto::StationInfo) -> Result<Self, StationBuildError> {
        Ok(Station {
            id: station.eva as i32, // TODO: Danger: cast from u32 to i32
            lat: Some(station.lat),
            lon: Some(station.lon),
            name: station.name,
            ds100: station.ds100.ok_or(StationBuildError::MissingDS100)?, // TODO: Maybe handle differently
        })
    }
}
