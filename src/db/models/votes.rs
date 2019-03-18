use super::authors::find_or_create_author;
use super::keys::find_or_create_key;
use crate::db::schema::votes::dsl::{link_from_author_id, link_to_key_id, value, votes};
use crate::db::SqliteConnection;
use crate::lib::*;
use diesel::prelude::*;
use diesel::replace_into;
use serde_json::Value;

pub fn insert_or_update_votes(connection: &SqliteConnection, message: &SsbMessage) {
    if let Value::Number(vote_value) = &message.value.content["vote"]["value"] {
        if let Value::String(link) = &message.value.content["vote"]["link"] {
            let author_id = find_or_create_author(&connection, &message.value.author).unwrap();
            let link_to_key = find_or_create_key(connection, link).unwrap();

            let vote_num: Option<i32> = vote_value.as_i64().map(|num| num as i32);

            replace_into(votes)
                .values((
                    link_from_author_id.eq(author_id),
                    link_to_key_id.eq(link_to_key),
                    value.eq(vote_num),
                ))
                .execute(connection)
                .unwrap();
        }
    }
}
