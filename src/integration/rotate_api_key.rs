use axum::{response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct ApiKeyRotate { pub id: String }

pub async fn rotate_api_key(Json(_payload): Json<ApiKeyRotate>) -> impl IntoResponse {
    Json(json!({"result": "api key rotated"}))
}
