use axum::response::{IntoResponse, Json};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

/// Request structure for the `/stablecoin/quote/{type}` endpoint.
///
/// ### Fields
/// - `stablecoinIndex`: Index of the stablecoin (e.g., 0 for USDC+).
/// - `depositAmount`: Amount to deposit or redeem. Must be positive.
///
/// ### Example Request
/// ```json
/// {
///   "stablecoinIndex": 0,
///   "depositAmount": 1000000
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct QuoteRequest {
    pub stablecoinIndex: u32,
    pub depositAmount: i64,
}

/// Success response structure for mint/redeem quote.
///
/// ### Example Success Response (HTTP 200)
/// ```json
/// {
///   "success": true,
///   "data": 999000
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct QuoteSuccessResponse {
    success: bool,
    data: i64,
}

/// Error response structure for mint/redeem quote.
///
/// ### Example Error Response (HTTP 400/404/500)
/// ```json
/// {
///   "success": false,
///   "message": "Invalid request data: depositAmount must be positive"
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct QuoteErrorResponse {
    success: bool,
    message: &'static str,
}

/// Handler for `POST /stablecoin/quote/{type}`.
///
/// Supports both `mint` and `redeem` types.  
/// Validates the request and returns either a success quote or an error.
///
/// # Examples
///
/// ```bash
/// curl --request POST \
///   --url http://localhost:3000/stablecoin/quote/mint \
///   --header 'Content-Type: application/json' \
///   --data '{
///     "stablecoinIndex": 0,
///     "depositAmount": 1000000
///   }'
/// ```
///
/// Expected output:
/// ```json
/// {
///   "success": true,
///   "data": 999000
/// }
/// ```
pub async fn get_mint_redeem_quote(
    quote_type: &str,
    req: QuoteRequest,
) -> impl IntoResponse {
    // Validate deposit amount
    if req.depositAmount <= 0 {
        let error = QuoteErrorResponse {
            success: false,
            message: "Invalid request data: depositAmount must be positive",
        };
        return (StatusCode::BAD_REQUEST, Json(error));
    }

    // Simulated calculation: apply a 0.1% fee
    let quoted_amount = req.depositAmount - (req.depositAmount / 1000);

    match quote_type {
        "mint" | "redeem" => {
            let response = QuoteSuccessResponse {
                success: true,
                data: quoted_amount,
            };
            (StatusCode::OK, Json(response))
        }
        _ => {
            let error = QuoteErrorResponse {
                success: false,
                message: "Invalid request type",
            };
            (StatusCode::NOT_FOUND, Json(error))
        }
    }
}

/// Example error handler for internal server errors.
pub async fn get_mint_redeem_quote_error() -> impl IntoResponse {
    let response = QuoteErrorResponse {
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

    #[tokio::test]
    async fn test_mint_success() {
        let req = QuoteRequest {
            stablecoinIndex: 0,
            depositAmount: 1_000_000,
        };
        let response = get_mint_redeem_quote("mint", req).await.into_response();
        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::OK);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], Value::Bool(true));
        assert_eq!(json["data"], Value::Number(999000.into()));
    }

    #[tokio::test]
    async fn test_redeem_success() {
        let req = QuoteRequest {
            stablecoinIndex: 0,
            depositAmount: 1_000_000,
        };
        let response = get_mint_redeem_quote("redeem", req).await.into_response();
        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::OK);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], Value::Bool(true));
        assert_eq!(json["data"], Value::Number(999000.into()));
    }

    #[tokio::test]
    async fn test_invalid_deposit_amount() {
        let req = QuoteRequest {
            stablecoinIndex: 0,
            depositAmount: -100,
        };
        let response = get_mint_redeem_quote("mint", req).await.into_response();
        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::BAD_REQUEST);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], Value::Bool(false));
        assert_eq!(
            json["message"],
            Value::String("Invalid request data: depositAmount must be positive".into())
        );
    }

    #[tokio::test]
    async fn test_invalid_type() {
        let req = QuoteRequest {
            stablecoinIndex: 0,
            depositAmount: 1_000_000,
        };
        let response = get_mint_redeem_quote("invalid", req).await.into_response();
        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::NOT_FOUND);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], Value::Bool(false));
        assert_eq!(json["message"], Value::String("Invalid request type".into()));
    }

    #[tokio::test]
    async fn test_internal_server_error() {
        let response = get_mint_redeem_quote_error().await.into_response();
        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::INTERNAL_SERVER_ERROR);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], Value::Bool(false));
        assert_eq!(json["message"], Value::String("Internal server error".into()));
    }
}
