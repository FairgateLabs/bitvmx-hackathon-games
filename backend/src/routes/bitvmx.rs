use crate::models::{
    AggregatedKeyResponse, ErrorResponse, OperatorKeys, P2PAddress, ProtocolCostResponse,
    TransactionResponse, WalletBalance,
};
use crate::state::AppState;
use crate::utils::http_errors;
use axum::{extract::Path, extract::State, routing::get, Json, Router};
use http::StatusCode;
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    // Base path is /api/bitvmx/
    Router::new()
        .route("/comm-info", get(comm_info))
        .route("/operator-keys", get(operator_keys))
        .route("/aggregated-key/{uuid}", get(get_aggregated_key))
        .route("/wallet-balance", get(wallet_balance))
        .route("/transaction/{txid}", get(get_transaction))
        .route("/protocol-cost", get(get_protocol_cost))
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
pub async fn comm_info(
    State(app_state): State<AppState>,
) -> Result<Json<P2PAddress>, (StatusCode, Json<ErrorResponse>)> {
    let service_guard = app_state.bitvmx_service.read().await;
    let p2p_address = service_guard
        .get_p2p_address()
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
        (status = 404, description = "Operator funding key not found", body = ErrorResponse)
    ),
    tag = "BitVMX"
)]
pub async fn operator_keys(
    State(app_state): State<AppState>,
) -> Result<Json<OperatorKeys>, (StatusCode, Json<ErrorResponse>)> {
    let service_guard = app_state.bitvmx_service.read().await;
    let pub_key = service_guard
        .get_pub_key()
        .ok_or(http_errors::not_found("Operator pub key not found"))?;
    let funding_key = service_guard
        .get_funding_key()
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
    let service_guard = app_state.bitvmx_service.read().await;
    let aggregated_key = service_guard.aggregated_key(uuid).await.map_err(|e| {
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
    let service_guard = app_state.bitvmx_service.read().await;
    let wallet_balance = service_guard.wallet_balance().await.map_err(|e| {
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
    let service_guard = app_state.bitvmx_service.read().await;
    let transaction = service_guard.get_transaction(txid).await.map_err(|e| {
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
    path = "/api/bitvmx/protocol-cost",
    responses(
        (status = 200, description = "Protocol cost", body = ProtocolCostResponse),
    ),
    tag = "BitVMX"
)]
pub async fn get_protocol_cost(
    State(app_state): State<AppState>,
) -> Result<Json<ProtocolCostResponse>, (StatusCode, Json<ErrorResponse>)> {
    let service_guard = app_state.bitvmx_service.read().await;
    let protocol_cost = service_guard.protocol_cost();

    Ok(Json(ProtocolCostResponse { protocol_cost }))
}
