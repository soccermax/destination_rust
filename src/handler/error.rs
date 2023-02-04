use crate::db::error::DbError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum ApirError {
    AlreadyExists { name: String },
    InternalServerError,
    NotFound,
}

impl From<DbError> for ApirError {
    fn from(value: DbError) -> Self {
        match value {
            DbError::NotReachable {} => ApirError::InternalServerError,
            DbError::AlreadyExists { name } => ApirError::AlreadyExists { name },
            DbError::NotFound => ApirError::NotFound,
        }
    }
}

impl IntoResponse for ApirError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApirError::AlreadyExists { name } => (
                StatusCode::CONFLICT,
                format!("The destination with the name: '{}' already exists", name),
            ),
            ApirError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal Server Error"),
            ),
            ApirError::NotFound => (StatusCode::NOT_FOUND, String::from("Not found")),
        };
        let body = Json(json!({
            "errorMessage": error_message,
        }));
        (status, body).into_response()
    }
}
