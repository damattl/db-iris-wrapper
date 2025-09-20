 use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

 use crate::{data::repos::utils::{map_pool_err, map_query_result_err}, model::StatusCode, ports::{Port, PortError, StatusCodePort}};
 use crate::data::db::{schema::status_codes, PgPool, row::StatusCodeRow};


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
