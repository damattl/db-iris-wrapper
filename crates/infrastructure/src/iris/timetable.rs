use chrono::NaiveDate;
use quick_xml::de::from_str;

use crate::iris::timetable_dto::{Timetable, GetTimetableError};



pub fn get_timetable_for_station(
    station_id: &str,
    date: &NaiveDate,
    time: &str, // TODO: Not sure if this is the time
) -> Result<Timetable, GetTimetableError> {
    let date_str = date.format("%y%m%d").to_string();

    println!(
        "Fetching timetable for station {} on {} at {}",
        station_id, date_str, time
    );
    let url = format!(
        "https://iris.noncd.db.de/iris-tts/timetable/plan/{}/{}/{}",
        station_id, date_str, time
    );
    println!("URL: {}", url);
    let body: String = ureq::get(&url)
        //.set("Example-Header", "header value")
        .call().map_err(Box::new)?
        .into_string()?;

    let timetable: Timetable = from_str(&body)?;

    println!("Body: {}", body);
    Ok(timetable)
}

pub fn get_timetable_messages_for_station(
    station_id: &str,
) -> Result<Timetable, GetTimetableError> {
    println!("Fetching timetable messages for station {}", station_id);
    let url = format!(
        "https://iris.noncd.db.de/iris-tts/timetable/fchg/{}",
        station_id
    );
    println!("URL: {}", url);
    let body: String = ureq::get(&url)
        //.set("Example-Header", "header value")
        .call().map_err(Box::new)?
        .into_string()?;

    let timetable: Timetable = from_str(&body)?;

    Ok(timetable)
}
