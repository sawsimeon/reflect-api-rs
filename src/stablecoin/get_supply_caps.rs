use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn get_supply_caps() -> impl IntoResponse {
    Json(json!({"supply_caps": {"rUSD": "1000000", "rEUR": "500000"}}))
}
