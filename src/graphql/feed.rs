use super::input_objects::*;
use super::thread::*;
use super::thread_connection::*;
use crate::db::Context;
use juniper::FieldResult;

#[derive(Default)]
pub struct Feed {
    pub threads: Vec<Thread>,
}

graphql_object!(Feed: Context |&self| {
    field threads_connection(&executor, order_by = (OrderBy::Received): OrderBy, after: Option<String>, next = 10: i32) -> FieldResult<ThreadConnection> {
        let connection = executor.context().connection.lock()?;

        let thread_connection = ThreadConnection::new(&(*connection), after, next )?;

        Ok(thread_connection)

    }


});
