use crate::schema::AppSchema;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use boulder::boulder_service::{BoulderService, BoulderServiceTrait};
use sea_orm::{DatabaseConnection, SqlxPostgresConnector};
use sqlx::PgPool;
use std::sync::Arc;

pub struct Context {
    pub boulders: Arc<dyn BoulderServiceTrait>,
    pub db_conn: Arc<DatabaseConnection>,
}

impl Context {
    pub async fn init(pool: PgPool) -> anyhow::Result<Context> {
        let db_conn = Arc::new(SqlxPostgresConnector::from_sqlx_postgres_pool(pool));
        Ok(Self {
            boulders: Arc::new(BoulderService::new(&db_conn)),
            db_conn,
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
