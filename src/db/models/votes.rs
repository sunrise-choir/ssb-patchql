use crate::db::{Error, SqliteConnection};
use crate::lib::*;

pub fn insert_or_update_votes(connection: &SqliteConnection, message: &SsbMessage) {
    unimplemented!()
    //    if let Value::Number(value) = &message.value.content["vote"]["value"] {
    //        if let Value::String(link) = &message.value.content["vote"]["link"] {
    //            let author_id = find_or_create_author(&connection, &message.value.author).unwrap();
    //            let link_to_key_id = find_or_create_key(connection, link).unwrap();
    //
    //            if value.as_i64().unwrap() == 1 {
    //                connection
    //                    .prepare_cached("INSERT INTO votes_raw (link_from_author_id, link_to_key_id) VALUES (?, ?)")
    //                    .unwrap()
    //                    .execute(&[&author_id, &link_to_key_id])
    //                    .unwrap();
    //            } else {
    //                connection
    //                    .prepare_cached("DELETE FROM votes_raw WHERE link_from_author_id = ? AND link_to_key_id = ?")
    //                    .unwrap()
    //                    .execute(&[&author_id, &link_to_key_id])
    //                    .unwrap();
    //            }
    //        }
    //    }
}
