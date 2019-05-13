use super::author::*;
use crate::db::*;
use juniper::FieldResult;

#[derive(Default)]
pub struct Like {
    pub author_id: i32,
    pub value: i32,
}

graphql_object!(Like: Context |&self| {
    description: "A like or vote published about a certain message.",

    /// The author of the like
    field author(&executor) -> FieldResult<Author> {
        let connection = executor.context().connection.lock()?;

        let author = Author{author_id: self.author_id};
        Ok(author)
    },

    /// The integer value of the like, may be positive or negative.
    field value()-> i32{self.value}
});
