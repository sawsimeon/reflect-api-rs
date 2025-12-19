use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Query parameters for historical exchange rate retrieval.
///
/// ### Fields
/// - `stablecoin`: Stablecoin index (e.g., 0 for USDC+).
/// - `days`: Number of days of historical data to retrieve.
///
/// ### Example
/// - `?days=1&stablecoin=0`
#[derive(Debug, Deserialize)]
pub struct HistoricalQuery {
    pub stablecoin: u32,
    pub days: u32,
}

/// Historical exchange rate data structure.
///
/// ### Fields
/// - `id`: Unique identifier for the exchange rate record.
/// - `stablecoin`: Stablecoin index.
/// - `base_usd_value_bps`: Base USD value in basis points.
/// - `receipt_usd_value_bps`: Receipt USD value in basis points.
/// - `timestamp`: Timestamp of the exchange rate snapshot.
///
/// ### Example
/// ```json
/// {
///   "id": 104135,
///   "stablecoin": 0,
///   "base_usd_value_bps": 1016733625,
///   "timestamp": "2025-12-18T17:46:10.274Z",
///   "receipt_usd_value_bps": 1016733625
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct HistoricalExchangeRateData {
    pub id: u64,
    pub stablecoin: u32,
    pub base_usd_value_bps: i64,
    pub timestamp: String,
    pub receipt_usd_value_bps: i64,
}

/// Success response structure for historical exchange rate retrieval.
///
/// ### Example Success Response (HTTP 200)
/// ```json
/// {
///   "success": true,
///   "data": [
///     {
///       "id": 104135,
///       "stablecoin": 0,
///       "base_usd_value_bps": 1016733625,
///       "timestamp": "2025-12-18T17:46:10.274Z",
///       "receipt_usd_value_bps": 1016733625
///     }
///   ]
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct HistoricalSuccessResponse {
    pub success: bool,
    pub data: Vec<HistoricalExchangeRateData>,
}

/// Error response structure for historical exchange rate retrieval.
///
/// ### Example Error Response (HTTP 500)
/// ```json
/// {
///   "success": false,
///   "message": "Internal server error"
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct HistoricalErrorResponse {
    pub success: bool,
    pub message: &'static str,
}

/// Handler for `GET /stablecoin/exchange-rates/historical`.
///
/// Accepts query parameters `days` and `stablecoin`.  
/// Returns simulated historical exchange rate data or an error.
///
/// # Example
///
/// ```bash
/// curl --request GET \
///   --url "http://localhost:3000/stablecoin/exchange-rates/historical?days=1&stablecoin=0"
/// ```
pub async fn get_historical_exchange_rates(
    Query(query): Query<HistoricalQuery>,
) -> impl IntoResponse {
    // Simulated historical data
    let data = vec![
        HistoricalExchangeRateData {
            id: 104135,
            stablecoin: query.stablecoin,
            base_usd_value_bps: 1016733625,
            timestamp: "2025-12-18T17:46:10.274Z".to_string(),
            receipt_usd_value_bps: 1016733625,
        },
        HistoricalExchangeRateData {
            id: 104137,
            stablecoin: query.stablecoin,
            base_usd_value_bps: 1016728666,
            timestamp: "2025-12-18T17:47:08.161Z".to_string(),
            receipt_usd_value_bps: 1016728667,
        },
    ];

    (
        StatusCode::OK,
        Json(json!(HistoricalSuccessResponse {
            success: true,
            data,
        })),
    )
}

/// Example error handler for internal server errors.
pub async fn get_historical_exchange_rates_error() -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!(HistoricalErrorResponse {
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
    async fn test_historical_exchange_rates_success() {
        let query = HistoricalQuery {
            stablecoin: 0,
            days: 1,
        };
        let response = get_historical_exchange_rates(Query(query))
            .await
            .into_response();

        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::OK);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], true);
        assert!(json["data"].is_array());
        assert_eq!(json["data"][0]["stablecoin"], 0);
        assert_eq!(json["data"][0]["id"], 104135);
    }

    #[tokio::test]
    async fn test_historical_exchange_rates_internal_error() {
        let response = get_historical_exchange_rates_error().await.into_response();
        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::INTERNAL_SERVER_ERROR);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], false);
        assert_eq!(json["message"], "Internal server error");
    }
}
