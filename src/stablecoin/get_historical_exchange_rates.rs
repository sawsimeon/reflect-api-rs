use axum::{response::IntoResponse, Json, extract::Path};
use serde_json::json;

pub async fn get_historical_exchange_rates(Path(stablecoin): Path<String>) -> impl IntoResponse {
    Json(json!({
        "stablecoin": stablecoin,
        "historical": [
            {"timestamp": 1700000000, "rate": 1.0},
            {"timestamp": 1700086400, "rate": 0.99}
        ]
    }))
}
