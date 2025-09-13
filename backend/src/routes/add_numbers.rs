use std::str::FromStr;

use crate::models::{
    AddNumbersGame, AddNumbersGameStatus, ErrorResponse, FundingUtxoRequest, FundingUtxosResponse,
    MakeGuessRequest, PlaceBetRequest, PlaceBetResponse, SetupParticipantsRequest,
    SetupParticipantsResponse, StartGameRequest, StartGameResponse, Utxo,
};
use crate::state::AppState;
use crate::utils::http_errors;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use bitvmx_client::bitcoin::PublicKey;
use bitvmx_client::bitvmx_wallet::wallet::Destination;
use bitvmx_client::program::participant::P2PAddress as BitVMXP2PAddress;
use bitvmx_client::program::protocols::dispute::{TIMELOCK_BLOCKS, TIMELOCK_BLOCKS_KEY};
use bitvmx_client::program::variables::VariableTypes;
use bitvmx_client::types::PROGRAM_TYPE_DRP;
use tracing::{debug, error};
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    // Base path is /api/add-numbers/
    Router::new()
        .route("/setup-participants", post(setup_participants))
        .route("/place-bet", post(place_bet))
        .route("/fundings_utxos/{id}", get(get_fundings_utxos))
        .route("/setup-funding-utxo", post(setup_funding_utxo)) // for player 2
        .route("/start-game", post(start_game)) // for player 1
        .route("/submit-sum", post(submit_sum))
        .route("/{id}", get(get_game))
        .route("/{id}/guess", post(make_guess))
        .route("/current-game-id", get(get_current_game_id))
}

/// Create a new add numbers game
#[utoipa::path(
    post,
    path = "/api/add-numbers/setup-participants",
    request_body = SetupParticipantsRequest,
    responses(
        (status = 201, description = "Game created successfully", body = SetupParticipantsResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 400, description = "Invalid aggregated id", body = ErrorResponse),
        (status = 400, description = "Invalid participants addresses", body = ErrorResponse),
        (status = 400, description = "Invalid participants keys", body = ErrorResponse),
        (status = 500, description = "Failed to setup game", body = ErrorResponse),
        (status = 500, description = "Failed to create aggregated key", body = ErrorResponse),
    ),
    tag = "AddNumbers"
)]
pub async fn setup_participants(
    State(app_state): State<AppState>,
    Json(request): Json<SetupParticipantsRequest>,
) -> Result<Json<SetupParticipantsResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate the aggregated ID
    if request.aggregated_id.is_empty() {
        return Err(http_errors::bad_request("Aggregated ID cannot be empty"));
    }
    let aggregated_id = Uuid::parse_str(&request.aggregated_id)
        .map_err(|_| http_errors::bad_request("Invalid Aggregated ID"))?;

    let leader_idx = request.leader_idx;

    // Validate the participants addresses
    if request.participants_addresses.is_empty() {
        return Err(http_errors::bad_request(
            "At least one participant address is required",
        ));
    }
    let participants_addresses: Vec<BitVMXP2PAddress> = request
        .participants_addresses
        .iter()
        .map(|p2p| p2p.clone().into())
        .collect();

    // Validate the participants keys
    let participants_keys = request
        .participants_keys
        .iter()
        .map(|key| {
            if key.is_empty() {
                return Err(http_errors::bad_request("Participants key cannot be empty"));
            }
            PublicKey::from_str(key)
                .map_err(|_| http_errors::bad_request("Invalid participants key"))
        })
        .collect::<Result<Vec<PublicKey>, (StatusCode, Json<ErrorResponse>)>>()?;

    // Create the aggregated key
    let aggregated_key = app_state.bitvmx_service
        .create_agregated_key(
            aggregated_id,
            participants_addresses,
            Some(participants_keys),
            leader_idx,
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to create aggregated key: {e:?}"
            ))
        })?;
    debug!("Aggregated key created: {:?}", aggregated_key);

    // Create the program id
    let program_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, request.aggregated_id.as_bytes());
    debug!("🎉 Setting up game with program id: {:?}", program_id);

    // Setup the game
    app_state.add_numbers_service
        .setup_game(
            program_id,
            aggregated_id,
            request.participants_addresses,
            request.participants_keys,
            aggregated_key,
            request.role,
        )
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to setup game: {e:?}"))
        })?;

    Ok(Json(SetupParticipantsResponse {
        program_id: program_id.to_string(),
    }))
}

/// Get a specific add numbers game by ID
#[utoipa::path(
    get,
    path = "/api/add-numbers/{id}",
    params(
        ("id" = String, Path, description = "Game ID")
    ),
    responses(
        (status = 200, description = "Game found"),
        (status = 404, description = "Game not found", body = ErrorResponse),
        (status = 500, description = "Failed to get game", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn get_game(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AddNumbersGame>, (StatusCode, Json<ErrorResponse>)> {
    let game = app_state.add_numbers_service
        .get_game(id)
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to get game: {e:?}"))
        })?
        .ok_or(http_errors::not_found("Game not found"))?;

    Ok(Json(game.clone()))
}

/// Make a guess for the sum
#[utoipa::path(
    post,
    path = "/api/add-numbers/{id}/guess",
    params(
        ("id" = String, Path, description = "Game ID")
    ),
    request_body = MakeGuessRequest,
    responses(
        (status = 200, description = "Guess made successfully"),
        (status = 400, description = "Invalid operation", body = ErrorResponse),
        (status = 404, description = "Game not found", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn make_guess(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<MakeGuessRequest>,
) -> Result<Json<AddNumbersGame>, (StatusCode, Json<ErrorResponse>)> {
    let game = app_state.add_numbers_service
        .make_guess(id, request.guess)
        .map_err(|error| {
            http_errors::error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_OPERATION",
                &error.to_string(),
            )
        })?;

    Ok(Json(game))
}

#[utoipa::path( get,
    path = "/api/add-numbers/current-game-id",
    responses(
        (status = 200, description = "Current game ID", body = String),
        (status = 500, description = "Failed to get current game ID", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn get_current_game_id(
    State(app_state): State<AppState>,
) -> Result<Json<Option<AddNumbersGame>>, (StatusCode, Json<ErrorResponse>)> {
    let game = app_state.add_numbers_service.get_current_game_id()
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to get current game ID: {e:?}"))
        })?;

    Ok(Json(game))
}

#[utoipa::path(
    post,
    path = "/api/add-numbers/place-bet",
    request_body = PlaceBetRequest,
    responses(
        (status = 200, description = "Place bet successfully", body = PlaceBetResponse),
        (status = 400, description = "Invalid program ID", body = ErrorResponse),
        (status = 400, description = "Amount cannot be 0", body = ErrorResponse),
        (status = 404, description = "Game not found", body = ErrorResponse),
        (status = 500, description = "Failed to send protocol funds", body = ErrorResponse),
        (status = 500, description = "Failed to send bet funds", body = ErrorResponse),
        (status = 500, description = "Failed to obtain protocol destination from aggregated key", body = ErrorResponse),
        (status = 500, description = "Failed to save my funding UTXO", body = ErrorResponse),
        (status = 500, description = "Failed to update game state", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn place_bet(
    State(app_state): State<AppState>,
    Json(request): Json<PlaceBetRequest>,
) -> Result<Json<PlaceBetResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate the program ID
    if request.program_id == Uuid::default() {
        return Err(http_errors::bad_request("Program ID cannot be empty"));
    }
    let program_id = request.program_id;

    // Validate the amount
    if request.amount == 0 {
        return Err(http_errors::bad_request("Amount cannot be 0"));
    }

    // Get the game
    let game = app_state.add_numbers_service
        .get_game(program_id)
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to get game: {e:?}"))
        })?
        .ok_or(http_errors::not_found("Game not found"))?;

    // Get the aggregated key
    let aggregated_key = game.bitvmx_program_properties.aggregated_key;

    // Get the protocol fees amount
    let protocol_amount = app_state.bitvmx_service.protocol_cost();
    // Preparer the utxo destination for the protocol fees
    let protocol_destination = app_state.add_numbers_service
        .protocol_destination(&aggregated_key, protocol_amount)
        .map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to obtain protocol destination from aggregated key: {e:?}"
            ))
        })?;

    // Prepare the utxo destination for the bet
    let bet_destination = app_state.add_numbers_service
        .protocol_destination(&aggregated_key, request.amount)
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to obtain bet destination from aggregated key: {e:?}"))
        })?;


    // Send funds to cover protocol fees to the aggregated key
    let (funding_uuid, funding_txid) = app_state.bitvmx_service
        .send_funds(&Destination::Batch(vec![protocol_destination, bet_destination]))
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to send protocol funds: {e:?}"))
        })?;
    debug!(
        "Sent {protocol_amount} satoshis to cover protocol fees and bet {} satoshis to the aggregated key txid: {:?} uuid: {:?}",
        request.amount, funding_txid, funding_uuid
    );


    // Wait for the Transaction Status responses
    debug!("Waiting for transaction status responses");
    let funding_tx_status = app_state.bitvmx_service.wait_for_transaction_response(funding_uuid).await.map_err(|e| {
        http_errors::internal_server_error(&format!("Failed to wait for transaction status response: {e:?}"))
    })?;

    debug!(
        "Received transaction status responses for correlation ids: {:?} and {:?}",
        funding_uuid, funding_txid
    );

    if funding_tx_status.confirmations == 0 {
        error!("Transaction {} not confirmed for correlation id: {:?}", funding_txid, funding_uuid);
        return Err(http_errors::internal_server_error("Transaction not confirmed"));
    }

    debug!("Protocol and bet transactions confirmed, marking funding UTXOs as mined");
    let funding_protocol_utxo: Utxo = Utxo {
        txid: funding_txid.to_string(),
        vout: 0,
        amount: protocol_amount,
        output_type: serde_json::Value::Null,
    };
    let funding_bet_utxo: Utxo = Utxo {
        txid: funding_txid.to_string(),
        vout: 1,
        amount: request.amount,
        output_type: serde_json::Value::Null,
    };

    // Save the funding UTXOs in AddNumbersService
    app_state.add_numbers_service
        .save_my_funding_utxos(
            program_id,
            funding_protocol_utxo.clone(),
            funding_bet_utxo.clone(),
        )
        .map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to save my funding UTXO: {e:?}"
            ))
        })?;
    debug!("Saved my funding UTXOs in AddNumbersService");

    
    Ok(Json(PlaceBetResponse {
        funding_protocol_utxo,
        funding_bet_utxo,
    }))
}

#[utoipa::path(
    get,
    path = "/api/add-numbers/fundings_utxos/{id}",
    responses(
        (status = 200, description = "Protocol fees and bet UTXO", body = FundingUtxosResponse),
        (status = 404, description = "Game not found", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn get_fundings_utxos(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<FundingUtxosResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Get the game
    let game = app_state.add_numbers_service
        .get_game(id)
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to get game: {e:?}"))
        })?
        .ok_or(http_errors::not_found("Game not found"))?;

    let funding_protocol_utxo = game.bitvmx_program_properties.funding_protocol_utxo.clone();
    let funding_bet_utxo = game.bitvmx_program_properties.funding_bet_utxo.clone();

    Ok(Json(FundingUtxosResponse {
        funding_protocol_utxo,
        funding_bet_utxo,
    }))
}

#[utoipa::path(
    post,
    path = "/api/add-numbers/setup-funding-utxo",
    request_body = FundingUtxoRequest,
    responses(
        (status = 200, description = "Funding UTXO setup successfully", body = FundingUtxosResponse),
        (status = 400, description = "Invalid UTXO", body = ErrorResponse),
        (status = 404, description = "Game not found", body = ErrorResponse),
        (status = 500, description = "Failed to add funding UTXO", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn setup_funding_utxo(
    State(app_state): State<AppState>,
    Json(request): Json<FundingUtxoRequest>,
) -> Result<Json<FundingUtxosResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate the protocol fees UTXO
    if request.funding_protocol_utxo.txid.is_empty() || request.funding_protocol_utxo.amount == 0 {
        return Err(http_errors::bad_request("Invalid UTXO"));
    }
    let funding_protocol_utxo = request.funding_protocol_utxo;

    // Validate the bet UTXO
    if request.funding_bet_utxo.txid.is_empty() || request.funding_bet_utxo.amount == 0 {
        return Err(http_errors::bad_request("Invalid UTXO"));
    }
    let funding_bet_utxo = request.funding_bet_utxo;

    // Validate the program ID
    let program_id = Uuid::parse_str(&request.program_id)
        .map_err(|_| http_errors::bad_request("Invalid program ID"))?;

    {
        app_state.add_numbers_service
            .save_other_funding_utxos(
                program_id,
                funding_protocol_utxo.clone(),
                funding_bet_utxo.clone(),
            )
            .map_err(|e| {
                http_errors::internal_server_error(&format!("Failed to add funding UTXO: {e:?}"))
            })?;
    }

    Ok(Json(FundingUtxosResponse {
        funding_protocol_utxo: Some(funding_protocol_utxo),
        funding_bet_utxo: Some(funding_bet_utxo),
    }))
}

#[utoipa::path(
    post,
    path = "/api/add-numbers/start-game",
    request_body = StartGameRequest,
    responses(
        (status = 200, description = "Game started successfully", body = StartGameResponse),
        (status = 400, description = "Invalid program ID", body = ErrorResponse),
        (status = 404, description = "Game not found", body = ErrorResponse),
        (status = 500, description = "Failed to set variable aggregated pubkey", body = ErrorResponse),
        (status = 500, description = "Failed to set variable protocol utxo", body = ErrorResponse),
        (status = 500, description = "Failed to set variable bet utxo", body = ErrorResponse),
        (status = 500, description = "Failed to set variable program definition", body = ErrorResponse),
        (status = 500, description = "Failed to set variable timelock blocks", body = ErrorResponse),
        (status = 500, description = "Failed to set variable program setup", body = ErrorResponse),
        (status = 500, description = "Failed to save start game state", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn start_game(
    State(app_state): State<AppState>,
    Json(request): Json<StartGameRequest>,
) -> Result<Json<StartGameResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate the program ID
    if request.program_id == Uuid::default() {
        return Err(http_errors::bad_request("Invalid program ID"));
    }
    let program_id = request.program_id;

    // Get the game
    let game = app_state.add_numbers_service
            .get_game(program_id)
            .map_err(|e| {
                http_errors::internal_server_error(&format!("Failed to get game: {e:?}"))
            })?
            .ok_or(http_errors::not_found("Game not found"))?
            .clone();

    // Set inputs values, Concatenate the two input numbers as bytes
    let mut concatenated_bytes = Vec::<u8>::new();
    concatenated_bytes.extend_from_slice(&request.number1.to_be_bytes());
    concatenated_bytes.extend_from_slice(&request.number2.to_be_bytes());

    // Set all necesary program variables in BitVMX

    // Set program input 0, the two numbers to sum
    app_state.bitvmx_service
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
    app_state.bitvmx_service
        .set_variable(
            program_id,
            "aggregated",
            VariableTypes::PubKey(game.bitvmx_program_properties.aggregated_key),
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to set variable aggregated pubkey: {e:?}"
            ))
        })?;

    // Set protocol cost utxo
    let protocol_utxo = game.bitvmx_program_properties.funding_protocol_utxo.ok_or(
        http_errors::internal_server_error("Protocol UTXO not found"),
    )?;
    app_state.bitvmx_service
        .set_variable(
            program_id,
            "utxo",
            VariableTypes::Utxo(protocol_utxo.into()),
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to set variable protocol utxo: {e:?}"
            ))
        })?;

    // Set bet utxo
    let bet_utxo = game
        .bitvmx_program_properties
        .funding_bet_utxo
        .ok_or(http_errors::internal_server_error("Bet UTXO not found"))?;
    
    app_state.bitvmx_service
        .set_variable(
            program_id,
            "utxo_prover_win_action",
            VariableTypes::Utxo(bet_utxo.into()),
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to set variable bet utxo: {e:?}"
            ))
        })?;

    // Set program definition, it should be the relative path from the bitvmx-client to the program definition file
    let program_path = "./verifiers/add-test-with-const-pre.yaml";
    app_state.bitvmx_service
        .set_variable(
            program_id,
            "program_definition",
            VariableTypes::String(program_path.to_string()),
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to set variable program definition: {e:?}"
            ))
        })?;

    // Set timelock blocks
    app_state.bitvmx_service
        .set_variable(
            program_id,
            TIMELOCK_BLOCKS_KEY,
            VariableTypes::Number(TIMELOCK_BLOCKS.into()),
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to set variable timelock blocks: {e:?}"
        ))
    })?;

    // Call setup program
    let participants_addresses: Vec<BitVMXP2PAddress> = game
        .bitvmx_program_properties
        .participants_addresses
        .iter()
        .map(|p2p| p2p.clone().into())
        .collect();

    app_state.bitvmx_service
            .program_setup(program_id, PROGRAM_TYPE_DRP, participants_addresses, 1)
            .await
            .map_err(|e| {
                http_errors::internal_server_error(&format!(
                    "Failed to set variable program setup: {e:?}"
                ))
            })?;

    // Set game as started
    app_state.add_numbers_service
        .start_game(program_id, request.number1, request.number2)
        .map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to save start game state: {e:?}"
            ))
        })?;

    // Return the program ID
    Ok(Json(StartGameResponse { program_id }))
}

#[utoipa::path(
    post,
    path = "/api/add-numbers/submit-sum/{id}",
    request_body = MakeGuessRequest,
    responses(
        (status = 200, description = "Sum submitted successfully", body = AddNumbersGame),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 404, description = "Game not found", body = ErrorResponse),
        (status = 500, description = "Failed to get game", body = ErrorResponse),
        (status = 500, description = "Failed to submit sum", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn submit_sum(
    State(app_state): State<AppState>,
    Json(request): Json<MakeGuessRequest>,
) -> Result<Json<AddNumbersGame>, (StatusCode, Json<ErrorResponse>)> {
    // TOOD: PEDRO Wait until you know the anser
    let game = app_state.add_numbers_service
        .make_guess(request.id, request.guess).map_err(|e| {
            match e.to_string().as_str() {
                "Game not found" => http_errors::not_found("Game not found"),
                "Game is not in waiting for guess state" => {
                    http_errors::bad_request("Invalid game state")
                }
                _ => http_errors::internal_server_error(&format!("Failed to submit sum: {e:?}")),
            }
        })?;

    Ok(Json(game))
}
