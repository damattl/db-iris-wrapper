use chrono::NaiveDate;
use diesel::{BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{data::repos::utils::{map_pool_err, map_query_result_err}, model::{Station, Train}, ports::{Port, PortError, TrainPort}};
use crate::data::db::{schema::{trains, stops, stations}, PgPool, row::TrainRow};


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
