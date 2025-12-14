use axum::{response::IntoResponse, Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct VaultInit { pub vault_name: String }

pub async fn initialize_integration_vault(State(_state): State<AppState>, Json(payload): Json<VaultInit>) -> impl IntoResponse {
    Json(json!({"result": "vault initialized", "vault": payload.vault_name}))
}
