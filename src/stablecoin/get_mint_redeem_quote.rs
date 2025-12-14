use axum::{response::IntoResponse, Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct QuoteRequest {
    pub stablecoin: String,
    pub amount: f64,
    pub side: String, // "mint" or "redeem"
}

pub async fn get_mint_redeem_quote(State(_state): State<AppState>, Json(payload): Json<QuoteRequest>) -> impl IntoResponse {
    // return a mock quote
    Json(json!({
        "stablecoin": payload.stablecoin,
        "amount": payload.amount,
        "side": payload.side,
        "quote": {"fee": 0.5, "total": payload.amount + 0.5}
    }))
}
