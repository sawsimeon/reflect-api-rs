use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn get_latest_exchange_rates() -> impl IntoResponse {
    Json(json!({"rates": {"rUSD": 1.0, "rEUR": 0.92}}))
}
