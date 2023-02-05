use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

use crate::db::destination;
use crate::handler::error;
use crate::model::destination::Destination;

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_destination(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<Destination>,
) -> Result<impl IntoResponse, error::ApirError> {
    let destination = Destination {
        id: None,
        name: payload.name,
        protocol: payload.protocol,
        port: payload.port,
        url: payload.url,
        authentication: payload.authentication,
    };

    let create_des_result = destination::create_destination(destination);

    match create_des_result {
        Ok(new_destination) => Ok((StatusCode::CREATED, Json(new_destination))),
        Err(err) => Err(err.into()),
    }
}

pub async fn get_destination(
    Path(destination_name): Path<String>,
) -> Result<impl IntoResponse, error::ApirError> {
    let create_des_result = destination::get_destination(destination_name);

    match create_des_result {
        Ok(destination) => Ok((StatusCode::OK, Json(destination))),
        Err(err) => Err(err.into()),
    }
}

pub async fn get_destinations() -> Result<impl IntoResponse, error::ApirError> {
    let create_des_result = destination::get_all();

    match create_des_result {
        Ok(destination) => Ok((StatusCode::OK, Json(destination))),
        Err(err) => Err(err.into()),
    }
}
