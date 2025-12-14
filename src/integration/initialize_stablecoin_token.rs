use axum::{response::IntoResponse, Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct TokenInit { pub symbol: String }

pub async fn initialize_stablecoin_token(State(_state): State<AppState>, Json(payload): Json<TokenInit>) -> impl IntoResponse {
    Json(json!({"result": "token initialized", "symbol": payload.symbol}))
}
