use chrono::NaiveDateTime;
use diesel::*;

use crate::model::stop::{Movement, Stop};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::stops)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StopRow {
    pub id: String,
    pub train_id: String,
    pub station_id: i32,
    pub arrival_platform: Option<String>,
    pub arrival_planned: Option<NaiveDateTime>,
    pub arrival_planned_path: Option<String>,
    pub arrival_changed_path: Option<String>,
    pub departure_platform: Option<String>,
    pub departure_planned: Option<NaiveDateTime>,
    pub departure_planned_path: Option<String>,
    pub departure_changed_path: Option<String>
}

impl StopRow {
    pub fn from_stop(
        stop: &Stop,
    ) -> Self {

        let dep_mov = movement_to_columns(&stop.departure);
        let arr_mov = movement_to_columns(&stop.arrival);

        StopRow {
            id: stop.id.to_owned(),
            train_id: stop.train_id.to_owned(),
            station_id: stop.station_id,
            arrival_platform: arr_mov.0,
            arrival_planned: arr_mov.1,
            arrival_planned_path: arr_mov.2,
            arrival_changed_path: arr_mov.3,
            departure_platform: dep_mov.0,
            departure_planned: dep_mov.1,
            departure_planned_path: dep_mov.2,
            departure_changed_path: dep_mov.3
        }
    }

    pub fn to_stop(&self) -> Stop {
        Stop {
            id: self.id.clone(),
            train_id: self.train_id.clone(),
            station_id: self.station_id,
            departure: movement_from_columns(
                self.departure_platform.clone(),
                self.departure_planned,
                self.departure_planned_path.clone(),
                self.departure_changed_path.clone()
            ),
            arrival: movement_from_columns(
                self.arrival_platform.clone(),
                self.arrival_planned,
                self.arrival_planned_path.clone(),
                self.arrival_changed_path.clone()
            )
        }
    }
}


fn movement_to_columns(movement: &Option<Movement>) -> (Option<String>, Option<NaiveDateTime>, Option<String>, Option<String>) {
    match movement {
        Some(movement) => {
            let planned_path = movement.planned_path.as_ref().map(|p| p.join(","));
            let current_path = movement.changed_path.as_ref().map(|p| p.join(","));
            (movement.platform.clone(), movement.planned, planned_path, current_path)
        }
        None => (None, None, None, None)
    }
}

fn movement_from_columns(
    platform: Option<String>,
    planned: Option<NaiveDateTime>,
    planned_path: Option<String>,
    changed_path: Option<String>
) -> Option<Movement>  {
    if platform.is_none() && planned.is_none() && planned_path.is_none() && changed_path.is_none() {
        None
    } else {
        Some(Movement {
            platform,
            planned,
            planned_path: planned_path.map(|p| p.split(',').map(String::from).collect()),
            changed_path: changed_path.map(|p| p.split(',').map(String::from).collect())
        })
    }
}
