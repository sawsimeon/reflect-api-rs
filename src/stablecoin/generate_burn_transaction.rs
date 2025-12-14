use axum::{response::IntoResponse, Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct BurnTxRequest {
    pub stablecoin: String,
    pub amount: f64,
    pub holder: String,
}

pub async fn generate_burn_transaction(State(_state): State<AppState>, Json(payload): Json<BurnTxRequest>) -> impl IntoResponse {
    Json(json!({
        "tx": "0xbadcafe",
        "stablecoin": payload.stablecoin,
        "amount": payload.amount,
        "holder": payload.holder
    }))
}
