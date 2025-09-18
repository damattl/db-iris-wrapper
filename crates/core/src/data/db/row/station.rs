use diesel::{prelude::{Insertable, Queryable, QueryableByName}, Selectable};

use crate::model::Station;

#[derive(Debug, Clone)]
#[derive(Queryable, QueryableByName, Selectable, Insertable)]
#[diesel(table_name = crate::data::db::schema::stations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StationRow {
    pub id: i32,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: String,
    pub ds100: String,
    //TODO: Perhaps include meta and platforms
}

impl From<&StationRow> for Station {
    fn from(row: &StationRow) -> Self {
        Station {
            id: row.id,
            lat: row.lat,
            lon: row.lon,
            name: row.name.clone(),
            ds100: row.ds100.clone(),
        }
    }
}

impl From<StationRow> for Station {
    fn from(row: StationRow) -> Self {
        Station::from(&row)
    }
}

impl From<&Station> for StationRow {
    fn from(station: &Station) -> Self {
        StationRow {
            id: station.id,
            lat: station.lat,
            lon: station.lon,
            name: station.name.clone(),
            ds100: station.ds100.clone(),
        }
    }
}

impl From<Station> for StationRow {
    fn from(station: Station) -> Self {
        StationRow::from(&station)
    }
}
