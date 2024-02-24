use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    dal::Dal,
    error::Error,
    models::{Boulder, NewBoulder},
    startup::AppState,
};

/// Immediately returns a `200 OK`
pub async fn health_check() {}

pub async fn create_boulder<D: Dal>(
    State(state): State<AppState<D>>,
    Json(boulder): Json<NewBoulder>,
) -> Result<Json<Boulder>, Error> {
    let boulder = state.dal.create_boulder(boulder).await?;

    Ok(Json(boulder))
}

pub async fn get_boulder<D: Dal>(
    State(state): State<AppState<D>>,
    Path(boulder_id): Path<String>,
) -> Result<Json<Boulder>, Error> {
    let boulder = state.dal.get_boulder(boulder_id).await?;

    Ok(Json(boulder))
}
