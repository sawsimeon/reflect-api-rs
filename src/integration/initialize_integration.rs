use axum::{response::IntoResponse, Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct InitRequest { pub name: String }

pub async fn initialize_integration(State(_state): State<AppState>, Json(payload): Json<InitRequest>) -> impl IntoResponse {
    Json(json!({"result": "integration initialized", "name": payload.name}))
}
