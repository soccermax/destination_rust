use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde_json::json;
use crate::db::error::DbError;

#[derive(Debug)]
pub enum ApirError {
    AlreadyExists { name: String },
    InternalServerError,
}

impl From<DbError> for ApirError {
    fn from(value: DbError) -> Self {
        match value {
            DbError::NotReachable {} => ApirError::InternalServerError,
            DbError::AlreadyExists {name } => ApirError::AlreadyExists {name }
        }
    }
}

impl IntoResponse for ApirError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApirError::AlreadyExists{name} => {
                (StatusCode::CONFLICT, format!("The destination with the name: '{}' already exists", name))
            }
            ApirError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, String::from("Internal Server Error"))
            }
            // AppError::UserRepo(UserRepoError::InvalidUsername) => {
            //     (StatusCode::UNPROCESSABLE_ENTITY, "Invalid username")
            // }
        };
        let body = Json(json!({
            "errorMessage": error_message,
        }));
        (status, body).into_response()
    }
}






// use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, Json};
// use serde_json::{json};
// use thiserror::Error;
//
//
// // We derive `thiserror::Error`
// #[derive(Debug, Error)]
// pub enum ApiError {
//     // The `#[from]` attribute generates `From<JsonRejection> for ApiError`
//     // implementation. See `thiserror` docs for more information
//     #[error(transparent)]
//     JsonExtractorRejection(#[from] JsonRejection),
// }
// // We implement `IntoResponse` so ApiError can be used as a response
// impl IntoResponse for ApiError {
//     fn into_response(self) -> axum::response::Response {
//         let payload = json!({
//             "message": self.to_string(),
//             "origin": "with_rejection"
//         });
//         let code = match self {
//             ApiError::JsonExtractorRejection(x) => match x {
//                 JsonRejection::JsonDataError(_) => StatusCode::UNPROCESSABLE_ENTITY,
//                 JsonRejection::JsonSyntaxError(_) => StatusCode::BAD_REQUEST,
//                 JsonRejection::MissingJsonContentType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
//                 _ => StatusCode::INTERNAL_SERVER_ERROR,
//             },
//         };
//         (code, Json(payload)).into_response()
//     }
// }