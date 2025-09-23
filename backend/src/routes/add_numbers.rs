use crate::models::{
    AddNumbersGame, ErrorResponse, FundingUtxoRequest, FundingUtxosResponse, PlaceBetRequest,
    PlaceBetResponse, SetupGameRequest, SetupGameResponse, SetupParticipantsRequest,
    SetupParticipantsResponse, StartGameRequest, StartGameResponse, SubmitSumRequest,
    SubmitSumResponse,
};
use crate::state::AppState;
use crate::utils::http_errors;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use tracing::{debug, info};
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    // Base path is /api/add-numbers/
    Router::new()
        .route("/{id}", get(get_game))
        .route("/current-game", get(get_current_game))
        .route("/setup-participants", post(setup_participants))
        .route("/place-bet", post(place_bet))
        .route("/setup-funding-utxo", post(setup_funding_utxo)) // for player 2
        .route("/setup-game", post(setup_game)) // for player 1 and player 2 (send the numbers to sum)
        .route("/start-game", post(start_game)) // for player 1 (send the challenge transaction to start the game)
        .route("/submit-sum", post(submit_sum)) // Player 2 will send the sum to answer the challenge once he see the challenge transaction.
}

/// Get the current game
#[utoipa::path( get,
    path = "/api/add-numbers/current-game",
    responses(
        (status = 200, description = "Current game", body = Option<AddNumbersGame>),
        (status = 500, description = "Failed to get current game", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn get_current_game(
    State(app_state): State<AppState>,
) -> Result<Json<Option<AddNumbersGame>>, (StatusCode, Json<ErrorResponse>)> {
    let game = app_state
        .add_numbers_service
        .get_current_game()
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to get current game: {e:?}"))
        })?;

    Ok(Json(game))
}

/// Create a new add numbers game
#[utoipa::path(
    post,
    path = "/api/add-numbers/setup-participants",
    request_body = SetupParticipantsRequest,
    responses(
        (status = 201, description = "Game created successfully", body = SetupParticipantsResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 400, description = "Aggregated ID cannot be empty", body = ErrorResponse),
        (status = 400, description = "At least one participant address is required", body = ErrorResponse),
        (status = 400, description = "At least one participant key is required", body = ErrorResponse),
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

    // Validate the participants keys
    if request.participants_keys.is_empty() {
        return Err(http_errors::bad_request(
            "At least one participant key is required",
        ));
    }

    // Create the aggregated key
    let (program_id, aggregated_key) = app_state
        .add_numbers_service
        .setup_participants(
            aggregated_id,
            leader_idx,
            request.participants_addresses,
            request.participants_keys,
            request.role,
        )
        .await
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to create aggregated key: {e:?}"))
        })?;
    debug!("Aggregated key created: {:?}", aggregated_key);

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
        .await
        .map_err(|e| http_errors::internal_server_error(&format!("Failed to get game: {e:?}")))?
        .ok_or(http_errors::not_found("Game not found"))?;

    Ok(Json(game.clone()))
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

    let game = app_state
        .add_numbers_service
        .place_bet(program_id, 10_000)
        .await
        .map_err(|e| http_errors::internal_server_error(&format!("Failed to place bet: {e:?}")))?;

    info!("Place bet successfully for program id: {:?}", program_id);

    Ok(Json(PlaceBetResponse { game }))
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
        .setup_funding_utxo(
            request.program_id,
            funding_protocol_utxo.clone(),
            funding_bet_utxo.clone(),
        )
        .await
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
        (status = 500, description = "Failed to start game", body = ErrorResponse),
        (status = 500, description = "Failed to convert challenge transaction to JSON", body = ErrorResponse),
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

    // Player 1 send the challenge transaction to start the game.
    let (_challenge_tx_name, challenge_tx) = app_state
        .add_numbers_service
        .start_game(program_id, app_state.worker_service.clone())
        .await
        .map_err(|e| http_errors::internal_server_error(&format!("Failed to start game: {e:?}")))?;

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
        (status = 500, description = "Failed to setup game", body = ErrorResponse)
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

    app_state
        .add_numbers_service
        .setup_game(
            program_id,
            request.number1,
            request.number2,
            app_state.worker_service.clone(),
        )
        .await
        .map_err(|e| http_errors::internal_server_error(&format!("Failed to setup game: {e:?}")))?;

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
        (status = 400, description = "Invalid program ID", body = ErrorResponse),
        (status = 500, description = "Failed to submit sum", body = ErrorResponse),
        (status = 500, description = "Failed to convert challenge input transaction to JSON", body = ErrorResponse),
        (status = 500, description = "Failed to convert challenge result transaction to JSON", body = ErrorResponse),
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

    // TOOD: PEDRO Wait until you know the answer
    let game = app_state
        .add_numbers_service
        .submit_sum(program_id, request.guess)
        .await
        .map_err(|e| http_errors::internal_server_error(&format!("Failed to submit sum: {e:?}")))?;

    Ok(Json(SubmitSumResponse { program_id, game }))
}
