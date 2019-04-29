# ssb-patchql

> (:construction: work in progress :construction: ) An example graphql api suitable for making ssb apps similar to patchwork or patchbay

## Caveats

- this is still a wip, we're still learning and working out the best shape for this api to take.

## Intention

- I'd like to see 15-20 new ssb apps by 2025. Doing this by:
  - lowering the technical barrier to entry by using industry standard technologies.
  - making well documented, easy to understand code, that's tested and reliable

- there's a high barrier to entry in the js ssb stack, even for just building a front-end client. Much of the tech stack is "Mad Science" / experimental.
  - you need to learn pull-streams, flume-db, ssb-msg-schema, ssb-server api, ...
  - drawbacks of the high barrier:
    - only a few people have the time to invest in learning all the tech in the stack
    - those few people become accidental maintainers / wield power in the community because they shape the apps that people use / single points of failure / bottlenecks for progress. (Not meant to malign any people who have done great work eg, Matt, Mix, Dominic)

## Tech Decisions

### Why the `process` mutation?

### Why sql?


## [graphql schema](/schema.graphql)

Example Queries:

```graphql

```

## Database schema

## Todos

## Development

- rustup
- `process` mutation
