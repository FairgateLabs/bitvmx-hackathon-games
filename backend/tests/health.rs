use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use tower::ServiceExt;
use bitvmx_tictactoe_backend::{app, types::HealthResponse};

#[tokio::test]
async fn test_health_check_integration() {
    let app = app::app();

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body();
    let body_bytes = to_bytes(body, usize::MAX).await.unwrap();
    let json: HealthResponse = serde_json::from_slice(&body_bytes).unwrap();

    // Verify it's valid JSON with expected structure
    assert_eq!(json.status, "healthy");
    assert!(json.timestamp > 0);
}
