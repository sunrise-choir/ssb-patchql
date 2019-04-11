#[derive(GraphQLObject, Default)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: String,
    pub start_cursor: Option<String>,
}
