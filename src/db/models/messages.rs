use super::keys::*;
use crate::db::{Error, SqliteConnection};
use crate::lib::SsbMessage;
use crate::db::schema::messages;

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
    unimplemented!();
    //    trace!("prepare stmt");
    //    let mut insert_msg_stmt = connection.prepare_cached("INSERT INTO messages_raw (flume_seq, key_id, seq, received_time, asserted_time, root_key_id, fork_key_id, author_id, content_type, content, is_decrypted) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")?;
    //
    //    trace!("get root key id");
    //    let root_key_id = match message.value.content["root"] {
    //        Value::String(ref key) => {
    //            let id = find_or_create_key(&connection, &key).unwrap();
    //            Some(id)
    //        }
    //        _ => None,
    //    };
    //
    //    trace!("get fork key id");
    //    let fork_key_id = match message.value.content["fork"] {
    //        Value::String(ref key) => {
    //            let id = find_or_create_key(&connection, &key).unwrap();
    //            Some(id)
    //        }
    //        _ => None,
    //    };
    //
    //    trace!("find or create author");
    //    let author_id = find_or_create_author(&connection, &message.value.author)?;
    //
    //    trace!("insert message");
    //    insert_msg_stmt.execute(&[
    //        &seq as &ToSql,
    //        &message_key_id,
    //        &message.value.sequence,
    //        &message.timestamp,
    //        &message.value.timestamp,
    //        &root_key_id as &ToSql,
    //        &fork_key_id as &ToSql,
    //        &author_id,
    //        &message.value.content["type"].as_str() as &ToSql,
    //        &message.value.content as &ToSql,
    //        &is_decrypted as &ToSql,
    //    ])
}
