use axum::{response::IntoResponse, Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct UserTokenInit { pub user_id: String }

pub async fn initialize_user_branded_token(State(_state): State<AppState>, Json(payload): Json<UserTokenInit>) -> impl IntoResponse {
    Json(json!({"result": "user token initialized", "user_id": payload.user_id}))
}
