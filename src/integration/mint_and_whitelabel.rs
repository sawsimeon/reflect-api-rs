use axum::{response::IntoResponse, Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct MintWL { pub amount: f64, pub label: Option<String> }

pub async fn mint_and_whitelabel(State(_state): State<AppState>, Json(payload): Json<MintWL>) -> impl IntoResponse {
    Json(json!({"result": "minted and whitelabeled", "amount": payload.amount, "label": payload.label}))
}
