use super::page_info::PageInfo;
use bytes::{ByteOrder, LittleEndian};
use diesel::dsl::sql;
use diesel::prelude::*;
use juniper::FieldResult;

use super::author::*;
use super::input_objects::*;
use super::post::*;
use super::post_connection::*;
use super::thread::*;
use super::thread_connection::*;
use crate::db::schema::contacts::dsl::{
    author_id as contacts_author_id, contact_author_id as contacts_contact_author_id,
    contacts as contacts_table, state as contacts_state,
};

use crate::db::schema::authors::dsl::{
    author as authors_author, authors as authors_table, id as authors_id,
};
use crate::db::schema::keys::dsl::{id as keys_id_col, key as keys_key_col, keys as keys_table};
use crate::db::schema::mentions::dsl::{
    link_from_key_id as mentions_link_from_key_id, link_to_author_id as mentions_link_to_author_id,
    mentions as mentions_table,
};
use crate::db::schema::messages::dsl::{
    author_id as messages_author_id, content_type as messages_content_type,
    flume_seq as messages_flume_seq, is_decrypted as messages_is_decrypted,
    key_id as messages_key_id, messages as messages_table,
};
use crate::db::schema::reply_posts::dsl::{
    author_id as reply_posts_author_id, flume_seq as reply_posts_flume_seq,
    key_id as reply_posts_key_id, reply_posts as reply_posts_table,
    root_post_id as reply_posts_root_post_id,
};
use crate::db::schema::root_posts::dsl::{
    author_id as root_posts_author_id, flume_seq as root_posts_flume_seq,
    key_id as root_posts_key_id, root_posts as root_posts_table,
};
use crate::db::Context;

use crate::db::schema::texts::dsl::{rowid as texts_key_id, texts as texts_table};

pub struct Query;

fn decode_cursor(encoded: &str) -> Result<i64, String> {
    match base64::decode(encoded) {
        Ok(ref bytes) if bytes.len() < 8 => {
            Err("Error decoding cursor. Is it a valid base64 encoded i64?".to_string())
        }
        Ok(bytes) => Ok(LittleEndian::read_i64(bytes.as_slice())),
        Err(err) => Err(err.to_string()),
    }
}

fn encode_cursor(cursor: i64) -> String {
    base64::encode(&(cursor as u64).to_le_bytes())
}

graphql_object!(Query: Context |&self| {

    description: "All the available root queries."

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
        /// Use a cursor string to get results before the cursor
        before: Option<String>,
        /// Use a cursor string to get results after the cursor
        after: Option<String>,
        /// Limit the number or results to get.
        last = 10: i32,
        /// Limit the number or results to get.
        first = 10: i32,
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
        /// Order threads by asserted time, received time or causal ordering.
        order_by = (OrderBy::Received): OrderBy,
        ) -> FieldResult<ThreadConnection> {

        //TODO Filtering by date ranges!

        let mut next = 10;

        // Get the context from the executor.
        let connection = executor.context().connection.lock()?;

        let mut query = root_posts_table
            .inner_join(messages_table.on(root_posts_key_id.eq(messages_key_id)))
            .inner_join(mentions_table.on(mentions_link_from_key_id.eq(messages_key_id)))
            .select((root_posts_key_id, root_posts_flume_seq))
            .into_boxed();

        if let Some(mentions_authors) = mentions_authors {
            let author_key_ids = authors_table
                .select(authors_id)
                .filter(authors_author.eq_any(mentions_authors))
                .load::<Option<i32>>(&(*connection))?;

            let sub_query = reply_posts_table
                .select(reply_posts_root_post_id)
                .filter(reply_posts_author_id.nullable().eq_any(author_key_ids.clone()));

            query = query
                .or_filter(mentions_link_to_author_id.nullable().eq_any(author_key_ids))
                .or_filter(root_posts_key_id.eq_any(sub_query));
        }


        if let Some(authors) = roots_authored_by {
            let author_key_ids = authors_table
                .select(authors_id)
                .filter(authors_author.eq_any(authors))
                .load::<Option<i32>>(&(*connection))?;

                query = query
                    .or_filter(root_posts_author_id.nullable().eq_any(author_key_ids));
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
                    .or_filter(root_posts_author_id.nullable().eq_any(author_key_ids));
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

            let sub_query = reply_posts_table
                .select(reply_posts_root_post_id)
                .filter(reply_posts_author_id.nullable().eq_any(author_key_ids));

                query = query
                    .or_filter(root_posts_key_id.eq_any(sub_query));
        }

        if let Some(authors) = has_replies_authored_by {
            let author_key_ids = authors_table
                .select(authors_id)
                .filter(authors_author.eq_any(authors))
                .load::<Option<i32>>(&(*connection))?;

            let sub_query = reply_posts_table
                .select(reply_posts_root_post_id)
                .filter(reply_posts_author_id.nullable().eq_any(author_key_ids));

            query = query
                .or_filter(root_posts_key_id.eq_any(sub_query));
        }

        query = match privacy {
            Privacy::Private => {
                query.filter(messages_is_decrypted.eq(true))
            },
            Privacy::Public => {
                query.filter(messages_is_decrypted.eq(false))
            },
            Privacy::All => {
                query
            },
        };

        query = match (&before, &after) {
            (Some(b), None) => {
                let start_cursor = decode_cursor(&b)?;
                next = last;

                query
                    .filter(root_posts_flume_seq.gt(start_cursor))
            },
            (None, Some(a)) => {
                let start_cursor = decode_cursor(&a)?;
                next = first;

                query
                    .filter(root_posts_flume_seq.lt(start_cursor))
            },
            (None, None) => {
                query
            },
            (Some(_), Some(_)) => {
                Err("Before and After can't be set at the same time.")?
            }
        };

        let query = query
            .order(root_posts_flume_seq.desc())
            .limit(next as i64)
            .distinct();


        let results = query
            .load::<(i32, i64)>(&(*connection))?;

        let thread_keys = results
            .iter()
            .map(|(key_id, _)| *key_id)
            .collect::<Vec<i32>>();

        let first_seq: i64 = results
            .first()
            .map(|(_, seq)| *seq)
            .ok_or("No results found")?;

        let last_seq: i64 = results
            .iter()
            .last()
            .map(|(_, seq)| *seq)
            .ok_or("No results found")?;

        let has_next_page = last_seq != 0; //TODO this hard to tell if there is a next page.

        let page_info = PageInfo {
            start_cursor: Some(encode_cursor(first_seq)),
            end_cursor: encode_cursor(last_seq),
            has_next_page,
            has_previous_page: true //TODO make this work.
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
    /// Note that filters for posts are **ANDED** together. Posts meet all the conditions of the
    /// filters to be included in the results.
    field posts(
        &executor,
        /// Use a cursor string to get results before the cursor
        before: Option<String>,
        /// Use a cursor string to get results after the cursor
        after: Option<String>,
        /// Limit the number or results to get.
        first = 10: i32,
        /// Limit the number or results to get.
        last = 10: i32,
        /// Find posts that match the query string.
        query: Option<String>,
        /// Find public, private or all threads.
        privacy = (Privacy::Public): Privacy,
        /// Find posts that are authored by the provided authors.
        authors: Option<Vec<String>>,
        /// Find posts that mention the provided authors.
        mentions_authors: Option<Vec<String>>,
        /// Find posts that mention the provided channels.
        order_by = (OrderBy::Received): OrderBy,
    ) -> FieldResult<PostConnection> {

        let mut next = 10;

        if before.is_some(){
            next = last
        }
        if after.is_some(){
            next = first
        }

        //TODO: Date range
        let connection = executor.context().connection.lock()?;

        let mut boxed_query = messages_table
            .inner_join(mentions_table.on(mentions_link_from_key_id.eq(messages_key_id)))
            .select((messages_key_id, messages_flume_seq))
            .into_boxed();

        if let Some(mentions_authors) = mentions_authors {
            let author_key_ids = authors_table
                .select(authors_id)
                .filter(authors_author.eq_any(mentions_authors))
                .load::<Option<i32>>(&(*connection))?;

            boxed_query = boxed_query
                .filter(mentions_link_to_author_id.nullable().eq_any(author_key_ids));
        }

        if let Some(query_string) = query {
            let matching_texts_keys = texts_table
                .select(texts_key_id)
                .filter(sql("text MATCH ").bind::<diesel::sql_types::Text, _>(query_string))
                .load::<i32>(&(*connection))?;

            boxed_query = boxed_query
                .filter(messages_key_id.eq_any(matching_texts_keys));
        }

        boxed_query = match privacy {
            Privacy::Private => {
                boxed_query.filter(messages_is_decrypted.eq(true))
            },
            Privacy::Public => {
                boxed_query.filter(messages_is_decrypted.eq(false))
            },
            Privacy::All => {
                boxed_query
            },
        };

        if let Some(authors) = authors {
            let author_key_ids = authors_table
                .select(authors_id)
                .filter(authors_author.eq_any(authors))
                .load::<Option<i32>>(&(*connection))?;

                boxed_query = boxed_query
                    .filter(messages_author_id.nullable().eq_any(author_key_ids));
        }

        boxed_query = match (&before, &after) {
            (Some(b), None) => {
                let start_cursor = decode_cursor(&b)?;
                next = last;

                boxed_query
                    .filter(messages_flume_seq.gt(start_cursor))
            },
            (None, Some(a)) => {
                let start_cursor = decode_cursor(&a)?;
                next = first;

                boxed_query
                    .filter(messages_flume_seq.lt(start_cursor))
            },
            (None, None) => {
                boxed_query
            },
            (Some(_), Some(_)) => {
                Err("Before and After can't be set at the same time.")?
            }
        };

        let results = boxed_query
            .filter(messages_content_type.eq("post"))
            .order(messages_flume_seq.desc()) // Hmmm should we switch this off when we're using a query and order by query ranking value?
            .limit(next as i64)
            .distinct()
            .load::<(i32, Option<i64>)>(&(*connection))?;

        let post_keys = results
            .iter()
            .map(|(key_id, _)| *key_id)
            .collect::<Vec<i32>>();

        let first_seq: i64 = results
            .first()
            .map(|(_, seq)| *seq)
            .ok_or("No results found")?
            .ok_or("No results found")?;

        let last_seq: i64 = results
            .iter()
            .last()
            .map(|(_, seq)| *seq)
            .ok_or("No results found")?
            .ok_or("No results found")?;

        let has_next_page = last_seq != 0; //TODO this hard to tell if there is a next page.

        let page_info = PageInfo {
            start_cursor: Some(encode_cursor(first_seq)),
            end_cursor: encode_cursor(last_seq),
            has_next_page,
            has_previous_page: true //TODO make this work.
        };

        Ok(PostConnection{
            next,
            page_info,
            post_keys
        })
    }

    /// Find an author by their public key string.
    field author(&executor, id: String) -> FieldResult<Author>{
        let connection = executor.context().connection.lock()?;

        let author_key_id = authors_table
            .select(authors_id)
            .filter(authors_author.eq(id))
            .first::<Option<i32>>(&(*connection))?
            .ok_or("No author found")?;

        Ok(Author{author_id: author_key_id})
    }

    /// Search for an author by a query string. Will search names and optionally descriptions too.
    field authors(&executor, query: String, exclude_if_blocked_by: Option<Vec<String>>, include_descriptions = false: bool) -> FieldResult<Vec<Author>>{
        Err("Not implemented")?
    }

    /// Find all the message types we know about
    field messageTypes(&executor) -> FieldResult<Vec<String>>{
        let connection = executor.context().connection.lock()?;
        let results = messages_table
            .select(messages_content_type)
            .filter(messages_content_type.is_not_null())
            .distinct()
            .load::<Option<String>>(&(*connection))?
            .into_iter()
            .filter_map(|message_type|{message_type})
            .collect();

        Ok(results)
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
