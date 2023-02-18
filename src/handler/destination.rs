use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use super::error;
use crate::db::destination;
use crate::model::Destination;

pub async fn get_all() -> Result<impl IntoResponse, error::ApirError> {
    let destinations = destination::get_all()?;
    Ok((StatusCode::OK, Json(destinations)))
}

pub async fn create(
    Json(payload): Json<Destination>,
) -> Result<impl IntoResponse, error::ApirError> {
    let new_destination = destination::create(payload)?;
    Ok((StatusCode::CREATED, Json(new_destination)))
}

pub async fn get(
    State(connection_manager): State<redis::aio::ConnectionManager>,
    Path(destination_name): Path<String>,
) -> Result<impl IntoResponse, error::ApirError> {
    let destination = destination::get(connection_manager, destination_name).await?;
    Ok((StatusCode::CREATED, Json(destination)))
}

pub async fn get_v2(
    State(connection_manager): State<redis::aio::ConnectionManager>,
    Path(destination_name): Path<String>,
) -> Result<impl IntoResponse, error::ApirError> {
    let destination = destination::get(connection_manager, destination_name).await?;
    Ok((StatusCode::OK, Json(destination)))
}

pub async fn delete(
    Path(destination_name): Path<String>,
) -> Result<impl IntoResponse, error::ApirError> {
    destination::delete(destination_name)?;
    Ok(StatusCode::NO_CONTENT)
}
