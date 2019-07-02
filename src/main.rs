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
extern crate staticfile;
#[macro_use]
extern crate diesel_migrations;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate iron_cors;
extern crate mount;
extern crate serde_json;

mod db;
mod graphql;
mod lib;

use dotenv::dotenv;
use std::env;

use db::*;
use graphql::db::DbMutation;
use graphql::root::*;
use iron::prelude::*;
use iron_cors::CorsMiddleware;
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use logger::Logger;
use mount::Mount;
//use staticfile::Static;
//use std::path::Path;

fn main() {
    env_logger::init();
    dotenv().ok();

    let mut mount = Mount::new();
    let middleware = CorsMiddleware::with_allow_any();

    println!("creating a new gql context");
    let offset_log_path =
        env::var("OFFSET_LOG_PATH").expect("OFFSET_LOG_PATH environment variable must be set");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pub_key_string =
        env::var("SSB_PUB_KEY").expect("SSB_PUB_KEY environment variable must be set");
    let context = Context::new(offset_log_path, database_url, pub_key_string);

    let graphql_endpoint =
        GraphQLHandler::new(move |_| Ok(context.clone()), Query, DbMutation::default());
    let graphiql_endpoint = GraphiQLHandler::new("/graphql");

    mount.mount("/", graphiql_endpoint);
    mount.mount("/graphql", graphql_endpoint);
    //mount.mount("/", Static::new(Path::new("public")));

    let (logger_before, logger_after) = Logger::new(None);

    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);
    chain.link_around(middleware);

    let host = env::var("LISTEN").unwrap_or_else(|_| "localhost:8080".to_owned());
    println!("GraphQL server started on {}", host);
    Iron::new(chain).http(host.as_str()).unwrap();
}
