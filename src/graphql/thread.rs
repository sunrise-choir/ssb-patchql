use super::post::*;
use crate::db::schema::messages::dsl::{
    author_id as messages_author_id, content as messages_content,
    content_type as messages_content_type, key_id as messages_key_id, messages as messages_table,
    root_key_id as messages_root_key_id,
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
    field root(&executor) -> &Post {
        &self.root
    }
    field replies(&executor) -> Vec<Post>{
        let connection = executor.context().connection.lock().unwrap();

        messages_table
            .select((messages_content, messages_key_id, messages_author_id))
            .filter(messages_root_key_id.eq(self.root.key_id))
            .filter(messages_content_type.eq("post"))
            .load::<(Option<String>, i32, i32)>(&(*connection))
            .into_iter()
            .flatten()
            .filter(|(content, key_id, _)|{
                content.is_some()
            })
            .map(|(content, key_id, author_id)|{
                (content.unwrap(), key_id, author_id)
            })
            .map(|(content, key_id, author_id)|{
                (serde_json::from_str::<PostText>(&content).unwrap().text, key_id, author_id)
            })
            .map(|(text, key_id, author_id)|{
                Post{key_id, text, author_id }
            })
            .collect::<Vec<Post>>()
    }
    field is_private() -> bool {self.is_private}
});
