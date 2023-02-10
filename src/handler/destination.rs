use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

use crate::db::destination;
use crate::handler::error;
use crate::model::destination::Destination;

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
    Path(destination_name): Path<String>,
) -> Result<impl IntoResponse, error::ApirError> {
    let destination = destination::get(destination_name)?;
    Ok((StatusCode::CREATED, Json(destination)))
}

pub async fn delete(
    Path(destination_name): Path<String>,
) -> Result<impl IntoResponse, error::ApirError> {
    destination::delete(destination_name)?;
    Ok(StatusCode::NO_CONTENT)
}
