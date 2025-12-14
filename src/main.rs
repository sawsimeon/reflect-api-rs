// src/main.rs

use axum::{
    Router,
    routing::get,
    Json,
};
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;

// Import your module routers
mod health;
mod stablecoin;
mod integration;
mod stats;
mod events;

#[derive(Clone)]
pub struct AppState {
    // Future: add database pools, clients, config, etc. here
}

#[tokio::main]
async fn main() {
    // Initialize tracing/logging
    tracing_subscriber::fmt::init();

    let state = AppState {};

    // Build the main router
    let app = Router::new()
        .nest("/health", health::router())
        .nest("/stablecoins", stablecoin::router())
        .nest("/integrations", integration::router())
        .nest("/stats", stats::router())
        .nest("/events", events::router())
        // Simple root endpoint
        .route("/", get(|| async {
            Json(json!({ "status": "reflect api running" }))
        }))
        .with_state(state);

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Reflect API listening on http://{}", addr);

    // New Axum 0.7+ way to serve
    axum::serve(listener, app).await.unwrap();
}