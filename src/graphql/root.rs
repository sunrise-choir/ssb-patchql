use diesel::prelude::*;
use juniper::FieldResult;
use bytes::{ByteOrder, LittleEndian};
use super::page_info::PageInfo;

use super::thread_connection::*;
use super::input_objects::*;
use super::post::*;
use super::post_connection::*;
use super::thread::*;
use super::author::*;
use crate::db::schema::contacts::dsl::{
    author_id as contacts_author_id,
    contact_author_id as contacts_contact_author_id,
    state as contacts_state,
    contacts as contacts_table
};

use crate::db::schema::authors::dsl::{
    author as authors_author, authors as authors_table, id as authors_id,
};
use crate::db::schema::keys::dsl::{id as keys_id_col, key as keys_key_col, keys as keys_table};
use crate::db::Context;
use crate::db::schema::messages::dsl::{
    key_id as messages_key_id, messages as messages_table,
};

use crate::db::schema::threads::dsl::{
    content_type as threads_content_type, flume_seq as threads_flume_seq,
    fork_key_id as threads_fork_key_id, key_id as threads_key_id, threads as threads_table,
    root_key_id as threads_root_key_id,
    author_id as threads_author_id,
    reply_author_id,
};


pub struct Query;

graphql_object!(Query: Context |&self| {

    //TODO Filtering by date ranges!

    /// Find a thread by the key string of the root message.
    field thread(&executor, root_id: String, order_by = (OrderBy::Received): OrderBy) -> FieldResult<Thread> {

        let connection = executor.context().connection.lock()?;

        let thread = keys_table
            .inner_join(messages_table.on(
                    messages_key_id.nullable().eq(keys_id_col)
                    ))
            .select(messages_key_id)
            .filter(keys_key_col.eq(root_id.clone()))
            .first::<i32>(&(*connection))
            .map(|key_id|{
                let root = Post{key_id};
                Thread{root}
            })?;

        Ok(thread)
    }

    /// Search for threads that match _any_ of the selectors.
    /// Eg. if `roots_authored_by` **and** `has_replies_authored_by` are used, you will get threads
    /// where _either_ is true. The selectors are logically OR'd, **not** AND'd.
    field threads(
        &executor,
        before: Option<String>,
        after: Option<String>,
        next = 10: i32,
        /// Find public, private or all threads.
        privacy = (Privacy::Public): Privacy,
        /// Include threads whose root message is authored by one of the provided authors
        roots_authored_by: Option<Vec<String>>,
        /// Include threads whose root message is authored by someone followed by one of the provided authors
        roots_authored_by_someone_followed_by: Option<Vec<String>>,
        /// Include threads that have replies by one of the provided authors.
        has_replies_authored_by: Option<Vec<String>>,
        /// Include threads that have replies by someone followed by one of the provided authors.
        has_replies_authored_by_someone_followed_by: Option<Vec<String>>,
        /// Include threads that mention the provided authors.
        mentions_authors: Option<Vec<String>>,
        /// Include threads that mention the provided channels.
        mentions_channels: Option<Vec<String>>,
        /// Order threads by asserted time, received time or causal ordering.
        order_by = (OrderBy::Received): OrderBy,
        ) -> FieldResult<ThreadConnection> {
        // Get the context from the executor.
        let connection = executor.context().connection.lock()?;

        //TODO: need to handle before and after cases
        let start_seq = match after {
            None => Ok(std::i64::MAX),
            Some(ref encoded) => match base64::decode(&encoded) {
                Ok(ref bytes) if bytes.len() < 8 => {
                    Err("Error decoding cursor. Is it a valid base64 encoded i64?".to_string())
                }
                Ok(bytes) => Ok(LittleEndian::read_i64(bytes.as_slice())),
                Err(err) => Err(err.to_string()),
            },
        }?;

        let mut query = threads_table
            .select((threads_key_id, threads_flume_seq))
            .into_boxed();


        if let Some(authors) = roots_authored_by {
            let author_key_ids = authors_table
                .select(authors_id)
                .filter(authors_author.eq_any(authors))
                .load::<Option<i32>>(&(*connection))?;

                query = query
                    .or_filter(threads_author_id.nullable().eq_any(author_key_ids));
        }

        if let Some(authors) = roots_authored_by_someone_followed_by {
            let author_key_ids = authors_table
                .inner_join(
                    contacts_table.on(authors_id.eq(contacts_author_id.nullable()))
                    )
                .select(contacts_contact_author_id)
                .filter(authors_author.eq_any(authors))
                .filter(contacts_state.eq(1))
                .load::<i32>(&(*connection))?;

                query = query
                    .or_filter(threads_author_id.nullable().eq_any(author_key_ids));
        }

        if let Some(authors) = has_replies_authored_by_someone_followed_by {
            let author_key_ids = authors_table
                .inner_join(
                    contacts_table.on(authors_id.eq(contacts_author_id.nullable()))
                    )
                .select(contacts_contact_author_id)
                .filter(authors_author.eq_any(authors))
                .filter(contacts_state.eq(1))
                .load::<i32>(&(*connection))?;

                query = query
                    .or_filter(reply_author_id.nullable().eq_any(author_key_ids));
        }

        if let Some(authors) = has_replies_authored_by {
            let author_key_ids = authors_table
                .select(authors_id)
                .filter(authors_author.eq_any(authors))
                .load::<Option<i32>>(&(*connection))?;

                query = query
                    .or_filter(reply_author_id.nullable().eq_any(author_key_ids));
        }

        let results = query
            .filter(threads_flume_seq.lt(start_seq))
            .filter(threads_root_key_id.is_null())
            .filter(threads_fork_key_id.is_null())
            .filter(threads_content_type.eq("post"))
            .order(threads_flume_seq.desc())
            .limit(next as i64)
            .distinct()
            .load::<(i32, Option<i64>)>(&(*connection))
            .unwrap();

        let thread_keys = results
            .iter()
            .map(|(key_id, _)| *key_id)
            .collect::<Vec<i32>>();

        let last_seq = results.iter().last().map(|(_, seq)| *seq).unwrap().unwrap();

        let end_cursor = base64::encode(&(last_seq as u64).to_le_bytes());
        let has_next_page = last_seq != 0;

        let page_info = PageInfo {
            start_cursor: after,
            end_cursor,
            has_next_page,
        };

        Ok(ThreadConnection {
            next,
            thread_keys,
            page_info,
        })

    }

    /// Find a post by key string.
    field post(&executor, id: String ) -> FieldResult<Post> {
        let connection = executor.context().connection.lock()?;

        let post = keys_table
            .inner_join(messages_table.on(
                    messages_key_id.nullable().eq(keys_id_col)
                    ))
            .select(messages_key_id)
            .filter(keys_key_col.eq(id.clone()))
            .first::<i32>(&(*connection))
            .map(|key_id|{
                Post{key_id}
            })?;

        Ok(post)
    }

    /// Search for posts that match certain filters.
    field posts(
        &executor,
        /// Find posts that match the query string.
        query: Option<String>,
        /// Find public, private or all threads.
        privacy = (Privacy::Public): Privacy,
        /// Find posts that are authored by the provided authors.
        authored_by: Option<String>,
        /// Find posts that are referenced by the provided authors.
        referenced_by_authors: Option<String>,
        /// Find posts that mention the provided authors.
        mentions_authors: Option<Vec<String>>,
        /// Find posts that mention the provided channels.
        mentions_channels: Option<Vec<String>>,
        /// Order posts by asserted time, received time. Causal ordering not supported.
        order_by = (OrderBy::Received): OrderBy,
    ) -> FieldResult<PostConnection> {

        Err("Not implemented")?
    }

    /// Find an author by their public key string.
    field author(&executor, id: String) -> FieldResult<Author>{
        Err("Not implemented")?
    }

    /// Search for an author by a query string. Will search names and optionally descriptions too.
    field authors(&executor, query: String, exclude_if_blocked_by: Option<Vec<String>>, include_descriptions = false: bool) -> FieldResult<Vec<Author>>{
        Err("Not implemented")?
    }

    /// Find all the message types we know about
    field messageTypes(&executor) -> FieldResult<Vec<String>>{
        Err("Not implemented")?
    }

    /// Find all messages by type
    field messagesByType(&executor, message_type: String) -> FieldResult<String> { //TODO define a Message type that would be compatible with existing js.
        Err("Not implemented")?
    }

    /// Find a message by key string
    field message(&executor, id: String) -> FieldResult<String> { // TODO: use message type. Should be a connection though so we can paginate it.
        Err("Not implemented")?
    }
});
