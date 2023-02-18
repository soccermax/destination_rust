use crate::handler::destination;
use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::db::client;

pub async fn initialize() -> Router {
    let connection_manager = client::create_connection_manager().await.unwrap();
    let mut state = crate::auth::app_context::AppContext {
        connection_manager,
        uaa_public_cert: None,
    };
    state.get_uaa_public_cert().await;

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
        .route("/auth/destination", get(destination::get_all_auth))
        .with_state(state)
}
