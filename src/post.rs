use crate::like::*;
use crate::author::*;

#[derive(GraphQLObject, Default)]
pub struct Post {
    pub id: String,
    pub text: String,
    pub likes: Vec<Like>,
    pub author: Author,
}
