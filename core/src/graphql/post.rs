use super::author::*;
use super::like::*;
use crate::db::*;
use diesel::prelude::*;
use juniper::FieldResult;

use crate::db::models::keys::*;
use crate::db::models::votes::*;
use crate::db::schema::keys::dsl::{id as keys_id, key as keys_key, keys as keys_table};
use crate::db::schema::links::dsl::{
    link_from_key_id as links_link_from_key_id, link_to_key_id as links_link_to_key_col,
    links as links_table,
};
use crate::db::schema::messages::dsl::{
    asserted_time as messages_asserted_time, author_id as messages_author_id,
    content_type as messages_content_type, fork_key_id, fork_key_id as messages_fork_key_id,
    key_id as messages_key_id, messages as messages_table, received_time as messages_received_time,
    root_key_id, root_key_id as messages_root_key_id,
};
use crate::db::schema::votes::dsl::{
    link_from_author_id as votes_link_from_author_id, link_to_key_id as votes_link_to_key_col,
    value as votes_value, votes as votes_table,
};

use crate::db::models::posts::get_text;
use crate::db::schema::authors::dsl::{
    authors as authors_table, id as authors_id, is_me as authors_is_me,
};

#[derive(Default)]
pub struct Post {
    pub key_id: i32,
}

graphql_object!(Post: Context |&self| {

    description: "A post by an author. Posts may contain text / images etc. Same idea as a facebook / twitter post"

    /// The globally unique identifier of this post, derived from the hash of this message.
    field id(&executor) -> FieldResult<String> {
        let connection = executor.context().connection.get()?;
        let key = keys_table.find(self.key_id)
            .first::<Key>(&connection)
            .map(|key| key.key)?;

        Ok(key)
    }

    /// The author of this post.
    field author(&executor) -> FieldResult<Author> {
        let connection = executor.context().connection.get()?;
        let author_id = messages_table
            .select(messages_author_id)
            .filter(messages_key_id.eq(self.key_id))
            .first(&connection)?;

        Ok(Author{author_id})
    }

    /// The likes other authors have published about this post.
    field likes(&executor) -> FieldResult<Vec<Like>> {
        let connection = executor.context().connection.get()?;

        let votes: Vec<Vote> = votes_table
            .filter(votes_link_to_key_col.eq(self.key_id))
            .load(&connection)?;

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

    /// Whether this post is liked by me. 
    field liked_by_me(&executor ) -> FieldResult<bool> {
        let connection = executor.context().connection.get()?;

        let count = votes_table
            .inner_join(authors_table.on(authors_id.eq(votes_link_from_author_id.nullable())))
            .filter(votes_link_to_key_col.eq(self.key_id))
            .filter(authors_is_me.is_not_null())
            .select(votes_value)
            .load::<i32>(&connection)?
            .iter()
            .filter(|vote| vote == &&1)
            .count();

        Ok(count > 0)
    }
    /// The number of likes on this post.
    field likes_count(&executor ) -> FieldResult<i32> {
        let connection = executor.context().connection.get()?;

        let count = votes_table
            .filter(votes_link_to_key_col.eq(self.key_id))
            .load::<Vote>(&connection)?
            .iter()
            .filter(|vote| vote.value == 1)
            .count();

        Ok(count as i32)
    }

    /// The asserted timestamp of the post.
    /// Asserted means that it's the time the author claims that they published the message.
    /// You can't totally trust this value, the author may have their clock set wrong, be in a
    /// different timezone, or they might be deliberately setting an incorrect published time for
    /// some reason, eg. to prevent leaking meta-data.
    field asserted_timestamp(&executor) -> FieldResult<Option<f64>> {
        let connection = executor.context().connection.get()?;
        let time = messages_table
            .select(messages_asserted_time)
            .filter(messages_key_id.eq(self.key_id))
            .first::<Option<f64>>(&connection)?;

        Ok(time)
    }

    /// The received timestamp of the post.
    /// This is the time that the message was inserted into your db on this machine.
    /// You know that a message cannot have been published any later than the received timestamp.
    /// **BUT** you can't tell if the message was originally published 5 seconds or 5 years before it was
    /// inserted into your database. This **can happen** eg when an author comes into your follow
    /// range but has been on the network for a long time and you download their entire feed in one
    /// go.
    field received_timestamp(&executor) -> FieldResult<f64> {
        let connection = executor.context().connection.get()?;
        let time = messages_table
            .select(messages_received_time)
            .filter(messages_key_id.eq(self.key_id))
            .first::<f64>(&connection)?;

        Ok(time)
    }

    /// The text body of the post.
    field text(&executor) -> FieldResult<String> {
        let connection = executor.context().connection.get()?;
        let text = get_text(&connection, self.key_id)?;
        Ok(text)
    }
    /// If this post forks from another discussion, the forksFromKey is the id of the message
    /// that it forks from.
    field forks_from_key(&executor) -> FieldResult<Option<String>> {
        let connection = executor.context().connection.get()?;
        let fork_key = messages_table
            .inner_join(keys_table.on(
                    messages_fork_key_id.eq(keys_id)
                    ))
            .select(keys_key)
            .filter(messages_key_id.eq(self.key_id))
            .first::<String>(&connection)
            .optional()?;
        Ok(fork_key)
    }
    /// If this post is a part of a thread then the root_key is the id of the messsage that started
    /// the thread.
    field root_key(&executor) -> FieldResult<Option<String>> {
        let connection = executor.context().connection.get()?;
        let root_key = messages_table
            .inner_join(keys_table.on(
                    messages_root_key_id.nullable().eq(keys_id)
                    ))
            .select(keys_key)
            .filter(messages_key_id.eq(self.key_id))
            .first::<String>(&connection)
            .optional()?;

        Ok(root_key)
    }
    /// Any other messages outside this thread that link / reference this one.
    field references(&executor) -> FieldResult<Vec<Post>> {
        let connection = executor.context().connection.get()?;

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
            .load::<i32>(&connection)?
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
        let connection = executor.context().connection.get()?;

        // first, we need this Post's root key id
        let this_root_key_id = messages_table
            .select(root_key_id)
            .filter(messages_key_id.eq(self.key_id))
            .first::<Option<i32>>(&connection)?;

        let posts = messages_table
            .select(messages_key_id)
            .filter(messages_content_type.ne("about"))
            .filter(messages_content_type.ne("tag"))
            .filter(messages_content_type.ne("vote"))
            .filter(root_key_id.eq(self.key_id)) // A fork message will have its root set pointing to this message
            .filter(fork_key_id.eq(this_root_key_id)) // A fork message will have its fork key pointing at the same root message as this one's root.
            .load::<i32>(&connection)?
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
