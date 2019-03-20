use super::author::*;
use super::like::*;
use crate::db::*;
use diesel::prelude::*;

use super::like_connection::*;
use crate::db::models::keys::*;
use crate::db::models::votes::*;
use crate::db::schema::keys::dsl::keys as keys_table;
use crate::db::schema::votes::dsl::{link_to_key_id as link_to_key_col, votes as votes_table};

#[derive(Default)]
pub struct Post {
    pub key_id: i32,
    pub author_id: i32,
    pub text: String,
}

graphql_object!(Post: Context |&self| {
    field id(&executor) -> String {
        let connection = executor.context().connection.lock().unwrap();
        keys_table.find(self.key_id)
            .first::<Key>(&(*connection))
            .map(|key| key.key)
            .unwrap_or_else(|_|"key_not_found".to_string())
    }

    field author(&executor) -> Author {
        let connection = executor.context().connection.lock().unwrap();

        Author{author_id: self.author_id}
    }

    field likes(&executor) -> Vec<Like> {
        let connection = executor.context().connection.lock().unwrap();

        let votes: Vec<Vote> = votes_table
            .filter(link_to_key_col.eq(self.key_id))
            .load(&(*connection)).unwrap();

        votes
            .iter()
            .filter(|vote| vote.value != 0)
            .map(|vote|{
                Like{
                    author_id: vote.link_from_author_id,
                    value: vote.value
                }
            })
            .collect()
    }
    field likes_connection(&executor) -> LikeConnection {
        let connection = executor.context().connection.lock().unwrap();

        let count = votes_table
            .filter(link_to_key_col.eq(self.key_id))
            .load::<Vote>(&(*connection)).unwrap()
            .iter()
            .filter(|vote| vote.value == 1)
            .count();

        LikeConnection{count: count as i32}
    }
    field text() -> &str {self.text.as_str() }
});
