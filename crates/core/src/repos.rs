use chrono::NaiveDateTime;
use diesel::{BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{db::run_sql_file, model::stop::StopWithStation};

use super::{db::{model::StopRow, schema::{messages, stations::{self}, stops, trains}, PgPool}, model::{message::Message, station::Station, stop::Stop, train::Train}, ports::{MessagePort, Port, StationPort, StopPort, TrainPort}};

pub struct StationRepo {
    pool: PgPool
}

impl StationRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Port<Station, i32> for StationRepo {
    fn persist(&self, station: &crate::model::station::Station) -> Result<Station, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        Ok(diesel::insert_into(stations::table)
            .values(station)
            .on_conflict_do_nothing()
            .returning(Station::as_returning())
            .get_result::<Station>(&mut conn).map_err(Box::new)?)
    }

    fn persist_all(&self, stations: &[crate::model::station::Station]) -> Result<Vec<Station>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        Ok(diesel::insert_into(stations::table)
            .values(stations)
            .on_conflict_do_nothing()
            .returning(Station::as_returning())
            .get_results::<Station>(&mut conn).map_err(Box::new)?)
    }

    fn get_by_id(&self, id: i32) -> Result<Station, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        Ok(stations::table.find(id).select(Station::as_select()).first(&mut conn).map_err(Box::new)?)
    }

    fn get_all(&self) -> Result<Vec<Station>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        Ok(stations::table.select(Station::as_select()).get_results(&mut conn).map_err(Box::new)?)
    }
}

impl StationPort for StationRepo {
    fn get_by_ds100(&self, ds100: &str) -> Result<Station, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        Ok(stations::table.filter(stations::ds100.eq(ds100)).select(Station::as_select()).first(&mut conn).map_err(Box::new)?)
    }
    fn import_from_sql(&self, path: &str) -> Result<Vec<Station>, Box<dyn std::error::Error>> {
        run_sql_file::<Station>(&self.pool, path)
    }
}

pub struct TrainRepo {
    pool: PgPool
}

impl TrainRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

 impl TrainPort for TrainRepo {
    fn get_by_station_and_date(&self, station: &Station, date: &chrono::NaiveDate) -> Result<Vec<Train>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;

        // TODO: Get all stops at this train station today,
        let results = trains::table
                .inner_join(stops::table.on(stops::train_id.eq(trains::id)))
                .inner_join(stations::table.on(stops::station_id.eq(stations::id)))
                .filter(stations::id.eq(station.id).and(trains::date.eq(date)))
                .select(trains::all_columns)
                .load::<Train>(&mut conn)
                .map_err(Box::new)?;

        Ok(results)
    }


    fn get_by_date(&self, date: &chrono::NaiveDate) -> Result<Vec<Train>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        Ok(trains::table.filter(trains::date.eq(date)).select(Train::as_select()).get_results(&mut conn).map_err(Box::new)?)
    }
}


impl Port<Train, String> for TrainRepo {
    fn persist(&self, train: &Train) -> Result<Train, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        Ok(diesel::insert_into(trains::table)
            .values(train)
            .on_conflict_do_nothing()
            .returning(Train::as_returning())
            .get_result::<Train>(&mut conn).map_err(Box::new)?)
    }

    fn persist_all(&self, trains: &[Train]) -> Result<Vec<Train>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        Ok(diesel::insert_into(trains::table)
            .values(trains)
            .on_conflict_do_nothing()
            .returning(Train::as_returning())
            .get_results::<Train>(&mut conn).map_err(Box::new)?)
    }

    fn get_by_id(&self, id: String) -> Result<Train, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        Ok(trains::table.find(id).select(Train::as_select()).first(&mut conn).map_err(Box::new)?)
    }

    fn get_all(&self) -> Result<Vec<Train>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        Ok(trains::table.select(Train::as_select()).get_results(&mut conn).map_err(Box::new)?)
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
     fn get_for_date(&self, date: &chrono::NaiveDate) -> Result<Vec<Stop>, Box<dyn std::error::Error>> {
         let mut conn = self.pool.get()?;

         let start: NaiveDateTime = date.and_hms_opt(0, 0, 0).unwrap();
         let end: NaiveDateTime = (date.succ_opt().unwrap()).and_hms_opt(0, 0, 0).unwrap();

         Ok(stops::table
             .filter(stops::arrival_planned.ge(start))
             .filter(stops::arrival_planned.lt(end))
             .select(StopRow::as_select())
             .get_results(&mut conn)
             .map_err(Box::new)
             .map(|v| v.iter().map(|s| s.to_stop()).collect())?
         )
     }

     fn get_for_train(&self, train_id: &str) -> Result<Vec<Stop>, Box<dyn std::error::Error>> {
         let mut conn = self.pool.get()?;
         Ok(stops::table
             .filter(stops::train_id.eq(train_id))
             .select(StopRow::as_select())
             .get_results(&mut conn)
             .map_err(Box::new)
             .map(|v| v.iter().map(|s| s.to_stop()).collect())?
         )
     }

    fn get_for_train_with_station(&self, train_id: &str) -> Result<Vec<StopWithStation>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        let results = stops::table
                .inner_join(stations::table)
                .filter(stops::train_id.eq(train_id))
                .select((StopRow::as_select(), Station::as_select()))
                .load::<(StopRow, Station)>(&mut conn)
                .map(|rows| {
                    rows.into_iter()
                        .map(|(stop_row, station)| StopWithStation { stop: stop_row.to_stop(), station })
                        .collect()
                })?;
        Ok(results)
    }
 }

impl Port<Stop, String> for StopRepo {
    fn persist(&self, stop: &Stop) -> Result<Stop, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        let result = diesel::insert_into(stops::table)
            .values(StopRow::from_stop(stop))
            .on_conflict_do_nothing()
            .returning(StopRow::as_returning())
            .get_result::<StopRow>(&mut conn).map_err(Box::new).map(|s| s.to_stop())?;
        Ok(result)
    }

    fn persist_all(&self, stops: &[Stop]) -> Result<Vec<Stop>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        let result = diesel::insert_into(stops::table)
            .values(stops.iter().map(StopRow::from_stop).collect::<Vec<StopRow>>())
            .on_conflict_do_nothing()
            .returning(StopRow::as_returning())
            .get_results::<StopRow>(&mut conn).map_err(Box::new).map(|v| v.iter().map(|s| s.to_stop()).collect())?;
        Ok(result)
    }

    fn get_by_id(&self, id: String) -> Result<Stop, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        Ok(stops::table
            .find(id)
            .select(StopRow::as_select())
            .first(&mut conn)
            .map_err(Box::new)
            .map(|s| s.to_stop())?
        )
    }

    fn get_all(&self) -> Result<Vec<Stop>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        Ok(stops::table
            .select(StopRow::as_select())
            .get_results(&mut conn)
            .map_err(Box::new)
            .map(|v| v.iter().map(|s| s.to_stop()).collect())?
        )
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
     fn get_by_date_and_code(&self, date: &chrono::NaiveDate, code: i32) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
         let mut conn = self.pool.get()?;


        let start: NaiveDateTime = date.and_hms_opt(0, 0, 0).unwrap();
        let end: NaiveDateTime = (date.succ_opt().unwrap()).and_hms_opt(0, 0, 0).unwrap();

         let results = messages::table
            .filter(messages::timestamp.ge(start))
            .filter(messages::timestamp.lt(end))
            .filter(messages::code.eq(code))
            .select(Message::as_select())
            .get_results(&mut conn)
            .map_err(Box::new)?;

         Ok(results)
     }

     fn get_by_train_id(&self, train_id: &str) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
         let mut conn = self.pool.get()?;

         let results = messages::table
            .filter(messages::train_id.eq(train_id))
            .select(Message::as_select())
            .get_results(&mut conn)
            .map_err(Box::new)?;

         Ok(results)
     }
 }

 impl Port<Message, String> for MessageRepo {
     fn persist(&self, message: &Message) -> Result<Message, Box<dyn std::error::Error>> {
         let mut conn = self.pool.get()?;
         Ok(diesel::insert_into(messages::table)
             .values(message)
             .on_conflict_do_nothing()
             .returning(Message::as_returning())
             .get_result::<Message>(&mut conn)
             .map_err(Box::new)?)
     }

     fn persist_all(&self, messages: &[Message]) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
         let mut conn = self.pool.get()?;
         Ok(diesel::insert_into(messages::table)
             .values(messages)
             .on_conflict_do_nothing()
             .returning(Message::as_returning())
             .get_results::<Message>(&mut conn)
             .map_err(Box::new)?)
     }

     fn get_by_id(&self, id: String) -> Result<Message, Box<dyn std::error::Error>> {
         let mut conn = self.pool.get()?;
         Ok(messages::table
             .find(id)
             .select(Message::as_select())
             .first(&mut conn)
             .map_err(Box::new)?)
     }

     fn get_all(&self) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
         let mut conn = self.pool.get()?;
         Ok(messages::table
             .select(Message::as_select())
             .get_results(&mut conn)
             .map_err(Box::new)?)
     }
 }
