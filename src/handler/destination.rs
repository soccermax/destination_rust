use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json
};

use crate::model::destination::{Destination};
use crate::db::{destination};
use crate::handler::error;


// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_destination(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<Destination>,
// ) -> impl IntoResponse {
) -> Result<impl IntoResponse, error::DestinationError> {
    let destination = Destination {
        id: None,
         name: payload.name,
         protocol: payload.protocol,
         port: payload.port,
         url: payload.url,
         authentication: payload.authentication
    };

    let destination_name = destination.name.to_string();
    let create_des_result = destination::create_destination(destination);

    match create_des_result {
        Ok(new_destination) => Ok((StatusCode::CREATED, Json(new_destination))),
        Err(_err) => Err(error::DestinationError::AlreadyExists {
            name: destination_name
        })
    }

    // Ok((StatusCode::CREATED, Json()))
    // match destination::create_destination(destination) {
    //     Ok(destination) => Ok((StatusCode::CREATED, Json(destination))),
    //     Err(err) => Err(err)
    // }
    // return match destination::create_destination(destination) {
    //     Ok(destination) =>  Ok((StatusCode::CREATED, Json(destination))),
    //     Err(err) => Err(err)
    // }
}
    // match destination::create_destination(destination) {
    //     Ok(new_destination) =>  Ok((StatusCode::CREATED, Json(new_destination))),
    //     Err(err) => {
    //         println!("insert of new destination went wrong: {:?}", err);
    //         Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(Destination {
    //             id: None,
    //             authentication: Some(Authentication::BasicAuth),
    //             name: String::from("test"),
    //             protocol: Protocol::HTTP,
    //             port: 8080,
    //             url: String::from("test"),
    //         })))
    //     }
    // }
