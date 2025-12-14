use axum::{response::IntoResponse, Json, extract::Query};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct ByAuthorityQuery { pub authority: String }

pub async fn get_integrations_by_authority(Query(_q): Query<ByAuthorityQuery>) -> impl IntoResponse {
    Json(json!({"integrations": [{"id": "int_1", "authority": "auth_1"}]}))
}
