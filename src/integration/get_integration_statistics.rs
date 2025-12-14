use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn get_integration_statistics() -> impl IntoResponse {
    Json(json!({"stats": {"minted": 10000, "redeemed": 2000}}))
}
