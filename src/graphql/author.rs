use crate::db::*;

#[derive(Default)]
pub struct Author {
    id: String,
    name: String,
    description: String,
    image_link: String,
}

graphql_object!(Author: Context |&self| {
    field name(&executor) -> Option<String> {
        //let id = self.id;
        let database = executor.context();
        Some(String::new())
    }
    field id() -> &str { self.id.as_str() }
});
