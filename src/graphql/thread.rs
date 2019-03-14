use super::author::*;
use super::like::*;
use super::post::*;
use crate::db::Context;

#[derive(Default)]
pub struct Thread {
    pub id: String,
    pub text: String,
    pub likes: Vec<Like>,
    pub author: Author,
    pub posts: Vec<Post>,
    pub is_private: bool,
}

graphql_object!(Thread: Context |&self| {
    field posts(&executor) -> Vec<Post> {
        let database = executor.context();

        vec![Post::default(), Post::default()]
    }

    field is_private() -> bool {self.is_private}
    field id() -> &str { self.id.as_str() }
});
