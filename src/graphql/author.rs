use crate::db::schema::authors::dsl::{
    author as author_col, authors as authors_table, id as author_id_col,
};

use crate::db::models::abouts::{get_author_abouts, AboutDescription, AboutImage, AboutName};
use crate::db::Context;
use diesel::prelude::*;
use juniper::FieldResult;

#[derive(Default)]
pub struct Author {
    pub author_id: i32,
}

#[derive(GraphQLEnum)]
pub enum ContactState {
    Follow,
    Block,
    Neutral,
}

#[derive(GraphQLObject)]
pub struct PublicPrivateContactStatus{
    pub public: ContactState,
    pub private: Option<ContactState>,
}

graphql_object!(Author: Context |&self| {
    field name(&executor) -> FieldResult<Option<String>> {
        let connection = executor.context().connection.lock().unwrap();
        let name = get_author_abouts::<AboutName>(&(*connection), self.author_id)?;
        Ok(name)

    }
    field description(&executor) -> FieldResult<Option<String>> {
        let connection = executor.context().connection.lock().unwrap();
        let description = get_author_abouts::<AboutDescription>(&(*connection), self.author_id)?;
        Ok(description)
    }
    field image_link(&executor) -> FieldResult<Option<String>> {
        let connection = executor.context().connection.lock().unwrap();
        let image_link = get_author_abouts::<AboutImage>(&(*connection), self.author_id)?;
        Ok(image_link)

    }
    field id(&executor) -> FieldResult<String> {
        let connection = executor.context().connection.lock().unwrap();
        let id = authors_table
            .select(author_col)
            .filter(author_id_col.eq(self.author_id))
            .first::<String>(&(*connection))?;
        Ok(id)
    }

    field contact_status(&executor, other_author: String) -> FieldResult<PublicPrivateContactStatus> {
        let status = PublicPrivateContactStatus{
            public: ContactState::Neutral,
            private: None
        };
        Ok(status)
    }
    // TODO: Think about how to do private follows / blocks. This could be exposed at the root
    // query level?
    field mutual_friends(&executor) -> FieldResult<Vec<Author>> {
        Err("Unimplemented")?
    }
    field follows(&executor) -> FieldResult<Vec<Author>> {
        Err("Unimplemented")?
    }
    field blocks(&executor) -> FieldResult<Vec<Author>> {
        Err("Unimplemented")?
    }
    field followedBy(&executor) -> FieldResult<Vec<Author>> {
        Err("Unimplemented")?
    }
    field blockedBy(&executor) -> FieldResult<Vec<Author>> {
        Err("Unimplemented")?
    }
});
