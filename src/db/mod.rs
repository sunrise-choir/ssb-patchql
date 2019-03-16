use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::result::Error;

pub mod schema;
pub mod models;

pub struct Context {
    // Use your real database pool here.
    pub pool: Pool<ConnectionManager<SqliteConnection>>
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

pub fn execute_pragmas(connection: &SqliteConnection) -> Result<(), Error> {
    connection.execute("PRAGMA synchronous = 0")?;
    connection.execute("PRAGMA threads = 4")?;
    connection.execute("PRAGMA page_size = 4096")?;
    Ok(())
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
