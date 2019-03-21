use diesel::prelude::*;
use juniper::{FieldResult, RootNode};

use super::author::*;
use super::db::DbMutation;
use super::feed::*;
use super::input_objects::*;
use super::like::*;
use super::post::*;
use super::thread::*;
use crate::db::schema::keys::dsl::{id as keys_id_col, key as keys_key_col, keys as keys_table};
use crate::db::schema::messages::dsl::{
    author_id as messages_author_id, content as messages_content, key_id as messages_key_id,
    messages as messages_table,
};
use crate::db::Context;
use serde_json::Value;

pub struct Query;

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = RootNode<'static, Query, DbMutation>;

graphql_object!(Query: Context |&self| {

    field thread(&executor, id: String, order_by = (OrderBy::Received): OrderBy) -> FieldResult<Thread> {

        let connection = executor.context().connection.lock()?;

        let thread = keys_table
            .inner_join(messages_table.on(
                    messages_key_id.nullable().eq(keys_id_col)
                    ))
            .select(messages_key_id)
            .filter(keys_key_col.eq(id.clone()))
            .first::<i32>(&(*connection))
            .map(|key_id|{
                let root = Post{key_id};
                Thread{root}
            })?;

        Ok(thread)

    }

    field feed(&executor, author_id: Option<String>, privacy = (Privacy::Public): Privacy, order_by = (OrderBy::Received): OrderBy) -> FieldResult<Feed> {
        // Get the context from the executor.
        let context = executor.context();
        let feed = Feed::default();
        Ok(feed)
    }

    field post(&executor, id: String ) -> FieldResult<Post> {
        unimplemented!();
        // Get the context from the executor.
        //let context = executor.context();
        //let mut post = Post::default();
        //post.key_id = id;
        //Ok(post)
    }

    field author(&executor, id: String) -> FieldResult<Author> {
        // Get the context from the executor.
        let context = executor.context();
        Ok(Author::default())
    }

    field likes(&executor, id: String) -> FieldResult<Vec<Like>> {
        // Get the context from the executor.
        let context = executor.context();
        Ok(vec![Like::default()])
    }
});
