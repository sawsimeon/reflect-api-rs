use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Exchange rate data structure for a stablecoin.
///
/// ### Fields
/// - `id`: Unique identifier for the exchange rate record.
/// - `stablecoin`: Stablecoin index (e.g., 0 for USDC+).
/// - `base_usd_value_bps`: Base USD value in basis points.
/// - `receipt_usd_value_bps`: Receipt USD value in basis points.
/// - `timestamp`: Timestamp of the exchange rate snapshot.
///
/// ### Example
/// ```json
/// {
///   "id": 105511,
///   "stablecoin": 0,
///   "base_usd_value_bps": 1016789908,
///   "timestamp": "2025-12-19T17:04:08.502Z",
///   "receipt_usd_value_bps": 1016791576
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct ExchangeRateData {
    pub id: u64,
    pub stablecoin: u32,
    pub base_usd_value_bps: i64,
    pub timestamp: String,
    pub receipt_usd_value_bps: i64,
}

/// Success response structure for exchange rate retrieval.
///
/// ### Example Success Response (HTTP 200)
/// ```json
/// {
///   "success": true,
///   "data": [
///     {
///       "id": 105511,
///       "stablecoin": 0,
///       "base_usd_value_bps": 1016789908,
///       "timestamp": "2025-12-19T17:04:08.502Z",
///       "receipt_usd_value_bps": 1016791576
///     }
///   ]
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct ExchangeRateSuccessResponse {
    pub success: bool,
    pub data: Vec<ExchangeRateData>,
}

/// Error response structure for exchange rate retrieval.
///
/// ### Example Error Response (HTTP 500)
/// ```json
/// {
///   "success": false,
///   "message": "Internal server error"
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct ExchangeRateErrorResponse {
    pub success: bool,
    pub message: &'static str,
}

/// Handler for `GET /stablecoin/exchange-rates`.
///
/// Returns simulated exchange rate data or an error.
///
/// # Example
///
/// ```bash
/// curl --request GET \
///   --url http://localhost:3000/stablecoin/exchange-rates
/// ```
pub async fn get_latest_exchange_rates() -> impl IntoResponse {
    // Simulated exchange rate data
    let rates = vec![ExchangeRateData {
        id: 105511,
        stablecoin: 0,
        base_usd_value_bps: 1016789908,
        timestamp: "2025-12-19T17:04:08.502Z".to_string(),
        receipt_usd_value_bps: 1016791576,
    }];

    (
        StatusCode::OK,
        Json(json!(ExchangeRateSuccessResponse {
            success: true,
            data: rates,
        })),
    )
}

/// Example error handler for internal server errors.
pub async fn get_latest_exchange_rates_error() -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!(ExchangeRateErrorResponse {
            success: false,
            message: "Internal server error",
        })),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::response::IntoResponse;
    use serde_json::Value;

    #[tokio::test]
    async fn test_exchange_rates_success() {
        let response = get_latest_exchange_rates().await.into_response();

        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::OK);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], true);
        assert!(json["data"].is_array());
        assert_eq!(json["data"][0]["id"], 105511);
        assert_eq!(json["data"][0]["stablecoin"], 0);
        assert_eq!(json["data"][0]["base_usd_value_bps"], 1016789908);
        assert_eq!(json["data"][0]["receipt_usd_value_bps"], 1016791576);
    }

    #[tokio::test]
    async fn test_exchange_rates_internal_error() {
        let response = get_latest_exchange_rates_error().await.into_response();
        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::INTERNAL_SERVER_ERROR);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], false);
        assert_eq!(json["message"], "Internal server error");
    }
}
