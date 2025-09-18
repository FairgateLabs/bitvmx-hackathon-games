use crate::models::{
    AggregatedKeyResponse, ErrorResponse, OperatorKeys, P2PAddress, ProtocolCostResponse,
    ProtocolVisualizationResponse, TransactionResponse, WalletBalance,
};
use crate::state::AppState;
use crate::utils::http_errors;
use axum::{extract::Path, extract::State, routing::get, Json, Router};
use http::StatusCode;
use tracing::info;
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    // Base path is /api/bitvmx/
    Router::new()
        .route("/comm-info", get(comm_info))
        .route("/operator-keys", get(operator_keys))
        .route("/aggregated-key/{uuid}", get(get_aggregated_key))
        .route("/wallet-balance", get(wallet_balance))
        .route("/transaction/{txid}", get(get_transaction))
        .route("/protocol/cost", get(get_protocol_cost))
        .route(
            "/protocol/visualization/{uuid}",
            get(get_protocol_visualization),
        )
}

/// Get BitVMX P2P address information
#[utoipa::path(
    get,
    path = "/api/bitvmx/comm-info",
    responses(
        (status = 200, description = "BitVMX P2P address information", body = P2PAddress),
        (status = 404, description = "P2P address not found", body = ErrorResponse),
        (status = 500, description = "Failed to get p2p address", body = ErrorResponse),
    ),
    tag = "BitVMX"
)]
pub async fn comm_info(
    State(app_state): State<AppState>,
) -> Result<Json<P2PAddress>, (StatusCode, Json<ErrorResponse>)> {
    let p2p_address = app_state
        .bitvmx_service
        .get_p2p_address()
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to get p2p address: {e:?}"))
        })?
        .ok_or(http_errors::not_found("P2P address not found"))?;
    Ok(Json(p2p_address))
}

/// Get BitVMX Operator key
#[utoipa::path(
    get,
    path = "/api/bitvmx/operator-keys",
    responses(
        (status = 200, description = "BitVMX Operator key", body = OperatorKeys),
        (status = 404, description = "Operator key not found", body = ErrorResponse),
        (status = 404, description = "Operator funding key not found", body = ErrorResponse),
        (status = 500, description = "Failed to get pub key", body = ErrorResponse),
        (status = 500, description = "Failed to get funding key", body = ErrorResponse),
    ),
    tag = "BitVMX"
)]
pub async fn operator_keys(
    State(app_state): State<AppState>,
) -> Result<Json<OperatorKeys>, (StatusCode, Json<ErrorResponse>)> {
    let pub_key = app_state
        .bitvmx_service
        .get_pub_key()
        .await
        .map_err(|e| http_errors::internal_server_error(&format!("Failed to get pub key: {e:?}")))?
        .ok_or(http_errors::not_found("Operator pub key not found"))?;
    let funding_key = app_state
        .bitvmx_service
        .get_funding_key()
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to get funding key: {e:?}"))
        })?
        .ok_or(http_errors::not_found("Operator funding key not found"))?;
    Ok(Json(OperatorKeys {
        pub_key,
        funding_key,
    }))
}

/// Get BitVMX aggregated key
#[utoipa::path(
    get,
    path = "/api/bitvmx/aggregated-key/{uuid}",
    params(
        ("uuid" = String, Path, description = "Aggregated key UUID")
    ),
    responses(
        (status = 200, description = "Aggregated key", body = AggregatedKeyResponse),
        (status = 404, description = "Aggregated key not found", body = ErrorResponse)
    ),
    tag = "BitVMX"
)]
pub async fn get_aggregated_key(
    State(app_state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<AggregatedKeyResponse>, (StatusCode, Json<ErrorResponse>)> {
    let aggregated_key = app_state
        .bitvmx_service
        .aggregated_key(uuid)
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to get aggregated key: {e:?}"))
        })?;
    Ok(Json(AggregatedKeyResponse {
        uuid: uuid.to_string(),
        aggregated_key: aggregated_key.to_string(),
    }))
}

/// Get BitVMX Wallet balance
#[utoipa::path(
    get,
    path = "/api/bitvmx/wallet-balance",
    responses(
        (status = 200, description = "BitVMX Wallet balance", body = WalletBalance),
        (status = 404, description = "Wallet balance not found", body = ErrorResponse)
    ),
    tag = "BitVMX"
)]
pub async fn wallet_balance(
    State(app_state): State<AppState>,
) -> Result<Json<WalletBalance>, (StatusCode, Json<ErrorResponse>)> {
    let wallet_balance = app_state
        .bitvmx_service
        .wallet_balance()
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to get wallet balance: {e:?}"))
        })?;
    Ok(Json(wallet_balance))
}

/// Get Bitcoin transaction dispatched by BitVMX
#[utoipa::path(
    get,
    path = "/api/bitvmx/transaction/{txid}",
    responses(
        (status = 200, description = "Transaction", body = TransactionResponse),
        (status = 404, description = "Transaction not found", body = ErrorResponse)
    ),
    tag = "BitVMX"
)]
pub async fn get_transaction(
    State(app_state): State<AppState>,
    Path(txid): Path<String>,
) -> Result<Json<TransactionResponse>, (StatusCode, Json<ErrorResponse>)> {
    let transaction = app_state
        .bitvmx_service
        .get_transaction(txid)
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to get transaction: {e:?}"))
        })?;
    let mut block_height = 0;
    let mut block_hash = String::new();
    if let Some(block_info) = transaction.block_info {
        block_height = block_info.height;
        block_hash = block_info.hash.to_string();
    }

    Ok(Json(TransactionResponse {
        txid: transaction.tx_id.to_string(),
        status: format!("{:?}", transaction.status),
        confirmations: transaction.confirmations,
        block_height,
        block_hash,
    }))
}

/// Get Bitcoin transaction dispatched by BitVMX
#[utoipa::path(
    get,
    path = "/api/bitvmx/protocol/cost",
    responses(
        (status = 200, description = "Protocol cost", body = ProtocolCostResponse),
    ),
    tag = "BitVMX"
)]
pub async fn get_protocol_cost(
    State(app_state): State<AppState>,
) -> Result<Json<ProtocolCostResponse>, (StatusCode, Json<ErrorResponse>)> {
    let protocol_cost = app_state.bitvmx_service.protocol_cost();

    Ok(Json(ProtocolCostResponse { protocol_cost }))
}

/// Get BitVMX protocol visualization
#[utoipa::path(
    get,
    path = "/api/bitvmx/protocol/visualization/{uuid}",
    params(
        ("uuid" = String, Path, description = "Aggregated key UUID")
    ),
    responses(
        (status = 200, description = "Protocol visualization", body = String),
        (status = 404, description = "Protocol visualization not found", body = ErrorResponse),
        (status = 500, description = "Failed to get protocol visualization", body = ErrorResponse)
    ),
    tag = "BitVMX"
)]
pub async fn get_protocol_visualization(
    State(app_state): State<AppState>,
    Path(program_id): Path<Uuid>,
) -> Result<Json<ProtocolVisualizationResponse>, (StatusCode, Json<ErrorResponse>)> {
    let visualization = app_state
        .bitvmx_service
        .get_protocol_visualization(program_id)
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to get protocol visualization: {e:?}"
            ))
        })?;

    let response = ProtocolVisualizationResponse {
        visualization: visualization.clone(),
    };

    info!("HOLAAAAAAAAAAAAAA: {:?}", response);
    Ok(Json(response))
}
