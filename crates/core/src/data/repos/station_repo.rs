use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{data::repos::utils::{map_pool_err, map_query_result_err}, model::Station, ports::{Port, PortError, StationPort}};
use crate::data::db::{schema::stations, PgPool, run_sql_file, row::StationRow};


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
