use axum::routing::get;
use axum::{Router, Server};
use sqlx::PgPool;
use std::net::SocketAddr;

pub struct Application(pub Router);

impl Application {
    pub async fn build(_pool: PgPool) -> anyhow::Result<Application> {
        let app = Router::new()
            .route("/health_check", get(health_check));

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

/// Immediately returns a `200 OK`
async fn health_check() {}
