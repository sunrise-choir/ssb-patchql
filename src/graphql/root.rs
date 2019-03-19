use diesel::prelude::*;
use juniper::{FieldResult, RootNode};

use super::author::*;
use super::db::DbMutation;
use super::feed::*;
use super::input_objects::*;
use super::like::*;
use super::post::*;
use super::thread::*;
use crate::db::models::keys::*;
use crate::db::models::messages::*;
use crate::db::schema::keys::dsl::{key as keys_key_row, keys as keys_table};
use crate::db::Context;
use serde_json::Value;

pub struct Query;

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = RootNode<'static, Query, DbMutation>;

graphql_object!(Query: Context |&self| {

    field thread(&executor, id: String, order_by = (OrderBy::Received): OrderBy) -> FieldResult<Thread> {
        let mut thread = Thread::default();

        let connection = executor.context().connection.lock().unwrap();

        let key = keys_table
            .filter(keys_key_row.eq(id.clone()))
            .first::<Key>(&(*connection))?;

        let message: Message = Message::belonging_to(&key)
            .first(&(*connection))?;

        if let Some(content) = &message.content {
            let parsed_content: Value = serde_json::from_str(content)?;

            if let Value::String(text) = &parsed_content["text"] {
                thread.text = text.clone();
                thread.id = id;
            }
        }

        Ok(thread)
    }

    field feed(&executor, author_id: Option<String>, privacy = (Privacy::Public): Privacy, order_by = (OrderBy::Received): OrderBy) -> FieldResult<Feed> {
        // Get the context from the executor.
        let context = executor.context();
        let feed = Feed::default();
        Ok(feed)
    }

    field post(&executor, id: String ) -> FieldResult<Post> {
        // Get the context from the executor.
        let context = executor.context();
        let mut post = Post::default();
        post.id = id;
        Ok(post)
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
