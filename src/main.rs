#[macro_use]
extern crate log as irrelevant_log;

#[macro_use] extern crate juniper_codegen;
#[macro_use] extern crate juniper;
extern crate warp;
extern crate juniper_warp;
extern crate env_logger;

use juniper::{FieldResult, EmptyMutation, RootNode};
use warp::{http::Response, log, Filter};

mod db;
mod author;
mod feed;
mod like;
mod mention;
mod post;
mod thread;

use db::Context;
use thread::*;
use post::*;
use author::*;
use like::*;

#[derive(GraphQLEnum)]
/// Retrieve objects that are private, public, or both.
enum Privacy {
    /// Only private.
    Private,
    /// Only public.
    Public,
    /// Both public and private.
    All
}

#[derive(GraphQLEnum)]
/// Retrieve objects ordered by asserted publish time, by received time, or attempt to causally sort
/// by cypher links.
enum OrderBy {
    /// Order by asserted timestamp (the time the author claimed they published the message).
    /// 
    /// Note that using asserted timestamp is not reliable. If the publisher of a message has their
    /// system clock set incorrectly then this can really break your ui. This has already happened
    /// before on the network. If you're sorting posts in a thread, prefer using causal sort.
    Asserted,

    /// Order by received timestamp (the time that the message was inserted into your db).
    /// 
    /// Note that using received timestamp does not work well when the db has downloaded many feeds
    /// all at once (like during onboarding to the network) because feeds are inserted into your db
    /// in a random order.
    Received,

    /// Order by causal timestamp.
    ///
    /// Use this for sorting posts in a thread. Don't use this for sorting all threads in the
    /// database, it's not supported.
    Causal
}

struct Query;

graphql_object!(Query: Context |&self| {

    field thread(&executor, id: String, privacy = (Privacy::Public): Privacy, orderBy = (OrderBy::Received): OrderBy) -> FieldResult<Thread> {
        let mut thread = Thread::default();

        if let Privacy::Private = privacy {
            thread.isPrivate = true;
        }
    
        Ok(thread)
    }

    field post(&executor, id: String ) -> FieldResult<Post> {
        // Get the context from the executor.
        let context = executor.context();
        let mut posts = Post::default();
        posts.id = id;
        Ok(posts)
    }

    field author(&executor, id: String) -> FieldResult<Author> {
        // Get the context from the executor.
        let context = executor.context();
        Ok(Author::default())
    }

    field likes(&executor, id: String) -> FieldResult<Vec<Like>> {
        // Get the context from the executor.
        let context = executor.context();
        Ok(vec![Like::default()])
    }
});

struct Mutation;

graphql_object!(Mutation: Context |&self| {

    //field createPost(&executor, new_post: NewPost) -> FieldResult<Post> {
    //    let db = executor.context().pool.get_connection()?;
    //    let human: Human = db.insert_human(&new_human)?;
    //    Ok(human)
    //}
});

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>>;

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

    let state = warp::any().map(move || Context{});
    let graphql_filter = juniper_warp::make_graphql_filter(Schema::new(Query, EmptyMutation::new()), state.boxed());

    warp::serve(
        warp::get2()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql"))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter))
            .with(log)
    )
    .run(([127, 0, 0, 1], 8080));

}
