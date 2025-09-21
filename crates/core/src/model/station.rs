use std::num::ParseIntError;


#[derive(Debug, Clone)]
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


#[cfg(test)]
mod tests {
    use super::*;
    use iris::dto::StationInfo;

    fn sample_info(ds100: Option<&str>) -> StationInfo {
        StationInfo {
            eva: 2_000_000_000,
            ds100: ds100.map(|s| s.to_string()),
            lat: 53.5511,
            lon: 9.9937,
            name: "Hamburg Hbf".to_string(),
            is_active_ris: true,
            is_active_iris: true,
            meta_evas: vec![],
            available_transports: vec!["INTERCITY_TRAIN".to_string()],
            number_of_events: Some(0),
        }
    }

    #[test]
    fn station_from_info_requires_ds100() {
        let err = Station::from_info(sample_info(None)).unwrap_err();
        assert!(matches!(err, StationBuildError::MissingDS100));
    }

    #[test]
    fn station_from_info_casts_eva_to_i32() {
        let station = Station::from_info(sample_info(Some("AH"))).expect("station should build");
        assert_eq!(2_000_000_000i32, station.id);
        assert_eq!("AH", station.ds100);
        assert_eq!(Some(53.5511), station.lat);
    }
}
