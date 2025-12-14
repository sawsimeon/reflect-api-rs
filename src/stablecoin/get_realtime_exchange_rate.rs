use axum::{response::IntoResponse, Json, extract::Path};
use serde_json::json;

pub async fn get_realtime_exchange_rate(Path(stablecoin): Path<String>) -> impl IntoResponse {
    Json(json!({"stablecoin": stablecoin, "realtime_rate": 1.0}))
}
