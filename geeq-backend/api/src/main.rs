use axum::http::{HeaderName, HeaderValue, Method};
use tower_http::cors::CorsLayer;

mod controller;

#[tokio::main]
async fn main() {
    let api = controller::Api {};
    let layer = CorsLayer::new()
        .allow_origin([
            "https://geeq.ningenme.net".parse::<HeaderValue>().unwrap(),
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([HeaderName::from_static("content-type")])
        .allow_credentials(true);

    let router = generated::server::new(api).layer(layer);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:50051").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
