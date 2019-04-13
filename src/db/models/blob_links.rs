use crate::db::SqliteConnection;
use crate::db::*;

use super::blobs::find_or_create_blob;
use crate::db::schema::blob_links::dsl::{blob_links, link_from_key_id, link_to_blob_id};
use diesel::insert_into;

pub fn insert_blob_links(
    connection: &SqliteConnection,
    links: &[&serde_json::Value],
    message_key_id: i32,
) {
    links
        .iter()
        .filter(|link| link.is_string())
        .map(|link| link.as_str().unwrap())
        .filter(|link| link.starts_with('&'))
        .map(|link| find_or_create_blob(&connection, link).unwrap())
        .for_each(|link_id| {
            insert_into(blob_links)
                .values((
                    link_from_key_id.eq(message_key_id),
                    link_to_blob_id.eq(link_id),
                ))
                .execute(connection)
                .unwrap();
        });
}
