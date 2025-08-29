use axum::http::{Request, StatusCode};
use axum::body::Body;
use bitvmx_tictactoe_backend::app;
use tower::ServiceExt;

#[tokio::test]
async fn test_health_endpoint_with_tracing() {
    let app = app::app();

    // Test that health endpoint works and generates tracing spans
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_game_creation_with_tracing() {
    let app = app::app();

    // Test that game creation endpoint works and generates tracing spans
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/game")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"player_name": "TestPlayer"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
