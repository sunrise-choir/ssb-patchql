use crate::db::{Error, SqliteConnection};
use crate::lib::SsbMessage;
use crate::lib::*;
use serde_json::Value;
pub fn insert_or_update_contacts(
    connection: &SqliteConnection,
    message: &SsbMessage,
    _message_key_id: i32,
    is_decrypted: bool,
) {
    return unimplemented!();
    //
    //    if let Value::String(contact) = &message.value.content["contact"] {
    //        //Ok what should this do:
    //        //  - if the record already exists
    //        //      - delete it if the new state is zero (this should only happen when record already
    //        //      exists because you can't unfollow someone you already don't follow.
    //        //      - update it if the new state is 1 or -1
    //        //  - else create the new record. State should be a 1 or a -1
    //        let is_blocking = message.value.content["blocking"].as_bool().unwrap_or(false);
    //        let is_following = message.value.content["following"]
    //            .as_bool()
    //            .unwrap_or(false);
    //        let state = if is_blocking {
    //            -1
    //        } else if is_following {
    //            1
    //        } else {
    //            0
    //        };
    //
    //        let author_id = find_or_create_author(&connection, &message.value.author).unwrap();
    //        let contact_author_id = find_or_create_author(&connection, contact).unwrap();
    //
    //        let mut stmt = connection.prepare_cached("SELECT id FROM contacts_raw WHERE author_id = ? AND contact_author_id = ? AND is_decrypted = ?").unwrap();
    //
    //        stmt.query_row(&[&author_id, &contact_author_id, &is_decrypted as &ToSql], |row| row.get(0))
    //            .and_then(|id: i64|{
    //                //Row exists so update
    //                connection
    //                    .prepare_cached("UPDATE contacts_raw SET state = ? WHERE id = ?")
    //                    .map(|mut stmt| stmt.execute(&[&state, &id]))
    //            })
    //            .or_else(|_| {
    //                //Row didn't exist so insert
    //                connection
    //                    .prepare_cached("INSERT INTO contacts_raw (author_id, contact_author_id, is_decrypted, state) VALUES (?, ?, ?, ?)")
    //                    .map(|mut stmt| stmt.execute(&[&author_id, &contact_author_id, &is_decrypted as &ToSql, &state]))
    //            })
    //            .unwrap()
    //            .unwrap();
    //    }
    //
}
