use std::str::FromStr;
use axum::{Json, Router, routing::{get, post}, extract::State, extract::Path};
use http::StatusCode;
use crate::models::{AggregatedKey, AggregatedKeySubmission, ErrorResponse, OperatorKeys, P2PAddress};
use crate::state::AppState;
use crate::http_errors;
use tracing::instrument;
use uuid::Uuid;
use bitcoin::PublicKey;

pub fn router() -> Router<AppState> {
    // Base path is /api/bitvmx/
    Router::new()
        .route("/comm-info", get(comm_info))
        .route("/operator-keys", get(operator_keys))
        .route("/aggregated-key", post(submit_aggregated_key))
        .route("/aggregated-key", get(get_aggregated_key))
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

/// Get BitVMX Operator key
#[utoipa::path(
    get,
    path = "/api/bitvmx/operator-keys",
    responses(
        (status = 200, description = "BitVMX Operator key", body = OperatorKeys),
        (status = 404, description = "Operator key not found", body = ErrorResponse),
        (status = 404, description = "Operator funding key not found", body = ErrorResponse)
    ),
    tag = "BitVMX"
)]
#[instrument(skip(app_state))]
pub async fn operator_keys(State(app_state): State<AppState>) -> Result<Json<OperatorKeys>, (StatusCode, Json<ErrorResponse>)> {
    let service_guard = app_state.bitvmx_service.read().await;
    let pub_key = service_guard.get_pub_key().ok_or(http_errors::not_found("Operator pub key not found"))?;
    let funding_key = service_guard.get_funding_key().ok_or(http_errors::not_found("Operator funding key not found"))?;
    Ok(Json(OperatorKeys {
        pub_key: pub_key,
        funding_key: funding_key,
    }))
}

/// Submit BitVMX aggregated key
#[utoipa::path(
    post,
    path = "/api/bitvmx/aggregated-key",
    request_body = AggregatedKeySubmission,
    responses(
        (status = 200, description = "Aggregated key submitted successfully"),
        (status = 400, description = "Invalid aggregated key", body = ErrorResponse),
        (status = 400, description = "Invalid operator key", body = ErrorResponse), 
        (status = 400, description = "Invalid UUID", body = ErrorResponse),
        (status = 500, description = "Failed to create aggregated key", body = ErrorResponse)
    ),
    tag = "BitVMX"
)]
#[instrument(skip(app_state))]
pub async fn submit_aggregated_key(
    State(app_state): State<AppState>,
    Json(aggregated_key_submission): Json<AggregatedKeySubmission>
) -> Result<Json<AggregatedKey>, (StatusCode, Json<ErrorResponse>)> {
    // Validate the id
    if aggregated_key_submission.uuid.is_empty() {
        return Err(http_errors::bad_request("Aggregated key ID cannot be empty"));
    }

    // Validate the p2p addresses
    if aggregated_key_submission.p2p_addresses.is_empty() {
        return Err(http_errors::bad_request("At least one P2P address is required"));
    }

    // Validate the operator keys
    if let Some(operator_keys) = &aggregated_key_submission.operator_keys {
        for operator_key in operator_keys {
            if operator_key.is_empty() {
                return Err(http_errors::bad_request("Operator key cannot be empty"));
            }
        }
    }

    let uuid = Uuid::parse_str(&aggregated_key_submission.uuid).map_err(|_| http_errors::bad_request("Invalid UUID"))?;
    let mut operator_keys = None;
    if let Some(keys) = aggregated_key_submission.operator_keys {
        operator_keys = Some(keys.iter().map(|key| PublicKey::from_str(key).map_err(|_| http_errors::bad_request("Invalid operator key"))).collect::<Result<Vec<PublicKey>, (StatusCode, Json<ErrorResponse>)>>()?);
    }

    let service_guard = app_state.bitvmx_service.read().await;
    let aggregated_key = service_guard.create_agregated_key(uuid, aggregated_key_submission.p2p_addresses, operator_keys, aggregated_key_submission.leader_idx).await.map_err(|e| http_errors::internal_server_error(&format!("Failed to create aggregated key: {:?}", e)))?;
    Ok(Json(aggregated_key))
}

/// Get BitVMX aggregated key
#[utoipa::path(
    get,
    path = "/api/bitvmx/aggregated-key",
    responses(
        (status = 200, description = "Aggregated key", body = AggregatedKey),
        (status = 404, description = "Aggregated key not found", body = ErrorResponse)
    ),
    tag = "BitVMX"
)]
#[instrument(skip(app_state))]
pub async fn get_aggregated_key(State(app_state): State<AppState>, Path(uuid): Path<Uuid>) -> Result<Json<AggregatedKey>, (StatusCode, Json<ErrorResponse>)> {
    let service_guard = app_state.bitvmx_service.read().await;
    let aggregated_key = service_guard.get_aggregated_key(uuid).await.map_err(|e| http_errors::internal_server_error(&format!("Failed to get aggregated key: {:?}", e)))?;
    Ok(Json(aggregated_key))
}