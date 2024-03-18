use askama::Template;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::Form;
use axum_login::tower_sessions::Session;
use axum_login::AuthSession;
use oauth2::CsrfToken;
use serde::Deserialize;

use crate::services::user::{Credentials, UserService};

#[derive(Debug, Clone, Deserialize)]
pub struct AuthzResp {
    code: String,
    state: CsrfToken,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub message: Option<String>,
    pub next: Option<String>,
}

pub const NEXT_URL_KEY: &str = "auth.next-url";
pub const CSRF_STATE_KEY: &str = "oauth.csrf-state";

pub async fn google_oauth_callback(
    mut auth_session: AuthSession<UserService>,
    session: Session,
    Query(AuthzResp {
        code,
        state: new_state,
    }): Query<AuthzResp>,
) -> impl IntoResponse {
    // Ensure the callback is coming from an auth flow initiated by us.
    let Ok(Some(old_state)) = session.get(CSRF_STATE_KEY).await else {
        tracing::error!("failed to get old state");
        return StatusCode::BAD_REQUEST.into_response();
    };

    let creds = Credentials {
        code,
        old_state,
        new_state,
    };

    // Ensure CSRF state has not been tampered with, send token request to Google, then send
    // user info request to google and insert the data in the response.
    let user = match auth_session.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                LoginTemplate {
                    message: Some("Invalid CSRF state.".to_string()),
                    next: None,
                },
            )
                .into_response()
        }
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if let Ok(Some(next)) = session.remove::<String>(NEXT_URL_KEY).await {
        Redirect::to(&next).into_response()
    } else {
        Redirect::to("/").into_response()
    }
}

// This allows us to extract the "next" field from the query string. We use this
// to redirect after log in.
#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

pub async fn post_login(
    auth_session: AuthSession<UserService>,
    session: Session,
    Form(NextUrl { next }): Form<NextUrl>,
) -> impl IntoResponse {
    let (auth_url, csrf_state) = auth_session.backend.authorize_url();

    session
        .insert(CSRF_STATE_KEY, csrf_state.secret())
        .await
        .expect("Serialization should not fail.");

    session
        .insert(NEXT_URL_KEY, next)
        .await
        .expect("Serialization should not fail.");

    Redirect::to(auth_url.as_str()).into_response()
}

pub async fn get_login(Query(NextUrl { next }): Query<NextUrl>) -> LoginTemplate {
    LoginTemplate {
        message: None,
        next,
    }
}

pub async fn logout(mut auth_session: AuthSession<UserService>) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => Redirect::to("/login").into_response(),
        Err(err) => {
            tracing::error!(
                err = &err as &(dyn std::error::Error),
                "failed to logout user"
            );

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
