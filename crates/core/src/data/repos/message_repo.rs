 use chrono::NaiveDateTime;
use diesel::{r2d2::{ConnectionManager, PooledConnection}, upsert::excluded, BelongingToDsl, ExpressionMethods, GroupedBy, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

 use crate::{data::{db::{row::MessageToStationRow}, repos::utils::{map_pool_err, map_query_result_err}}, model::Message, ports::{MessagePort, Port, PortError}};
 use crate::data::db::{schema::{messages, messages_to_stations}, PgPool, row::MessageRow};


 pub struct MessageRepo {
     pool: PgPool,
 }

 impl MessageRepo {
     pub fn new(pool: PgPool) -> Self {
         Self { pool }
     }
 }


fn fetch_station_mappings(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, msgs: &[MessageRow]) -> Result<Vec<MessageToStationRow>, PortError> {
    MessageToStationRow::belonging_to(msgs)
        .load(conn).map_err(map_query_result_err)
}

fn fetch_station_mapping(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, msg: &MessageRow) -> Result<Vec<MessageToStationRow>, PortError> {
    messages_to_stations::table
        .filter(messages_to_stations::message_id.eq(&msg.id))
        .select(MessageToStationRow::as_select())
        .load(conn).map_err(map_query_result_err)
}

fn fetch_stations_and_build_models(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, msgs: &[MessageRow]) -> Result<Vec<Message>, PortError> {
    let mappings = fetch_station_mappings(conn, msgs)?;
    let results: Vec<Message> =
        mappings.grouped_by(msgs).into_iter().zip(msgs).map(|(s, m)| m.to_message(&s)).collect();
    Ok(results)
}

fn fetch_stations_and_build_model(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, msg: &MessageRow) -> Result<Message, PortError> {
    let mappings = fetch_station_mapping(conn, msg)?;
    Ok(msg.to_message(&mappings))
}

fn update_stations_relation(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, msg: &Message) -> Result<(), PortError> {
    let _ = diesel::insert_into(messages_to_stations::table)
        .values(msg.stations.iter().map(|s_id| MessageToStationRow {
            message_id: msg.id.clone(),
            station_id: *s_id,
        }).collect::<Vec<MessageToStationRow>>())
       .on_conflict_do_nothing()
       .execute(conn)
       .map_err(map_query_result_err)?;
    Ok(())
}

fn update_stations_relations(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, msgs: &[Message]) -> Result<(), PortError> {
    let inserts = msgs.iter().flat_map(|msg| msg.stations.iter().map(|s_id| MessageToStationRow {
        message_id: msg.id.clone(),
        station_id: *s_id,
    })).collect::<Vec<MessageToStationRow>>();

    let _ = diesel::insert_into(messages_to_stations::table)
        .values(&inserts)
        .on_conflict_do_nothing()
        .execute(conn)
        .map_err(map_query_result_err)?;
    Ok(())
}


 impl MessagePort for MessageRepo {
     fn get_by_date_and_code(&self, date: &chrono::NaiveDate, code: i32) -> Result<Vec<Message>, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;


        let start: NaiveDateTime = date.and_hms_opt(0, 0, 0).unwrap();
        let end: NaiveDateTime = (date.succ_opt().unwrap()).and_hms_opt(0, 0, 0).unwrap();

         let msgs: Vec<MessageRow> = messages::table
            .filter(messages::timestamp.ge(start))
            .filter(messages::timestamp.lt(end))
            .filter(messages::code.eq(code))
            .select(MessageRow::as_select())
            .get_results(&mut conn)
            .map_err(map_query_result_err)?;

         fetch_stations_and_build_models(&mut conn, &msgs)
     }

     fn get_by_train_id(&self, train_id: &str) -> Result<Vec<Message>, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;

         let msgs = messages::table
            .filter(messages::train_id.eq(train_id))
            .select(MessageRow::as_select())
            .get_results(&mut conn)
            .map_err(map_query_result_err)?;

         fetch_stations_and_build_models(&mut conn, &msgs)
     }
 }

 impl Port<Message, String> for MessageRepo {
     fn persist(&self, message: &Message) -> Result<Message, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;
         let msg = diesel::insert_into(messages::table)
             .values(MessageRow::from(message))
             .on_conflict_do_nothing()
             .returning(MessageRow::as_returning())
             .get_result::<MessageRow>(&mut conn)
             .map_err(map_query_result_err)?;

         update_stations_relation(&mut conn, message)?;
         fetch_stations_and_build_model(&mut conn, &msg)
     }

     fn persist_all(&self, messages: &[Message]) -> Result<Vec<Message>, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;
         let result = diesel::insert_into(messages::table)
             .values(messages.iter().map(MessageRow::from).collect::<Vec<MessageRow>>())
             .on_conflict(messages::id)
             .do_update()
             .set((
                 messages::last_updated.eq(excluded(messages::last_updated)),
             ))
             .returning(MessageRow::as_returning())
             .get_results::<MessageRow>(&mut conn)
             .map_err(map_query_result_err)?;

         update_stations_relations(&mut conn, messages)?;
         fetch_stations_and_build_models(&mut conn, &result)
     }

     fn get_by_id(&self, id: String) -> Result<Message, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;
         let result = messages::table
             .find(id)
             .select(MessageRow::as_select())
             .first(&mut conn)
             .map_err(map_query_result_err)?;

         fetch_stations_and_build_model(&mut conn, &result)
     }

     fn get_all(&self) -> Result<Vec<Message>, PortError> {
         let mut conn = self.pool.get().map_err(map_pool_err)?;
         let msgs = messages::table
             .select(MessageRow::as_select())
             .get_results(&mut conn)
             .map_err(map_query_result_err)?;
         fetch_stations_and_build_models(&mut conn, &msgs)
     }
 }
