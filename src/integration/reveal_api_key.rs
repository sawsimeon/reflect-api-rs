use axum::{response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct ApiKeyReveal { pub id: String }

pub async fn reveal_api_key(Json(_payload): Json<ApiKeyReveal>) -> impl IntoResponse {
    Json(json!({"api_key": "REDACTED-KEY"}))
}
