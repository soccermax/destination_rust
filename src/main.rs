extern crate core;

use std::net::SocketAddr;

mod db;
mod handler;
mod model;
mod router;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = router::initialize().await;
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::destination::Destination;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        Router,
    };
    use redis::Value;
    use serde_json::json;
    use tower::Service; // for `call`
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn get_all_empty() {
        let mut app = start_and_cleanup().await;
        let req = Request::builder()
            .uri("/destination")
            .body(Body::empty())
            .unwrap();
        let response = app.ready().await.unwrap().call(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let destinations: Vec<Destination> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();
        assert_eq!(destinations.len(), 0)
    }

    #[tokio::test]
    async fn not_found() {
        let mut app = start_and_cleanup().await;

        let req = Request::builder()
            .uri("/destination/does-not-exist")
            .body(Body::empty())
            .unwrap();
        let response = app.ready().await.unwrap().call(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn create_destination() {
        let mut app = start_and_cleanup().await;
        let payload = json!({
            "name": "max99",
            "protocol": "Http",
            "port": 8081,
            "url": "http://google.de",
            "authentication": "BasicAuth"
        });
        let req = Request::builder()
            .method(http::Method::POST)
            .header(http::header::CONTENT_TYPE, String::from("Application/JSON"))
            .uri("/destination")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();
        let response = app.ready().await.unwrap().call(req).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn get_destination() {
        let mut app = start_and_cleanup().await;
        let req = Request::builder()
            .method(http::Method::POST)
            .header(http::header::CONTENT_TYPE, String::from("Application/JSON"))
            .uri("/destination")
            .body(Body::from(get_dummy_create_payload()))
            .unwrap();
        app.ready().await.unwrap().call(req).await.unwrap();

        let req = Request::builder()
            .method(http::Method::GET)
            .uri("/destination")
            .body(Body::empty())
            .unwrap();
        let response = app.ready().await.unwrap().call(req).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let destinations: Vec<Destination> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();
        assert_eq!(destinations.len(), 1);
        assert_eq!(destinations[0].name, String::from("max99"));
    }

    // async fn exec_request(&app: &mut Router, request: Request<Body>) -> Response {
    //     app.ready().await.unwrap().call(request).await.unwrap()
    // }

    async fn start_and_cleanup() -> Router {
        let mut app = router::initialize().await;
        let req = Request::builder()
            .uri("/destination")
            .body(Body::empty())
            .unwrap();
        let response = app.ready().await.unwrap().call(req).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let destinations: Vec<Destination> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        for destination in destinations {
            let req = Request::builder()
                .method(http::Method::DELETE)
                .uri(format!("/destination/{}", destination.name))
                .body(Body::empty())
                .unwrap();
            let response = app.ready().await.unwrap().call(req).await.unwrap();
            assert_eq!(response.status(), StatusCode::NO_CONTENT)
        }
        app
    }

    fn get_dummy_create_payload() -> String {
        let payload = json!({
            "name": "max99",
            "protocol": "Http",
            "port": 8081,
            "url": "http://google.de",
            "authentication": "BasicAuth"
        });
        serde_json::to_string(&payload).unwrap()
    }
}
