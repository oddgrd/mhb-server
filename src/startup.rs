use axum::routing::{get, post};
use axum::Router;
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::dal::{Dal, Postgres};
use crate::handlers::{create_boulder, get_boulder, health_check};
pub struct Application(pub Router);

#[derive(Clone)]
pub struct AppState<D: Dal> {
    pub dal: D,
}

impl Application {
    pub async fn build(pool: PgPool) -> anyhow::Result<Application> {
        let state = AppState {
            dal: Postgres::new(pool),
        };
        let app = Router::new()
            .route("/health_check", get(health_check))
            .route("/boulders", post(create_boulder))
            .route("/boulders/:boulder_id", get(get_boulder))
            .with_state(state);

        Ok(Self(app))
    }

    /// Utility function to run the application in tests, in production Shuttle
    /// will assign an address and run it
    pub async fn run_until_stopped(self, addr: SocketAddr) {
        let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, self.0).await.unwrap();
    }
}
