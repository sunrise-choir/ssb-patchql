use diesel::prelude::*;
use diesel::replace_into;

use super::authors::find_or_create_author;
use crate::db::schema::contacts::dsl::{
    author_id, contact_author_id, contacts, is_decrypted as is_decrypted_column, state,
};
use crate::db::{SqliteConnection};
use crate::lib::*;

use serde_json::Value;
pub fn insert_or_update_contacts(
    connection: &SqliteConnection,
    message: &SsbMessage,
    _message_key_id: i32,
    is_decrypted: bool,
) {
    if let Value::String(contact) = &message.value.content["contact"] {
        let is_blocking = message.value.content["blocking"].as_bool().unwrap_or(false);
        let is_following = message.value.content["following"]
            .as_bool()
            .unwrap_or(false);
        let follow_state = if is_blocking {
            Some(-1)
        } else if is_following {
            Some(1)
        } else {
            None
        };

        let author = find_or_create_author(&connection, &message.value.author).unwrap();
        let contact_author = find_or_create_author(&connection, contact).unwrap();

        replace_into(contacts)
            .values((
                author_id.eq(author),
                contact_author_id.eq(contact_author),
                is_decrypted_column.eq(is_decrypted),
                state.eq(follow_state),
            ))
            .execute(connection)
            .unwrap();
    }
}
