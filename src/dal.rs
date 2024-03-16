use core::fmt;

use axum::async_trait;
use sqlx::PgPool;

use crate::models::boulder::{Boulder, NewBoulder};
use crate::models::user::User;

#[async_trait]
pub trait Dal: Send + Sync {
    /// Insert a new user.
    async fn insert_user(
        &self,
        username: &str,
        avatar_url: &str,
        google_id: &str,
        locale: &str,
        access_token: &str,
    ) -> Result<User, DalError>;
    /// Fetch a user by their id.
    async fn get_user_by_id(&self, id: &str) -> Result<Option<User>, DalError>;
    /// Insert a new boulder.
    async fn insert_boulder(&self, boulder: NewBoulder) -> Result<Boulder, DalError>;
    /// Retrieve a boulder by the boulder ID.
    async fn get_boulder_by_id(&self, boulder_id: String) -> Result<Boulder, DalError>;
}

#[derive(thiserror::Error, Debug)]
pub enum DalError {
    Sqlx(#[from] sqlx::Error),
}

// We are not using the `thiserror`'s `#[error]` syntax to prevent sensitive details from bubbling
// up to the users. Instead we are logging it as an error which we can inspect.
impl fmt::Display for DalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            DalError::Sqlx(err) => {
                tracing::error!(
                    error = err as &(dyn std::error::Error),
                    "database request failed"
                );

                "failed to interact with database"
            }
        };

        write!(f, "{msg}")
    }
}

#[derive(Clone)]
pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Dal for Postgres {
    async fn insert_user(
        &self,
        username: &str,
        avatar_url: &str,
        google_id: &str,
        locale: &str,
        access_token: &str,
    ) -> Result<User, DalError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, avatar_url, google_id, locale, access_token)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT(google_id) DO UPDATE
            SET access_token = excluded.access_token
            RETURNING *;
            "#,
        )
        .bind(username)
        .bind(avatar_url)
        .bind(google_id)
        .bind(locale)
        .bind(access_token)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_user_by_id(&self, id: &str) -> Result<Option<User>, DalError> {
        Ok(sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?)
    }

    async fn insert_boulder(&self, boulder: NewBoulder) -> Result<Boulder, DalError> {
        let boulder = sqlx::query_as::<_, Boulder>(
            r#"
            INSERT INTO boulders(title, suggested_grade, published)
            VALUES($1, $2, $3)
            RETURNING *;
        "#,
        )
        .bind(boulder.title)
        .bind(boulder.suggested_grade)
        .bind(boulder.published)
        .fetch_one(&self.pool)
        .await?;

        Ok(boulder)
    }

    async fn get_boulder_by_id(&self, boulder_id: String) -> Result<Boulder, DalError> {
        let boulder = sqlx::query_as::<_, Boulder>("SELECT * FROM boulders WHERE id = $1")
            .bind(boulder_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(boulder)
    }
}
