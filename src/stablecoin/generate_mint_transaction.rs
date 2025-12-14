use axum::{response::IntoResponse, Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct MintTxRequest {
    pub stablecoin: String,
    pub amount: f64,
    pub recipient: String,
}

pub async fn generate_mint_transaction(State(_state): State<AppState>, Json(payload): Json<MintTxRequest>) -> impl IntoResponse {
    Json(json!({
        "tx": "0xdeadbeef",
        "stablecoin": payload.stablecoin,
        "amount": payload.amount,
        "recipient": payload.recipient
    }))
}
