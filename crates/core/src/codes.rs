use std::error::Error;

use iris::{self, dto::Stop};

use crate::model::Station;

pub fn get_all_stops_with_codes(id: &str, codes: Vec<i32>) -> Result<Vec<Stop>, Box<dyn Error>> {
    let station = iris::fetch::get_station(id).map(Station::from_iris)??;
    let messages = iris::fetch::get_timetable_changes_for_station(station.id)?;

    let stops = messages.stops.into_iter().filter(|s| {
        if s.msgs.iter().any(|m| codes.contains(&m.code.unwrap_or(200))) {
            return true;
        }

        if s.arrival.is_some() && s.arrival.as_ref().unwrap().msgs.iter().any(|m| codes.contains(&m.code.unwrap_or(200))) {
            return true;
        }

        if s.departure.is_some() && s.departure.as_ref().unwrap().msgs.iter().any(|m| codes.contains(&m.code.unwrap_or(200))) {
            return true;
        }

        false
    }).collect();

    Ok(stops)
}
