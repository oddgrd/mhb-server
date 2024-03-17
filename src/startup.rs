use axum::routing::{get, post};
use axum::Router;
use axum_login::tower_sessions::cookie::SameSite;
use axum_login::tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use axum_login::AuthManagerLayerBuilder;
use oauth2::{basic::BasicClient, AuthUrl, TokenUrl};
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use time::Duration;
use tokio::net::TcpListener;

use crate::config::AppConfig;
use crate::dal::{Dal, Postgres};
use crate::handlers::boulder::{create_boulder, get_boulder};
use crate::handlers::user::{get_login, google_oauth_callback, logout, post_login};
use crate::services::user::UserService;

pub struct Application(pub Router);

#[derive(Clone)]
pub struct AppState {
    pub dal: Arc<Box<dyn Dal>>,
    pub user_service: UserService,
}

impl Application {
    pub fn build(
        pool: PgPool,
        AppConfig {
            client_id,
            client_secret,
            google_callback_url,
        }: AppConfig,
    ) -> anyhow::Result<Application> {
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;
        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(google_callback_url);

        let dal: Arc<Box<dyn Dal>> = Arc::new(Box::new(Postgres::new(pool)));
        let user_service = UserService::new(dal.clone(), client);

        let state = AppState {
            dal,
            user_service: user_service.clone(),
        };

        let session_store = MemoryStore::default();

        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_same_site(SameSite::Lax) // Ensure we send the cookie from the OAuth redirect.
            .with_expiry(Expiry::OnInactivity(Duration::days(7)));

        let auth_layer = AuthManagerLayerBuilder::new(user_service, session_layer).build();

        let app = Router::new()
            .route("/boulders", post(create_boulder))
            .route("/boulders/:boulder_id", get(get_boulder))
            .route("/login", get(get_login).post(post_login))
            .route("/logout", get(logout))
            // If the callback route is changed, the "Authorized redirect URIs" in the Google Cloud
            // console has to be updated as well.
            .route("/oauth/google/callback", get(google_oauth_callback))
            .layer(auth_layer)
            .route("/health_check", get(|| async {}))
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
