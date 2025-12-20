use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Realtime exchange rate data structure.
///
/// ### Fields
/// - `base`: Base USD value in basis points
/// - `receipt`: Receipt USD value in basis points
///
/// ### Example
/// ```json
/// {
///   "base": 1016858791,
///   "receipt": 1016858791
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct RealtimeExchangeRateData {
    pub base: i64,
    pub receipt: i64,
}

/// Success response structure for realtime exchange rate retrieval.
///
/// ### Example Success Response (HTTP 200)
/// ```json
/// {
///   "success": true,
///   "data": {
///     "base": 1016858791,
///     "receipt": 1016858791
///   }
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct RealtimeExchangeRateSuccessResponse {
    pub success: bool,
    pub data: RealtimeExchangeRateData,
}

/// Error response structure.
///
/// ### Example Error Response (HTTP 400/500)
/// ```json
/// {
///   "success": false,
///   "message": "Internal server error"
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct RealtimeExchangeRateErrorResponse {
    pub success: bool,
    pub message: &'static str,
}

/// Handler for:
///
/// ### `GET /stablecoin/{index}/exchange-rate`
///
/// Retrieves the realtime exchange rate for a specific stablecoin.
///
/// # Example
///
/// ```bash
/// curl --request GET \
///   --url "http://localhost:3000/stablecoin/0/exchange-rate"
/// ```
pub async fn get_realtime_exchange_rate(
    Path(index): Path<u32>,
) -> impl IntoResponse {
    // Validate stablecoin index (only 0 exists in Reflect API)
    if index != 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!(RealtimeExchangeRateErrorResponse {
                success: false,
                message: "Invalid request data: depositAmount must be positive",
            })),
        );
    }

    // Simulated realtime exchange rate data
    let data = RealtimeExchangeRateData {
        base: 1016858791,
        receipt: 1016858791,
    };

    (
        StatusCode::OK,
        Json(json!(RealtimeExchangeRateSuccessResponse {
            success: true,
            data,
        })),
    )
}

/// Example internal server error handler.
pub async fn get_realtime_exchange_rate_error() -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!(RealtimeExchangeRateErrorResponse {
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
    async fn test_realtime_exchange_rate_success() {
        let response = get_realtime_exchange_rate(Path(0))
            .await
            .into_response();

        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::OK);

        let bytes = to_bytes(body, 2048).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], true);
        assert_eq!(json["data"]["base"], 1016858791);
        assert_eq!(json["data"]["receipt"], 1016858791);
    }

    #[tokio::test]
    async fn test_realtime_exchange_rate_invalid_index() {
        let response = get_realtime_exchange_rate(Path(99))
            .await
            .into_response();

        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::BAD_REQUEST);

        let bytes = to_bytes(body, 2048).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], false);
        assert_eq!(
            json["message"],
            "Invalid request data: depositAmount must be positive"
        );
    }

    #[tokio::test]
    async fn test_realtime_exchange_rate_internal_error() {
        let response = get_realtime_exchange_rate_error()
            .await
            .into_response();

        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::INTERNAL_SERVER_ERROR);

        let bytes = to_bytes(body, 2048).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], false);
        assert_eq!(json["message"], "Internal server error");
    }
}
