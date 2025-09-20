 use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

 use crate::{data::repos::utils::{map_pool_err, map_query_result_err}, model::Message, ports::{Port, PortError, MessagePort}};
 use crate::data::db::{schema::messages, PgPool, row::MessageRow};


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
