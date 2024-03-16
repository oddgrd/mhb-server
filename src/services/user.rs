use std::sync::Arc;

use axum::{
    async_trait,
    http::header::{AUTHORIZATION, USER_AGENT},
};
use axum_login::{AuthnBackend, UserId};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope,
    TokenResponse,
};
use reqwest::{Client, Url};
use serde::Deserialize;

use crate::dal::Dal;
use crate::error::Error;
use crate::models::user::User;

/// User info struct used to deserialize the fields we care about from the google user info response.
#[derive(Debug, Deserialize)]
struct GoogleUserInfo {
    /// A unique Google user ID.
    id: String,
    /// The user's name.
    name: String,
    /// A URL to the user's profile picture/avatar.
    picture: String,
    /// The locale of the user.
    locale: String,
}

#[derive(Clone)]
pub struct UserService {
    dal: Arc<Box<dyn Dal>>,
    client: BasicClient,
}

impl UserService {
    pub fn new(dal: Arc<Box<dyn Dal>>, client: BasicClient) -> Self {
        Self { dal, client }
    }

    pub fn authorize_url(&self) -> (Url, CsrfToken) {
        self.client
            .authorize_url(CsrfToken::new_random)
            // We want the username as well as the avatar url.
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.profile".to_string(),
            ))
            .url()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub code: String,
    pub old_state: CsrfToken,
    pub new_state: CsrfToken,
}

#[async_trait]
impl AuthnBackend for UserService {
    type User = User;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        // Ensure the CSRF state has not been tampered with.
        if creds.old_state.secret() != creds.new_state.secret() {
            tracing::error!("CSRF state has been tampered with");
            return Ok(None);
        };

        // Process authorization code, expecting a token response back.
        let token_res = self
            .client
            .exchange_code(AuthorizationCode::new(creds.code))
            .request_async(async_http_client)
            .await
            .map_err(anyhow::Error::new)?;

        // Use access token to request user info.
        tracing::info!("calling google");

        // TODO: use reusable client here
        let GoogleUserInfo {
            id,
            name,
            picture,
            locale,
        } = Client::new()
            .get("https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
            // See: https://docs.github.com/en/rest/overview/resources-in-the-rest-api?apiVersion=2022-11-28#user-agent-required
            .header(USER_AGENT.as_str(), "axum-login")
            .header(
                AUTHORIZATION.as_str(),
                format!("Bearer {}", token_res.access_token().secret()),
            )
            .send()
            .await
            .map_err(anyhow::Error::new)?
            .json()
            .await
            .map_err(anyhow::Error::new)?;

        // Persist the user in our database.
        let user = self
            .dal
            .insert_user(
                &name,
                &picture,
                &id,
                &locale,
                token_res.access_token().secret(),
            )
            .await?;

        Ok(Some(user))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.dal.get_user_by_id(user_id).await?)
    }
}
