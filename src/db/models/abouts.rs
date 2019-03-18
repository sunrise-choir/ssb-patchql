use serde_json::Value;

//pub fn insert_abouts(connection: &Connection, message: &SsbMessage, message_key_id: i64) {
//    if let Value::String(about_key) = &message.value.content["about"] {
//        let mut key;
//
//        let (link_to_author_id, link_to_key_id): (&ToSql, &ToSql) = match about_key.get(0..1) {
//            Some("@") => {
//                key = find_or_create_author(connection, about_key).unwrap();
//                (&key, &Null)
//            }
//            Some("%") => {
//                key = find_or_create_key(connection, about_key).unwrap();
//                (&Null, &key)
//            }
//            _ => (&Null, &Null),
//        };
//
//        let mut insert_abouts_stmt = connection
//            .prepare_cached("INSERT INTO abouts_raw (link_from_key_id, link_to_author_id, link_to_key_id) VALUES (?, ?, ?)")
//            .unwrap();
//
//        insert_abouts_stmt
//            .execute(&[&message_key_id, link_to_author_id, link_to_key_id])
//            .unwrap();
//    }
//}
