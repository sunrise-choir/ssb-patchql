use super::author::*;
use super::like::*;
use crate::db::*;
use diesel::dsl::sql;
use diesel::prelude::*;
use juniper::FieldResult;

use super::like_connection::*;
use crate::db::models::keys::*;
use crate::db::models::votes::*;
use crate::db::schema::keys::dsl::keys as keys_table;
use crate::db::schema::messages::dsl::{
    author_id as messages_author_id, content as messages_content, key_id as messages_key_id,
    messages as messages_table,
};
use crate::db::schema::votes::dsl::{link_to_key_id as link_to_key_col, votes as votes_table};

#[derive(Default)]
pub struct Post {
    pub key_id: i32,
}

#[derive(Deserialize)]
struct PostText {
    text: String,
}
graphql_object!(Post: Context |&self| {
    field id(&executor) -> FieldResult<String> {
        let connection = executor.context().connection.lock()?;
        let key = keys_table.find(self.key_id)
            .first::<Key>(&(*connection))
            .map(|key| key.key)?;

        Ok(key)
    }

    field author(&executor) -> FieldResult<Author> {
        let connection = executor.context().connection.lock()?;
        let author_id = messages_table
            .select(messages_author_id)
            .filter(messages_key_id.eq(self.key_id))
            .first(&(*connection))?;

        Ok(Author{author_id})
    }

    field likes(&executor) -> FieldResult<Vec<Like>> {
        let connection = executor.context().connection.lock()?;

        let votes: Vec<Vote> = votes_table
            .filter(link_to_key_col.eq(self.key_id))
            .load(&(*connection))?;

        let result = votes
            .iter()
            .filter(|vote| vote.value != 0)
            .map(|vote|{
                Like{
                    author_id: vote.link_from_author_id,
                    value: vote.value
                }
            })
            .collect();

        Ok(result)
    }
    field likes_connection(&executor) -> FieldResult<LikeConnection> {
        let connection = executor.context().connection.lock()?;

        let count = votes_table
            .filter(link_to_key_col.eq(self.key_id))
            .load::<Vote>(&(*connection))?
            .iter()
            .filter(|vote| vote.value == 1)
            .count();

        Ok(LikeConnection{count: count as i32})
    }
    field text(&executor) -> FieldResult<String> {
        let connection = executor.context().connection.lock()?;
        let content = messages_table
            .select(sql::<diesel::sql_types::Text>("content"))
            .filter(messages_key_id.eq(self.key_id))
            .filter(messages_content.is_not_null())
            .first::<String>(&(*connection))?;

        let value: PostText = serde_json::from_str(&content)?;

        Ok(value.text)
    }
});
