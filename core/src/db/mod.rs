use diesel::dsl::max;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::any_pending_migrations;

#[cfg(feature = "ssb-go-log")]
use flumedb::go_offset_log::GoOffsetLog;
#[cfg(not(feature = "ssb-go-log"))]
use flumedb::offset_log::OffsetLog;

use private_box::Keypair;
use std::sync::{Arc, Mutex};

pub mod models;
pub mod schema;

use schema::messages::dsl::*;

embed_migrations!();

#[derive(Clone)]
pub struct Context {
    pub rw_connection: Arc<Mutex<SqliteConnection>>,
    pub connection: Pool<ConnectionManager<SqliteConnection>>,

    #[cfg(not(feature = "ssb-go-log"))]
    pub log: Arc<Mutex<OffsetLog<u32>>>,
    #[cfg(feature = "ssb-go-log")]
    pub log: Arc<Mutex<GoOffsetLog>>,

    pub keys: Vec<Keypair>,
}

impl Context {
    pub fn new(
        offset_log_path: String,
        database_path: String,
        pub_key_string: String,
        secret_key_string: String,
    ) -> Context {
        #[cfg(not(feature = "ssb-go-log"))]
        let offset_log = match OffsetLog::open_read_only(&offset_log_path) {
            Ok(log) => log,
            Err(_) => {
                panic!("failed to open offset log at {}", offset_log_path);
            }
        };
        #[cfg(feature = "ssb-go-log")]
        let offset_log = match GoOffsetLog::open_read_only(&offset_log_path) {
            Ok(log) => log,
            Err(_) => {
                panic!("failed to open offset log at {}", offset_log_path);
            }
        };

        let locked_log_ref = Arc::new(Mutex::new(offset_log));

        let rw_connection = open_connection(&to_sqlite_uri(&database_path, "rwc"));

        let manager = ConnectionManager::new(&to_sqlite_uri(&database_path, "ro"));
        let pool = Pool::builder().build(manager).unwrap();

        models::authors::set_is_me(&rw_connection, &pub_key_string).unwrap();

        let rw_locked_connection_ref = Arc::new(Mutex::new(rw_connection));
        let locked_connection_ref = pool;

        let secret_key_bytes =
            base64::decode(&secret_key_string.trim_end_matches(".ed25519")).unwrap_or(vec![0u8]);

        let secret_key = Keypair::from_slice(&secret_key_bytes).unwrap_or_else(|| {
            warn!(
                "Could not parse valid ssb-secret for decryption. Messages will not be decrypted"
            );
            Keypair::from_slice(&[0; 64]).unwrap()
        });

        let keys = vec![secret_key];

        Context {
            rw_connection: rw_locked_connection_ref.clone(),
            connection: locked_connection_ref.clone(),
            log: locked_log_ref.clone(),
            keys,
        }
    }
}

fn to_sqlite_uri(path: &str, rw_mode: &str) -> String {
    format!("file:{}?mode={}", path, rw_mode)
}
// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

pub fn execute_pragmas(connection: &SqliteConnection) -> Result<(), Error> {
    connection.execute("PRAGMA synchronous=normal")?;
    connection.execute("PRAGMA page_size=8192")?;
    connection.execute("PRAGMA journal_mode=wal")?;
    connection.execute("PRAGMA threads=4")?;
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
