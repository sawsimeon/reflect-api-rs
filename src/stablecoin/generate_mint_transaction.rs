use axum::{
    extract::{Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Request structure for the `/stablecoin/mint` endpoint.
///
/// ### Fields
/// - `stablecoinIndex`: Index of the stablecoin (e.g., 0 for USDC+).
/// - `depositAmount`: Amount to mint in smallest unit. Must be positive.
/// - `signer`: User's Solana wallet address.
/// - `minimumReceived`: Minimum amount to receive (slippage protection).
/// - `collateralMint`: Optional collateral mint address.
///
/// ### Example Request
/// ```json
/// {
///   "stablecoinIndex": 0,
///   "depositAmount": 1000000,
///   "signer": "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",
///   "minimumReceived": 999000,
///   "collateralMint": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct MintRequest {
    pub stablecoinIndex: u32,
    pub depositAmount: i64,
    pub signer: String,
    pub minimumReceived: i64,
    pub collateralMint: Option<String>,
}

/// Query parameter for cluster selection.
///
/// ### Example
/// - `?cluster=mainnet`
/// - `?cluster=devnet`
#[derive(Debug, Deserialize)]
pub struct ClusterQuery {
    pub cluster: Option<String>,
}

/// Success response structure for mint transaction.
///
/// ### Example Success Response (HTTP 200)
/// ```json
/// {
///   "success": true,
///   "data": {
///     "transaction": "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAED..."
///   }
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct MintSuccessResponse {
    success: bool,
    data: TransactionData,
}

#[derive(Debug, Serialize)]
pub struct TransactionData {
    transaction: String,
}

/// Error response structure for mint transaction.
///
/// ### Example Error Response (HTTP 400/404/500)
/// ```json
/// {
///   "success": false,
///   "message": "Invalid request data: depositAmount must be positive"
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct MintErrorResponse {
    success: bool,
    message: &'static str,
}

/// Handler for `POST /stablecoin/mint`.
///
/// Supports `cluster` query parameter (`mainnet` or `devnet`).  
/// Validates the request and returns either a simulated transaction or an error.
///
/// # Examples
///
/// ```bash
/// curl --request POST \
///   --url http://localhost:3000/stablecoin/mint?cluster=mainnet \
///   --header 'Content-Type: application/json' \
///   --data '{
///     "stablecoinIndex": 0,
///     "depositAmount": 1000000,
///     "signer": "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",
///     "minimumReceived": 999000,
///     "collateralMint": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
///   }'
/// ```
pub async fn generate_mint_transaction(
    Query(cluster): Query<ClusterQuery>,
    Json(req): Json<MintRequest>,
) -> impl IntoResponse {
    // Validate deposit amount
    if req.depositAmount <= 0 {
        let error = MintErrorResponse {
            success: false,
            message: "Invalid request data: depositAmount must be positive",
        };
        return (StatusCode::BAD_REQUEST, Json(json!(error)));
    }

    // Validate stablecoin index (only 0 supported in this scaffold)
    if req.stablecoinIndex != 0 {
        let error = MintErrorResponse {
            success: false,
            message: "Stablecoin with the specified index not found",
        };
        return (StatusCode::NOT_FOUND, Json(json!(error)));
    }

    // Simulated transaction string
    let tx = "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAED...";

    let response = MintSuccessResponse {
        success: true,
        data: TransactionData {
            transaction: tx.to_string(),
        },
    };

    (StatusCode::OK, Json(json!(response)))
}

/// Example error handler for internal server errors.
pub async fn generate_mint_transaction_error() -> impl IntoResponse {
    let response = MintErrorResponse {
        success: false,
        message: "Internal server error",
    };

    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(response)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use serde_json::Value;

    #[tokio::test]
    async fn test_mint_success() {
        let req = MintRequest {
            stablecoinIndex: 0,
            depositAmount: 1_000_000,
            signer: "test_signer".to_string(),
            minimumReceived: 999000,
            collateralMint: Some("test_mint".to_string()),
        };
        let response = generate_mint_transaction(
            Query(ClusterQuery { cluster: Some("mainnet".to_string()) }),
            Json(req),
        )
        .await
        .into_response();

        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::OK);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], Value::Bool(true));
        assert!(json["data"]["transaction"].is_string());
    }

    #[tokio::test]
    async fn test_invalid_deposit_amount() {
        let req = MintRequest {
            stablecoinIndex: 0,
            depositAmount: -100,
            signer: "test_signer".to_string(),
            minimumReceived: 999000,
            collateralMint: None,
        };
        let response = generate_mint_transaction(
            Query(ClusterQuery { cluster: Some("mainnet".to_string()) }),
            Json(req),
        )
        .await
        .into_response();

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
    async fn test_invalid_index() {
        let req = MintRequest {
            stablecoinIndex: 99,
            depositAmount: 1_000_000,
            signer: "test_signer".to_string(),
            minimumReceived: 999000,
            collateralMint: None,
        };
        let response = generate_mint_transaction(
            Query(ClusterQuery { cluster: Some("mainnet".to_string()) }),
            Json(req),
        )
        .await
        .into_response();

        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::NOT_FOUND);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], Value::Bool(false));
        assert_eq!(
            json["message"],
            Value::String("Stablecoin with the specified index not found".into())
        );
    }

    #[tokio::test]
    async fn test_internal_server_error() {
        let response = generate_mint_transaction_error().await.into_response();
        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::INTERNAL_SERVER_ERROR);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], Value::Bool(false));
        assert_eq!(json["message"], Value::String("Internal server error".into()));
    }
}
