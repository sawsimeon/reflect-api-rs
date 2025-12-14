use axum::{response::IntoResponse, Json, extract::Query};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct SignerQuery { pub signer: String }

pub async fn get_events_by_signer(Query(_q): Query<SignerQuery>) -> impl IntoResponse {
    Json(json!({"events": [{"id": "evt_1", "signer": "0xabc"}]}))
}
