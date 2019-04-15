use crate::db::models::abouts::{get_author_abouts, AboutDescription, AboutImage, AboutName};
use crate::db::schema::authors::dsl::{
    author as authors_author, authors as authors_table, id as authors_id,
};
use crate::db::schema::contacts::dsl::{
    author_id as contacts_author_id, contact_author_id as contacts_contact_author_id,
    contacts as contacts_table, state as contacts_state,
};
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
pub struct PublicPrivateContactStatus {
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
            .select(authors_author)
            .filter(authors_id.eq(self.author_id))
            .first::<String>(&(*connection))?;
        Ok(id)
    }
    field contact_status_to(&executor, other_author: String) -> FieldResult<PublicPrivateContactStatus> {

        let connection = executor.context().connection.lock().unwrap();

        let other_author_id = authors_table
            .select(authors_id)
            .filter(authors_author.eq(other_author))
            .first::<Option<i32>>(&(*connection))?;

        let state = contacts_table
            .select(contacts_state)
            .filter(contacts_author_id.eq(self.author_id))
            .filter(contacts_contact_author_id.nullable().eq(other_author_id))
            .first::<Option<i32>>(&(*connection))
            .optional()?;

        let status = match state {
            Some(Some(1)) => {
                PublicPrivateContactStatus{
                    public: ContactState::Follow,
                    private: None
                }
            },
            Some(Some(-1)) => {
                PublicPrivateContactStatus{
                    public: ContactState::Block,
                    private: None
                }
            },
            _ => {
                PublicPrivateContactStatus{
                    public: ContactState::Neutral,
                    private: None
                }
            }
        };

        Ok(status)
    }
    field contact_status_from(&executor, other_author: String) -> FieldResult<PublicPrivateContactStatus> {

        let connection = executor.context().connection.lock().unwrap();

        let other_author_id = authors_table
            .select(authors_id)
            .filter(authors_author.eq(other_author))
            .first::<Option<i32>>(&(*connection))?;

        let state = contacts_table
            .select(contacts_state)
            .filter(contacts_contact_author_id.eq(self.author_id))
            .filter(contacts_author_id.nullable().eq(other_author_id))
            .first::<Option<i32>>(&(*connection))
            .optional()?;

        let status = match state {
            Some(Some(1)) => {
                PublicPrivateContactStatus{
                    public: ContactState::Follow,
                    private: None
                }
            },
            Some(Some(-1)) => {
                PublicPrivateContactStatus{
                    public: ContactState::Block,
                    private: None
                }
            },
            _ => {
                PublicPrivateContactStatus{
                    public: ContactState::Neutral,
                    private: None
                }
            }
        };

        Ok(status)
    }
    // TODO: Think about how to do private follows / blocks. This could be exposed at the root
    // query level?

    field follows(&executor) -> FieldResult<Vec<Author>> {
        let connection = executor.context().connection.lock().unwrap();

        let authors = authors_table
            .inner_join(
                contacts_table.on(authors_id.eq(contacts_author_id.nullable()))
                )
            .select(contacts_contact_author_id)
            .filter(authors_id.eq(self.author_id))
            .filter(contacts_state.eq(1))
            .load::<i32>(&(*connection))?
            .into_iter()
            .map(|author_id|{
                Author{author_id}
            })
            .collect();

        Ok(authors)
    }
    field blocks(&executor) -> FieldResult<Vec<Author>> {
        let connection = executor.context().connection.lock().unwrap();

        let authors = authors_table
            .inner_join(
                contacts_table.on(authors_id.eq(contacts_author_id.nullable()))
                )
            .select(contacts_contact_author_id)
            .filter(authors_id.eq(self.author_id))
            .filter(contacts_state.eq(-1))
            .load::<i32>(&(*connection))?
            .into_iter()
            .map(|author_id|{
                Author{author_id}
            })
            .collect();

        Ok(authors)
    }
    field followedBy(&executor) -> FieldResult<Vec<Author>> {
        let connection = executor.context().connection.lock().unwrap();

        let authors = authors_table
            .inner_join(
                contacts_table.on(authors_id.eq(contacts_contact_author_id.nullable()))
                )
            .select(contacts_author_id)
            .filter(authors_id.eq(self.author_id))
            .filter(contacts_state.eq(1))
            .load::<i32>(&(*connection))?
            .into_iter()
            .map(|author_id|{
                Author{author_id}
            })
            .collect();

        Ok(authors)
    }
    field blockedBy(&executor) -> FieldResult<Vec<Author>> {

        let connection = executor.context().connection.lock().unwrap();

        let authors = authors_table
            .inner_join(
                contacts_table.on(authors_id.eq(contacts_contact_author_id.nullable()))
                )
            .select(contacts_author_id)
            .filter(authors_id.eq(self.author_id))
            .filter(contacts_state.eq(-1))
            .load::<i32>(&(*connection))?
            .into_iter()
            .map(|author_id|{
                Author{author_id}
            })
            .collect();

        Ok(authors)
    }
});
