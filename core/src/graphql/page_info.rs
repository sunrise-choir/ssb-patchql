/// A relay-spec PageInfo object used for pagination of queries.
#[derive(GraphQLObject, Default)]
pub struct PageInfo {
    /// Is there a next page available to read?
    pub has_next_page: bool,
    /// Is there a previous page available to read?
    pub has_previous_page: bool,
    /// The cursor for the last item in the page.
    pub end_cursor: Option<String>,
    /// The cursor for the first item in the page.
    pub start_cursor: Option<String>,
}
