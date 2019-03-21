use juniper::FieldResult;
use super::author::*;
use crate::db::*;

#[derive(Default)]
pub struct Like {
    pub author_id: i32,
    pub value: i32,
}

graphql_object!(Like: Context |&self| {
    field author(&executor) -> FieldResult<Author> {
        let connection = executor.context().connection.lock()?;

        let author = Author{author_id: self.author_id};
        Ok(author)
    },
    field value()-> i32{self.value}
});
