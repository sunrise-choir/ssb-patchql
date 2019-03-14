use crate::author::*;
use crate::like::*;

#[derive(GraphQLObject, Default)]
pub struct Post {
    pub id: String,
    pub text: String,
    pub likes: Vec<Like>,
    pub author: Author,
}
