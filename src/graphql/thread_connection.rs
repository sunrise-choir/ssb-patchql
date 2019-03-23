use juniper::FieldResult;
use crate::db::Context;
use super::thread::Thread;
use super::post::Post;
use diesel::prelude::*;
use crate::db::schema::messages::dsl::{
    flume_seq as messages_flume_seq,
    content_type as messages_content_type, key_id as messages_key_id, messages as messages_table,
    root_key_id as messages_root_key_id,
    fork_key_id as messages_fork_key_id,
};
#[derive(Default)]
pub struct ThreadConnection {
    pub cursor: f64,
    pub next: i32
}
#[derive(GraphQLObject, Default)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: f64,
    pub start_cursor: f64
}

graphql_object!(ThreadConnection: Context |&self| {
    field page_info(&executor) -> FieldResult<PageInfo> {
        Ok(PageInfo{has_next_page: true, start_cursor: self.cursor, end_cursor: self.cursor + self.next as f64})
    }
    field total_count(&executor) -> FieldResult<i32> {
        Ok(0)
    }
    field threads(&executor) -> FieldResult<Vec<Thread>>{
        // need to get root post key_ids. Map them into threads. A thread takes a root post. A post
        // takes a key_id
        let connection = executor.context().connection.lock().unwrap();

        let threads = messages_table
            .select(messages_key_id)
            .order(messages_flume_seq.desc())
            .filter(messages_flume_seq.lt(std::i64::MAX))
            .filter(messages_root_key_id.is_null())
            .filter(messages_fork_key_id.is_null())
            .filter(messages_content_type.eq("post"))
            .limit(self.next as i64)
            .load::<i32>(&(*connection))
            .into_iter()
            .flatten()
            .map(|key_id|{
                Thread{root: Post{key_id}}
            })
            .collect::<Vec<Thread>>();

            Ok(threads)

    }
});
