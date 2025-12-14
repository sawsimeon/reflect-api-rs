use axum::{response::IntoResponse, Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct FlowInit { pub flow_name: String }

pub async fn initialize_flow(State(_state): State<AppState>, Json(payload): Json<FlowInit>) -> impl IntoResponse {
    Json(json!({"result": "flow initialized", "flow": payload.flow_name}))
}
