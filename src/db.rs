pub struct Context {
    // Use your real database pool here.
    //pool: DatabasePool,
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}


