use core::fmt;

use sqlx::PgPool;

use crate::models::{Boulder, NewBoulder};

#[allow(async_fn_in_trait)]
pub trait Dal {
    /// Insert a new boulder.
    async fn create_boulder(&self, boulder: NewBoulder) -> Result<Boulder, DalError>;
    /// Retrieve a boulder by the boulder ID.
    async fn get_boulder(&self, boulder_id: String) -> Result<Boulder, DalError>;
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
            DalError::Sqlx(error) => {
                tracing::error!(error = error.to_string(), "database request failed");

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

impl Dal for Postgres {
    async fn create_boulder(&self, boulder: NewBoulder) -> Result<Boulder, DalError> {
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

    async fn get_boulder(&self, boulder_id: String) -> Result<Boulder, DalError> {
        let boulder = sqlx::query_as::<_, Boulder>("SELECT * FROM boulders WHERE id = $1")
            .bind(boulder_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(boulder)
    }
}
