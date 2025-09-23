use diesel::{prelude::{Associations, Identifiable, Insertable, Queryable}, Selectable};

use crate::data::db::row::MessageRow;

#[derive(Debug, Clone)]
#[derive(Queryable, Selectable, Insertable, Identifiable, Associations)]
#[diesel(table_name = crate::data::db::schema::messages_to_stations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(message_id, station_id))]
#[diesel(belongs_to(MessageRow, foreign_key = message_id))]
pub struct MessageToStationRow {
    pub station_id: i32,
    pub message_id: String,
}
