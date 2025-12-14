use axum::{response::IntoResponse, Json, extract::Path};
use serde_json::json;

pub async fn get_historical_apy(Path(stablecoin): Path<String>) -> impl IntoResponse {
    Json(json!({
        "stablecoin": stablecoin,
        "historical_apy": [
            {"timestamp": 1700000000, "apy": 0.02},
            {"timestamp": 1700086400, "apy": 0.021}
        ]
    }))
}
