use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use tower::ServiceExt;
use bitvmx_tictactoe_backend::{api, app_state, types::{P2PAddress, SetupKey}};

#[tokio::test]
async fn test_bitvmx_comm_info_integration() {
    // Create a test app state
    let app_state = app_state::AppState::new(bitvmx_tictactoe_backend::config::Config::default());
    
    // Set up the store with a P2P address
    let test_address = P2PAddress {
        address: "127.0.0.1:8080".to_string(),
        peer_id: "L2_ID".to_string(),
    };
    {
        let mut store_guard = app_state.bitvmx_store.write().await;
        store_guard.set_p2p_address(test_address);
    }
    
    let app = api::app(app_state).await;

    let response = app
        .oneshot(Request::builder().uri("/api/bitvmx/comm-info").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body();
    let body_bytes = to_bytes(body, usize::MAX).await.unwrap();
    let comm_info: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

    // Verify the response structure
    assert!(comm_info.get("address").is_some());
    assert!(comm_info.get("peer_id").is_some());

    // Verify specific values
    assert_eq!(comm_info["peer_id"], "L2_ID");
    assert!(comm_info["address"].as_str().unwrap().contains("127.0.0.1:"));
    assert!(comm_info["address"].as_str().unwrap().contains(":"));
}

#[tokio::test]
async fn test_bitvmx_aggregated_key_integration() {
    // Create a test app state
    let app_state = app_state::AppState::new(bitvmx_tictactoe_backend::config::Config::default());
    let app = api::app(app_state).await;

    let setup_key = SetupKey {
        id: "test-id-123".to_string(),
        addresses: vec![
            P2PAddress {
                address: "127.0.0.1:8080".to_string(),
                peer_id: "L2_ID".to_string(),
            }
        ],
    };

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/bitvmx/aggregated-key")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&setup_key).unwrap()))
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
