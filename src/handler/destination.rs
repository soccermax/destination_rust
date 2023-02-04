use super::error;
use crate::db::destination;
use crate::model::Destination;
use axum::{http::StatusCode, response::IntoResponse, Json};

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_destination(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<Destination>,
    // ) -> impl IntoResponse {
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
