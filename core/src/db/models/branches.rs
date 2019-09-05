use super::keys::find_or_create_key;
use crate::db::schema::branches::dsl::{
    branches as branches_table, link_from_key_id, link_to_key_id,
};
use crate::db::SqliteConnection;
use crate::ssb_message::*;
use diesel::insert_into;
use diesel::prelude::*;
use serde_json::Value;

pub fn insert_branches(connection: &SqliteConnection, message: &SsbMessage, message_key_id: i32) {
    if let Some(branches_value) = message.value.content.get("branch") {
        let branches = match branches_value {
            Value::Array(arr) => arr
                .iter()
                .map(|value| value.as_str().unwrap().to_string())
                .collect(),
            Value::String(branch) => vec![branch.as_str().to_string()],
            _ => Vec::new(),
        };

        branches
            .iter()
            .map(|branch| find_or_create_key(connection, branch).unwrap())
            .for_each(|link_to_key| {
                insert_into(branches_table)
                    .values((
                        link_from_key_id.eq(message_key_id),
                        link_to_key_id.eq(link_to_key),
                    ))
                    .execute(connection)
                    .unwrap();
            })
    }
}
