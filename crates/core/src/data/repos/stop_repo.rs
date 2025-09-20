use chrono::NaiveDateTime;
use diesel::{BoolExpressionMethods, Connection, ExpressionMethods, JoinOnDsl, OptionalEmptyChangesetExtension, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{data::{db::row::{StationRow, StopUpdateRow}, repos::utils::{map_pool_err, map_query_result_err}}, model::{Station, Stop, StopUpdate, StopWithStation}, ports::{Port, PortError, StopPort}};
use crate::data::db::{schema::{stops, stations, trains}, PgPool, row::StopRow};


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
