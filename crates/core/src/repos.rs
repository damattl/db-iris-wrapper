use diesel::{BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper};

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
}

pub struct TrainRepo {
    pool: PgPool
}

impl TrainRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

 impl TrainPort<'_> for TrainRepo {
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
}


impl Port<Train, &str> for TrainRepo {
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

    fn get_by_id(&self, id: &str) -> Result<Train, Box<dyn std::error::Error>> {
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

 impl StopPort<'_> for StopRepo {}

impl Port<Stop, &str> for StopRepo {
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

    fn get_by_id(&self, id: &str) -> Result<Stop, Box<dyn std::error::Error>> {
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

 impl MessagePort<'_> for MessageRepo {}

 impl Port<Message, &str> for MessageRepo {
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

     fn get_by_id(&self, id: &str) -> Result<Message, Box<dyn std::error::Error>> {
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
