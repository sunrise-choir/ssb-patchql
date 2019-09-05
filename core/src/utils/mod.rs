use crate::db::execute_pragmas;
use crate::db::open_connection;
use crate::diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("TEST_DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = open_connection(&database_url);

    execute_pragmas(&connection).unwrap();

    connection
}
