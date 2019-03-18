use crate::schema::*;
use super::keys::*;

#[derive(Queryable, Insertable, Associations, Identifiable, Debug, Default)]
#[table_name = "messages"]
#[primary_key(flume_seq)]
#[belongs_to(Key)]
pub struct Message {
    pub flume_seq: Option<i64>,
    pub key_id: Option<i32>,
    pub seq: Option<i32>,
    pub received_time: Option<f64>,
    pub asserted_time: Option<f64>,
    pub root_key_id: Option<i32>,
    pub fork_key_id: Option<i32>,
    pub author_id: Option<i32>,
    pub content_type: Option<String>,
    pub content: Option<String>,
    pub is_decrypted: Option<bool>,
}
