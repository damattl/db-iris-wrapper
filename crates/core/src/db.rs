use std::{env, fs};

use diesel::{pg::Pg, r2d2::{self, ConnectionManager}, sql_query, Connection, PgConnection, QueryableByName, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;

pub mod schema;
pub mod model;

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_migrations(pool: PgPool) {
    let mut conn = pool.get().unwrap();
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Migrations failed");
}

pub fn establish_default_pg_pool() -> PgPool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    establish_pg_pool(&db_url)
}

pub fn establish_pg_pool(db_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .max_size(15) // max number of connections
        .build(manager)
        .expect("Failed to create pool.")
}


pub fn run_sql_file<T>(pool: &PgPool, path: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: QueryableByName<diesel::pg::Pg> + 'static,
{
    let mut conn = pool.get()?;
    let sql = fs::read_to_string(path)?;

    let mut last_result: Option<Vec<T>> = None;

    for stmt in sql.split(';') {
        let trimmed = stmt.trim();
        if trimmed.is_empty() {
            continue;
        }

        match diesel::sql_query(trimmed).get_results::<T>(&mut conn) {
            Ok(rows) => last_result = Some(rows),
            Err(_) => {
                diesel::sql_query(format!{"{} ON CONFLICT DO NOTHING", trimmed}).execute(&mut conn)?;
            }
        }
    }

    Ok(last_result.unwrap_or_default())
}
