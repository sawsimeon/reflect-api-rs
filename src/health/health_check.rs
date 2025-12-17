use axum::response::{IntoResponse, Json};
use chrono::Utc;
use serde::Serialize;

/// Response structure for the `/health` endpoint, matching the official Reflect API.
///
/// ### Fields
/// - `success`: Always `true` when the server is healthy and responding.
/// - `message`: Human-readable status message (e.g., `"API is running"`).
/// - `timestamp`: Current UTC timestamp in ISO 8601 format with milliseconds and 'Z' suffix.
///
/// ### Example Successful Response (HTTP 200)
/// ```json
/// {
///   "success": true,
///   "message": "API is running",
///   "timestamp": "2025-12-17T12:34:56.789Z"
/// }
/// ```
///
/// Note: The official Reflect API does not document error responses for this endpoint.
/// In production, you might return a 503 Service Unavailable on failure,
/// but for this scaffold we always return 200 with a healthy response.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    success: bool,
    message: &'static str,
    timestamp: String,
}

/// Handler for `GET /health`.
///
/// Mimics the official Reflect API health check:
/// ```bash
/// curl https://prod.api.reflect.money/health
/// ```
///
/// Returns a JSON response indicating the API is operational, along with the current timestamp.
/// No authentication is required.
///
/// # Examples
///
/// ```
/// use reflect_api_rs::health::health_check;
/// use axum::response::IntoResponse;
///
/// # tokio_test::block_on(async {
/// let response = health_check().await.into_response();
/// assert_eq!(response.status(), axum::http::StatusCode::OK);
/// # });
/// ```
pub async fn health_check() -> impl IntoResponse {
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S.%3fZ").to_string();

    Json(HealthResponse {
        success: true,
        message: "API is running",
        timestamp,
    })
}

#[cfg(test)]
mod tests {
    //! Unit tests for the `/health` endpoint.
    //!
    //! These tests validate:
    //! - The response status code is always `200 OK`.
    //! - The JSON body contains the expected fields (`success`, `message`, `timestamp`).
    //! - The timestamp is in valid RFC3339 format and close to the current time.

    use super::*;
    use axum::body::to_bytes;
    use axum::http::StatusCode;
    use chrono::{DateTime, Utc};
    use serde_json::Value;

    /// Ensure that `health_check` returns a 200 response with the correct JSON structure.
    #[tokio::test]
    async fn health_check_returns_success_response() {
        let response = health_check().await.into_response();
        let (parts, body) = response.into_parts();

        // Status should be 200 OK
        assert_eq!(parts.status, StatusCode::OK);

        // Collect body into bytes (limit set high enough for small JSON)
        let bytes = to_bytes(body, 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        // Validate fields
        assert_eq!(json["success"], Value::Bool(true));
        assert_eq!(json["message"], Value::String("API is running".into()));

        // Validate timestamp format and recency
        let timestamp_str = json["timestamp"].as_str().unwrap();
        let parsed = DateTime::parse_from_rfc3339(&timestamp_str.replace('Z', "+00:00"))
            .expect("Invalid timestamp format");
        let now = Utc::now();
        let diff = (now - parsed.with_timezone(&Utc)).num_seconds();
        assert!(
            diff.abs() < 5,
            "Timestamp should be within 5 seconds of current time"
        );
    }
}
