use diesel::dsl::max;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::any_pending_migrations;
use flumedb::offset_log::OffsetLog;
use std::sync::{Arc, Mutex};

pub mod models;
pub mod schema;

use schema::messages::dsl::*;

embed_migrations!();

pub struct Context {
    pub connection: Arc<Mutex<SqliteConnection>>,
    pub log: Arc<Mutex<OffsetLog<u32>>>,
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

pub fn execute_pragmas(connection: &SqliteConnection) -> Result<(), Error> {
    connection.execute("PRAGMA synchronous = 0")?;
    connection.execute("PRAGMA page_size = 8192")?;
    //connection.execute("PRAGMA journal_mode = OFF")?; //makes ~13% faster.
    Ok(())
}

pub fn open_connection(database_url: &str) -> SqliteConnection {

    let connection = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    if let Err(_) = any_pending_migrations(&connection) {
        info!("sqlite db may be empty or not exist. Running migrations");
        embedded_migrations::run(&connection).unwrap();
    }

    if let Ok(true) = any_pending_migrations(&connection) {
        info!("sqlite db has pending migrations. Deleting db and it will be rebuilt.");
        std::fs::remove_file(&database_url).unwrap();
        embedded_migrations::run(&connection).unwrap();
    }

    execute_pragmas(&connection).unwrap();

    connection
}

pub fn get_latest(connection: &SqliteConnection) -> Result<Option<f64>, Error> {
    messages
        .select(max(flume_seq))
        .first(connection)
        .map(|res: Option<i64>| res.map(|val| val as f64))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
