use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn get_historical_integration_stats() -> impl IntoResponse {
    Json(json!({"historical": [{"timestamp": 1700000000, "minted": 1000}]}))
}
