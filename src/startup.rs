use axum::routing::get;
use axum::{Extension, Router, Server};
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::graphql::{graphql_handler, graphql_playground, Context};
use crate::schema::create_schema;

pub struct Application(pub Router);

impl Application {
    pub async fn build(pool: PgPool) -> anyhow::Result<Application> {
        // Initialize the GraphQL context with the postgres pool provided by Shuttle
        // in production or from a local Docker container in testing
        let graphql_context = Arc::new(Context::init(pool).await?);

        let graphql_schema = create_schema(graphql_context)?;

        let app = Router::new()
            .route(
                "/api/graphql",
                get(graphql_playground).post(graphql_handler),
            )
            .layer(Extension(graphql_schema));

        Ok(Self(app))
    }

    /// Utility function to run the application in tests, in production Shuttle
    /// will assign an address and run it
    pub async fn run_until_stopped(self, addr: SocketAddr) {
        Server::bind(&addr)
            .serve(self.0.into_make_service())
            .await
            .unwrap()
    }
}
