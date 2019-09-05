use super::mention_connection::MentionConnection;
use crate::db::schema::mentions::dsl::{
    link_from_key_id as mentions_link_from_key_id, link_to_author_id as mentions_link_to_author_id,
    mentions as mentions_table,
};
use crate::db::schema::messages::dsl::{
    flume_seq as messages_flume_seq, key_id as messages_key_id, messages as messages_table,
};
use crate::db::Context;
use diesel::dsl::count_star;
use diesel::prelude::*;
use juniper::FieldResult;

#[derive(Default)]
pub struct Notification {
    pub after_cursor: i64,
    pub author_id: i32,
}

graphql_object!(Notification: Context |&self| {

    field mentions_connection(&executor) -> FieldResult<MentionConnection> {
        let connection = executor.context().connection.get()?;

        let count = mentions_table
            .inner_join(messages_table.on(mentions_link_from_key_id.eq(messages_key_id)))
            .select(count_star())
            .filter(mentions_link_to_author_id.eq(self.author_id))
            .filter(messages_flume_seq.gt(self.after_cursor))
            .first::<i64>(&connection)?;

        Ok(MentionConnection{count: count as i32})
    }
});
