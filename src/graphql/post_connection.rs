use super::page_info::PageInfo;
use super::post::Post;
use crate::db::Context;
use crate::lib::cursor::*;
use diesel::prelude::*;
use juniper::FieldResult;

use crate::db::schema::messages::dsl::{
    flume_seq as messages_flume_seq, key_id as messages_key_id, messages as messages_table,
};

#[derive(Default)]
pub struct PostConnection {
    pub next: i32,
    pub page_info: PageInfo,
    pub post_keys: Vec<i32>,
}

graphql_object!(PostConnection: Context |&self| {
    description: "Connection to collections of posts"

    /// The total count of posts in this connection.
    field total_count(&executor) -> i32 {
        self.post_keys.len() as i32
    }
    /// The nodes in this connection
    field edges(&executor) -> Vec<PostEdge>{
        self.post_keys
            .iter()
            .map(|key_id|{
                Post{key_id: *key_id}
            })
            .map(|post|{
                PostEdge{
                    node: post
                }
            })
            .collect::<Vec<PostEdge>>()
    }

    /// The relay-spec pageInfo for this connection
    field page_info(&executor) -> &PageInfo{
        &self.page_info
    }
});

#[derive(Default)]
pub struct PostEdge {
    pub node: Post,
}

graphql_object!(PostEdge: Context |&self| {
    description: "Edge connection to a post"

    /// The nodes in this connection
    field node(&executor) -> &Post {
        &self.node
    }

    /// The cursor for this node
    field cursor(&executor) -> FieldResult<Option<String>> {
        let connection = executor.context().connection.lock()?;

        let cursor = messages_table
            .select(messages_flume_seq)
            .filter(messages_key_id.eq(self.node.key_id))
            .first::<Option<i64>>(&(*connection))?
            .map(encode_cursor);

        Ok(cursor)
    }

});
