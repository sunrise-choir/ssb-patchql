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
    field nodes(&executor) -> Vec<Post>{

        self.post_keys
            .iter()
            .map(|key_id|{
                Post{key_id: *key_id}
            })
            .collect::<Vec<Post>>()
    }

    field page_info(&executor) -> &PageInfo{
        &self.page_info
    }
});
