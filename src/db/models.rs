use crate::schema::*;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "messages"]
pub struct Message {
    pub flume_seq: Option<i64>,
    pub key_id: Option<i32>,
    pub seq: Option<i32>,
    pub received_time: Option<f64>,
    pub asserted_time: Option<f64>,
    pub author_id: Option<i32>,
    pub content_type: Option<String>,
    pub content: Option<String>,
    pub is_decrypted: Option<bool>
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "keys"]
pub struct Key {
    pub id: Option<i32>,
    pub key: String
}

impl Default for Message {
    fn default() -> Message {
        Message{
            flume_seq: Some(0),
            key_id: None,
            seq: None,
            received_time: None,
            asserted_time: None,
            author_id: None,
            content_type: None,
            content: None,
            is_decrypted: None
        }
    }

}

#[cfg(test)]
mod tests {

use crate::diesel::prelude::*;
use crate::schema::messages::dsl::*;
use crate::schema::keys::dsl::*;
use crate::establish_connection;
use crate::models::*;
use diesel::result::Error;

    #[test]
    fn insert_message() {
        let connection = establish_connection();
        connection.test_transaction::<_, Error, _>(||{
            let mut new_message = Message::default();
            new_message.flume_seq = Some(1234);

            diesel::insert_into(messages)
                .values(&new_message)
                .execute(&connection)?;
        
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
        connection.test_transaction::<_, Error, _>(||{
            Ok(())
        })
    }
    #[test]
    fn find_or_create_key_when_key_does_not_exist() {
        let connection = establish_connection();
        connection.test_transaction::<_, Error, _>(||{
            diesel::insert_or_ignore_into(keys)
                .values((crate::schema::keys::id.eq(0), key.eq("piet")))
                .execute(&connection)?;

            let results = keys
                .load::<Key>(&connection)
                .expect("Error loading posts");

            assert_eq!(results.len(), 1);
            assert_eq!(results[0].key, "piet");
            Ok(())
        })
    }
}
