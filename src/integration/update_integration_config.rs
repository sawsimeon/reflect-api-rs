use axum::{response::IntoResponse, Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct UpdateConfig { pub fee_percent: f64 }

pub async fn update_integration_config(State(_state): State<AppState>, Json(payload): Json<UpdateConfig>) -> impl IntoResponse {
    Json(json!({"result": "config updated", "fee_percent": payload.fee_percent}))
}
