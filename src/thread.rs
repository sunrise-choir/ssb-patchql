use crate::db::Context;
use crate::like::*;
use crate::author::*;
use crate::post::*;

#[derive(Default)]
pub struct Thread {
    pub id: String,
    pub text: String,
    pub likes: Vec<Like>,
    pub author: Author,
    pub posts: Vec<Post>,
    pub isPrivate: bool
}

graphql_object!(Thread: Context |&self| {
    field posts(&executor) -> Vec<Post> {
        let database = executor.context();

        vec![Post::default(), Post::default()]
    }

    field isPrivate() -> bool {self.isPrivate}
    field id() -> &str { self.id.as_str() }
});
