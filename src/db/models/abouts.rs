use super::authors::find_or_create_author;
use super::keys::find_or_create_key;
use crate::db::schema::abouts::dsl::{abouts, link_from_key_id, link_to_author_id, link_to_key_id};
use crate::db::SqliteConnection;
use crate::lib::*;
use diesel::insert_into;
use diesel::prelude::*;
use serde_json::Value;

pub fn insert_abouts(connection: &SqliteConnection, message: &SsbMessage, message_key_id: i32) {
    if let Value::String(about_key) = &message.value.content["about"] {
        let (link_to_author, link_to_key): (Option<i32>, Option<i32>) = match about_key.get(0..1) {
            Some("@") => {
                let key = find_or_create_author(connection, about_key).unwrap();
                (Some(key), None)
            }
            Some("%") => {
                let key = find_or_create_key(connection, about_key).unwrap();
                (None, Some(key))
            }
            _ => (None, None),
        };

        insert_into(abouts)
            .values((
                link_from_key_id.eq(message_key_id),
                link_to_key_id.eq(link_to_key),
                link_to_author_id.eq(link_to_author),
            ))
            .execute(connection)
            .unwrap();
    }
}
