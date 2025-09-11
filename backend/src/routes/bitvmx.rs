use crate::models::{
    AggregatedKeyResponse, ErrorResponse, OperatorKeys, P2PAddress, ProgramSetupRequest,
    ProgramSetupResponse, ProtocolCostResponse, SetupParticipantsRequest,
    TransactionResponse, WalletBalance,
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
use tracing::instrument;
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
#[instrument(skip(app_state))]
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

    // Set variables in BitVMX
    let service_guard = app_state.bitvmx_service.read().await;
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

    service_guard
        .set_variable(program_id, "utxo", VariableTypes::Utxo(initial_utxo.into()))
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to set variable: {e:?}"))
        })?;

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
#[instrument(skip(app_state))]
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

/// Get Bitcoin transaction dispatched by BitVMX
#[utoipa::path(
    get,
    path = "/api/bitvmx/protocol-cost",
    responses(
        (status = 200, description = "Protocol cost", body = ProtocolCostResponse),
    ),
    tag = "BitVMX"
)]
#[instrument(skip(app_state))]
pub async fn get_protocol_cost(
    State(app_state): State<AppState>,
) -> Result<Json<ProtocolCostResponse>, (StatusCode, Json<ErrorResponse>)> {
    let service_guard = app_state.bitvmx_service.read().await;
    let protocol_cost = service_guard.protocol_cost();

    Ok(Json(ProtocolCostResponse { protocol_cost }))
}

// /// Submit aggregated key request asynchronously - non-blocking version
// /// This endpoint immediately returns a success response and processes the request in the background
// #[utoipa::path(
//     post,
//     path = "/api/bitvmx/aggregated-key-async",
//     request_body = SetupParticipantsRequest,
//     responses(
//         (status = 202, description = "Request accepted and will be processed asynchronously"),
//         (status = 400, description = "Bad request", body = ErrorResponse)
//     ),
//     tag = "BitVMX"
// )]
// #[instrument(skip(app_state))]
// pub async fn submit_aggregated_key_async(
//     State(app_state): State<AppState>,
//     Json(aggregated_key_request): Json<SetupParticipantsRequest>,
// ) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
//     // Validate the id
//     if aggregated_key_request.agregated_id.is_empty() {
//         return Err(http_errors::bad_request(
//             "Aggregated key ID cannot be empty",
//         ));
//     }

//     // Validate the p2p addresses
//     if aggregated_key_request.participants_addresses.is_empty() {
//         return Err(http_errors::bad_request(
//             "At least one P2P address is required",
//         ));
//     }

//     // Validate the operator keys
//     if let Some(operator_keys) = &aggregated_key_request.participants_keys {
//         for operator_key in operator_keys {
//             if operator_key.is_empty() {
//                 return Err(http_errors::bad_request("Operator key cannot be empty"));
//             }
//         }
//     }

//     let uuid = Uuid::parse_str(&aggregated_key_request.agregated_id)
//         .map_err(|_| http_errors::bad_request("Invalid UUID"))?;
//     let mut participants_keys = None;
//     if let Some(keys) = aggregated_key_request.participants_keys {
//         participants_keys = Some(
//             keys.iter()
//                 .map(|key| {
//                     PublicKey::from_str(key)
//                         .map_err(|_| http_errors::bad_request("Invalid operator key"))
//                 })
//                 .collect::<Result<Vec<PublicKey>, (StatusCode, Json<ErrorResponse>)>>()?,
//         );
//     }

//     let participants: Vec<BitVMXP2PAddress> = aggregated_key_request
//         .participants_addresses
//         .iter()
//         .map(|p2p| BitVMXP2PAddress {
//             address: p2p.address.clone(),
//             peer_id: PeerId(p2p.peer_id.clone()),
//         })
//         .collect();

//     // Send the request asynchronously with a callback
//     let service_guard = app_state.bitvmx_service.read().await;
//     service_guard
//         .create_agregated_key_with_callback(
//             uuid,
//             participants,
//             participants_keys,
//             aggregated_key_request.leader_idx,
//             move |result| async move {
//                 // Business logic for aggregated key creation result:
//                 // - Store the result in a database
//                 // - Send a notification to the client via WebSocket
//                 // - Update some internal state
//                 // - Trigger follow-up actions
//                 // - Send email/SMS notifications
//                 // - Update cache
//                 // - Log to external monitoring systems
//                 // - Send notifications to administrators
//                 // - Update error state in database
//                 // - Trigger retry mechanisms
//                 // - Send user notifications
//                 // - etc.

//                 // Handle the result - any errors in this callback will be caught by map_err
//                 if let Ok(_aggregated_key) = result {
//                     // Handle successful aggregated key creation
//                     // Example: store_in_database(_aggregated_key).await?;
//                     // Example: send_websocket_notification(_aggregated_key).await?;
//                 }
//                 // If result is Err, we just ignore it since it's already logged at service level
//             },
//         )
//         .await
//         .map_err(|e| {
//             http_errors::internal_server_error(&format!(
//                 "Failed to submit aggregated key request: {e:?}"
//             ))
//         })?;

//     // Return immediately with a 202 Accepted status
//     Ok(Json(serde_json::json!({
//         "status": "accepted",
//         "message": "Request submitted successfully and will be processed asynchronously",
//         "uuid": uuid.to_string()
//     })))
// }
