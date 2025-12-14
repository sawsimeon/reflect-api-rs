use axum::{response::IntoResponse, Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct RedeemWL { pub amount: f64, pub label: Option<String> }

pub async fn redeem_whitelabeled(State(_state): State<AppState>, Json(payload): Json<RedeemWL>) -> impl IntoResponse {
    Json(json!({"result": "redeemed whitelabeled", "amount": payload.amount, "label": payload.label}))
}
