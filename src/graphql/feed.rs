use super::input_objects::*;
use super::thread::*;
use crate::db::Context;

#[derive(Default)]
pub struct Feed {
    pub threads: Vec<Thread>,
}

graphql_object!(Feed: Context |&self| {
    field threads(&executor, order_by = (OrderBy::Received): OrderBy) -> Vec<Thread> {
        let database = executor.context();

        vec![Thread::default(), Thread::default()]
    }

});
