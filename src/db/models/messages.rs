use super::keys::*;
use crate::db::{Error, SqliteConnection};
use crate::lib::SsbMessage;
use serde_json::Value;

use super::authors::find_or_create_author;
use super::keys::find_or_create_key;
use crate::db::schema::messages;
use crate::db::schema::messages::dsl::messages as messages_table;
use diesel::insert_into;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Associations, Identifiable, Debug, Default)]
#[table_name = "messages"]
#[primary_key(flume_seq)]
#[belongs_to(Key)]
pub struct Message {
    pub flume_seq: Option<i64>,
    pub key_id: Option<i32>,
    pub seq: Option<i32>,
    pub received_time: Option<f64>,
    pub asserted_time: Option<f64>,
    pub root_key_id: Option<i32>,
    pub fork_key_id: Option<i32>,
    pub author_id: Option<i32>,
    pub content_type: Option<String>,
    pub content: Option<String>,
    pub is_decrypted: Option<bool>,
}

pub fn insert_message(
    connection: &SqliteConnection,
    message: &SsbMessage,
    seq: i64,
    message_key_id: i32,
    is_decrypted: bool,
) -> Result<usize, Error> {
    let root_key_id = match message.value.content["root"] {
        Value::String(ref key) => {
            let id = find_or_create_key(&connection, &key).unwrap();
            Some(id)
        }
        _ => None,
    };

    let fork_key_id = match message.value.content["fork"] {
        Value::String(ref key) => {
            let id = find_or_create_key(&connection, &key).unwrap();
            Some(id)
        }
        _ => None,
    };

    let author_id = find_or_create_author(&connection, &message.value.author)?;

    let message = Message {
        flume_seq: Some(seq),
        key_id: Some(message_key_id),
        seq: Some(message.value.sequence as i32),
        received_time: Some(message.timestamp),
        asserted_time: Some(message.value.timestamp),
        root_key_id,
        fork_key_id,
        author_id: Some(author_id),
        content_type: message.value.content["type"]
            .as_str()
            .map(|content_type| content_type.to_string()),
        content: Some(message.value.content.to_string()),
        is_decrypted: Some(is_decrypted),
    };

    insert_into(messages_table)
        .values(message)
        .execute(connection)
}
