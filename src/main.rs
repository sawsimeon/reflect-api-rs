// src/main.rs

use axum::{
    routing::get,
    Json, Router,
};
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;

// Import module routers
mod health;
mod stablecoin;
mod integration;
mod stats;
mod events;

/// Global application state shared across routes.
///
/// Add database pools, API clients, configuration, etc. here.
#[derive(Clone)]
pub struct AppState {}

#[tokio::main]
async fn main() {
    // Initialize tracing/logging
    tracing_subscriber::fmt::init();

    let state = AppState {};

    // Build the main router
    let app = Router::new()
        // Grouped route namespaces
        .nest("/health", health::router())
        .nest("/stablecoins", stablecoin::router())
        .nest("/integrations", integration::router())
        .nest("/stats", stats::router())
        .nest("/events", events::router())

        // Root endpoint
        .route(
            "/",
            get(|| async {
                Json(json!({
                    "status": "reflect api running"
                }))
            }),
        )

        // Attach shared state
        .with_state(state);

    // Bind to 0.0.0.0:3000
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("Reflect API listening on http://{}", addr);

    // Axum 0.7+ server
    axum::serve(listener, app)
        .await
        .expect("Server error");
}
