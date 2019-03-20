use crate::db::schema::authors::dsl::{
    author as author_col, authors as authors_table, id as author_id_col,
};

use crate::db::models::abouts::{get_author_abouts, AboutDescription, AboutImage, AboutName};
use crate::db::Context;
use diesel::prelude::*;

#[derive(Default)]
pub struct Author {
    pub author_id: i32,
}

graphql_object!(Author: Context |&self| {
    field name(&executor) -> Option<String> {
        let connection = executor.context().connection.lock().unwrap();
        get_author_abouts::<AboutName>(&(*connection), self.author_id)

    }
    field description(&executor) -> Option<String> {
        let connection = executor.context().connection.lock().unwrap();
        get_author_abouts::<AboutDescription>(&(*connection), self.author_id)
    }
    field image_link(&executor) -> Option<String> {
        let connection = executor.context().connection.lock().unwrap();
        get_author_abouts::<AboutImage>(&(*connection), self.author_id)

    }
    field id(&executor) -> String {
        let connection = executor.context().connection.lock().unwrap();
        authors_table
            .select(author_col)
            .filter(author_id_col.eq(self.author_id))
            .first::<String>(&(*connection))
            .unwrap()

    }
});
