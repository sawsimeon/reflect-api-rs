use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn get_available_stablecoins() -> impl IntoResponse {
    let data = json!([
        {"symbol": "rUSD", "name": "Reflect USD"},
        {"symbol": "rEUR", "name": "Reflect EUR"}
    ]);
    Json(json!({"stablecoins": data}))
}
