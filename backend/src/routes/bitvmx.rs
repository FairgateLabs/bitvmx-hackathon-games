use crate::models::{
    AggregatedKeyResponse, ErrorResponse, OperatorKeys, P2PAddress, ProgramSetupRequest,
    ProgramSetupResponse, ProtocolCostResponse, SetupParticipantsRequest, TransactionResponse,
    WalletBalance,
};
use crate::state::AppState;
use crate::utils::http_errors;
use axum::{
    extract::Path,
    extract::State,
    routing::{get, post},
    Json, Router,
};
use bitvmx_client::bitcoin::PublicKey;
use bitvmx_client::p2p_handler::PeerId;
use bitvmx_client::program::participant::P2PAddress as BitVMXP2PAddress;
use bitvmx_client::program::protocols::dispute::{TIMELOCK_BLOCKS, TIMELOCK_BLOCKS_KEY};
use bitvmx_client::program::variables::VariableTypes;
use bitvmx_client::types::PROGRAM_TYPE_DRP;
use http::StatusCode;
use std::str::FromStr;
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    // Base path is /api/bitvmx/
    Router::new()
        .route("/comm-info", get(comm_info))
        .route("/operator-keys", get(operator_keys))
        .route("/aggregated-key/{uuid}", get(get_aggregated_key))
        .route("/wallet-balance", get(wallet_balance))
        .route("/transaction/{txid}", get(get_transaction))
        .route("/program-setup", post(program_setup))
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

/// Submit BitVMX aggregated key
#[utoipa::path(
    post,
    path = "/api/bitvmx/program-setup",
    request_body = SetupParticipantsRequest,
    responses(
        (status = 200, description = "Program setup successfully"),
        (status = 400, description = "Invalid program id", body = ErrorResponse),
        (status = 400, description = "Invalid participants", body = ErrorResponse), 
        (status = 400, description = "Invalid aggregated key", body = ErrorResponse),
        (status = 400, description = "Invalid initial utxo", body = ErrorResponse),
        (status = 400, description = "Invalid prover win utxo", body = ErrorResponse),
        (status = 500, description = "Failed to setup program", body = ErrorResponse)
    ),
    tag = "BitVMX"
)]
pub async fn program_setup(
    State(app_state): State<AppState>,
    Json(program_setup_request): Json<ProgramSetupRequest>,
) -> Result<Json<ProgramSetupResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate the id
    if program_setup_request.program_id.is_empty() {
        return Err(http_errors::bad_request("Program ID cannot be empty"));
    }
    let program_id = Uuid::parse_str(&program_setup_request.program_id)
        .map_err(|_| http_errors::bad_request("Invalid program_id"))?;

    // Validate the participants
    let participants: Vec<BitVMXP2PAddress> = program_setup_request
        .participants
        .iter()
        .map(|p2p| BitVMXP2PAddress {
            address: p2p.address.clone(),
            peer_id: PeerId(p2p.peer_id.clone()),
        })
        .collect();

    // Validate the aggregated key
    let aggregated_key =
        PublicKey::from_str(&program_setup_request.aggregated_key).map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to convert aggregated key to public key: {e:?}"
            ))
        })?;

    // Validate the initial utxo
    let initial_utxo = program_setup_request.initial_utxo;

    // Validate the prover win utxo
    let prover_win_utxo = program_setup_request.prover_win_utxo;

    // Set inputs values
    let first_number: u32 = 1;
    let second_number: u32 = 2;

    // Concatenate the two input numbers as bytes
    let mut concatenated_bytes = Vec::<u8>::new();
    concatenated_bytes.extend_from_slice(&first_number.to_be_bytes());
    concatenated_bytes.extend_from_slice(&second_number.to_be_bytes());

    // Set variables in BitVMX
    let service_guard = app_state.bitvmx_service.read().await;
    service_guard
        .set_variable(
            program_id,
            "program_input_0",
            VariableTypes::Input(concatenated_bytes.clone()),
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to set variable: {e:?}"))
        })?;

    // Set aggregated key
    service_guard
        .set_variable(
            program_id,
            "aggregated",
            VariableTypes::PubKey(aggregated_key),
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to set variable: {e:?}"))
        })?;

    // Set protocol cost utxo
    service_guard
        .set_variable(program_id, "utxo", VariableTypes::Utxo(initial_utxo.into()))
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to set variable: {e:?}"))
        })?;

    // Set bet utxo
    service_guard
        .set_variable(
            program_id,
            "utxo_prover_win_action",
            VariableTypes::Utxo(prover_win_utxo.into()),
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to set variable: {e:?}"))
        })?;

    // Set program definition, it should be the relative path from the bitvmx-client to the program definition file
    let program_path = "./verifiers/add-test-with-const-pre.yaml";
    service_guard
        .set_variable(
            program_id,
            "program_definition",
            VariableTypes::String(program_path.to_string()),
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to set variable: {e:?}"))
        })?;

    // Set timelock blocks
    service_guard
        .set_variable(
            program_id,
            TIMELOCK_BLOCKS_KEY,
            VariableTypes::Number(TIMELOCK_BLOCKS.into()),
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to set variable: {e:?}"))
        })?;

    // Call setup
    service_guard
        .program_setup(program_id, PROGRAM_TYPE_DRP, participants, 1)
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to set variable: {e:?}"))
        })?;

    // Return the program ID
    Ok(Json(ProgramSetupResponse {
        program_id: program_id.to_string(),
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
