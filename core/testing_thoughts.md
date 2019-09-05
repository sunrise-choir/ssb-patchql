# Where to from here?

Not sure what kind of testing here is the best idea.

I could do "model" tests, in the rails sense of model testing. In that sense the graphql part becomes more like a "controller". This has been something I was unhappy about in the code for a while.

It'd be good to do a handful of e2e tests.

There's also how to seperate out the flume part from the rest of the system.
Ideally the lib can expose an `add_message` method.

Interested to see if we still need the mutex for doing RO queries. This might be limited by the borrow checker at the moment. I might have to use r2d2.

Do a spike then. Things to try:
  - [ ] try out using an r2d2 connection pool for queries.
  - how hard is it to factor stuff out of the graphql folder into the models folder?
    - this might need more of split where it's like gql -> model -> orm
  - how hard is it make factories (is there prior art here with Juniper?)
