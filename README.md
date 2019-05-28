[![Build Status](https://travis-ci.org/sunrise-choir/ssb-patchql.svg?branch=master)](https://travis-ci.org/sunrise-choir/ssb-patchql)
[![Build status](https://ci.appveyor.com/api/projects/status/w1c7n87463g99vls/branch/master?svg=true)](https://ci.appveyor.com/project/pietgeursen/ssb-patchql/branch/master)
# ssb-patchql

> (:construction: work in progress :construction: ) An example graphql api suitable for making ssb apps similar to patchwork or patchbay

## Heads Up:

- this is still a wip, we're still learning and working out the best shape for this api to take. The API **will change**.

## Intention

- I'd like to see 150-2000 new ssb apps by 2025. Doing this by:
  - lowering the technical barrier to entry by using industry standard technologies.
  - making well documented, easy to understand code, that's tested and reliable

- there's a high barrier to entry in the js ssb stack, even for just building a front-end client. Much of the tech stack is "Mad Science" / experimental.
  - you need to learn pull-streams, flume-db, ssb-msg-schema, ssb-server api, ...
  - drawbacks of the high barrier:
    - only a few people have the time to invest in learning all the tech in the stack
    - those few people become accidental maintainers / wield power in the community because they shape the apps that people use / single points of failure / bottlenecks for progress. (Not meant to malign any people who have done great work eg, Matt, Mix, Dominic)

## Tech Decisions

### Why the `process` mutation?

This db will lag behind the offset log and needs calls to `process` to bring the db up to date. At first this might seem annoying and that the db should do this automatically. But this is a conscious design decision to give the app control of when cpu is used. This is important on resource constrained devices, or even just when starting up the app. This is a major pain point in the javascript flume-db implementation that we're learning from.

### Why sql?

- SSB data is highly relational. It suits a relational db very well.
- Each person has a db that only contains data from their own social network (and not all the data of the entire network like in a centralised system) we don't have to be able to scale to millions or billions of users.

## Graphql Schema

[graphql schema](/schema.graphql) lives here.

Example Queries:

```graphql

```

## Environment Variables

Note, there's an `.env_example` file in the root of the repo you can use as a starting point. Copy it to a file called `.env`.

### `DATABASE_URL` (required)

This is poorly named and will be changed to DATABASE_PATH.
The absolute or relative path to the sqlite database. If it doesn't exist it will be created.

### `OFFSET_LOG_PATH` (required)

The absolute or relative path to the offset log. Typically lives in `~/.ssb/flume/log.offset`

### `SSB_PUB_KEY` (required)

The `id` field from `~/.ssb/secret` including the '.ed25519' suffix

### `SSB_SECRET_KEY`

Not strictly required, it will run without a secret key. But it can't decrypt private messages.
The `private` field from `~/.ssb/secret` including the '.ed25519' suffix

### `LISTEN`

The host and port to bind to. eg:

- `LISTEN=localhost:8080` (default)
- `LISTEN=localhost:9967` (use some other port)
- `LISTEN=0.0.0.0:8080` (expose to more than just localhost. **Careful**: People could read your private messages.)

### `RUST_LOG`

logging level. eg:

- `RUST_LOG=info` (will log server response times, useful for checking performance)

## Database schema

![schema](/docs/images/ssb-patchql.jpg)

## Todos

## Development

### Install the graphql-cli

With node / npm installed:

`$ npm install -g graphql-cli`

### Generate a new `schema.graphql`

Copy `.env_example` to a file called `.env` and edit parameters appropriately for your environment.

Start the server using `$ cargo run`

and in another terminal:

`$ graphql get-schema`

### Lint the generated schema for errors

`$ graphql lint` (and press enter to accept the default option)

