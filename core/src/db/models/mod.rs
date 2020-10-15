pub mod abouts;
pub mod authors;
pub mod blob_links;
pub mod blobs;
pub mod branches;
pub mod contacts;
pub mod keys;
pub mod links;
pub mod mentions;
pub mod messages;
pub mod posts;
pub mod texts;
pub mod votes;

use crate::db::{Error, SqliteConnection};
use crate::ssb_message::*;
use base64::decode;
use flumedb::flume_view::Sequence as FlumeSequence;
use private_box::Keypair;
use serde_json::Value;

use abouts::insert_abouts;
use blob_links::insert_blob_links;
use branches::insert_branches;
use contacts::insert_or_update_contacts;
use keys::find_or_create_key;
use links::insert_links;
use mentions::insert_mentions;
use messages::insert_message;
use posts::insert_post;
use texts::insert_texts;
use votes::insert_or_update_votes;

use authors::find_or_create_author;

pub fn append_item(
    connection: &SqliteConnection,
    secret_keys: &[Keypair],
    seq: FlumeSequence,
    item: &[u8],
) -> Result<(), Error> {
    let result = serde_json::from_slice(item);

    // If there are deleted records with all bytes zerod then we should just skip this message.
    if let Err(_) = result {
        return Ok(());
    }

    let message = result.unwrap();

    let (is_decrypted, message) = attempt_decryption(message, secret_keys);

    let message_key_id = find_or_create_key(&connection, &message.key)?;
    let author_id = find_or_create_author(&connection, &message.value.author)?;

    // votes are a kind of backlink, but we want to put them in their own table.
    match &message.value.content["type"] {
        Value::String(type_string) if type_string == "vote" => {
            insert_or_update_votes(connection, &message);
        }
        _ => {
            let mut links = Vec::new();
            find_values_in_object_by_key(&message.value.content, "link", &mut links);
            insert_links(connection, links.as_slice(), message_key_id);
            insert_mentions(connection, links.as_slice(), message_key_id);
            insert_blob_links(connection, links.as_slice(), message_key_id);
        }
    }

    match &message.value.content["type"] {
        Value::String(type_string) if type_string == "post" => {
            insert_post(connection, &message, message_key_id, author_id, seq as i64)?;
        }
        _ => {}
    }

    insert_branches(connection, &message, message_key_id);
    insert_message(
        connection,
        &message,
        seq as i64,
        message_key_id,
        is_decrypted,
        author_id,
    )?;
    insert_or_update_contacts(connection, &message, message_key_id, is_decrypted);
    insert_abouts(connection, &message, message_key_id);
    insert_texts(connection, &message, message_key_id);

    Ok(())
}

fn attempt_decryption(mut message: SsbMessage, secret_keys: &[Keypair]) -> (bool, SsbMessage) {
    let mut is_decrypted = false;

    message = match message.value.content["type"] {
        Value::Null => {
            let content = message.value.content.clone();
            let strrr = &content.as_str().unwrap().trim_end_matches(".box");

            let bytes = decode(strrr).unwrap();

            message.value.content = secret_keys
                .iter()
                .find_map(|secret_key| private_box::decrypt(&bytes, secret_key))
                .map(|ref data| {
                    is_decrypted = true;
                    serde_json::from_slice(&data).unwrap_or(Value::Null) // Whatever was decrypted wasn't json.
                })
                .unwrap_or(Value::Null); //If we can't decrypt it, throw it away.

            message
        }
        _ => message,
    };

    (is_decrypted, message)
}

#[cfg(test)]
mod tests {
    use crate::db::models::keys::Key;
    use crate::db::models::messages::Message;
    use crate::db::schema::keys::dsl::*;
    use crate::db::schema::messages::dsl::*;
    use crate::utils::establish_connection;
    use diesel::prelude::*;
    use diesel::result::Error;

    //TODO: skipping this.
    fn insert_message() {
        let connection = establish_connection();
        connection.test_transaction::<_, Error, _>(|| {
            let mut new_message = Message::default();
            new_message.flume_seq = Some(1234);

            diesel::insert_into(messages)
                .values(&new_message)
                .execute(&connection)
                .expect("Error inserting message");

            let results = messages
                .limit(1)
                .load::<Message>(&connection)
                .expect("Error loading posts");

            assert_eq!(results[0].flume_seq, Some(1234));
            Ok(())
        })
    }
    #[test]
    fn find_or_create_key_when_key_exists() {
        let connection = establish_connection();
        connection.test_transaction::<_, Error, _>(|| Ok(()))
    }
    #[test]
    fn find_or_create_key_when_key_does_not_exist() {
        let connection = establish_connection();
        connection.test_transaction::<_, Error, _>(|| {
            diesel::insert_or_ignore_into(keys)
                .values((crate::db::schema::keys::id.eq(0), key.eq("piet")))
                .execute(&connection)?;

            let results = keys.load::<Key>(&connection).expect("Error loading posts");

            assert_eq!(results.len(), 1);
            assert_eq!(results[0].key, "piet");
            Ok(())
        })
    }
}

pub fn find_values_in_object_by_key<'a>(
    obj: &'a serde_json::Value,
    key: &str,
    values: &mut Vec<&'a serde_json::Value>,
) {
    if let Some(val) = obj.get(key) {
        values.push(val)
    }

    match obj {
        Value::Array(arr) => {
            for val in arr {
                find_values_in_object_by_key(val, key, values);
            }
        }
        Value::Object(kv) => {
            for val in kv.values() {
                match val {
                    Value::Object(_) => find_values_in_object_by_key(val, key, values),
                    Value::Array(_) => find_values_in_object_by_key(val, key, values),
                    _ => (),
                }
            }
        }
        _ => (),
    }
}
