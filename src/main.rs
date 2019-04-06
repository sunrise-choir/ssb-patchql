#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate juniper_codegen;
#[macro_use]
extern crate juniper;
extern crate juniper_iron;
#[macro_use]
extern crate log as irrelevant_log;
extern crate iron;
extern crate logger;
#[macro_use]
extern crate diesel_migrations;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate mount;
extern crate iron_cors;

mod db;
mod graphql;
mod lib;

use dotenv::dotenv;
use flumedb::offset_log::OffsetLog;
use std::env;

use db::*;
use graphql::db::DbMutation;
use graphql::root::*;
use std::sync::{Arc, Mutex};
use iron::prelude::*;
use iron_cors::CorsMiddleware;
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use logger::Logger;
use mount::Mount;

fn main() {
    env_logger::init();
    dotenv().ok();

    let offset_log_path =
        env::var("OFFSET_LOG_PATH").expect("OFFSET_LOG_PATH environment variable must be set");

    let connection = open_connection();
    let locked_connection_ref = Arc::new(Mutex::new(connection));
    let offset_log = OffsetLog::new(offset_log_path).unwrap();
    let locked_log_ref = Arc::new(Mutex::new(offset_log));

    let mut mount = Mount::new();

    let middleware = CorsMiddleware::with_allow_any();

    let graphql_endpoint = GraphQLHandler::new(
        move |_| Ok(Context{
            connection: locked_connection_ref.clone(),
            log: locked_log_ref.clone(),
        }),
        Query,
        DbMutation::default()
    );
    let graphiql_endpoint = GraphiQLHandler::new("/graphql");

    mount.mount("/", graphiql_endpoint);
    mount.mount("/graphql", graphql_endpoint);

    let (logger_before, logger_after) = Logger::new(None);

    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);
    chain.link_around(middleware);

    let host = env::var("LISTEN").unwrap_or_else(|_| "0.0.0.0:8080".to_owned());
    println!("GraphQL server started on {}", host);
    Iron::new(chain).http(host.as_str()).unwrap();
}
