use crate::db::schema::*;
use crate::db::*;

use crate::db::schema::authors::dsl::{
    author as authors_author_row, authors as authors_table, id as authors_id_row,
};
use diesel::insert_into;

#[derive(Queryable, Insertable, Identifiable, Debug)]
#[table_name = "authors"]
pub struct Author {
    pub id: Option<i32>,
    pub author: String,
}

pub fn find_or_create_author(connection: &SqliteConnection, author: &str) -> Result<i32, Error> {
    authors_table
        .select(authors_id_row)
        .filter(authors_author_row.eq(author))
        .first::<Option<i32>>(connection)
        .map(|res| res.unwrap())
        .or_else(|_| {
            insert_into(authors_table)
                .values(authors_author_row.eq(author))
                .execute(connection)?;

            authors_table
                .select(authors_id_row)
                .order(authors_id_row.desc())
                .first::<Option<i32>>(connection)
                .map(|key| key.unwrap())
        })
}
