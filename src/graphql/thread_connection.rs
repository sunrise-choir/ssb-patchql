use super::post::Post;
use super::thread::Thread;
use super::page_info::PageInfo;
use crate::db::schema::messages::dsl::{
    content_type as messages_content_type, flume_seq as messages_flume_seq,
    fork_key_id as messages_fork_key_id, key_id as messages_key_id, messages as messages_table,
    root_key_id as messages_root_key_id,
};
use crate::db::Context;
use bytes::{ByteOrder, LittleEndian};
use diesel::prelude::*;


#[derive(Default)]
pub struct ThreadConnection {
    pub next: i32,
    pub page_info: PageInfo,
    pub thread_keys: Vec<i32>,
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
