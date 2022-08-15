use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Extension, Router, Server};
use sea_orm::DatabaseConnection;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::configuration::Settings;
use crate::graphql::{create_schema, AppSchema};

pub struct Context {
    pub configuration: Settings,
    pub db_pool: Arc<DatabaseConnection>,
}

impl Context {
    pub async fn init(configuration: Settings) -> anyhow::Result<Self> {
        let db_pool =
            Arc::new(sea_orm::Database::connect(configuration.database.connection_string()).await?);

        Ok(Self {
            configuration,
            db_pool,
        })
    }
}

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

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}
