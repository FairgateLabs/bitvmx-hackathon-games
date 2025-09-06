use crate::models::ErrorResponse;
use axum::Json;
use http::StatusCode;

/// Create a bad request error response
pub fn bad_request(message: &str) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            error: "BAD_REQUEST".to_string(),
            message: message.to_string(),
        }),
    )
}

/// Create a not found error response
pub fn not_found(message: &str) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "NOT_FOUND".to_string(),
            message: message.to_string(),
        }),
    )
}

/// Create an internal server error response
pub fn internal_server_error(message: &str) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            error: "INTERNAL_SERVER_ERROR".to_string(),
            message: message.to_string(),
        }),
    )
}

/// Create a custom error response
pub fn error_response(
    status: StatusCode,
    error_type: &str,
    message: &str,
) -> (StatusCode, Json<ErrorResponse>) {
    (
        status,
        Json(ErrorResponse {
            error: error_type.to_string(),
            message: message.to_string(),
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_request() {
        let (status, response) = bad_request("Test error message");
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(response.0.error, "BAD_REQUEST");
        assert_eq!(response.0.message, "Test error message");
    }

    #[test]
    fn test_not_found() {
        let (status, response) = not_found("Resource not found");
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(response.0.error, "NOT_FOUND");
        assert_eq!(response.0.message, "Resource not found");
    }

    #[test]
    fn test_internal_server_error() {
        let (status, response) = internal_server_error("Internal error occurred");
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(response.0.error, "INTERNAL_SERVER_ERROR");
        assert_eq!(response.0.message, "Internal error occurred");
    }

    #[test]
    fn test_error_response() {
        let (status, response) =
            error_response(StatusCode::UNAUTHORIZED, "UNAUTHORIZED", "Access denied");
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(response.0.error, "UNAUTHORIZED");
        assert_eq!(response.0.message, "Access denied");
    }
}
