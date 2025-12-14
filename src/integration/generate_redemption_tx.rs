use axum::{response::IntoResponse, Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct RedeemReq { pub amount: f64, pub holder: String }

pub async fn generate_redemption_tx(State(_state): State<AppState>, Json(payload): Json<RedeemReq>) -> impl IntoResponse {
    Json(json!({"tx": "0xintredeem", "amount": payload.amount, "holder": payload.holder}))
}
