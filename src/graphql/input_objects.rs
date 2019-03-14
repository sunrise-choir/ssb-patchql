
#[derive(GraphQLEnum)]
/// Retrieve objects that are private, public, or both.
pub enum Privacy {
    /// Only private.
    Private,
    /// Only public.
    Public,
    /// Both public and private.
    All,
}

#[derive(GraphQLEnum)]
/// Retrieve objects ordered by asserted publish time, by received time, or attempt to causally sort
/// by cypher links.
pub enum OrderBy {
    /// Order by asserted timestamp (the time the author claimed they published the message).
    ///
    /// Note that using asserted timestamp is not reliable. If the publisher of a message has their
    /// system clock set incorrectly then this can really break your ui. This has already happened
    /// before on the network. If you're sorting posts in a thread, prefer using causal sort.
    Asserted,

    /// Order by received timestamp (the time that the message was inserted into your db).
    ///
    /// Note that using received timestamp does not work well when the db has downloaded many feeds
    /// all at once (like during onboarding to the network) because feeds are inserted into your db
    /// in a random order.
    Received,

    /// Order by causal timestamp.
    ///
    /// Use this for sorting posts in a thread. Don't use this for sorting all threads in the
    /// database, it's not supported.
    Causal,
}


