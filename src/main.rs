use axum;
use std::net::SocketAddr;
mod router;

// pub mod db;
mod db;
mod model;
mod handler;

#[tokio::main]
async fn main() {

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let app = router::initialize();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}