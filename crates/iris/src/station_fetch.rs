use std::{fs};

use quick_xml::de::from_str;
use crate::station_dto::{GetStationError, GetStationInfosError, IRISStation, StationInfo, StationInfosPayload, Stations};


pub fn get_station(id: &str) -> Result<IRISStation, GetStationError> {
    let body: String = ureq::get(&format!(
        "https://iris.noncd.db.de/iris-tts/timetable/station/{}",
        id
    ))
    //.set("Example-Header", "header value")
    .call().map_err(Box::new)?
    .into_string()?;

    debug!("body: {}", body);

    let stations: Stations = from_str(&body)?;
    let station = stations
        .stations
        .into_iter()
        .find(|s| s.ds100 == id)
        .ok_or_else(|| GetStationError::NotFound(id.to_owned()))?;

    Ok(station)
}


pub fn get_station_infos(path: &str, from_api: bool) -> Result<Vec<StationInfo>, GetStationInfosError> {
    info!("Fetching stations from bahnvorhersage");
    let body = match from_api {
        true => {
            ureq::get(path)
            //.set("Example-Header", "header value")
            .call().map_err(Box::new)?
            .into_string()
        },
        false => {
            fs::read_to_string(path)
        },
    }?;

    let payload: StationInfosPayload = serde_json::from_str(&body)?;
    info!("Number of stations {} fetched from bahnvorhersage", payload.stations.len());
    Ok(payload.stations)
}
