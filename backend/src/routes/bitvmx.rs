use axum::{Json, Router, routing::{get, post}, extract::State};
use http::StatusCode;
use crate::models::{ErrorResponse, P2PAddress, SetupKey};
use crate::state::AppState;
use crate::http_errors;
use tracing::instrument;

pub fn router() -> Router<AppState> {
    // Base path is /api/bitvmx/
    Router::new()
        .route("/comm-info", get(comm_info))
        .route("/aggregated-key", post(submit_aggregated_key))
}

/// Get BitVMX P2P address information
#[utoipa::path(
    get,
    path = "/api/bitvmx/comm-info",
    responses(
        (status = 200, description = "BitVMX P2P address information", body = P2PAddress),
        (status = 404, description = "P2P address not found", body = ErrorResponse)
    ),
    tag = "BitVMX"
)]
#[instrument(skip(app_state))]
pub async fn comm_info(State(app_state): State<AppState>) -> Result<Json<P2PAddress>, (StatusCode, Json<ErrorResponse>)> {
    let service_guard = app_state.bitvmx_service.read().await;
    let p2p_address = service_guard.get_p2p_address().ok_or(http_errors::not_found("P2P address not found"))?;
    Ok(Json(p2p_address))
}

/// Submit BitVMX aggregated key
#[utoipa::path(
    post,
    path = "/api/bitvmx/aggregated-key",
    request_body = SetupKey,
    responses(
        (status = 200, description = "Aggregated key submitted successfully"),
        (status = 400, description = "Invalid aggregated key", body = ErrorResponse)
    ),
    tag = "BitVMX"
)]
#[instrument(skip(_app_state))]
pub async fn submit_aggregated_key(
    State(_app_state): State<AppState>,
    Json(setup_key): Json<SetupKey>
) -> Result<Json<()>, (StatusCode, Json<ErrorResponse>)> {
    // Validate the setup key
    if setup_key.id.is_empty() {
        return Err(http_errors::bad_request("Setup key ID cannot be empty"));
    }

    if setup_key.addresses.is_empty() {
        return Err(http_errors::bad_request("At least one P2P address is required"));
    }

    // For now, just log the submission
    tracing::info!("Submitting setup aggregated key: {:?}", setup_key);
    
    Ok(Json(()))
}