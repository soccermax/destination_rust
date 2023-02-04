use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::CONFLICT,
            "the destination already exists",
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
    where
        E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}



// #[derive(Error, Debug)]
// pub enum DestinationError {
//     /// Represents an empty source. For example, an empty text file being given
//     /// as input to `count_words()`.
//     #[error("the destination already exists")]
//     AlreadyExists {},
//
//     /// Represents a failure to read from input.
//     #[error("Read error")]
//     ReadError { source: std::io::Error },
//
//     /// Represents all other cases of `std::io::Error`.
//     #[error(transparent)]
//     IOError(#[from] std::io::Error),
// }
//
// impl DestinationError {
//     const STATUS_CODE: StatusCode = StatusCode::CONFLICT;
// }
//
// impl IntoResponse for DestinationError {
//     fn into_response(self) -> Response {
//         (
//             StatusCode::CONFLICT,
//             format!("The destination already exists"),
//         )
//             .into_response()
//     }
// }


// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, DestinationAlreadyExists>`. That way you don't need to do that manually.

//
// impl<E> From<E> for AppError
//     where
//         E: Into<anyhow::Error>,
// {
//     fn from(err: E) -> Self {
//         Self(err.into())
//     }
// }


// impl DestinationError {
//     pub fn get_status_code(&self) -> StatusCode {
//         match  { self.na }
//     }
// }



// pub struct DestinationAlreadyExists(anyhow::Error);
//
// impl IntoResponse for DestinationAlreadyExists {
//     fn into_response(self) -> Response {
//         (
//             StatusCode::CONFLICT,
//             format!("The destination already exists"),
//         )
//             .into_response()
//     }
// }
//
// // This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// // `Result<_, DestinationAlreadyExists>`. That way you don't need to do that manually.
// impl<E> From<E> for DestinationAlreadyExists
//     where
//         E: Into<anyhow::Error>,
// {
//     fn from(err: E) -> Self {
//         Self(err.into())
//     }
// }