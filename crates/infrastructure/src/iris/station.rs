use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use crate::iris::station_dto::{GetStationError, IRISStation, Stations};


pub fn get_station(id: &str) -> Result<IRISStation, GetStationError> {
    let body: String = ureq::get(&format!(
        "https://iris.noncd.db.de/iris-tts/timetable/station/{}",
        id
    ))
    //.set("Example-Header", "header value")
    .call().map_err(Box::new)?
    .into_string()?;

    println!("body: {}", body);

    let stations: Stations = from_str(&body)?;
    let station = stations
        .stations
        .into_iter()
        .find(|s| s.ds100 == id)
        .ok_or_else(|| GetStationError::NotFound(id.to_owned()))?;

    Ok(station)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationInfosPayload {
    pub stations: Vec<StationInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationInfo {
    pub eva: u64,
    pub ds100: Option<String>,
    pub lat: f64,
    pub lon: f64,
    pub name: String,
    pub is_active_ris: bool,
    pub is_active_iris: bool,
    pub meta_evas: Vec<u64>,
    pub available_transports: Vec<String>,
    pub number_of_events: Option<u64>,
}

#[derive(thiserror::Error, Debug)]
pub enum GetStationInfosError {
    #[error(transparent)]
    Network(#[from] Box<ureq::Error>),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub fn get_station_infos() -> Result<Vec<StationInfo>, GetStationInfosError> {
    let body: String = ureq::get("https://bahnvorhersage.de/api/stations.json")
    //.set("Example-Header", "header value")
    .call().map_err(Box::new)?
    .into_string()?;

    let payload: StationInfosPayload = serde_json::from_str(&body)?;

    Ok(payload.stations)
}
