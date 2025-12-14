use axum::{response::IntoResponse, Json, extract::Path};
use serde_json::json;

pub async fn get_specific_apy(Path(stablecoin): Path<String>) -> impl IntoResponse {
    Json(json!({"stablecoin": stablecoin, "apy": 0.02}))
}
