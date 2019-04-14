use crate::db::SqliteConnection;
use crate::lib::*;
use diesel::insert_into;
use diesel::prelude::*;
use serde_json::Value;

use crate::db::schema::texts::dsl::{rowid as texts_rid, text as texts_text, texts as texts_table};

pub fn insert_texts(connection: &SqliteConnection, message: &SsbMessage, key_id: i32) {
    if let Value::String(text) = &message.value.content["text"] {
        insert_into(texts_table)
            .values((texts_rid.eq(key_id), texts_text.eq(text)))
            .execute(connection)
            .unwrap();
    }
}
