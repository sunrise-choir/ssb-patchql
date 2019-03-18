pub mod abouts;
pub mod messages;
pub mod keys;

#[cfg(test)]
mod tests {

    use crate::diesel::prelude::*;
    use crate::execute_pragmas;
    use crate::models::*;
    use crate::schema::keys::dsl::*;
    use crate::schema::messages::dsl::*;
    use diesel::result::Error;
    use dotenv::dotenv;
    use std::env;

    pub fn establish_connection() -> SqliteConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = SqliteConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));

        execute_pragmas(&connection).unwrap();

        connection
    }
    #[test]
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
                .values((crate::schema::keys::id.eq(0), key.eq("piet")))
                .execute(&connection)?;

            let results = keys.load::<Key>(&connection).expect("Error loading posts");

            assert_eq!(results.len(), 1);
            assert_eq!(results[0].key, "piet");
            Ok(())
        })
    }
}
