use diesel::*;
use chrono::NaiveDate;

use crate::model::Train;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::data::db::schema::trains)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone)]
pub struct TrainRow {
    pub id: String, // Custom ID: format: number-date
    pub operator: Option<String>,
    pub category: String,
    pub number: String,
    pub line: Option<String>,
    // pub stops: Vec<Stop>,
    pub date: NaiveDate,
}

impl From<&TrainRow> for Train {
    fn from(row: &TrainRow) -> Self {
        Train {
            id: row.id.clone(),
            operator: row.operator.clone(),
            category: row.category.clone(),
            number: row.number.clone(),
            line: row.line.clone(),
            date: row.date,
        }
    }
}

impl From<TrainRow> for Train {
    fn from(row: TrainRow) -> Self {
        Train::from(&row)
    }
}

impl From<&Train> for TrainRow {
    fn from(train: &Train) -> Self {
        TrainRow {
            id: train.id.clone(),
            operator: train.operator.clone(),
            category: train.category.clone(),
            number: train.number.clone(),
            line: train.line.clone(),
            date: train.date,
        }
    }
}

impl From<Train> for TrainRow {
    fn from(train: Train) -> Self {
        TrainRow::from(&train)
    }
}
