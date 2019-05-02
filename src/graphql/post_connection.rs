use super::page_info::PageInfo;
use super::post::Post;
use crate::db::Context;

#[derive(Default)]
pub struct PostConnection {
    pub next: i32,
    pub page_info: PageInfo,
    pub post_keys: Vec<i32>,
}

graphql_object!(PostConnection: Context |&self| {
    description: "Connection to collections of posts"

    /// The total count of posts in this connection.
    field total_count(&executor) -> i32 {
        self.post_keys.len() as i32
    }
    /// The nodes in this connection
    field nodes(&executor) -> Vec<Post>{

        self.post_keys
            .iter()
            .map(|key_id|{
                Post{key_id: *key_id}
            })
            .collect::<Vec<Post>>()
    }

    /// The relay-spec pageInfo for this connection
    field page_info(&executor) -> &PageInfo{
        &self.page_info
    }
});
