use diesel::prelude::*;
use juniper::FieldResult;

use super::feed::*;
use super::input_objects::*;
use super::post::*;
use super::thread::*;
use crate::db::schema::keys::dsl::{id as keys_id_col, key as keys_key_col, keys as keys_table};
use crate::db::schema::messages::dsl::{key_id as messages_key_id, messages as messages_table};
use crate::db::Context;

pub struct Query;

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
// Although this is not used by the iron interface so is making warnings.
//pub type Schema = RootNode<'static, Query, DbMutation>;

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
        let connection = executor.context().connection.lock()?;
        let feed = Feed::default();
        Ok(feed)
    }

    field post(&executor, id: String ) -> FieldResult<Post> {
        let connection = executor.context().connection.lock()?;

        let post = keys_table
            .inner_join(messages_table.on(
                    messages_key_id.nullable().eq(keys_id_col)
                    ))
            .select(messages_key_id)
            .filter(keys_key_col.eq(id.clone()))
            .first::<i32>(&(*connection))
            .map(|key_id|{
                Post{key_id}
            })?;

        Ok(post)
    }

});
