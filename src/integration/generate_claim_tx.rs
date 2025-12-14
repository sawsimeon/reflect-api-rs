use axum::{response::IntoResponse, Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct ClaimReq { pub claimant: String }

pub async fn generate_claim_tx(State(_state): State<AppState>, Json(payload): Json<ClaimReq>) -> impl IntoResponse {
    Json(json!({"tx": "0xclaimtx", "claimant": payload.claimant}))
}
