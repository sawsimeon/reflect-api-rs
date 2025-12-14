use axum::{response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct Metadata { pub url: String }

pub async fn upload_integration_metadata(Json(_payload): Json<Metadata>) -> impl IntoResponse {
    Json(json!({"result": "metadata uploaded"}))
}
