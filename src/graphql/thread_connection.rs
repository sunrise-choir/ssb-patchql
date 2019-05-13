use super::page_info::PageInfo;
use super::post::Post;
use super::thread::Thread;
use crate::db::Context;
use crate::lib::cursor::*;
use diesel::prelude::*;
use juniper::FieldResult;

use crate::db::schema::messages::dsl::{
    flume_seq as messages_flume_seq, key_id as messages_key_id, messages as messages_table,
};

#[derive(Default)]
pub struct ThreadConnection {
    pub next: i32,
    pub page_info: PageInfo,
    pub thread_keys: Vec<i32>,
}

graphql_object!(ThreadConnection: Context |&self| {
    description: "Connection to collections of threads"

    /// The edges in this connection
    field edges(&executor) -> Vec<ThreadEdge>{
        self.thread_keys
            .iter()
            .map(|key_id|{
                Thread{root: Post{key_id: *key_id}}
            })
            .map(|thread|{
                ThreadEdge{
                    node: thread
                }
            })
            .collect::<Vec<ThreadEdge>>()
    }

    /// The relay-spec pageInfo for this connection
    field page_info(&executor) -> &PageInfo{
        &self.page_info
    }

    /// The total count of posts in this connection.
    field total_count(&executor) -> i32 {
        self.thread_keys.len() as i32
    }
});

#[derive(Default)]
pub struct ThreadEdge {
    pub node: Thread,
}

graphql_object!(ThreadEdge: Context |&self| {
    description: "Edge connection to a thread"

    /// The nodes in this connection
    field node(&executor) -> &Thread {
        &self.node
    }

    /// The cursor for this node
    field cursor(&executor) -> FieldResult<Option<String>> {
        let connection = executor.context().connection.lock()?;

        let cursor = messages_table
            .select(messages_flume_seq)
            .filter(messages_key_id.eq(self.node.root.key_id))
            .first::<Option<i64>>(&(*connection))?
            .map(encode_cursor);

        Ok(cursor)
    }

});
