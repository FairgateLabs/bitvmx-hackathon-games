use std::str::FromStr;

use crate::models::{
    AddNumbersGame, AddNumbersGameStatus, ErrorResponse, FundingUtxoRequest, FundingUtxosResponse,
    PlaceBetRequest, PlaceBetResponse, PlayerRole, SetupGameRequest, SetupGameResponse,
    SetupParticipantsRequest, SetupParticipantsResponse, StartGameRequest, StartGameResponse,
    SubmitSumRequest, SubmitSumResponse, Utxo,
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
use bitvmx_client::program::protocols::dispute::{
    ACTION_PROVER_WINS, TIMELOCK_BLOCKS, TIMELOCK_BLOCKS_KEY,
};
use bitvmx_client::program::variables::VariableTypes;
use bitvmx_client::protocol_builder::types::OutputType;
use bitvmx_client::types::PROGRAM_TYPE_DRP;
use tracing::{debug, error};
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    // Base path is /api/add-numbers/
    Router::new()
        .route("/setup-participants", post(setup_participants))
        .route("/place-bet", post(place_bet))
        .route("/setup-funding-utxo", post(setup_funding_utxo)) // for player 2
        .route("/setup-game", post(setup_game)) // for player 1 and player 2 (send the numbers to sum)
        .route("/start-game", post(start_game)) // for player 1 (send the challenge transaction to start the game)
        .route("/submit-sum", post(submit_sum)) // Player 2 will send the sum to answer the challenge once he see the challenge transaction.
        .route("/{id}", get(get_game))
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
    if request.aggregated_id == Uuid::default() {
        return Err(http_errors::bad_request("Aggregated ID cannot be empty"));
    }
    let aggregated_id = request.aggregated_id;

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
    let aggregated_key = app_state
        .bitvmx_service
        .create_agregated_key(
            aggregated_id,
            participants_addresses,
            Some(participants_keys),
            leader_idx,
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to create aggregated key: {e:?}"))
        })?;
    debug!("Aggregated key created: {:?}", aggregated_key);

    // Create the program id
    let program_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, aggregated_id.as_bytes());
    debug!("🎉 Setting up game with program id: {:?}", program_id);

    // Setup the game
    app_state
        .add_numbers_service
        .setup_participants(
            program_id,
            aggregated_id,
            request.participants_addresses,
            request.participants_keys,
            aggregated_key,
            request.role,
        )
        .map_err(|e| http_errors::internal_server_error(&format!("Failed to setup game: {e:?}")))?;

    Ok(Json(SetupParticipantsResponse {
        program_id,
        aggregated_key,
    }))
}

/// Get a specific add numbers game by ID
#[utoipa::path(
    get,
    path = "/api/add-numbers/{id}",
    params(
        ("id" = String, Path, description = "Game ID", example = "123e4567-e89b-12d3-a456-426614174000")
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
    let game = app_state
        .add_numbers_service
        .get_game(id)
        .map_err(|e| http_errors::internal_server_error(&format!("Failed to get game: {e:?}")))?
        .ok_or(http_errors::not_found("Game not found"))?;

    Ok(Json(game.clone()))
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
    let game = app_state
        .add_numbers_service
        .get_current_game_id()
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to get current game ID: {e:?}"))
        })?;

    Ok(Json(game))
}

/// Place a bet for the add numbers game
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
) -> Result<Json<()>, (StatusCode, Json<ErrorResponse>)> {
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
    let game = app_state
        .add_numbers_service
        .get_game(program_id)
        .map_err(|e| http_errors::internal_server_error(&format!("Failed to get game: {e:?}")))?
        .ok_or(http_errors::not_found("Game not found"))?;

    if game.status != AddNumbersGameStatus::PlaceBet {
        return Err(http_errors::bad_request("Game is not in place bet state"));
    }

    if game.role == PlayerRole::Player2 {
        app_state
            .add_numbers_service
            .change_state(program_id, AddNumbersGameStatus::SetupFunding)
            .map_err(|e| {
                http_errors::internal_server_error(&format!("Failed to update game state: {e:?}"))
            })?;
        return Ok(Json(()));
    }

    // Get the aggregated key
    let aggregated_key = game.bitvmx_program_properties.aggregated_key;

    // Get the protocol fees amount
    let protocol_amount = app_state.bitvmx_service.protocol_cost();
    // Preparer the utxo destination for the protocol fees
    let protocol_destination = app_state
        .add_numbers_service
        .protocol_destination(&aggregated_key, protocol_amount)
        .map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to obtain protocol destination from aggregated key: {e:?}"
            ))
        })?;

    // Prepare the utxo destination for the bet
    let bet_destination = app_state
        .add_numbers_service
        .protocol_destination(&aggregated_key, request.amount)
        .map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to obtain bet destination from aggregated key: {e:?}"
            ))
        })?;

    // Send funds to cover protocol fees to the aggregated key
    let (funding_uuid, funding_txid) = app_state
        .bitvmx_service
        .send_funds(&Destination::Batch(vec![
            protocol_destination,
            bet_destination,
        ]))
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
    let funding_tx_status = app_state
        .bitvmx_service
        .wait_transaction_response(funding_uuid)
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to wait for transaction status response: {e:?}"
            ))
        })?;

    debug!(
        "Received transaction status responses for correlation ids: {:?} and {:?}",
        funding_uuid, funding_txid
    );

    if funding_tx_status.confirmations == 0 {
        error!(
            "Transaction {} not confirmed for correlation id: {:?}",
            funding_txid, funding_uuid
        );
        return Err(http_errors::internal_server_error(
            "Transaction not confirmed",
        ));
    }

    debug!("Protocol and bet transactions confirmed, marking funding UTXOs as mined");
    let protocol_leaves = app_state
        .add_numbers_service
        .protocol_scripts(&aggregated_key);

    let protocol_output_type =
        OutputType::taproot(protocol_amount, &aggregated_key, &protocol_leaves).map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to obtain protocol output type from aggregated key: {e:?}"
            ))
        })?;
    let bet_output_type = OutputType::taproot(protocol_amount, &aggregated_key, &protocol_leaves)
        .map_err(|e| {
        http_errors::internal_server_error(&format!(
            "Failed to obtain protocol output type from aggregated key: {e:?}"
        ))
    })?;

    let funding_protocol_utxo: Utxo = Utxo {
        txid: funding_txid.to_string(),
        vout: 0,
        amount: protocol_amount,
        output_type: serde_json::to_value(protocol_output_type).map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to convert protocol output type to JSON: {e:?}"
            ))
        })?,
    };
    let funding_bet_utxo: Utxo = Utxo {
        txid: funding_txid.to_string(),
        vout: 1,
        amount: request.amount,
        output_type: serde_json::to_value(bet_output_type).map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to convert bet output type to JSON: {e:?}"
            ))
        })?,
    };

    // Save the funding UTXOs in AddNumbersService
    app_state
        .add_numbers_service
        .save_funding_utxos(
            program_id,
            funding_protocol_utxo.clone(),
            funding_bet_utxo.clone(),
        )
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to save my funding UTXO: {e:?}"))
        })?;
    debug!("Saved my funding UTXOs in AddNumbersService");

    Ok(Json(()))
}

/// Setup the game for the add numbers game
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
    // Validate the program ID
    if request.program_id == Uuid::default() {
        return Err(http_errors::bad_request("Invalid program ID"));
    }

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

    // Save the funding UTXOs
    app_state
        .add_numbers_service
        .save_funding_utxos(
            request.program_id,
            funding_protocol_utxo.clone(),
            funding_bet_utxo.clone(),
        )
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to add funding UTXO: {e:?}"))
        })?;

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
        (status = 500, description = "Failed to get game", body = ErrorResponse),
        (status = 500, description = "Failed to start game", body = ErrorResponse)
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
    let game = app_state
        .add_numbers_service
        .get_game(program_id)
        .map_err(|e| http_errors::internal_server_error(&format!("Failed to get game: {e:?}")))?
        .ok_or(http_errors::not_found("Game not found"))?;

    if game.status != AddNumbersGameStatus::StartGame {
        return Err(http_errors::bad_request("Game is not in start game state"));
    }

    if game.role != PlayerRole::Player1 {
        return Err(http_errors::bad_request("Invalid game role"));
    }

    // Player 1 send the challenge transaction to start the game.
    let challenge_tx = app_state
        .bitvmx_service
        .start_challenge(program_id)
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to start challenge: {e:?}"))
        })?;

    // Set the game as setup
    app_state
        .add_numbers_service
        .start_game(program_id, &challenge_tx)
        .map_err(|e| http_errors::internal_server_error(&format!("Failed to setup game: {e:?}")))?;

    Ok(Json(StartGameResponse {
        program_id,
        challenge_tx: serde_json::to_value(challenge_tx).map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to convert challenge transaction to JSON: {e:?}"
            ))
        })?,
    }))
}

#[utoipa::path(
    post,
    path = "/api/add-numbers/setup-game",
    request_body = StartGameRequest,
    responses(
        (status = 200, description = "Game setup successfully", body = SetupGameResponse),
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
pub async fn setup_game(
    State(app_state): State<AppState>,
    Json(request): Json<SetupGameRequest>,
) -> Result<Json<SetupGameResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate the program ID
    if request.program_id == Uuid::default() {
        return Err(http_errors::bad_request("Invalid program ID"));
    }
    let program_id = request.program_id;

    // Get the game
    let game = app_state
        .add_numbers_service
        .get_game(program_id)
        .map_err(|e| http_errors::internal_server_error(&format!("Failed to get game: {e:?}")))?
        .ok_or(http_errors::not_found("Game not found"))?
        .clone();

    // Set inputs values, Concatenate the two input numbers as bytes
    let mut concatenated_bytes = Vec::<u8>::new();
    concatenated_bytes.extend_from_slice(&request.number1.to_be_bytes());
    concatenated_bytes.extend_from_slice(&request.number2.to_be_bytes());

    // Set all necesary program variables in BitVMX

    // Set program input 0, the two numbers to sum
    app_state
        .bitvmx_service
        .set_program_input(program_id, 0, concatenated_bytes)
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to set program input: {e:?}"))
        })?;

    // Set aggregated key
    app_state
        .bitvmx_service
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
    app_state
        .bitvmx_service
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

    app_state
        .bitvmx_service
        .set_variable(
            program_id,
            "utxo_prover_win_action",
            VariableTypes::Utxo(bet_utxo.into()),
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to set variable bet utxo: {e:?}"))
        })?;

    // Set program definition, it should be the relative path from the bitvmx-client to the program definition file
    let program_path = "./verifiers/add-test-with-const-pre.yaml";
    app_state
        .bitvmx_service
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
    app_state
        .bitvmx_service
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

    // Get the participants addresses
    let participants_addresses: Vec<BitVMXP2PAddress> = game
        .bitvmx_program_properties
        .participants_addresses
        .iter()
        .map(|p2p| p2p.clone().into())
        .collect();

    // Setup program in BitVMX
    app_state
        .bitvmx_service
        .program_setup(program_id, PROGRAM_TYPE_DRP, participants_addresses, 1)
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to set variable program setup: {e:?}"
            ))
        })?;

    // Set game as started
    app_state
        .add_numbers_service
        .setup_game(program_id, request.number1, request.number2)
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to save start game state: {e:?}"))
        })?;

    // Return the program ID
    Ok(Json(SetupGameResponse { program_id }))
}

/// Submit the sum for the add numbers game
#[utoipa::path(
    post,
    path = "/api/add-numbers/submit-sum",
    request_body = SubmitSumRequest,
    responses(
        (status = 200, description = "Sum submitted successfully", body = SubmitSumResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 404, description = "Game not found", body = ErrorResponse),
        (status = 500, description = "Failed to get game", body = ErrorResponse),
        (status = 500, description = "Failed to submit sum", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn submit_sum(
    State(app_state): State<AppState>,
    Json(request): Json<SubmitSumRequest>,
) -> Result<Json<SubmitSumResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate the program ID
    if request.id == Uuid::default() {
        return Err(http_errors::bad_request("Invalid program ID"));
    }
    let program_id = request.id;

    // Get the game
    let game = app_state
        .add_numbers_service
        .get_game(program_id)
        .map_err(|e| http_errors::internal_server_error(&format!("Failed to get game: {e:?}")))?
        .ok_or(http_errors::not_found("Game not found"))?;

    if game.role != PlayerRole::Player2 {
        return Err(http_errors::bad_request("Invalid game role"));
    }
    // TODO fix the state
    // if game.status != AddNumbersGameStatus::SubmitGameData {
    //     return Err(http_errors::bad_request(
    //         "Game is not in submit game data state",
    //     ));
    // }

    // The input index is 1 because the first input is the numbers to sum
    let input_index = 1;

    // Player 2 sets the input transaction with the sum in BitVMX
    app_state
        .bitvmx_service
        .set_program_input(
            program_id,
            input_index,
            request.guess.to_be_bytes().to_vec(),
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to set program input: {e:?}"))
        })?;

    // Send the input transaction to BitVMX
    let (challenge_input_tx, _challenge_input_tx_name) = app_state
        .bitvmx_service
        .send_challenge_input(program_id, input_index)
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to send challenge input: {e:?}"))
        })?;
    debug!(
        "Challenge input transaction: {:?}",
        challenge_input_tx.tx_id
    );

    // Wait for the challenge result
    let challenge_result_tx = app_state
        .bitvmx_service
        .wait_transaction_by_name_response(program_id, ACTION_PROVER_WINS)
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to send challenge input: {e:?}"))
        })?;

    // TOOD: PEDRO Wait until you know the answer
    app_state
        .add_numbers_service
        .make_guess(
            request.id,
            request.guess,
            challenge_input_tx.clone(),
            challenge_result_tx.clone(),
        )
        .map_err(|e| match e.to_string().as_str() {
            "Game not found" => http_errors::not_found("Game not found"),
            "Game is not in waiting for guess state" => {
                http_errors::bad_request("Invalid game state")
            }
            _ => http_errors::internal_server_error(&format!("Failed to submit sum: {e:?}")),
        })?;

    Ok(Json(SubmitSumResponse {
        program_id,
        challenge_input_tx: serde_json::to_value(challenge_input_tx).map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to convert challenge input transaction to JSON: {e:?}"
            ))
        })?,
        challenge_result_tx: serde_json::to_value(challenge_result_tx).map_err(|e| {
            http_errors::internal_server_error(&format!(
                "Failed to convert challenge result transaction to JSON: {e:?}"
            ))
        })?,
    }))
}
