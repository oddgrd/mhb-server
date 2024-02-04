use axum::routing::get;
use axum::Router;
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub struct Application(pub Router);

impl Application {
    pub async fn build(_pool: PgPool) -> anyhow::Result<Application> {
        let app = Router::new().route("/health_check", get(health_check));

        Ok(Self(app))
    }

    /// Utility function to run the application in tests, in production Shuttle
    /// will assign an address and run it
    pub async fn run_until_stopped(self, addr: SocketAddr) {
        let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, self.0).await.unwrap();
    }
}

/// Immediately returns a `200 OK`
async fn health_check() {}
