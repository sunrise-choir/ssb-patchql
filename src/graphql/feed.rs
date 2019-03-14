use crate::db::Context;
use super::thread::*;

#[derive(Default)]
pub struct Feed {
    pub threads: Vec<Thread>,
}

graphql_object!(Feed: Context |&self| {
    field threads(&executor) -> Vec<Thread> {
        let database = executor.context();

        vec![Thread::default(), Thread::default()]
    }

});
