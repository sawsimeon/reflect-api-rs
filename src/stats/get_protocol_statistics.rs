use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn get_protocol_statistics() -> impl IntoResponse {
    Json(json!({"total_minted": 50000, "total_redeemed": 10000}))
}
