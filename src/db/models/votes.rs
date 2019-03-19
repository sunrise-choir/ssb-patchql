use super::authors::{find_or_create_author, Author};
use super::keys::{find_or_create_key, Key};
use crate::db::schema::votes;
use crate::db::schema::votes::dsl::{
    link_from_author_id as link_from_author_col, link_to_key_id as link_to_key_col, value,
    votes as votes_table,
};
use crate::db::SqliteConnection;
use crate::lib::*;
use diesel::prelude::*;
use diesel::replace_into;
use serde_json::Value;

#[derive(Queryable, Identifiable, Associations, Debug, Default)]
#[belongs_to(Key, foreign_key = "link_to_key_id")]
#[belongs_to(Author, foreign_key = "link_from_author_id")]
pub struct Vote {
    pub id: Option<i32>,
    pub link_from_author_id: Option<i32>,
    pub link_to_key_id: Option<i32>,
    pub value: Option<i32>,
}

pub fn insert_or_update_votes(connection: &SqliteConnection, message: &SsbMessage) {
    if let Value::Number(vote_value) = &message.value.content["vote"]["value"] {
        if let Value::String(link) = &message.value.content["vote"]["link"] {
            let author_id = find_or_create_author(&connection, &message.value.author).unwrap();
            let link_to_key = find_or_create_key(connection, link).unwrap();

            let vote_num: Option<i32> = vote_value.as_i64().map(|num| num as i32);

            replace_into(votes_table)
                .values((
                    link_from_author_col.eq(author_id),
                    link_to_key_col.eq(link_to_key),
                    value.eq(vote_num),
                ))
                .execute(connection)
                .unwrap();
        }
    }
}
