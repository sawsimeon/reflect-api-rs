use axum::{
    extract::{Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Request structure for the `/stablecoin/burn` endpoint.
///
/// ### Fields
/// - `stablecoin_index`: Index of the stablecoin (e.g., 0 for USDC+).
/// - `deposit_amount`: Amount to burn in smallest unit. Must be positive.
/// - `signer`: User's Solana wallet address.
/// - `minimum_received`: Minimum amount to receive (slippage protection).
/// - `collateral_mint`: Optional collateral mint address.
///
/// ### Example Request
/// ```json
/// {
///   "stablecoin_index": 0,
///   "deposit_amount": 1000000,
///   "signer": "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",
///   "minimum_received": 999000,
///   "collateral_mint": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct BurnRequest {
    pub stablecoin_index: u32,
    pub deposit_amount: i64,
    pub signer: String,
    pub minimum_received: i64,
    pub collateral_mint: Option<String>,
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

/// Success response structure for burn transaction.
#[derive(Debug, Serialize)]
pub struct BurnSuccessResponse {
    pub success: bool,
    pub data: TransactionData,
}

#[derive(Debug, Serialize)]
pub struct TransactionData {
    pub transaction: String,
}

/// Error response structure for burn transaction.
#[derive(Debug, Serialize)]
pub struct BurnErrorResponse {
    pub success: bool,
    pub message: &'static str,
}

/// Handler for `POST /stablecoin/burn`.
///
/// Supports `cluster` query parameter (`mainnet` or `devnet`).  
/// Validates the request and returns either a simulated transaction or an error.
pub async fn generate_burn_transaction(
    Query(cluster): Query<ClusterQuery>,
    Json(req): Json<BurnRequest>,
) -> impl IntoResponse {
    // Validate deposit amount
    if req.deposit_amount <= 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!(BurnErrorResponse {
                success: false,
                message: "Invalid request data: depositAmount must be positive",
            })),
        );
    }

    // Validate stablecoin index
    if req.stablecoin_index != 0 {
        return (
            StatusCode::NOT_FOUND,
            Json(json!(BurnErrorResponse {
                success: false,
                message: "Stablecoin with the specified index not found",
            })),
        );
    }

    // Simulated transaction string
    let tx = "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAED...";

    (
        StatusCode::OK,
        Json(json!(BurnSuccessResponse {
            success: true,
            data: TransactionData {
                transaction: tx.to_string(),
            },
        })),
    )
}

/// Example error handler for internal server errors.
pub async fn generate_burn_transaction_error() -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!(BurnErrorResponse {
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

    fn make_request(stablecoin_index: u32, deposit_amount: i64) -> BurnRequest {
        BurnRequest {
            stablecoin_index,
            deposit_amount,
            signer: "test_signer".to_string(),
            minimum_received: 999000,
            collateral_mint: Some("test_mint".to_string()),
        }
    }

    #[tokio::test]
    async fn test_burn_success() {
        let req = make_request(0, 1_000_000);
        let response = generate_burn_transaction(
            Query(ClusterQuery { cluster: Some("mainnet".to_string()) }),
            Json(req),
        )
        .await
        .into_response();

        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::OK);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], true);
        assert!(json["data"]["transaction"].is_string());
    }

    #[tokio::test]
    async fn test_invalid_deposit_amount() {
        let req = make_request(0, -100);
        let response = generate_burn_transaction(
            Query(ClusterQuery { cluster: Some("mainnet".to_string()) }),
            Json(req),
        )
        .await
        .into_response();

        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::BAD_REQUEST);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], false);
        assert_eq!(
            json["message"],
            "Invalid request data: depositAmount must be positive"
        );
    }

    #[tokio::test]
    async fn test_invalid_index() {
        let req = make_request(99, 1_000_000);
        let response = generate_burn_transaction(
            Query(ClusterQuery { cluster: Some("mainnet".to_string()) }),
            Json(req),
        )
        .await
        .into_response();

        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::NOT_FOUND);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], false);
        assert_eq!(
            json["message"],
            "Stablecoin with the specified index not found"
        );
    }

    #[tokio::test]
    async fn test_internal_server_error() {
        let response = generate_burn_transaction_error().await.into_response();
        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::INTERNAL_SERVER_ERROR);

        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["success"], false);
        assert_eq!(json["message"], "Internal server error");
    }
}
