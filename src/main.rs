use axum::{Router, routing::get, Json};
use hyper::Server;
use serde_json::json;
use std::net::SocketAddr;
use tracing_subscriber;

mod health;
mod stablecoin;
mod integration;
mod stats;
mod events;

#[derive(Clone)]
pub struct AppState {
    // Add DB pools or clients here later
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState {};

    let app = Router::new()
        .nest("/health", health::router())
        .nest("/stablecoins", stablecoin::router())
        .nest("/integrations", integration::router())
        .nest("/stats", stats::router())
        .nest("/events", events::router())
        // basic root route
        .route("/", get(|| async { Json(json!({"status": "reflect api running"})) }))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
