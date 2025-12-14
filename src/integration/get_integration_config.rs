use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn get_integration_config() -> impl IntoResponse {
    Json(json!({"config": {"fee_percent": 0.5}}))
}
