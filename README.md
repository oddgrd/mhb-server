This (WIP) project is a refactor of my previous [MyHomeBoard](https://github.com/oddgrd/myhomeboard) backend which was written in Typescript. The design of the Rust refactor is heavily inspired by [Brian Konkle's](https://twitter.com/bkonkle) great article series on [Async GrapQL with Rust](https://konkle.us/async-graphql-rust-1-introduction/).

## Development

### Run locally:

1. Install [Shuttle](https://www.shuttle.rs/) with cargo: <br>`cargo install cargo-shuttle`
2. Run the server locally with Shuttle (Shuttle will setup a database): <br>`cargo shuttle run`
3. (optional) Connect to `http://127.0.0.1:8000/api/graphql` in your browser to play around with the GraphQL Playground. <br>Tip: copy the mutation/query from `tests/api/boulder.rs`

### Run integration tests:

1. Start a local Docker postgres service: <br>`cargo make docker up -d`
2. Run tests: <br>`cargo test -- --nocapture`
    