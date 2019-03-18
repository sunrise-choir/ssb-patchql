use crate::db::*;
use crate::db::{Error, SqliteConnection};
use crate::lib::*;
use crate::db::schema::*;

use super::keys::find_or_create_key;

pub fn insert_links(
    connection: &SqliteConnection,
    links: &[&serde_json::Value],
    message_key_id: i32,
) {
    //
    //    let mut insert_link_stmt = connection
    //        .prepare_cached("INSERT INTO links_raw (link_from_key_id, link_to_key_id) VALUES (?, ?)")
    //        .unwrap();
    //
    //    links
    //        .iter()
    //        .filter(|link| link.is_string())
    //        .map(|link| link.as_str().unwrap())
    //        .filter(|link| link.starts_with('%'))
    //        .map(|link| find_or_create_key(&connection, link).unwrap())
    //        .for_each(|link_id| {
    //            insert_link_stmt
    //                .execute(&[&message_key_id, &link_id])
    //                .unwrap();
    //        });
}
