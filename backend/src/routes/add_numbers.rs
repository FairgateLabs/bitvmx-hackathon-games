use std::str::FromStr;

use crate::models::{
    AddNumbersGame, AddNumbersGameStatus, ErrorResponse, FundingUtxoRequest, FundingUtxosResponse,
    MakeGuessRequest, PlaceBetRequest, PlaceBetResponse, SetupParticipantsRequest,
    SetupParticipantsResponse, Utxo,
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
use bitvmx_client::bitcoin_coordinator::TransactionStatus;
use bitvmx_client::p2p_handler::PeerId;
use bitvmx_client::program::participant::P2PAddress;
use bitvmx_client::types::Destination;
use tracing::{debug};
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    // Base path is /api/add-numbers/
    Router::new()
        .route("/setup-participants", post(setup_participants))
        .route("/place-bet", post(place_bet))
        .route("/fundings_utxos/{id}", get(get_fundings_utxos))
        .route("/setup-funding-utxo", post(setup_funding_utxo)) // for player 2
        // .route("/create-game", post(create_game)) // for player 1
        // .route("/", post(create_game))
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
    let participants_addresses: Vec<P2PAddress> = request
        .participants_addresses
        .iter()
        .map(|p2p| P2PAddress {
            address: p2p.address.clone(),
            peer_id: PeerId(p2p.peer_id.clone()),
        })
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
    let aggregated_key: PublicKey;
    {
        let service = app_state.bitvmx_service.read().await;
        aggregated_key = service
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
    }
    debug!("Aggregated key created: {:?}", aggregated_key);

    // Create the program id
    let program_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, request.aggregated_id.as_bytes());
    debug!("🎉 Setting up game with program id: {:?}", program_id);

    // Setup the game
    {
        let mut service = app_state.add_numbers_service.write().await;
        service
            .setup_game(
                program_id,
                aggregated_id,
                request.participants_addresses,
                request.participants_keys,
                aggregated_key,
                request.role,
            )
            .map_err(|e| http_errors::internal_server_error(&format!("Failed to setup game: {e:?}")))?;
    }

    Ok(Json(SetupParticipantsResponse {
        program_id: program_id.to_string(),
    }))
}

// /// Create a new add numbers game
// #[utoipa::path(
//     post,
//     path = "/api/add-numbers/",
//     request_body = AddNumbersResponse,
//     responses(
//         (status = 201, description = "Game created successfully", body = AddNumbersResponse),
//         (status = 400, description = "Invalid request", body = ErrorResponse)
//     ),
//     tag = "AddNumbers"
// )]
// pub async fn create_game(
//     State(app_state): State<AppState>,
//     Json(request): Json<AddNumbersRequest>,
// ) -> Result<Json<AddNumbersGame>, (StatusCode, Json<ErrorResponse>)> {
//     let mut service = app_state.add_numbers_service.write().await;
//     let game = service.setup_game(request.id, request.number1, request.number2);

//     Ok(Json(game))
// }

/// Get a specific add numbers game by ID
#[utoipa::path(
    get,
    path = "/api/add-numbers/{id}",
    params(
        ("id" = String, Path, description = "Game ID")
    ),
    responses(
        (status = 200, description = "Game found"),
        (status = 404, description = "Game not found", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn get_game(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AddNumbersGame>, (StatusCode, Json<ErrorResponse>)> {
    let service = app_state.add_numbers_service.read().await;
    let game = service
        .get_game(id)
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
    let mut service = app_state.add_numbers_service.write().await;
    let game = service.make_guess(id, request.guess).map_err(|error| {
        http_errors::error_response(StatusCode::BAD_REQUEST, "INVALID_OPERATION", &error.to_string())
    })?;

    Ok(Json(game))
}

#[utoipa::path( get,
    path = "/api/add-numbers/current-game-id",
    responses(
        (status = 200, description = "Current game ID", body = String),
        (status = 404, description = "Current game ID not found", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn get_current_game_id(
    State(app_state): State<AppState>,
) -> Result<Json<Option<AddNumbersGame>>, (StatusCode, Json<ErrorResponse>)> {
    let service = app_state.add_numbers_service.read().await;
    let game = service.get_current_game_id();

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

    let aggregated_key: PublicKey;
    let destination: Destination;
    {
        // Get the game
        let service = app_state.add_numbers_service.read().await;
        let game = service
            .get_game(program_id)
            .ok_or(http_errors::not_found("Game not found"))?;
        // Get the aggregated key
        aggregated_key = game.bitvmx_program_properties.aggregated_key;
        // Get the protocol information
        destination = service.protocol_destination(&aggregated_key)
        .map_err(|e| http_errors::internal_server_error(&format!("Failed to obtain protocol destination from aggregated key: {e:?}")))?;
    
    }
    
    let funding_protocol_utxo: Utxo;
    let funding_bet_utxo: Utxo;
    let protocol_tx_status: TransactionStatus;
    let bet_tx_status: TransactionStatus;
    {
        // Get the protocol fees amount
        let service = app_state.bitvmx_service.read().await;
        let protocol_amount = service.protocol_cost();

        // Send funds to cover protocol fees to the aggregated key
        let (funding_protocol_uuid, funding_protocol) = service
            .send_funds(&destination, protocol_amount)
            .await
            .map_err(|e| {
                http_errors::internal_server_error(&format!("Failed to send protocol funds: {e:?}"))
            })?;
        debug!(
            "Sent {protocol_amount} satoshis to cover protocol fees to the aggregated key txid: {:?}",
            funding_protocol.0
        );

        // Send the amount that the players will bet to the aggregated key
        let (funding_bet_uuid, funding_bet) = service
            .send_funds(&destination, request.amount)
            .await
            .map_err(|e| {
                http_errors::internal_server_error(&format!("Failed to send bet funds: {e:?}"))
            })?;
        debug!(
            "Funds {} satoshis sent to the aggregated key to cover the players bet txid: {:?}", request.amount,
            funding_bet.0
        );

        // Wait for the Transaction Status responses
        debug!("Waiting for transaction status responses");
        let (protocol_tx_result, bet_tx_result) = tokio::join!(
            service.wait_for_transaction_response(funding_protocol_uuid),
            service.wait_for_transaction_response(funding_bet_uuid),
        );
        
        debug!("Received transaction status responses for correlation ids: {:?} and {:?}", protocol_tx_result, bet_tx_result);
        // Handle any errors from waiting for responses
        protocol_tx_status = protocol_tx_result.map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to wait for protocol transaction response: {e:?}"))
        })?;
        bet_tx_status = bet_tx_result.map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to wait for bet transaction response: {e:?}"))
        })?;

        funding_protocol_utxo = funding_protocol.clone().into();
        funding_bet_utxo = funding_bet.clone().into();
    }
   

    if protocol_tx_status.confirmations > 0 && bet_tx_status.confirmations > 0 {
        debug!("Protocol and bet transactions confirmed, marking funding UTXOs as mined");
        // Mark the funding UTXOs as mined
        {
            let mut service = app_state.add_numbers_service.write().await;
            service.mark_my_funding_utxos_as_mined(program_id).map_err(|e| {
                http_errors::internal_server_error(&format!("Failed to mark my funding UTXOs as mined: {e:?}"))
            })?;
        }
    }


    // Save the funding UTXOs in AddNumbersService
    {
        let mut service = app_state.add_numbers_service.write().await;
        service.save_my_funding_utxos(
            program_id,
            funding_protocol_utxo.clone(),
            funding_bet_utxo.clone(),
        ).map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to save my funding UTXO: {e:?}"))
        })?;
    }
    debug!("Saved my funding UTXOs in AddNumbersService");

    let mut game_service = app_state.add_numbers_service.write().await;
    game_service
        .update_game_state(program_id, AddNumbersGameStatus::SetupFunding)
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to update game state: {e:?}"))
        })?;

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
    let service = app_state.add_numbers_service.read().await;
    let game = service
        .get_game(id)
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
        (status = 400, description = "Invalid UTXO", body = ErrorResponse)
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
        let mut service = app_state.add_numbers_service.write().await;
        service
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

// #[utoipa::path(
//     post,
//     path = "/api/add-numbers/create-game",
//     request_body = CreateGameRequest,
//     responses(
//         (status = 200, description = "Game created successfully"),
//         (status = 400, description = "Invalid request", body = ErrorResponse)
//     ),
//     tag = "AddNumbers"
// )]
// pub async fn create_game(
//     State(app_state): State<AppState>,
//     Json(request): Json<CreateGameRequest>,
// ) -> Result<Json<()>, (StatusCode, Json<ErrorResponse>)> {
//     let program_id = Uuid::parse_str(&request.program_id)
//         .map_err(|_| http_errors::bad_request("Invalid program ID"))?;
//     let mut service = app_state.add_numbers_service.write().await;

//     service
//         .create_game(program_id, request.number1, request.number2)
//         .map_err(|e| {
//             http_errors::internal_server_error(&format!("Failed to create game: {e:?}"))
//         })?;
//     Ok(Json(()))
// }
