use crate::models::{
    AggregatedKeyRequest, AggregatedKeyResponse, ErrorResponse, OperatorKeys, P2PAddress,
    SendFundsRequest, SendFundsResponse, TransactionResponse, WalletBalance,
};
use crate::state::AppState;
use crate::utils::{bitcoin, http_errors};
use axum::{
    extract::Path,
    extract::State,
    routing::{get, post},
    Json, Router,
};
use bitvmx_client::bitcoin::{Amount, PublicKey};
use bitvmx_client::program::participant::P2PAddress as BitVMXP2PAddress;
use bitvmx_client::p2p_handler::PeerId;
use bitvmx_client::program::protocols::dispute::{TIMELOCK_BLOCKS, TIMELOCK_BLOCKS_KEY};
use bitvmx_client::program::variables::VariableTypes;
use bitvmx_client::protocol_builder::scripts;
use bitvmx_client::types::PROGRAM_TYPE_DRP;
use http::StatusCode;
use std::str::FromStr;
use tracing::{debug, instrument};
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    // Base path is /api/bitvmx/
    Router::new()
        .route("/comm-info", get(comm_info))
        .route("/operator-keys", get(operator_keys))
        .route("/aggregated-key", post(submit_aggregated_key))
        .route("/aggregated-key/{uuid}", get(get_aggregated_key))
        .route("/wallet-balance", get(wallet_balance))
        .route("/send-funds", post(send_funds))
        .route("/transaction/{txid}", get(get_transaction))
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
#[instrument(skip(app_state))]
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

/// Submit BitVMX aggregated key
#[utoipa::path(
    post,
    path = "/api/bitvmx/aggregated-key",
    request_body = AggregatedKeyRequest,
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
    Json(aggregated_key_request): Json<AggregatedKeyRequest>,
) -> Result<Json<(AggregatedKeyResponse, String)>, (StatusCode, Json<ErrorResponse>)> {
    // Validate the id
    if aggregated_key_request.uuid.is_empty() {
        return Err(http_errors::bad_request(
            "Aggregated key ID cannot be empty",
        ));
    }

    // Validate the p2p addresses
    if aggregated_key_request.p2p_addresses.is_empty() {
        return Err(http_errors::bad_request(
            "At least one P2P address is required",
        ));
    }

    // Validate the operator keys
    if let Some(operator_keys) = &aggregated_key_request.operator_keys {
        for operator_key in operator_keys {
            if operator_key.is_empty() {
                return Err(http_errors::bad_request("Operator key cannot be empty"));
            }
        }
    }

    let uuid = Uuid::parse_str(&aggregated_key_request.uuid)
        .map_err(|_| http_errors::bad_request("Invalid UUID"))?;
    let mut participants_keys = None;
    if let Some(keys) = aggregated_key_request.operator_keys {
        participants_keys = Some(
            keys.iter()
                .map(|key| {
                    PublicKey::from_str(key)
                        .map_err(|_| http_errors::bad_request("Invalid operator key"))
                })
                .collect::<Result<Vec<PublicKey>, (StatusCode, Json<ErrorResponse>)>>()?,
        );
    }

    let participants: Vec<BitVMXP2PAddress> = aggregated_key_request.p2p_addresses
        .iter()
        .map(|p2p| BitVMXP2PAddress {
            address: p2p.address.clone(),
            peer_id: PeerId(p2p.peer_id.clone()),
        })
        .collect();

    // Create aggregated key
    let service_guard = app_state.bitvmx_service.read().await;
    let aggregated_key = service_guard
        .create_agregated_key(
            uuid,
            participants.clone(),
            participants_keys,
            aggregated_key_request.leader_idx,
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to create aggregated key: {e:?}"))
        })?;

    let aggregated_key_response = AggregatedKeyResponse {
        uuid: uuid.to_string(),
        aggregated_key: aggregated_key.to_string(),
    };
    // TODO this should go in a separated method in the future
    let x_only_pubkey = bitcoin::pub_key_to_xonly(&aggregated_key).map_err(|e| {
        http_errors::internal_server_error(&format!(
            "Failed to convert aggregated key to x only pubkey: {e:?}"
        ))
    })?;
    // Todo check if this tap leaves are correct
    let tap_leaves = vec![
        scripts::check_aggregated_signature(&aggregated_key, scripts::SignMode::Aggregate),
        scripts::check_aggregated_signature(&aggregated_key, scripts::SignMode::Aggregate),
    ];
    let p2tr_address = bitcoin::pub_key_to_p2tr(&x_only_pubkey, &tap_leaves).map_err(|e| {
        http_errors::internal_server_error(&format!(
            "Failed to convert aggregated key to p2tr address: {e:?}"
        ))
    })?;
    debug!(
        "Aggregated key created: {:?} taproot address: {:?}",
        aggregated_key, p2tr_address
    );

    // Send funds to cover protocol fees to the aggregated key
    let amount = Amount::from_btc(22_000.0).map_err(|e| {
        http_errors::internal_server_error(&format!("Failed to convert amount: {e:?}"))
    })?;
    let initial_utxo = service_guard
        .send_funds(p2tr_address.to_string(), amount.to_sat(), None)
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to send funds: {e:?}"))
        })?;
    debug!("Funds {amount} satoshis sent to cover protocol fees to the aggregated key txid: {:?}", initial_utxo.0);

    // Send the amount that the players will bet to the aggregated key
    let amount = Amount::from_btc(1.0).map_err(|e| {
        http_errors::internal_server_error(&format!("Failed to convert amount: {e:?}"))
    })?;
    let prover_win_utxo = service_guard
        .send_funds(p2tr_address.to_string(), amount.to_sat(), None)
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to send funds: {e:?}"))
        })?;
    debug!("Funds {amount} satoshis sent to the aggregated key to cover the players bet txid: {:?}", prover_win_utxo.0);

    Ok(Json((
        aggregated_key_response,
        p2tr_address.to_string(),
        txid.to_string(),
    )))
}

/// Get BitVMX aggregated key
#[utoipa::path(
    get,
    path = "/api/bitvmx/aggregated-key/{uuid}",
    responses(
        (status = 200, description = "Aggregated key", body = AggregatedKeyResponse),
        (status = 404, description = "Aggregated key not found", body = ErrorResponse)
    ),
    tag = "BitVMX"
)]
#[instrument(skip(app_state))]
pub async fn get_aggregated_key(
    State(app_state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<AggregatedKeyResponse>, (StatusCode, Json<ErrorResponse>)> {
    let service_guard = app_state.bitvmx_service.read().await;
    let aggregated_key = service_guard.aggregated_key(uuid).await.map_err(|e| {
        http_errors::internal_server_error(&format!("Failed to get aggregated key: {e:?}"))
    })?;
    Ok(Json(aggregated_key))
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
#[instrument(skip(app_state))]
pub async fn wallet_balance(
    State(app_state): State<AppState>,
) -> Result<Json<WalletBalance>, (StatusCode, Json<ErrorResponse>)> {
    let service_guard = app_state.bitvmx_service.read().await;
    let wallet_balance = service_guard.wallet_balance().await.map_err(|e| {
        http_errors::internal_server_error(&format!("Failed to get wallet balance: {e:?}"))
    })?;
    Ok(Json(wallet_balance))
}

/// Send funds to a destination
#[utoipa::path(
    post,
    path = "/api/bitvmx/send-funds",
    request_body = SendFundsRequest,
    responses(
        (status = 200, description = "Funds sent successfully", body = SendFundsResponse),
        (status = 400, description = "Invalid destination", body = ErrorResponse),
        (status = 400, description = "Invalid amount", body = ErrorResponse),
        (status = 500, description = "Failed to send funds", body = ErrorResponse)
    ),
    tag = "BitVMX"
)]
#[instrument(skip(app_state))]
pub async fn send_funds(
    State(app_state): State<AppState>,
    Json(send_funds_request): Json<SendFundsRequest>,
) -> Result<Json<SendFundsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let service_guard = app_state.bitvmx_service.read().await;
    let txid = service_guard
        .send_funds(
            send_funds_request.destination,
            send_funds_request.amount,
            send_funds_request.scripts,
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to send funds: {e:?}"))
        })?;
    Ok(Json(SendFundsResponse { txid: txid.0.to_string() }))
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
#[instrument(skip(app_state))]
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
