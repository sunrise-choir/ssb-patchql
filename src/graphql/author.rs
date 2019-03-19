use crate::db::schema::authors::dsl::{
    author as author_col, authors as authors_table, id as author_id_col,
};

use crate::db::schema::abouts::dsl::{abouts as abouts_table, link_from_key_id, link_to_author_id};

use crate::db::schema::messages::dsl::{
    author_id as messages_author_id, content as messages_content, flume_seq as messages_flume_seq,
    key_id as messages_key_id, messages as messages_table,
};
use crate::db::Context;
use diesel::prelude::*;

#[derive(Default)]
pub struct Author {
    pub author_id: i32,
}

#[derive(Deserialize)]
struct AboutName {
    name: String,
}

#[derive(Deserialize)]
struct AboutDescription {
    description: String,
}
#[derive(Deserialize)]
struct ImageInfo {
    link: String,
}
#[derive(Deserialize)]
struct AboutImage {
    image: ImageInfo,
}
graphql_object!(Author: Context |&self| {
    field name(&executor) -> Option<String> {
        let connection = executor.context().connection.lock().unwrap();

        abouts_table
            .inner_join(messages_table.on(
                    messages_key_id.eq(link_from_key_id)
                    ))
            .select(messages_content)
            .order(messages_flume_seq.desc())
            .filter(link_to_author_id.eq(self.author_id))
            .filter(messages_author_id.eq(self.author_id))
            .load::<Option<String>>(&(*connection))
            .unwrap()
            .into_iter()
            .filter_map(|item| item)
            .map(|item| {
                serde_json::from_str::<AboutName>(&item)
                    .map(|item| item.name)
            })
            .filter_map(Result::ok)
            .take(1)
            .collect::<Vec<_>>()
            .first()
            .map(|s| {(*s).clone()})

    }
    field description(&executor) -> Option<String> {
        let connection = executor.context().connection.lock().unwrap();
        abouts_table
            .inner_join(messages_table.on(
                    messages_key_id.eq(link_from_key_id)
                    ))
            .select(messages_content)
            .order(messages_flume_seq.desc())
            .filter(link_to_author_id.eq(self.author_id))
            .filter(messages_author_id.eq(self.author_id))
            .load::<Option<String>>(&(*connection))
            .unwrap()
            .into_iter()
            .filter_map(|item| item)
            .map(|item| {
                serde_json::from_str::<AboutDescription>(&item)
                    .map(|item| item.description)
            })
            .filter_map(Result::ok)
            .take(1)
            .collect::<Vec<_>>()
            .first()
            .map(|s| {(*s).clone()})
    }
    field image_link(&executor) -> Option<String> {
        let connection = executor.context().connection.lock().unwrap();
        abouts_table
            .inner_join(messages_table.on(
                    messages_key_id.eq(link_from_key_id)
                    ))
            .select(messages_content)
            .order(messages_flume_seq.desc())
            .filter(link_to_author_id.eq(self.author_id))
            .filter(messages_author_id.eq(self.author_id))
            .load::<Option<String>>(&(*connection))
            .unwrap()
            .into_iter()
            .filter_map(|item| item)
            .map(|item| {
                serde_json::from_str::<AboutImage>(&item)
                    .map(|item| item.image.link)
            })
            .filter_map(Result::ok)
            .take(1)
            .collect::<Vec<_>>()
            .first()
            .map(|s| {(*s).clone()})

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
