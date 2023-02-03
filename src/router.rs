use axum::{
    routing::{get, post},
    Router,
};
use crate::handler::destination;

pub fn initialize() -> Router  {
    // build our application with a route
    let app = Router::new()
        .route("/", get(destination::root))
        .route("/users", post(destination::create_destination));

    return app;
}