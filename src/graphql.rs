use crate::schema::AppSchema;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use boulder::boulder_service::{BoulderService, BoulderServiceTrait};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::configuration::Settings;

pub struct Context {
    pub configuration: Settings,
    pub boulders: Arc<dyn BoulderServiceTrait>,
    pub db_pool: Arc<DatabaseConnection>,
}

impl Context {
    pub async fn init(configuration: Settings) -> anyhow::Result<Self> {
        let db_pool =
            Arc::new(sea_orm::Database::connect(configuration.database.connection_string()).await?);
        Ok(Self {
            configuration,
            boulders: Arc::new(BoulderService::new(&db_pool)),
            db_pool,
        })
    }
}

pub async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}
