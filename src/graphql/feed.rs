use juniper::FieldResult;
use diesel::prelude::*;
use super::input_objects::*;
use super::thread::*;
use super::thread_connection::*;
use crate::db::Context;
use crate::db::schema::messages::dsl::{
    author_id as messages_author_id, content as messages_content,
    content_type as messages_content_type, key_id as messages_key_id, messages as messages_table,
    root_key_id as messages_root_key_id,
};


#[derive(Default)]
pub struct Feed {
    pub threads: Vec<Thread>,
}

graphql_object!(Feed: Context |&self| {
    field threads_connection(&executor, order_by = (OrderBy::Received): OrderBy, after: Option<f64>, next = 10: i32) -> FieldResult<ThreadConnection> {
        let database = executor.context();

        //Need to get each thread's root Post
        //messages_table
        //    .select((messages_content, messages_key_id, messages_author_id))
        //    .limit(next)
        //    .load::<(Option<String>, i32, i32)>(&(*connection))?;
        
        let mut connection = ThreadConnection::default();
        connection.next = next;

        connection.cursor = match after{
            Some(cursor) => cursor,
            None => std::i64::MAX as f64
        };

        Ok(connection)

    }


});
