use super::author::*;
use super::input_objects::*;
use super::like::*;
use super::post::*;
use super::like_connection::*;
use crate::db::models::keys::*;
use crate::db::models::votes::*;
use crate::db::schema::keys::dsl::{key as key_column, keys as keys_table};
use crate::db::Context;
use diesel::prelude::*;

#[derive(Default)]
pub struct Thread {
    pub id: String,
    pub text: String,
    pub is_private: bool,
}

graphql_object!(Thread: Context |&self| {
    field author(&executor) -> Author {
        let database = executor.context();
        Author::default()
    }
    field posts(&executor, order_by = (OrderBy::Received): OrderBy) -> Vec<Post> {
        let database = executor.context();

        vec![Post::default(), Post::default()]
    }
    field likes(&executor) -> Vec<Like> {
        let connection = executor.context().connection.lock().unwrap();

        let key: Key = keys_table
            .filter(key_column.eq(self.id.clone()))
            .first::<Key>(&(*connection)).unwrap();

        let votes: Vec<Vote> = Vote::belonging_to(&key)
            .load(&(*connection)).unwrap();

        votes
            .iter()
            .filter(|vote| vote.value.is_some() && vote.value.unwrap() != 0)
            .map(|vote|{
                Like{
                    author_id: vote.link_from_author_id.unwrap(),
                    value: vote.value.unwrap()
                }
            })
            .collect()
    }
    field likes_connection(&executor) -> LikeConnection {
        let connection = executor.context().connection.lock().unwrap();

        let key: Key = keys_table
            .filter(key_column.eq(self.id.clone()))
            .first::<Key>(&(*connection)).unwrap();

        let count = Vote::belonging_to(&key)
            .load::<Vote>(&(*connection)).unwrap()
            .iter()
            .filter(|vote| vote.value.is_some() && vote.value.unwrap() == 1)
            .count();

        LikeConnection{count: count as i32}
    }
    field is_private() -> bool {self.is_private}
    field id() -> &str { self.id.as_str() }
    field text() -> &str {self.text.as_str() }
});
