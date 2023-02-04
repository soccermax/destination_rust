use crate::handler::destination;
use axum::{
    routing::{get, post},
    Router,
};

pub fn initialize() -> Router {
    // build our application with a route
    Router::new()
        .route("/", get(destination::root))
        .route("/users", post(destination::create_destination))
}
