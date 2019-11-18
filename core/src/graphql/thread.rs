use super::post::*;
use crate::db::schema::keys::dsl::{id as keys_id, key as keys_key, keys as keys_table};
use crate::db::schema::messages::dsl::{
    content as messages_content, content_type as messages_content_type, key_id as messages_key_id,
    messages as messages_table, root_key_id as messages_root_key_id,
};
use crate::db::Context;
use diesel::prelude::*;
use juniper::FieldResult;
use ssb_causal_sort::causal_sort;
use ssb_multiformats::multihash::Multihash;

pub struct Thread {
    pub root: Post,
    pub cursor: String,
}

graphql_object!(Thread: Context |&self| {
    description: "A thread of posts. Threads have a root post and a collection of reply posts."

    /// The root (intitial) post.
    field root(&executor) -> &Post {
        &self.root
    }
    /// The reply posts.
    /// The replies are sorted by causal ordering based on which messages reference other messages.
    field replies(&executor) -> FieldResult<Vec<Post>>{
        let connection = executor.context().connection.get()?;

        // causal sort wants a collection of (multihash, key_id, bytes)
        let replies = messages_table
            .inner_join(keys_table.on(messages_key_id.nullable().eq(keys_id)))
            .select((messages_content, messages_key_id,  keys_key))
            .filter(messages_root_key_id.eq(self.root.key_id))
            .filter(messages_content_type.eq("post"))
            .load::<(Option<String>, i32, String)>(&connection)
            .into_iter()
            .flatten()
            .filter_map(|(content_string, message_id, key_string)|{
                match Multihash::from_legacy(key_string.as_bytes()){
                    Ok((key, _)) => Some((key, message_id, content_string.unwrap_or("{}".into()))),
                    _ => None

                }
            })
            .collect::<Vec<_>>();

        let sorted_reply_keys = causal_sort(replies.as_slice());

        let sorted_replies = sorted_reply_keys
            .into_iter()
            .map(|key_id| Post{key_id})
            .rev()
            .collect();

        Ok(sorted_replies)
    }
    /// Whether or not the messages are encrypted.
    field is_private() -> bool {false}
});
