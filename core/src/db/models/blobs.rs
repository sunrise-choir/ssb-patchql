use crate::db::schema::*;
use crate::db::*;

use crate::db::schema::blobs::dsl::{blob as blobs_blob, blobs as blobs_table, id as blobs_id};
use diesel::insert_into;

#[derive(Queryable, Insertable, Identifiable, Debug)]
#[table_name = "blobs"]
pub struct Blob {
    pub id: Option<i32>,
    pub blob: String,
}

pub fn find_or_create_blob(connection: &SqliteConnection, blob: &str) -> Result<i32, Error> {
    blobs_table
        .select(blobs_id)
        .filter(blobs_blob.eq(blob))
        .first::<Option<i32>>(connection)
        .map(|res| res.unwrap())
        .or_else(|_| {
            insert_into(blobs_table)
                .values(blobs_blob.eq(blob))
                .execute(connection)?;

            blobs_table
                .select(blobs_id)
                .order(blobs_id.desc())
                .first::<Option<i32>>(connection)
                .map(|key| key.unwrap())
        })
}
