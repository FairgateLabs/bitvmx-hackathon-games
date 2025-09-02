use axum::{Router, routing::get, Json, extract::State};
use crate::models::HealthResponse;
use crate::app_state::AppState;
use tracing::instrument;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn router() -> Router<AppState> {
    // Base path is /api/health
    Router::new().route("/", get(health_check))
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    ),
    tag = "Health"
)]
#[instrument(skip(_app_state))]
pub async fn health_check(State(_app_state): State<AppState>) -> Json<HealthResponse> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp,
    })
}



