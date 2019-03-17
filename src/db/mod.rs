use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::any_pending_migrations;
use std::env;

pub mod models;
pub mod schema;

embed_migrations!();

pub struct Context {
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

pub fn execute_pragmas(connection: &SqliteConnection) -> Result<(), Error> {
    connection.execute("PRAGMA synchronous = 0")?;
    connection.execute("PRAGMA threads = 4")?;
    connection.execute("PRAGMA page_size = 4096")?;
    Ok(())
}

pub fn open_connection() -> Pool<ConnectionManager<SqliteConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = diesel::r2d2::ConnectionManager::new(database_url.clone());
    let pool = diesel::r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .unwrap();

    let connection = pool.clone().get().unwrap();

    if let Err(_) = any_pending_migrations(&connection) {
        info!("sqlite db may be empty or not exist. Running migrations");
        embedded_migrations::run(&connection).unwrap();
    }

    if let Ok(true) = any_pending_migrations(&connection) {
        info!("sqlite db has pending migrations. Deleting db and it will be rebuilt.");
        std::fs::remove_file(database_url).unwrap();
        embedded_migrations::run(&connection).unwrap();
    }

    execute_pragmas(&connection).unwrap();

    pool
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
