use chrono::{NaiveDate, NaiveDateTime};
use diesel::{BoolExpressionMethods, Connection, ExpressionMethods, JoinOnDsl, OptionalEmptyChangesetExtension, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{model::{Message, Station, StatusCode, Stop, StopUpdate, Train, StopWithStation}, ports::{MessagePort, Port, PortError, StationPort, StatusCodePort, TrainPort, StopPort}};
use super::db::{schema::{stations, trains, status_codes, messages, stops}, PgPool, run_sql_file, row::{StatusCodeRow, StopRow, MessageRow, StationRow, TrainRow, StopUpdateRow}};

fn map_pool_err<E>(err: E) -> PortError
where E: std::error::Error {
    error!("{}", err);
    PortError::Connection
}

fn map_query_result_err(err: diesel::result::Error) -> PortError {
    error!("QueryResultError: {:#?}, {}", err, err);
    match err {
        diesel::result::Error::DatabaseError(_, _) => PortError::Database, // TODO: Handle kind
        diesel::result::Error::NotFound => PortError::NotFound,
        diesel::result::Error::QueryBuilderError(_) => PortError::InvalidInput,
        diesel::result::Error::DeserializationError(_) => PortError::MalformedData,
        diesel::result::Error::SerializationError(_) => PortError::InvalidInput,
        err => PortError::Custom(Box::new(err)),
    }
}

pub struct StationRepo {
    pool: PgPool,
}

impl StationRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Port<Station, i32> for StationRepo {
    fn persist(&self, station: &Station) -> Result<Station, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;
        let result = diesel::insert_into(stations::table)
            .values(StationRow::from(station))
            .on_conflict_do_nothing()
            .returning(StationRow::as_returning())
            .get_result::<StationRow>(&mut conn)
            .map_err(map_query_result_err)?;
        Ok(Station::from(result))
    }

    fn persist_all(&self, stations: &[Station]) -> Result<Vec<Station>, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;
        let result = diesel::insert_into(stations::table)
            .values(stations.iter().map(StationRow::from).collect::<Vec<StationRow>>())
            .on_conflict_do_nothing()
            .returning(StationRow::as_returning())
            .get_results::<StationRow>(&mut conn)
            .map_err(map_query_result_err)?;
        Ok(result.iter().map(Station::from).collect())
    }

    fn get_by_id(&self, id: i32) -> Result<Station, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;
        let row = stations::table
            .find(id)
            .select(StationRow::as_select())
            .first(&mut conn)
            .map_err(map_query_result_err)?;
        Ok(Station::from(row))
    }

    fn get_all(&self) -> Result<Vec<Station>, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;
        let rows = stations::table
            .select(StationRow::as_select())
            .get_results::<StationRow>(&mut conn)
            .map_err(map_query_result_err)?;
        Ok(rows.iter().map(Station::from).collect())
    }
}

impl StationPort for StationRepo {
    fn get_by_ds100(&self, ds100: &str) -> Result<Station, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;
        let row = stations::table
            .filter(stations::ds100.eq(ds100))
            .select(StationRow::as_select())
            .first::<StationRow>(&mut conn)
            .map_err(map_query_result_err)?;
        Ok(Station::from(row))
    }

    fn import_from_sql(&self, path: &str) -> Result<Vec<Station>, PortError> {
        run_sql_file::<StationRow>(&self.pool, path)
            .map(|rows| rows.into_iter().map(Station::from).collect())
            .map_err(|e| {
                error!("Error importing stations from SQL file: {}", e);
                PortError::Custom(e)
            })
    }
}


pub struct TrainRepo {
    pool: PgPool,
}

impl TrainRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl TrainPort for TrainRepo {
    fn get_by_station_and_date(&self, station: &Station, date: &NaiveDate) -> Result<Vec<Train>, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;

        let results = trains::table
            .inner_join(stops::table.on(stops::train_id.eq(trains::id)))
            .inner_join(stations::table.on(stops::station_id.eq(stations::id)))
            .filter(stations::id.eq(station.id).and(trains::date.eq(date)))
            .select(TrainRow::as_select())
            .load::<TrainRow>(&mut conn)
            .map_err(map_query_result_err)?;

        Ok(results.iter().map(Train::from).collect())
    }

    fn get_by_date(&self, date: &NaiveDate) -> Result<Vec<Train>, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;
        let rows = trains::table
            .filter(trains::date.eq(date))
            .select(TrainRow::as_select())
            .get_results::<TrainRow>(&mut conn)
            .map_err(map_query_result_err)?;
        Ok(rows.iter().map(Train::from).collect())
    }
}

impl Port<Train, String> for TrainRepo {
    fn persist(&self, train: &Train) -> Result<Train, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;
        let row = diesel::insert_into(trains::table)
            .values(TrainRow::from(train))
            .on_conflict_do_nothing()
            .returning(TrainRow::as_returning())
            .get_result::<TrainRow>(&mut conn)
            .map_err(map_query_result_err)?;
        Ok(Train::from(row))
    }

    fn persist_all(&self, trains: &[Train]) -> Result<Vec<Train>, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;
        let rows = diesel::insert_into(trains::table)
            .values(trains.iter().map(TrainRow::from).collect::<Vec<TrainRow>>())
            .on_conflict_do_nothing()
            .returning(TrainRow::as_returning())
            .get_results::<TrainRow>(&mut conn)
            .map_err(map_query_result_err)?;
        Ok(rows.iter().map(Train::from).collect())
    }

    fn get_by_id(&self, id: String) -> Result<Train, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;
        let row = trains::table
            .find(id)
            .select(TrainRow::as_select())
            .first::<TrainRow>(&mut conn)
            .map_err(map_query_result_err)?;
        Ok(Train::from(row))
    }

    fn get_all(&self) -> Result<Vec<Train>, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;
        let rows = trains::table
            .select(TrainRow::as_select())
            .get_results::<TrainRow>(&mut conn)
            .map_err(map_query_result_err)?;
        Ok(rows.iter().map(Train::from).collect())
    }
}





/*
 * StopRepo
 */

pub struct StopRepo {
    pool: PgPool
}

impl StopRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

 impl StopPort for StopRepo {
     fn get_for_date(&self, date: &chrono::NaiveDate) -> Result<Vec<Stop>, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;

         let start: NaiveDateTime = date.and_hms_opt(0, 0, 0).unwrap();
         let end: NaiveDateTime = (date.succ_opt().unwrap()).and_hms_opt(0, 0, 0).unwrap();

         stops::table
             .filter(stops::arrival_planned.ge(start))
             .filter(stops::arrival_planned.lt(end))
             .select(StopRow::as_select())
             .get_results(&mut conn)
             .map_err(map_query_result_err)
             .map(|v| v.iter().map(|s| s.to_stop()).collect())
     }

     fn get_for_train(&self, train_id: &str) -> Result<Vec<Stop>, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;
         stops::table
             .filter(stops::train_id.eq(train_id))
             .select(StopRow::as_select())
             .get_results(&mut conn)
             .map_err(map_query_result_err)
             .map(|v| v.iter().map(|s| s.to_stop()).collect())
     }

    fn get_for_train_with_station(&self, train_id: &str) -> Result<Vec<StopWithStation>, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;
        let results = stops::table
                .inner_join(stations::table)
                .filter(stops::train_id.eq(train_id))
                .select((StopRow::as_select(), StationRow::as_select()))
                .load::<(StopRow, StationRow)>(&mut conn)
                .map(|rows| {
                    rows.into_iter()
                        .map(|(stop_row, station_row)| StopWithStation {
                            stop: stop_row.to_stop(),
                            station: Station::from(station_row)
                        })
                        .collect()
                }).map_err(map_query_result_err)?;
        Ok(results)
    }

    fn get_by_station_and_date(&self, station: &Station, date: &chrono::NaiveDate) -> Result<Vec<Stop>, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;

        let results = stops::table
                .inner_join(trains::table.on(stops::train_id.eq(trains::id)))
                .inner_join(stations::table.on(stops::station_id.eq(stations::id)))
                .filter(stations::id.eq(station.id).and(trains::date.eq(date)))
                .select(stops::all_columns)
                .load::<StopRow>(&mut conn)
                .map_err(map_query_result_err)?;

        Ok(results.iter().map(|s| s.to_stop()).collect())
    }

    fn update(&self, update: &StopUpdate) -> Result<Stop, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;

        let row = diesel::update(stops::table.find(update.id.clone()))
            .set(StopUpdateRow::from(update))
            .returning(StopRow::as_returning())
            .get_result::<StopRow>(&mut conn)
            .map_err(map_query_result_err)?;

        Ok(row.to_stop())
    }

    fn update_many(&self, updates: &[StopUpdate]) -> Result<Vec<Stop>, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;

        let results = conn.transaction::<_, diesel::result::Error, _>(|tx| {
            let mut out = Vec::with_capacity(updates.iter().len());
            for patch in updates {
                let result = diesel::update(stops::table.find(&patch.id))
                    .set(StopUpdateRow::from(patch))
                    .returning(StopRow::as_returning())
                    .get_result(tx).optional_empty_changeset()?;
                if let Some(row) = result {
                    out.push(row.to_stop());
                }
            }
            Ok(out)
        }).map_err(map_query_result_err)?; // TODO: Map Update Result error?

        Ok(results)
    }

 }

impl Port<Stop, String> for StopRepo {
    fn persist(&self, stop: &Stop) -> Result<Stop, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;
        let result = diesel::insert_into(stops::table)
            .values(StopRow::from(stop))
            .on_conflict_do_nothing()
            .returning(StopRow::as_returning())
            .get_result::<StopRow>(&mut conn).map_err(map_query_result_err).map(|s| s.to_stop())?;
        Ok(result)
    }

    fn persist_all(&self, stops: &[Stop]) -> Result<Vec<Stop>, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;
        let result = diesel::insert_into(stops::table)
            .values(stops.iter().map(StopRow::from).collect::<Vec<StopRow>>())
            .on_conflict_do_nothing()
            .returning(StopRow::as_returning())
            .get_results::<StopRow>(&mut conn).map_err(map_query_result_err).map(|v| v.iter().map(|s| s.to_stop()).collect())?;
        Ok(result)
    }

    fn get_by_id(&self, id: String) -> Result<Stop, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;
        stops::table
            .find(id)
            .select(StopRow::as_select())
            .first(&mut conn)
            .map_err(map_query_result_err)
            .map(|s| s.to_stop())
    }

    fn get_all(&self) -> Result<Vec<Stop>, PortError> {
        let mut conn = self.pool.get().map_err(map_pool_err)?;
        stops::table
            .select(StopRow::as_select())
            .get_results(&mut conn)
            .map_err(map_query_result_err)
            .map(|v| v.iter().map(|s| s.to_stop()).collect())

    }
}


/*
 * MessageRepo
 */

 pub struct MessageRepo {
     pool: PgPool,
 }

 impl MessageRepo {
     pub fn new(pool: PgPool) -> Self {
         Self { pool }
     }
 }

 impl MessagePort for MessageRepo {
     fn get_by_date_and_code(&self, date: &chrono::NaiveDate, code: i32) -> Result<Vec<Message>, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;


        let start: NaiveDateTime = date.and_hms_opt(0, 0, 0).unwrap();
        let end: NaiveDateTime = (date.succ_opt().unwrap()).and_hms_opt(0, 0, 0).unwrap();

         let results = messages::table
            .filter(messages::timestamp.ge(start))
            .filter(messages::timestamp.lt(end))
            .filter(messages::code.eq(code))
            .select(MessageRow::as_select())
            .get_results(&mut conn)
            .map_err(map_query_result_err)?;

         Ok(results.iter().map(Message::from).collect())
     }

     fn get_by_train_id(&self, train_id: &str) -> Result<Vec<Message>, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;

         let results = messages::table
            .filter(messages::train_id.eq(train_id))
            .select(MessageRow::as_select())
            .get_results(&mut conn)
            .map_err(map_query_result_err)?;

         Ok(results.iter().map(Message::from).collect())
     }
 }

 impl Port<Message, String> for MessageRepo {
     fn persist(&self, message: &Message) -> Result<Message, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;
         let result = diesel::insert_into(messages::table)
             .values(MessageRow::from(message))
             .on_conflict_do_nothing()
             .returning(MessageRow::as_returning())
             .get_result::<MessageRow>(&mut conn)
             .map_err(map_query_result_err)?;
        Ok(Message::from(result))
     }

     fn persist_all(&self, messages: &[Message]) -> Result<Vec<Message>, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;
         let result = diesel::insert_into(messages::table)
             .values(messages.iter().map(MessageRow::from).collect::<Vec<MessageRow>>())
             .on_conflict_do_nothing()
             .returning(MessageRow::as_returning())
             .get_results::<MessageRow>(&mut conn)
             .map_err(map_query_result_err)?;
        Ok(result.iter().map(Message::from).collect())
     }

     fn get_by_id(&self, id: String) -> Result<Message, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;
         let result = messages::table
             .find(id)
             .select(MessageRow::as_select())
             .first(&mut conn)
             .map_err(map_query_result_err)?;

         Ok(Message::from(result))
     }

     fn get_all(&self) -> Result<Vec<Message>, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;
         let result = messages::table
             .select(MessageRow::as_select())
             .get_results(&mut conn)
             .map_err(map_query_result_err)?;
         Ok(result.iter().map(Message::from).collect())
     }
 }


 pub struct StatusCodeRepo {
     pool: PgPool
 }

 impl StatusCodeRepo {
     pub fn new(pool: PgPool) -> Self {
         Self { pool }
     }
 }

 impl Port<StatusCode, i16> for StatusCodeRepo {
     fn persist(&self, code: &StatusCode) -> Result<StatusCode, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;
         diesel::insert_into(status_codes::table)
             .values(StatusCodeRow::from(code))
             .on_conflict_do_nothing()
             .returning(StatusCodeRow::as_returning())
             .get_result::<StatusCodeRow>(&mut conn)
             .map(StatusCode::from)
             .map_err(map_query_result_err)
     }

     fn persist_all(&self, codes: &[StatusCode]) -> Result<Vec<StatusCode>, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;
         diesel::insert_into(status_codes::table)
             .values(codes.iter().map(StatusCodeRow::from).collect::<Vec<StatusCodeRow>>())
             .on_conflict_do_nothing()
             .returning(StatusCodeRow::as_returning())
             .get_results::<StatusCodeRow>(&mut conn)
             .map(|rows| rows.iter().map(StatusCode::from).collect::<Vec<StatusCode>>())
             .map_err(map_query_result_err)
     }

     fn get_by_id(&self, id: i16) -> Result<StatusCode, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;
         status_codes::table
             .find(id)
             .select(StatusCodeRow::as_select())
             .first(&mut conn)
             .map(StatusCode::from)
             .map_err(map_query_result_err)
     }

     fn get_all(&self) -> Result<Vec<StatusCode>, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;
         status_codes::table
             .select(StatusCodeRow::as_select())
             .get_results(&mut conn)
             .map(|rows| rows.iter().map(StatusCode::from).collect::<Vec<StatusCode>>())
             .map_err(map_query_result_err)
     }
}

 impl StatusCodePort for StatusCodeRepo {
 }
