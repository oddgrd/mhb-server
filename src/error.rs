use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

use crate::dal::DalError;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("failed to interact with database: {0}")]
    Dal(#[from] DalError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code = match self {
            Error::Dal(DalError::Sqlx(sqlx::Error::RowNotFound)) => StatusCode::NOT_FOUND,
            Error::Dal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Internal(ref err) => {
                tracing::error!(
                    error = err.as_ref() as &(dyn std::error::Error),
                    "handler error"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (status_code, Json(self.to_string())).into_response()
    }
}
