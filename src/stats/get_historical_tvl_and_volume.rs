use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn get_historical_tvl_and_volume() -> impl IntoResponse {
    Json(json!({"historical": [{"timestamp": 1700000000, "tvl": 1000000, "volume": 50000}]}))
}
