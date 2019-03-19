use serde_json::Value;
use crate::db::schema::authors::dsl::{
    author as author_col, authors as authors_table, id as author_id_col,
};

use crate::db::schema::abouts::dsl::{
    abouts as abouts_table, link_from_key_id, link_to_author_id
};

use crate::db::schema::messages::dsl::{
    messages as messages_table,
    key_id as messages_key_id,
    content as messages_content,
    flume_seq as messages_flume_seq,
    author_id as messages_author_id,
};
use crate::db::Context;
use diesel::prelude::*;

#[derive(Default)]
pub struct Author {
    pub author_id: i32,
}

#[derive(Deserialize)]
struct AboutName {
    name: String
}

graphql_object!(Author: Context |&self| {
    field name(&executor) -> Option<String> {
        let connection = executor.context().connection.lock().unwrap();

        let contents: Vec<Option<String>> = abouts_table
            .inner_join(messages_table.on(
                    messages_key_id.eq(link_from_key_id)
                    ))
            .select(messages_content)
            .order(messages_flume_seq.desc())
            .filter(link_to_author_id.eq(self.author_id))
            .filter(messages_author_id.eq(self.author_id))
            .load(&(*connection))
            .unwrap();

        println!("{:?}", contents);

        //remove empty strings (doubt it will ever happen anyway)
        let strings = contents.into_iter()
            .filter(|item| item.is_some())
            .map(|item| item.unwrap())
            .collect::<Vec<String>>();

        let names = strings
            .iter()
            .map(|item| {
                serde_json::from_str::<AboutName>(&item)
            })
            .filter(|item| item.is_ok())
            .map(|item| item.unwrap())
            .map(|item| item.name)
            .take(1)
            .collect::<Vec<_>>();

        let name: String = names
            .first()
            .map(|s| {(*s).clone()})
            .unwrap_or_else(|| String::new());


        Some(name)
    }
    field description(&executor) -> Option<String> {
        let connection = executor.context().connection.lock().unwrap();
        Some(String::new())
    }
    field image_link(&executor) -> Option<String> {
        let connection = executor.context().connection.lock().unwrap();
        Some(String::new())
    }
    field id(&executor) -> String {
        let connection = executor.context().connection.lock().unwrap();
        authors_table
            .select(author_col)
            .filter(author_id_col.eq(self.author_id))
            .first::<String>(&(*connection))
            .unwrap()

    }
});
