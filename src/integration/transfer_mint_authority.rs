use axum::{response::IntoResponse, Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct TransferRequest { pub to: String }

pub async fn transfer_mint_authority(State(_state): State<AppState>, Json(payload): Json<TransferRequest>) -> impl IntoResponse {
    Json(json!({"result": "authority transferred", "to": payload.to}))
}
