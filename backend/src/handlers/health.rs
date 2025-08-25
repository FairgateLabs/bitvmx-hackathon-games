use axum::Json;
use crate::stores::HealthStore;
use crate::types::HealthResponse;
use tracing::instrument;

/// Health check endpoint
#[instrument]
pub async fn health_check() -> Json<HealthResponse> {
    let health_store = HealthStore::new();
    let timestamp = health_store.get_current_timestamp();
    
    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check_function() {
        let response = health_check().await;
        let health_response = response.0;

        assert_eq!(health_response.status, "healthy");
        assert!(health_response.timestamp > 0);
    }
}
