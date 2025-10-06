# Juniper Relay Helpers example app

This crate is an example app for the juniper_relay_helpers crate, built using Juniper and [Axum](https://github.com/tokio-rs/axum).

The library itself only needs Juniper, you should be able to use it with any other web framework.

## Running the example

Clone the repo.

```sh
cd juniper_relay_helpers_test
cargo run
```

Navigate to [http://localhost:3030/gui](http://localhost:3030/gui) to see the GraphQL playground.

## App structure

`main.rs` contains the main function and stands up the application. it also contains the integration tests.

The `schema` folder contains the GraphQL schema and query resolvers etc.

The `schema/mod.rs` file builds the schema and the query resolvers - you'll want to look at those query resolvers to see
how you'd use this crate. Also check out the domain types (`schema/character.rs` and `schema/location.rs`).

The app doesn't use any database and instead just works on in-memory vectors, but it should give you an idea of how to use
things in a real application.

## Running the integration tests

From the root of the repo, run:

```
make test-integration
```
