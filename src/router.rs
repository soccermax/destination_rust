use crate::handler::destination;
use axum::{
    routing::{get, post},
    Router,
};

pub fn initialize() -> Router {
    // build our application with a route
    Router::new()
        .route("/", get(destination::root))
        .route("/destination", post(destination::create_destination))
        .route("/destination", get(destination::get_destination))
}
