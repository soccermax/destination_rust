use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde_json::json;

#[derive(Debug)]
pub enum DestinationError {
    AlreadyExists { name: String }
}

impl IntoResponse for DestinationError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DestinationError::AlreadyExists{name} => {
                (StatusCode::CONFLICT, format!("The destination with the name: '{}' already exists", name))
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