use axum::{response::IntoResponse, Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct IntMintReq { pub amount: f64, pub recipient: String }

pub async fn generate_integration_mint_tx(State(_state): State<AppState>, Json(payload): Json<IntMintReq>) -> impl IntoResponse {
    Json(json!({"tx": "0xintmint", "amount": payload.amount, "recipient": payload.recipient}))
}
