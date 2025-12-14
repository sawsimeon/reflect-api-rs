use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn get_current_exchange_rate() -> impl IntoResponse {
    Json(json!({"rate": 1.0}))
}
