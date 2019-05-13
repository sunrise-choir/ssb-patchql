use super::keys::*;
use crate::db::{Error, SqliteConnection};
use crate::lib::SsbMessage;
use serde_json::Value;

use crate::db::schema::reply_posts;
use crate::db::schema::reply_posts::dsl::reply_posts as reply_posts_table;
use crate::db::schema::root_posts;
use crate::db::schema::root_posts::dsl::root_posts as root_posts_table;
use diesel::insert_into;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Associations, Identifiable, Debug, Default)]
#[table_name = "root_posts"]
#[primary_key(flume_seq)]
pub struct RootPost {
    pub flume_seq: i64,
    pub key_id: i32,
    pub author_id: i32,
}

#[derive(Queryable, Insertable, Associations, Identifiable, Debug, Default)]
#[table_name = "reply_posts"]
#[primary_key(flume_seq)]
pub struct ReplyPost {
    pub flume_seq: i64,
    pub key_id: i32,
    pub root_post_id: i32,
    pub author_id: i32,
}

// Caller must check that the message is actually a post.
pub fn insert_post(
    connection: &SqliteConnection,
    message: &SsbMessage,
    message_key_id: i32,
    author_id: i32,
    seq: i64,
) -> Result<(), Error> {
    match message.value.content["root"] {
        //A reply
        Value::String(ref key) => {
            let id = find_or_create_key(&connection, &key).unwrap();
            let reply = ReplyPost {
                flume_seq: seq,
                key_id: message_key_id,
                root_post_id: id,
                author_id,
            };

            insert_into(reply_posts_table)
                .values(reply)
                .execute(connection)?;
        }
        // A root
        _ => {
            let root = RootPost {
                flume_seq: seq,
                key_id: message_key_id,
                author_id,
            };

            insert_into(root_posts_table)
                .values(root)
                .execute(connection)?;
        }
    };
    Ok(())
}
