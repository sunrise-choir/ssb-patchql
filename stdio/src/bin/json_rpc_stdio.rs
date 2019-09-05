//extern crate env_logger;
//
//use jsonrpc_stdio_server::ServerBuilder;
//use jsonrpc_stdio_server::jsonrpc_core::*;
//
//use dotenv::dotenv;
//use std::env;
//
//use ssb_patchql::db::Context;
//use ssb_patchql::graphql::db::DbMutation;
//use ssb_patchql::graphql::root::*;
//use juniper_iron::{GraphQLHandler, GraphiQLHandler};
//use logger::Logger;
//use juniper::http::GraphQLRequest;
//use juniper::RootNode;
//
//fn main() {
//    env_logger::init();
//    dotenv().ok();
//    let mut io = IoHandler::default();
//
//    let offset_log_path =
//        env::var("OFFSET_LOG_PATH").expect("OFFSET_LOG_PATH environment variable must be set");
//
//    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//
//    let pub_key_string =
//        env::var("SSB_PUB_KEY").expect("SSB_PUB_KEY environment variable must be set");
//
//    let secret_key_string =
//        env::var("SSB_SECRET_KEY").expect("SSB_SECRET_KEY environment variable must be set");
//
//    let context = Context::new(offset_log_path.clone(), database_url.clone(), pub_key_string.clone(), secret_key_string.clone());
//    let context2 = context.clone();
//
//    io.add_method("query", move |params: Params| {
//        let request: GraphQLRequest = params.parse()?;
//
//        let root_node = RootNode::new(Query, DbMutation::default() );
//        let response = request.execute(&root_node, &context);
//
//        let val: Value = serde_json::to_value(response).unwrap();
//        Ok(val)
//    });
//
//
//    io.add_method("process", move |params: Params| {
//        let request: GraphQLRequest = params.parse()?;
//
//        let root_node = RootNode::new(Query, DbMutation::default() );
//        let response = request.execute(&root_node, &context2);
//
//        let val: Value = serde_json::to_value(response).unwrap();
//        Ok(val)
//    });
//
//    ServerBuilder::new(io).build();
//}
