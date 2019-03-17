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
#[macro_use]
extern crate diesel_migrations;

use dotenv::dotenv;
use juniper::EmptyMutation;
use warp::{http::Response, log, Filter};

mod db;
mod graphql;

use db::*;
use graphql::root::*;

fn main() {
    env_logger::init();
    dotenv().ok();

    let log = log("ssb-patchql-server");

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(format!(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
            ))
    });

    let pool = open_connection();

    let state = warp::any().map(move || Context { pool: pool.clone() });
    let graphql_filter =
        juniper_warp::make_graphql_filter(Schema::new(Query, EmptyMutation::new()), state.boxed());

    info!("Listening on 127.0.0.1:8080");
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
