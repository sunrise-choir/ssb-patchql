use super::author::*;
use super::like::*;
use crate::db::*;
use diesel::dsl::sql;
use diesel::prelude::*;
use juniper::FieldResult;

use super::like_connection::*;
use crate::db::models::keys::*;
use crate::db::models::votes::*;
use crate::db::schema::keys::dsl::{id as keys_id, key as keys_key, keys as keys_table};
use crate::db::schema::links::dsl::{
    link_from_key_id as links_link_from_key_id, link_to_key_id as links_link_to_key_col,
    links as links_table,
};
use crate::db::schema::messages::dsl::{
    author_id as messages_author_id, content as messages_content,
    content_type as messages_content_type, fork_key_id, fork_key_id as messages_fork_key_id,
    key_id as messages_key_id, messages as messages_table, root_key_id,
    root_key_id as messages_root_key_id,
};
use crate::db::schema::votes::dsl::{
    link_to_key_id as votes_link_to_key_col, votes as votes_table,
};

#[derive(Default)]
pub struct Post {
    pub key_id: i32,
}

#[derive(Deserialize)]
struct PostText {
    text: String,
}
graphql_object!(Post: Context |&self| {

    description: "A post by an author. Posts may contain text / images etc. Same idea as a facebook / twitter post"

    /// The globally unique identifier of this post, derived from the hash of this message.
    field id(&executor) -> FieldResult<String> {
        let connection = executor.context().connection.lock()?;
        let key = keys_table.find(self.key_id)
            .first::<Key>(&(*connection))
            .map(|key| key.key)?;

        Ok(key)
    }

    /// The author of this post.
    field author(&executor) -> FieldResult<Author> {
        let connection = executor.context().connection.lock()?;
        let author_id = messages_table
            .select(messages_author_id)
            .filter(messages_key_id.eq(self.key_id))
            .first(&(*connection))?;

        Ok(Author{author_id})
    }

    /// The likes other authors have published about this post. TODO: move this into
    /// likes_connection
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
    /// The connection to likes on this post.
    field likes_connection(&executor, after: Option<String>, first = 10: i32) -> FieldResult<LikeConnection> {
        //TODO: I think the LikeConnection type needs a rethink. It should probaly use cursors
        //properly and have edges / nodes with likes.
        let connection = executor.context().connection.lock()?;

        let count = votes_table
            .filter(votes_link_to_key_col.eq(self.key_id))
            .load::<Vote>(&(*connection))?
            .iter()
            .filter(|vote| vote.value == 1)
            .count();

        Ok(LikeConnection{count: count as i32})
    }

    /// The text body of the post.
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
    /// If this post forks from another discussion, the forksFromKey is the id of the message
    /// that it forks from.
    field forks_from_key(&executor) -> FieldResult<Option<String>> {
        let connection = executor.context().connection.lock()?;
        let fork_key = messages_table
            .inner_join(keys_table.on(
                    messages_fork_key_id.eq(keys_id)
                    ))
            .select(keys_key)
            .filter(messages_key_id.eq(self.key_id))
            .first::<String>(&(*connection))
            .optional()?;
        Ok(fork_key)
    }
    /// If this post is a part of a thread then the root_key is the id of the messsage that started
    /// the thread.
    field root_key(&executor) -> FieldResult<Option<String>> {
        let connection = executor.context().connection.lock()?;
        let root_key = messages_table
            .inner_join(keys_table.on(
                    messages_root_key_id.nullable().eq(keys_id)
                    ))
            .select(keys_key)
            .filter(messages_key_id.eq(self.key_id))
            .first::<String>(&(*connection))
            .optional()?;

        Ok(root_key)
    }
    /// Any other messages outside this thread that link / reference this one.
    field references(&executor) -> FieldResult<Vec<Post>> {
        let connection = executor.context().connection.lock()?;

        let posts = links_table
            .inner_join(messages_table.on(
                    messages_key_id.eq(links_link_to_key_col)
                    ))
            .select(links_link_from_key_id)
            .filter(links_link_to_key_col.eq(self.key_id))
            .filter(messages_content_type.ne("about"))
            .filter(messages_content_type.ne("tag"))
            .filter(messages_content_type.ne("vote"))
            .filter(
                root_key_id.is_not_null().and(root_key_id.ne(self.key_id))
                    .or(root_key_id.is_null())
                )

            .filter(
                fork_key_id.is_not_null().and(fork_key_id.ne(self.key_id))
                    .or(fork_key_id.is_null())
                )
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
    /// Any other threads that have forked from this one.
    field forks(&executor) -> FieldResult<Vec<Post>> {
        let connection = executor.context().connection.lock()?;

        // first, we need this Post's root key id
        let this_root_key_id = messages_table
            .select(root_key_id)
            .filter(messages_key_id.eq(self.key_id))
            .first::<Option<i32>>(&(*connection))?;

        let posts = messages_table
            .select(messages_key_id)
            .filter(messages_content_type.ne("about"))
            .filter(messages_content_type.ne("tag"))
            .filter(messages_content_type.ne("vote"))
            .filter(root_key_id.eq(self.key_id)) // A fork message will have its root set pointing to this message
            .filter(fork_key_id.eq(this_root_key_id)) // A fork message will have its fork key pointing at the same root message as this one's root.
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
