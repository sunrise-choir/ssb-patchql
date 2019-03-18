use crate::schema::*;
use crate::db::*;

use diesel::prelude::*;
use diesel::insert_into;
use crate::db::schema::keys::dsl::{key as keys_key_row, keys as keys_table, id as keys_id_row};

#[derive(Queryable, Insertable, Identifiable, Debug)]
#[table_name = "keys"]
pub struct Key {
    pub id: Option<i32>,
    pub key: String,
}

pub fn find_or_create_key(connection: &SqliteConnection, key: &str) -> Result<i32, Error> {
    keys_table
        .select(keys_id_row)
        .filter(keys_key_row.eq(key))
        .first::<Option<i32>>(connection)
        .map(|res| res.unwrap())
        .or_else(|_|{
            insert_into(keys_table)
                .values(keys_key_row.eq(key))
                .execute(connection)?;

            keys_table
                .order(keys_id_row.desc())
                .first::<Key>(connection)
                .map(|key| key.id.unwrap())
        }) 
}
