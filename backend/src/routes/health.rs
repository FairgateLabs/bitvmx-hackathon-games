use axum::{Router, routing::get};
use crate::handlers::health;
use crate::types::HealthResponse;

pub fn router() -> Router {
    Router::new().route("/", get(health_check))
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    ),
    tag = "Health"
)]
pub async fn health_check() -> axum::Json<HealthResponse> {
    health::health_check().await
}



