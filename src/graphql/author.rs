#[derive(GraphQLObject, Default)]
pub struct Author {
    id: String,
    name: String,
    description: String,
    image_link: String,
}
