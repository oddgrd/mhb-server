use crate::{
    error::Error,
    models::boulder::{Boulder, NewBoulder},
    startup::AppState,
};
use axum::{
    extract::{Path, State},
    Json,
};

pub async fn create_boulder(
    State(state): State<AppState>,
    Json(boulder): Json<NewBoulder>,
) -> Result<Json<Boulder>, Error> {
    let boulder = state.dal.insert_boulder(boulder).await?;

    Ok(Json(boulder))
}

pub async fn get_boulder(
    State(state): State<AppState>,
    Path(boulder_id): Path<String>,
) -> Result<Json<Boulder>, Error> {
    let boulder = state.dal.get_boulder_by_id(boulder_id).await?;

    Ok(Json(boulder))
}
