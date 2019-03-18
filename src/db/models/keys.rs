use crate::schema::*;

#[derive(Queryable, Insertable, Identifiable, Debug)]
#[table_name = "keys"]
pub struct Key {
    pub id: Option<i32>,
    pub key: String,
}


