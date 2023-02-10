use crate::handler::destination;
use axum::{
    routing::{delete, get, post},
    Router,
};

pub fn initialize() -> Router {
    // build our application with a route
    Router::new()
        .route("/destination", get(destination::get_all))
        .route("/destination", post(destination::create))
        .route("/destination/:destination_name", get(destination::get))
        .route(
            "/destination/:destination_name",
            delete(destination::delete),
        )
}
