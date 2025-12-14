use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn get_integration_events() -> impl IntoResponse {
    Json(json!({"events": [{"id": "evt_1", "type": "mint"}]}))
}
