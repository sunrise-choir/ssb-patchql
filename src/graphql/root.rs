use juniper::{FieldResult, RootNode};

use super::author::*;
use super::db::DbMutation;
use super::feed::*;
use super::input_objects::*;
use super::like::*;
use super::post::*;
use super::thread::*;
use crate::db::Context;

pub struct Query;

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = RootNode<'static, Query, DbMutation>;

graphql_object!(Query: Context |&self| {

    field thread(&executor, id: String, order_by = (OrderBy::Received): OrderBy) -> FieldResult<Thread> {
        let thread = Thread::default();

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
