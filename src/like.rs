use crate::author::*;

#[derive(GraphQLObject, Default)]
pub struct Like {
    author: Author,
}


