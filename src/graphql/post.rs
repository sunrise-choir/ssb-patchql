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
    root_key_id, fork_key_id,
    messages as messages_table,
};
use crate::db::schema::votes::dsl::{link_to_key_id as votes_link_to_key_col, votes as votes_table};
use crate::db::schema::links::dsl::{link_to_key_id as links_link_to_key_col, links as links_table, link_from_key_id as links_link_from_key_id};

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
            .filter(votes_link_to_key_col.eq(self.key_id))
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
            .filter(votes_link_to_key_col.eq(self.key_id))
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
    field references(&executor) -> FieldResult<Vec<Post>> {
        let connection = executor.context().connection.lock()?;

        //want to get all messages of type post that link_to this post's key_id
        //and who have a different root
        //and who have a different fork

        // first, we need this Post's root and fork values
        //
        let (this_root_key_id, this_fork_key_id) = messages_table
            .select((root_key_id, fork_key_id))
            .filter(messages_key_id.eq(self.key_id))
            .first::<(Option<i32>, Option<i32>)>(&(*connection))?;

        let posts = links_table
            .inner_join(messages_table.on(
                    messages_key_id.eq(links_link_to_key_col)
                    ))
            .select(links_link_from_key_id)
            .filter(links_link_to_key_col.eq(self.key_id))
            .filter(root_key_id.ne(self.key_id)) //If this message is the root, then they are a reply to this message, not a backlink reference
            .filter(fork_key_id.ne(self.key_id)) //If this message is the head of the fork, then they are a fork of this message, not a backlink reference
            .load::<i32>(&(*connection))?
            .iter()
            .map(|key_id|{
                Post{
                    key_id: *key_id
                }
            })
            .collect::<Vec<Post>>();

        Ok(posts)
    }
});
