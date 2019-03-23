use super::post::Post;
use super::thread::Thread;
use crate::db::schema::messages::dsl::{
    content_type as messages_content_type, flume_seq as messages_flume_seq,
    fork_key_id as messages_fork_key_id, key_id as messages_key_id, messages as messages_table,
    root_key_id as messages_root_key_id,
};
use crate::db::Context;
use bytes::{ByteOrder, LittleEndian};
use diesel::prelude::*;
#[derive(GraphQLObject, Default)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: String,
    pub start_cursor: Option<String>,
}

#[derive(Default)]
pub struct ThreadConnection {
    pub next: i32,
    pub page_info: PageInfo,
    pub thread_keys: Vec<i32>,
}

impl ThreadConnection {
    pub fn new(
        connection: &SqliteConnection,
        start_cursor: Option<String>,
        next: i32,
    ) -> ThreadConnection {
        let start_seq = match start_cursor {
            None => std::i64::MAX,
            Some(ref encoded) => match base64::decode(&encoded) {
                Ok(bytes) => LittleEndian::read_i64(bytes.as_slice()),
                Err(_) => std::i64::MAX,
            },
        };

        let results = messages_table
            .select((messages_key_id, messages_flume_seq))
            .order(messages_flume_seq.desc())
            .filter(messages_flume_seq.lt(start_seq))
            .filter(messages_root_key_id.is_null())
            .filter(messages_fork_key_id.is_null())
            .filter(messages_content_type.eq("post"))
            .limit(next as i64)
            .load::<(i32, Option<i64>)>(connection)
            .unwrap();

        let thread_keys = results
            .iter()
            .map(|(key_id, _)| *key_id)
            .collect::<Vec<i32>>();

        let last_seq = results.iter().last().map(|(_, seq)| *seq).unwrap().unwrap();

        let end_cursor = base64::encode(&(last_seq as u64).to_le_bytes());
        let has_next_page = last_seq != 0;

        let page_info = PageInfo {
            start_cursor,
            end_cursor,
            has_next_page,
        };

        ThreadConnection {
            next,
            thread_keys,
            page_info,
        }
    }
}

graphql_object!(ThreadConnection: Context |&self| {
    field threads(&executor) -> Vec<Thread>{

        self.thread_keys
            .iter()
            .map(|key_id|{
                Thread{root: Post{key_id: *key_id}}
            })
            .collect::<Vec<Thread>>()
    }

    field page_info(&executor) -> &PageInfo{
        &self.page_info
    }
});
