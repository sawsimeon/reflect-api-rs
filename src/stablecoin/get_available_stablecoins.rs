use axum::response::{IntoResponse, Json};
use axum::http::StatusCode;
use serde::Serialize;

/// Response structure for the `/stablecoin/types` endpoint, matching the official Reflect API.
///
/// ### Success Response (HTTP 200)
/// ```json
/// {
///   "success": true,
///   "data": [
///     {
///       "index": 0,
///       "name": "USDC+"
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
///   --url http://localhost:3000/stablecoin/types
/// ```
///
/// Expected output:
/// ```json
/// {
///   "success": true,
///   "data": [
///     { "index": 0, "name": "USDC+" }
///   ]
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct StablecoinSuccessResponse {
    success: bool,
    data: Vec<Stablecoin>,
}

#[derive(Debug, Serialize)]
pub struct Stablecoin {
    index: u32,
    name: String,
}

#[derive(Debug, Serialize)]
pub struct StablecoinErrorResponse {
    success: bool,
    message: &'static str,
}

/// Handler for `GET /stablecoin/types`.
///
/// Returns a JSON response with available stablecoins.  
/// In this scaffold, we always return a successful response with a static list.
/// In production, you would query a database or external service.
///
/// # Examples
///
/// ```
/// use reflect_api_rs::stablecoin::get_available_stablecoins;
/// use axum::response::IntoResponse;
///
/// # tokio_test::block_on(async {
/// let response = get_available_stablecoins().await.into_response();
/// assert_eq!(response.status(), axum::http::StatusCode::OK);
/// # });
/// ```
pub async fn get_available_stablecoins() -> impl IntoResponse {
    // Only USDC+ is available in this scaffold
    let stablecoins = vec![Stablecoin {
        index: 0,
        name: "USDC+".to_string(),
    }];

    let response = StablecoinSuccessResponse {
        success: true,
        data: stablecoins,
    };

    (StatusCode::OK, Json(response))
}

/// Example error handler for `/stablecoin/types`.
///
/// In production, you might return this if a database query fails.
pub async fn get_available_stablecoins_error() -> impl IntoResponse {
    let response = StablecoinErrorResponse {
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

    /// Unit test: ensure `get_available_stablecoins` returns a 200 response with correct JSON structure.
    #[tokio::test]
    async fn get_available_stablecoins_success() {
        let response = get_available_stablecoins().await.into_response();
        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::OK);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], Value::Bool(true));
        assert!(json["data"].is_array());
        assert_eq!(json["data"][0]["index"], Value::Number(0.into()));
        assert_eq!(json["data"][0]["name"], Value::String("USDC+".into()));
    }

    /// Unit test: ensure `get_available_stablecoins_error` returns a 500 response with correct JSON structure.
    #[tokio::test]
    async fn get_available_stablecoins_error_test() {
        let response = get_available_stablecoins_error().await.into_response();
        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::INTERNAL_SERVER_ERROR);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], Value::Bool(false));
        assert_eq!(json["message"], Value::String("Internal server error".into()));
    }
}
