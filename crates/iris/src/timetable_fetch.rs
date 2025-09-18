use chrono::NaiveDate;
use log::{debug, info};
use quick_xml::de::from_str;

use crate::timetable_dto::{Timetable, IRISTimetableError};


pub fn get_timetable_for_station(
    station_id: i32,
    date: &NaiveDate,
    time: u16, // TODO: Not sure if this is the time
) -> Result<Timetable, IRISTimetableError> {
    let date_str = date.format("%y%m%d").to_string();

    info!(
        "Fetching timetable for station {} on {} at {:02}",
        station_id, date_str, time
    );
    let url = format!(
        "https://iris.noncd.db.de/iris-tts/timetable/plan/{}/{}/{:02}",
        station_id, date_str, time
    );
    info!("URL: {}", url);
    let response = ureq::get(&url)
        //.set("Example-Header", "header value")
        .call()
        .inspect_err(|err| error!("Error fetching timetable: {}", err))
        .map_err(Box::new)?;

    let status = response.status();
    let body = response.into_string()?;

    if status != 200 {
        warn!("Fetching timetable resulted in status code {}", status);
        return Err(IRISTimetableError::RequestFailed(status, body));
    }

    if body.starts_with("<timetable/>") {
        return Err(IRISTimetableError::EmptyTimetable(time));
    }
    debug!("Body: {}", body);
    let timetable: Timetable = from_str(&body).inspect_err(|_| {
        error!("Error parsing Timetable Body {}", body);
    })?;


    Ok(timetable)
}

pub fn get_timetable_messages_for_station(
    station_id: i32,
) -> Result<Timetable, IRISTimetableError> {
    info!("Fetching timetable messages for station {}", station_id);
    let url = format!(
        "https://iris.noncd.db.de/iris-tts/timetable/fchg/{}",
        station_id
    );
    info!("URL: {}", url);
    let body: String = ureq::get(&url)
        //.set("Example-Header", "header value")
        .call().map_err(Box::new)?
        .into_string()?;

    if body.starts_with("<timetable/>") {
        return Err(IRISTimetableError::EmptyTimetable(25));
    }
    debug!("Body: {}", body);
    let timetable: Timetable = from_str(&body).inspect_err(|_| {
        error!("Error parsing timetable messages body {}", body);
    })?;

    Ok(timetable)
}
