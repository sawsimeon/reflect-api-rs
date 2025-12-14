use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn get_recent_events() -> impl IntoResponse {
    Json(json!({"events": [{"id": "evt_1", "type": "mint"}, {"id": "evt_2", "type": "burn"}]}))
}
