use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// APY data structure for a stablecoin.
///
/// ### Fields
/// - `index`: Stablecoin index (e.g., 0 for USDC+).
/// - `apy`: Annual Percentage Yield (basis points).
/// - `timestamp`: Timestamp of the APY snapshot.
///
/// ### Example
/// ```json
/// {
///   "index": 0,
///   "apy": 224,
///   "timestamp": "2025-12-19T16:55:42.407Z"
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct ApyData {
    pub index: u32,
    pub apy: i64,
    pub timestamp: String,
}

/// Success response structure for APY retrieval.
///
/// ### Example Success Response (HTTP 200)
/// ```json
/// {
///   "success": true,
///   "data": [
///     {
///       "index": 0,
///       "apy": 224,
///       "timestamp": "2025-12-19T16:55:42.407Z"
///     }
///   ]
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct ApySuccessResponse {
    pub success: bool,
    pub data: Vec<ApyData>,
}

/// Error response structure for APY retrieval.
///
/// ### Example Error Response (HTTP 404/500)
/// ```json
/// {
///   "success": false,
///   "message": "Internal server error"
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct ApyErrorResponse {
    pub success: bool,
    pub message: &'static str,
}

/// Handler for `GET /stablecoin/apy`.
///
/// Returns simulated APY data or an error.
///
/// # Example
///
/// ```bash
/// curl --request GET \
///   --url http://localhost:3000/stablecoin/apy
/// ```
pub async fn get_all_apy() -> impl IntoResponse {
    // Simulated APY data
    let apy_data = vec![ApyData {
        index: 0,
        apy: 224,
        timestamp: "2025-12-19T16:55:42.407Z".to_string(),
    }];

    (
        StatusCode::OK,
        Json(json!(ApySuccessResponse {
            success: true,
            data: apy_data,
        })),
    )
}

/// Example error handler for not found.
pub async fn get_all_apy_not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!(ApyErrorResponse {
            success: false,
            message: "Invalid request data: depositAmount must be positive",
        })),
    )
}

/// Example error handler for internal server errors.
pub async fn get_all_apy_error() -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!(ApyErrorResponse {
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
    async fn test_get_all_apy_success() {
        let response = get_all_apy().await.into_response();

        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::OK);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], true);
        assert!(json["data"].is_array());
        assert_eq!(json["data"][0]["apy"], 224);
    }

    #[tokio::test]
    async fn test_get_all_apy_not_found() {
        let response = get_all_apy_not_found().await.into_response();
        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::NOT_FOUND);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], false);
        assert_eq!(
            json["message"],
            "Invalid request data: depositAmount must be positive"
        );
    }

    #[tokio::test]
    async fn test_get_all_apy_internal_error() {
        let response = get_all_apy_error().await.into_response();
        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::INTERNAL_SERVER_ERROR);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], false);
        assert_eq!(json["message"], "Internal server error");
    }
}
