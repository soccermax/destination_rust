use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use super::error;
use crate::auth;
use crate::db::destination;
use crate::model::Destination;

pub async fn get_all(
    app_context: State<auth::app_context::AppContext>,
) -> Result<impl IntoResponse, error::ApirError> {
    let destinations = destination::get_all(app_context).await?;
    Ok((StatusCode::OK, Json(destinations)))
}

pub async fn get_all_auth(
    app_context: State<auth::app_context::AppContext>,
    claims: auth::auth::Claims,
) -> Result<impl IntoResponse, error::ApirError> {
    println!("received token: subdomain: {}", claims.ext_attr.zdn);
    let destinations = destination::get_all(app_context).await?;
    Ok((StatusCode::OK, Json(destinations)))
}

pub async fn create(
    Json(payload): Json<Destination>,
) -> Result<impl IntoResponse, error::ApirError> {
    let new_destination = destination::create(payload)?;
    Ok((StatusCode::CREATED, Json(new_destination)))
}

pub async fn get(
    State(state): State<auth::app_context::AppContext>,
    Path(destination_name): Path<String>,
) -> Result<impl IntoResponse, error::ApirError> {
    let destination = destination::get(state.connection_manager, destination_name).await?;
    Ok((StatusCode::CREATED, Json(destination)))
}

pub async fn get_v2(
    State(state): State<auth::app_context::AppContext>,
    Path(destination_name): Path<String>,
) -> Result<impl IntoResponse, error::ApirError> {
    let destination = destination::get(state.connection_manager, destination_name).await?;
    Ok((StatusCode::OK, Json(destination)))
}

pub async fn delete(
    Path(destination_name): Path<String>,
) -> Result<impl IntoResponse, error::ApirError> {
    destination::delete(destination_name)?;
    Ok(StatusCode::NO_CONTENT)
}
