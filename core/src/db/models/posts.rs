use super::keys::*;
use crate::db::{Error, SqliteConnection};
use crate::ssb_message::*;
use serde_json::Value;

use crate::db::schema::reply_posts;
use crate::db::schema::reply_posts::dsl::reply_posts as reply_posts_table;
use crate::db::schema::root_posts;
use crate::db::schema::root_posts::dsl::root_posts as root_posts_table;
use diesel::dsl::sql;
use diesel::insert_into;
use diesel::prelude::*;

use crate::db::schema::messages::dsl::{
    content as messages_content, key_id as messages_key_id, messages as messages_table,
};

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

#[derive(Deserialize, Serialize, Debug, Default)]
struct PostText {
    text: String,
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
                .execute(connection)
                .map(|_| ())
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
                .execute(connection)
                .map(|_| ())
        }
    }
}

pub fn get_text(connection: &SqliteConnection, key_id: i32) -> Result<String, Error> {
    let content = messages_table
        .select(sql::<diesel::sql_types::Text>("content"))
        .filter(messages_key_id.eq(key_id))
        .filter(messages_content.is_not_null())
        .first::<String>(connection)?;

    let value: PostText = serde_json::from_str(&content).unwrap();
    Ok(value.text)
}

#[cfg(test)]
mod tests {
    use crate::db::models::messages::insert_message;
    use crate::db::models::posts::{get_text, insert_post, PostText};
    use crate::ssb_message::{SsbMessage, SsbValue};
    use crate::utils::establish_connection;
    use diesel::prelude::*;
    use diesel::result::Error;
    use serde_json::{json, Value};

    #[test]
    fn get_post_text() {
        let connection = establish_connection();
        connection.test_transaction::<_, Error, _>(|| {
            let expected_text = "hello";
            let post = json!({
                "text": expected_text,
                "type": "post"
            });
            let mut val = SsbValue::default();
            let mut msg = SsbMessage::default();
            val.content = post;
            msg.value = val;
            msg.key = "test_key_123".to_string();

            let id = insert_message(&connection, &msg, 1, 1, false, 1).unwrap();
            let actual = get_text(&connection, 1).unwrap();
            assert_eq!(actual, expected_text);
            Ok(())
        });
    }
}
