use super::author::*;
use super::post::*;
use crate::db::schema::messages::dsl::{
    content as messages_content, content_type as messages_content_type, key_id as messages_key_id,
    messages as messages_table, root_key_id as messages_root_key_id,
};
use crate::db::Context;
use diesel::prelude::*;

#[derive(Default)]
pub struct Thread {
    pub is_private: bool,
    pub root: Post,
}

#[derive(Deserialize)]
struct PostText {
    text: String,
}

graphql_object!(Thread: Context |&self| {
    field author(&executor) -> Author {
        let database = executor.context();
        Author::default()
    }
    field root(&executor) -> &Post {
        &self.root
    }
    field replies(&executor) -> Vec<Post>{
        let connection = executor.context().connection.lock().unwrap();


        messages_table
            .select((messages_content, messages_key_id))
            .filter(messages_root_key_id.eq(self.root.key_id))
            .filter(messages_content_type.eq("post"))
            .load::<(Option<String>, i32)>(&(*connection))
            .into_iter()
            .flatten()
            .filter(|(content, key_id)|{
                content.is_some()
            })
            .map(|(content, key_id)|{
                (content.unwrap(), key_id)
            })
            .map(|(content, key_id)|{
                (serde_json::from_str::<PostText>(&content).unwrap().text, key_id)
            })
            .map(|(text, key_id)|{
                Post{key_id, text }
            })
            .collect::<Vec<Post>>()
    }
    field is_private() -> bool {self.is_private}
});
