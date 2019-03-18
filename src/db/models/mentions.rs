use diesel::insert_into;
use diesel::prelude::*;

use super::keys::find_or_create_key;
use crate::db::schema::mentions::dsl::{link_from_key_id, link_to_author_id, mentions};
use crate::db::SqliteConnection;

pub fn insert_mentions(
    connection: &SqliteConnection,
    links: &[&serde_json::Value],
    message_key_id: i32,
) {
    links
        .iter()
        .filter(|link| link.is_string())
        .map(|link| link.as_str().unwrap())
        .filter(|link| link.starts_with('@'))
        .map(|link| find_or_create_key(&connection, link).unwrap())
        .for_each(|link_id| {
            insert_into(mentions)
                .values((
                    link_from_key_id.eq(message_key_id),
                    link_to_author_id.eq(link_id),
                ))
                .execute(connection)
                .unwrap();
        });
}
