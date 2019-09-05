#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate juniper_codegen;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate log as irrelevant_log;
#[macro_use]
extern crate diesel_migrations;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod cursor;
pub mod db;
pub mod graphql;
mod ssb_message;

use db::Context;
use graphql::db::DbMutation;
use graphql::root::*;
use juniper::http::GraphQLRequest;
use juniper::RootNode;
use serde_json::Error;

#[derive(Clone)]
pub struct Patchql {
    context: Context,
}

impl Patchql {
    pub fn new(
        offset_log_path: String,
        database_path: String,
        pub_key: String,
        secret_key: String,
    ) -> Patchql {
        let context = Context::new(offset_log_path, database_path, pub_key, secret_key);

        Patchql { context }
    }
    pub fn query(&self, query_string: &str) -> Result<String, Error> {
        let request: GraphQLRequest = serde_json::from_str(query_string)?;

        let root_node = RootNode::new(Query, DbMutation::default());
        let response = request.execute(&root_node, &self.context);
        serde_json::to_string(&response)
    }
}
