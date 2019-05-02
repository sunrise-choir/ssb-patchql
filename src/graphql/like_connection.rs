/// The connection to the likes on a message
#[derive(GraphQLObject)]
pub struct LikeConnection {
    /// The number of likes
    pub count: i32,
}
