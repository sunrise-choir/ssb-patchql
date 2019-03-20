use super::authors::find_or_create_author;
use super::keys::find_or_create_key;

use crate::db::schema::abouts::dsl::{abouts, link_from_key_id, link_to_author_id, link_to_key_id};
use crate::db::schema::messages::dsl::{
    author_id as messages_author_id, content as messages_content, flume_seq as messages_flume_seq,
    key_id as messages_key_id, messages as messages_table,
};
use crate::db::SqliteConnection;
use crate::lib::*;
use diesel::dsl::sql;
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

#[derive(Deserialize)]
pub struct AboutName {
    pub name: String,
}

#[derive(Deserialize)]
pub struct AboutDescription {
    pub description: String,
}
#[derive(Deserialize)]
pub struct ImageInfo {
    pub link: String,
}
#[derive(Deserialize)]
pub struct AboutImage {
    pub image: ImageInfo,
}

pub trait About {
    fn about(&self) -> &str;
}

impl About for AboutName {
    fn about(&self) -> &str {
        &self.name
    }
}

impl About for AboutDescription {
    fn about(&self) -> &str {
        &self.description
    }
}

impl About for AboutImage {
    fn about(&self) -> &str {
        &self.image.link
    }
}

pub fn get_author_abouts<T: About + serde::de::DeserializeOwned>(
    connection: &SqliteConnection,
    author_id: i32,
) -> Option<String> {
    abouts
        .inner_join(messages_table.on(messages_key_id.nullable().eq(link_from_key_id)))
        .select(sql::<diesel::sql_types::Text>("content"))
        .order(messages_flume_seq.desc())
        .filter(link_to_author_id.eq(author_id))
        .filter(messages_author_id.eq(author_id))
        .filter(messages_content.is_not_null())
        .load::<String>(&(*connection))
        .unwrap()
        .into_iter()
        .map(|item| {
            serde_json::from_str::<T>(&item).map(|item| item.about().to_string() )
        })
        .filter_map(Result::ok)
        .take(1)
        .collect::<Vec<_>>()
        .pop()
}
