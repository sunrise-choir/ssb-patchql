use super::author::*;
use super::like::*;
use crate::db::*;

#[derive(Default)]
pub struct Post {
    pub id: String,
    pub text: String,
    pub likes: Vec<Like>,
    pub author: Author,
}

graphql_object!(Post: Context |&self| {
    field id(&executor) -> Option<String> {
        let database = executor.context();
        Some(String::new())
    }
});
