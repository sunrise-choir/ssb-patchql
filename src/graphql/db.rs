use crate::db::*;

#[derive(Default)]
pub struct DbMutation {}

#[derive(GraphQLObject)]
struct ProcessResults {
    chunk_size: i32,
    latest_sequence: f64,
}

graphql_object!(DbMutation: Context |&self| {
    field process(&executor, chunk_size = 100: i32) -> ProcessResults {
        //let id = self.id;
        let database = executor.context();
        //TODO: latest_sequence needs to come from somewhere.
        ProcessResults{chunk_size, latest_sequence: chunk_size as f64}
    }
});
