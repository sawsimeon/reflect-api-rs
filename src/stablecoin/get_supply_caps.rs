use axum::response::{IntoResponse, Json};
use axum::http::StatusCode;
use serde::Serialize;

/// Response structure for the `/stablecoin/limits` endpoint, matching the official Reflect API.
///
/// ### Description
/// Get supply cap information for all stablecoins.  
/// Retrieve supply caps, current supply, and remaining capacity for all stablecoins.
///
/// ### Success Response (HTTP 200)
/// ```json
/// {
///   "success": true,
///   "data": [
///     {
///       "index": 0,
///       "supplyCap": 1000000000,
///       "currentSupply": 500000000,
///       "remainingCapacity": 500000000,
///       "utilizationPercentage": 50
///     }
///   ]
/// }
/// ```
///
/// ### Error Response (HTTP 500)
/// ```json
/// {
///   "success": false,
///   "message": "Internal server error"
/// }
/// ```
///
/// # Examples
///
/// ```bash
/// curl --request GET \
///   --url http://localhost:3000/stablecoin/limits
/// ```
#[derive(Debug, Serialize)]
pub struct SupplyCapsSuccessResponse {
    success: bool,
    data: Vec<SupplyCap>,
}

#[derive(Debug, Serialize)]
pub struct SupplyCap {
    index: u32,
    #[serde(rename = "supplyCap")]
    supply_cap: u64,
    #[serde(rename = "currentSupply")]
    current_supply: u64,
    #[serde(rename = "remainingCapacity")]
    remaining_capacity: u64,
    #[serde(rename = "utilizationPercentage")]
    utilization_percentage: u32,
}

#[derive(Debug, Serialize)]
pub struct SupplyCapsErrorResponse {
    success: bool,
    message: &'static str,
}

/// Handler for `GET /stablecoin/limits`.
///
/// Returns a JSON response with supply cap information for all stablecoins.  
/// In this scaffold, we return a static example response.
/// In production, you would query a database or external service.
pub async fn get_supply_caps() -> impl IntoResponse {
    // Static example data for USDC+
    let caps = vec![SupplyCap {
        index: 0,
        supply_cap: 1_000_000_000,
        current_supply: 500_000_000,
        remaining_capacity: 500_000_000,
        utilization_percentage: 50,
    }];

    let response = SupplyCapsSuccessResponse {
        success: true,
        data: caps,
    };

    (StatusCode::OK, Json(response))
}

/// Example error handler for `/stablecoin/limits`.
///
/// In production, you might return this if a database query fails.
pub async fn get_supply_caps_error() -> impl IntoResponse {
    let response = SupplyCapsErrorResponse {
        success: false,
        message: "Internal server error",
    };

    (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;
    use axum::http::StatusCode;
    use axum::body::to_bytes;
    use serde_json::Value;

    /// Unit test: ensure `get_supply_caps` returns a 200 response with correct JSON structure.
    #[tokio::test]
    async fn get_supply_caps_success() {
        let response = get_supply_caps().await.into_response();
        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::OK);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], Value::Bool(true));
        assert!(json["data"].is_array());
        assert_eq!(json["data"][0]["index"], Value::Number(0.into()));
        assert_eq!(json["data"][0]["supplyCap"], Value::Number(1_000_000_000.into()));
        assert_eq!(json["data"][0]["currentSupply"], Value::Number(500_000_000.into()));
        assert_eq!(json["data"][0]["remainingCapacity"], Value::Number(500_000_000.into()));
        assert_eq!(json["data"][0]["utilizationPercentage"], Value::Number(50.into()));
    }

    /// Unit test: ensure `get_supply_caps_error` returns a 500 response with correct JSON structure.
    #[tokio::test]
    async fn get_supply_caps_error_test() {
        let response = get_supply_caps_error().await.into_response();
        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::INTERNAL_SERVER_ERROR);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], Value::Bool(false));
        assert_eq!(json["message"], Value::String("Internal server error".into()));
    }
}
