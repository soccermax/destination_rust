use crate::handler::destination;
use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::db::client;

pub async fn initialize() -> Router {
    let connection_manager = client::create_connection_manager().await.unwrap();

    // build our application with a route
    Router::new()
        .route("/destination", get(destination::get_all))
        .route("/destination", post(destination::create))
        .route("/destination/:destination_name", get(destination::get))
        .route(
            "/v2/destination/:destination_name",
            get(destination::get_v2),
        )
        .route(
            "/destination/:destination_name",
            delete(destination::delete),
        )
        .with_state(connection_manager)
}
