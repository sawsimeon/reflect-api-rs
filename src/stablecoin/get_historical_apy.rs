use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Query parameters for historical APY retrieval.
///
/// ### Fields
/// - `days`: Number of days of historical APY data (default: 365, must be >= 1)
///
/// ### Example
/// ```text
/// ?days=365
/// ```
#[derive(Debug, Deserialize)]
pub struct HistoricalApyQuery {
    pub days: Option<u32>,
}

/// Historical APY data structure.
///
/// ### Fields
/// - `index`: Stablecoin index (0 = USDC+)
/// - `apy`: APY value (percentage)
/// - `timestamp`: Timestamp of the APY snapshot
///
/// ### Example
/// ```json
/// {
///   "index": 0,
///   "apy": 5.25,
///   "timestamp": "2023-11-07T05:31:56Z"
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct HistoricalApyData {
    pub index: u32,
    pub apy: f64,
    pub timestamp: String,
}

/// Success response structure for historical APY retrieval.
///
/// ### Example Success Response (HTTP 200)
/// ```json
/// {
///   "success": true,
///   "data": {
///     "index": 0,
///     "apy": 5.25,
///     "timestamp": "2023-11-07T05:31:56Z"
///   }
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct HistoricalApySuccessResponse {
    pub success: bool,
    pub data: HistoricalApyData,
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
pub struct HistoricalApyErrorResponse {
    pub success: bool,
    pub message: &'static str,
}

/// Handler for:
///
/// ### `GET /stablecoin/{index}/apy/historical`
///
/// Retrieves historical APY data for a specific stablecoin over a given number of days.
///
/// # Example
///
/// ```bash
/// curl --request GET \
///   --url "http://localhost:3000/stablecoin/0/apy/historical?days=365"
/// ```
pub async fn get_historical_apy(
    Path(index): Path<u32>,
    Query(query): Query<HistoricalApyQuery>,
) -> impl IntoResponse {
    let days = query.days.unwrap_or(365);

    // Validate days >= 1
    if days < 1 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!(HistoricalApyErrorResponse {
                success: false,
                message: "Invalid request data: depositAmount must be positive",
            })),
        );
    }

    // Simulated APY data (mirrors real API)
    let data = HistoricalApyData {
        index,
        apy: 5.25,
        timestamp: "2023-11-07T05:31:56Z".to_string(),
    };

    (
        StatusCode::OK,
        Json(json!(HistoricalApySuccessResponse {
            success: true,
            data,
        })),
    )
}

/// Example internal server error handler.
pub async fn get_historical_apy_error() -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!(HistoricalApyErrorResponse {
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
    async fn test_historical_apy_success() {
        let response = get_historical_apy(
            Path(0),
            Query(HistoricalApyQuery { days: Some(365) }),
        )
        .await
        .into_response();

        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::OK);

        let bytes = to_bytes(body, 2048).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], true);
        assert_eq!(json["data"]["index"], 0);
        assert_eq!(json["data"]["apy"], 5.25);
    }

    #[tokio::test]
    async fn test_historical_apy_invalid_days() {
        let response = get_historical_apy(
            Path(0),
            Query(HistoricalApyQuery { days: Some(0) }),
        )
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
    async fn test_historical_apy_internal_error() {
        let response = get_historical_apy_error().await.into_response();

        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::INTERNAL_SERVER_ERROR);

        let bytes = to_bytes(body, 2048).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], false);
        assert_eq!(json["message"], "Internal server error");
    }
}
