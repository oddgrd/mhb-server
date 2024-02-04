## Development

### Run locally:

1. Install [Shuttle](https://www.shuttle.rs/) with cargo: <br>`cargo install cargo-shuttle`
2. Run the server locally with Shuttle (Shuttle will setup a database): <br>`cargo shuttle run`

### Run integration tests:

1. Start a local Docker postgres service: <br>`cargo make docker up -d`
2. Run tests: <br>`cargo test -- --nocapture`
    
