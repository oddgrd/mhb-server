use axum::routing::get;
use axum::{Extension, Router, Server};
use std::net::SocketAddr;
use std::sync::Arc;

use crate::configuration::Settings;
use crate::graphql::{graphql_handler, graphql_playground, Context};
use crate::schema::create_schema;

pub struct Application {
    address: SocketAddr,
    app: Router,
}

impl Application {
    pub async fn build(configuration: Settings) -> anyhow::Result<Self> {
        let address: SocketAddr = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        )
        .parse()
        .expect("valid address");

        // TODO: make configuration static?
        let graphql_context = Arc::new(Context::init(configuration).await?);
        let graphql_schema = create_schema(graphql_context)?;
        let app = Router::new()
            .route(
                "/api/graphql",
                get(graphql_playground).post(graphql_handler),
            )
            .layer(Extension(graphql_schema));

        Ok(Self { address, app })
    }

    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(self) {
        Server::bind(&self.address)
            .serve(self.app.into_make_service())
            .await
            .unwrap()
    }
}
