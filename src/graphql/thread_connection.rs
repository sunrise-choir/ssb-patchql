use super::page_info::PageInfo;
use super::post::Post;
use super::thread::Thread;
use crate::db::Context;

#[derive(Default)]
pub struct ThreadConnection {
    pub next: i32,
    pub page_info: PageInfo,
    pub thread_keys: Vec<i32>,
}

graphql_object!(ThreadConnection: Context |&self| {
    field nodes(&executor) -> Vec<Thread>{
        self.thread_keys
            .iter()
            .map(|key_id|{
                Thread{root: Post{key_id: *key_id}}
            })
            .collect::<Vec<Thread>>()
    }

    field page_info(&executor) -> &PageInfo{
        &self.page_info
    }

    field total_count(&executor) -> i32 {
        0
    }
});
