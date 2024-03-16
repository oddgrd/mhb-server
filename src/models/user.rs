use axum_login::AuthUser;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// User database model
#[derive(Clone, Eq, FromRow, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    /// A URL to the user's profile picture/avatar.
    pub avatar_url: String,
    /// A unique Google user ID.
    pub google_id: String,
    /// The locale of the user, which specifies the language and regional settings (like date,
    /// time, and number formats).
    pub locale: String,
    /// The access token is a credential that allows us to request a user's data from an oauth
    /// provider on behalf of the user. It is specific to the user and the permissions they've
    /// granted in the oauth consent screen.
    pub access_token: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

// Here we've implemented `Debug` manually to avoid accidentally logging the
// access token.
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("avatar_url", &self.avatar_url)
            .field("google_id", &self.google_id)
            .field("locale", &self.locale)
            .field("access_token", &"[redacted]")
            .field("updated_at", &self.updated_at)
            .field("created_at", &self.created_at)
            .finish()
    }
}

impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.access_token.as_bytes()
    }
}
