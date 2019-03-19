use crate::db::models::authors::Author as AuthorModel;
use crate::db::schema::authors::dsl::{
    author as author_col, authors as authors_table, id as author_id_col,
};
use crate::db::Context;
use diesel::prelude::*;

#[derive(Default)]
pub struct Author {
    pub author_id: i32,
}

graphql_object!(Author: Context |&self| {
    field name(&executor) -> Option<String> {
        let connection = executor.context().connection.lock().unwrap(); 
        Some(String::new())
    }
    field description(&executor) -> Option<String> {
        let connection = executor.context().connection.lock().unwrap(); 
        Some(String::new())
    }
    field image_link(&executor) -> Option<String> {
        let connection = executor.context().connection.lock().unwrap(); 
        Some(String::new())
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
