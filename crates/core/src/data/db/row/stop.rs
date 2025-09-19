use chrono::NaiveDateTime;
use diesel::*;
use crate::model::{Movement, Stop, StopUpdate};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::data::db::schema::stops)]
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
    pub departure_changed_path: Option<String>,
    pub arrival_current: Option<NaiveDateTime>,
    pub departure_current: Option<NaiveDateTime>,
}

impl From<&Stop> for StopRow {
    fn from(
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
            arrival_current: arr_mov.2,
            arrival_planned_path: arr_mov.3,
            arrival_changed_path: arr_mov.4,
            departure_platform: dep_mov.0,
            departure_planned: dep_mov.1,
            departure_current: dep_mov.2,
            departure_planned_path: dep_mov.3,
            departure_changed_path: dep_mov.4
        }
    }
}

impl StopRow {
    pub fn to_stop(&self) -> Stop {
        Stop {
            id: self.id.clone(),
            train_id: self.train_id.clone(),
            station_id: self.station_id,
            departure: movement_from_columns(
                self.departure_platform.clone(),
                self.departure_planned,
                self.departure_current,
                self.departure_planned_path.clone(),
                self.departure_changed_path.clone()
            ),
            arrival: movement_from_columns(
                self.arrival_platform.clone(),
                self.arrival_planned,
                self.arrival_current,
                self.arrival_planned_path.clone(),
                self.arrival_changed_path.clone()
            )
        }
    }
}

type MovementRowPart = (Option<String>, Option<NaiveDateTime>, Option<NaiveDateTime>, Option<String>, Option<String>);

fn movement_to_columns(movement: &Option<Movement>) -> MovementRowPart {
    match movement {
        Some(movement) => {
            let planned_path = movement.planned_path.as_ref().map(|p| p.join(","));
            let current_path = movement.changed_path.as_ref().map(|p| p.join(","));
            (movement.platform.clone(), movement.planned, movement.current, planned_path, current_path)
        }
        None => (None, None, None, None, None)
    }
}

fn movement_from_columns(
    platform: Option<String>,
    planned: Option<NaiveDateTime>,
    current: Option<NaiveDateTime>,
    planned_path: Option<String>,
    changed_path: Option<String>
) -> Option<Movement>  {
    if platform.is_none() && planned.is_none() && planned_path.is_none() && changed_path.is_none() {
        None
    } else {
        Some(Movement {
            platform,
            planned,
            current,
            planned_path: planned_path.map(|p| p.split(',').map(String::from).collect()),
            changed_path: changed_path.map(|p| p.split(',').map(String::from).collect())
        })
    }
}


#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = crate::data::db::schema::stops)]
pub struct StopUpdateRow {
    pub arrival_platform: Option<String>,
    pub arrival_planned: Option<NaiveDateTime>,
    pub arrival_planned_path: Option<String>,
    pub arrival_changed_path: Option<String>,
    pub departure_platform: Option<String>,
    pub departure_planned: Option<NaiveDateTime>,
    pub departure_planned_path: Option<String>,
    pub departure_changed_path: Option<String>,
    pub arrival_current: Option<NaiveDateTime>,
    pub departure_current: Option<NaiveDateTime>,
}


impl From<&StopUpdate> for StopUpdateRow {
    fn from(stop: &StopUpdate) -> Self {
        let dep_mov = movement_to_columns(&stop.departure);
        let arr_mov = movement_to_columns(&stop.arrival);

        StopUpdateRow {
            arrival_platform: arr_mov.0,
            arrival_planned: arr_mov.1,
            arrival_current: arr_mov.2,
            arrival_planned_path: arr_mov.3,
            arrival_changed_path: arr_mov.4,
            departure_platform: dep_mov.0,
            departure_planned: dep_mov.1,
            departure_current: dep_mov.2,
            departure_planned_path: dep_mov.3,
            departure_changed_path: dep_mov.4,
        }
    }
}
