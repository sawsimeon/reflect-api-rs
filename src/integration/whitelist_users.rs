use axum::{response::IntoResponse, Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct Whitelist { pub users: Vec<String> }

pub async fn whitelist_users(State(_state): State<AppState>, Json(payload): Json<Whitelist>) -> impl IntoResponse {
    Json(json!({"result": "users whitelisted", "count": payload.users.len()}))
}
