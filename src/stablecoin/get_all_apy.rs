use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn get_all_apy() -> impl IntoResponse {
    Json(json!({"apy": {"rUSD": 0.02, "rEUR": 0.015}}))
}
