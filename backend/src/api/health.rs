use axum::{extract::State, Json};
use crate::types::HealthResponse;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use crate::models::GameStore;

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    ),
    tag = "Health"
)]
pub async fn health_check(
    State(_store): State<Arc<Mutex<GameStore>>>,
) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    })
}
