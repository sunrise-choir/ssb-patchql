use super::page_info::PageInfo;
use super::post::Post;
use super::thread::Thread;
use crate::db::Context;
use juniper::FieldResult;

pub struct ThreadConnection {
    pub next: i32,
    pub page_info: PageInfo,
    pub thread_keys_and_cursor: Vec<(i32, String)>,
}

graphql_object!(ThreadConnection: Context |&self| {
    description: "Connection to collections of threads"

    /// The edges in this connection
    field edges(&executor) -> Vec<ThreadEdge>{
        self.thread_keys_and_cursor
            .iter()
            .map(|(key_id, cursor)|{
                Thread{root: Post{key_id: *key_id, cursor: None}, cursor: cursor.to_owned()}
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
        self.thread_keys_and_cursor.len() as i32
    }
});

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
    field cursor(&executor) -> FieldResult<String> {
        Ok(self.node.cursor.clone())
    }

});
