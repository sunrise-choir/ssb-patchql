
#[macro_use] extern crate juniper;
extern crate warp;
extern crate juniper_warp;
extern crate pretty_env_logger;

use juniper::{FieldResult, EmptyMutation, RootNode};
use warp::{http::Response, log, Filter};

#[derive(GraphQLObject, Default)]
struct Author {
    id: String,
    name: String
}

#[derive(GraphQLObject, Default)]
struct Like {
    author: Author,
}

#[derive(GraphQLObject, Default)]
struct Post {
    id: String,
    text: String,
    likes: Vec<Like>,
    author: Author,
    thread: Thread
}

#[derive(GraphQLObject, Default)]
struct Thread {
    id: String,
    text: String,
    likes: Vec<Like>,
    author: Author,
    replies: Vec<Post>
}


struct Context {
    // Use your real database pool here.
    //pool: DatabasePool,
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        "1.0"
    }

    field thread(&executor, id: String) -> FieldResult<Thread> {
        // Get the context from the executor.
        let context = executor.context();
        // Get a db connection.
        //let connection = context.pool.get_connection()?;
        // Execute a db query.
        // Note the use of `?` to propagate errors.
        //let human = connection.find_human(&id)?;
        // Return the result.
        
        Ok(Thread::default())
    }

    field post(&executor, id: String) -> FieldResult<Post> {
        // Get the context from the executor.
        let context = executor.context();
        Ok(Post::default())
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
    //env_logger::init();

    //let log = log("warp_server");

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(format!(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
            ))
    });

    //info!("Listening on 127.0.0.1:8080");

    let state = warp::any().map(move || Context{});
    let graphql_filter = juniper_warp::make_graphql_filter(Schema::new(Query, EmptyMutation::new()), state.boxed());

    warp::serve(
        warp::get2()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql"))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter))
    )
    .run(([127, 0, 0, 1], 8080));

}
