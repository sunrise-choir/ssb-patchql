use super::author::*;

#[derive(GraphQLObject, Default)]
pub struct Like {
    author: Author,
}
