use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{Pool, ConnectionManager};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

pub mod schema;
pub mod models;

pub struct Context {
    // Use your real database pool here.
    pub pool: Pool<ConnectionManager<SqliteConnection>>
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    connection.execute("PRAGMA synchronous = 0").unwrap();
    connection.execute("PRAGMA threads = 4").unwrap();
    connection.execute("PRAGMA page_size = 4096").unwrap();
     
    connection
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
