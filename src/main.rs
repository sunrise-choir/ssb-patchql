#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate juniper_codegen;
#[macro_use]
extern crate juniper;
extern crate juniper_warp;
#[macro_use]
extern crate log as irrelevant_log;
extern crate warp;

use juniper::EmptyMutation;
use warp::{http::Response, log, Filter};
use dotenv::dotenv;
use std::env;

mod db;
mod graphql;

use db::*;
use graphql::root::*;
use std::sync::Arc;

fn main() {
    ::std::env::set_var("RUST_LOG", "warp_server");
    env_logger::init();

    let log = log("warp_server");

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(format!(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
            ))
    });

    info!("Listening on 127.0.0.1:8080");

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set"); 
    let manager = diesel::r2d2::ConnectionManager::new(database_url);
    let pool = diesel::r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .unwrap();

    let state = warp::any().map(move || Context {pool: pool.clone()});
    let graphql_filter =
        juniper_warp::make_graphql_filter(Schema::new(Query, EmptyMutation::new()), state.boxed());

    warp::serve(
        warp::get2()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql"))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter))
            .with(log),
    )
    .run(([127, 0, 0, 1], 8080));
}
