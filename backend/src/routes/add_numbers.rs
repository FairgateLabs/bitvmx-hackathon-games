use std::str::FromStr;

use crate::models::{
    SetupParticipantsResponse, SetupParticipantsRequest,
    AddNumbersGame, AddNumbersRequest, AddNumbersResponse, ErrorResponse, MakeGuessRequest,
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
use bitvmx_client::p2p_handler::PeerId;
use bitvmx_client::program::participant::P2PAddress;
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    // Base path is /api/add-numbers/
    Router::new()
        // .route("/", post(create_game))
        .route("/{id}", get(get_game))
        .route("/{id}/guess", post(make_guess))
        .route("/current-game-id", get(get_current_game_id))
        .route("/setup-participants", post(setup_participants))
}

/// Create a new add numbers game
#[utoipa::path(
    post,
    path = "/api/setup-participants/",
    request_body = SetupParticipantsResponse,
    responses(
        (status = 201, description = "Game created successfully", body = SetupParticipantsResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn setup_participants(
    State(app_state): State<AppState>,
    Json(request): Json<SetupParticipantsRequest>,
) -> Result<Json<()>, (StatusCode, Json<ErrorResponse>)> {
    // Validate the id
    if request.agregated_id.is_empty() {
        return Err(http_errors::bad_request(
            "Aggregated key ID cannot be empty",
        ));
    }

    // Validate the p2p addresses
    if request.participants_addresses.is_empty() {
        return Err(http_errors::bad_request(
            "At least one P2P address is required",
        ));
    }

    // Validate the operator keys
    for operator_key in &request.participants_keys {
        if operator_key.is_empty() {
            return Err(http_errors::bad_request("Operator key cannot be empty"));
        }
    }

    let agregated_id = Uuid::parse_str(&request.agregated_id)
        .map_err(|_| http_errors::bad_request("Invalid UUID"))?;
    let participants_keys =
        request
            .participants_keys
            .iter()
            .map(|key| PublicKey::from_str(key).map_err(|_| http_errors::bad_request("Invalid operator key")))
            .collect::<Result<Vec<PublicKey>, (StatusCode, Json<ErrorResponse>)>>()?;

    let participants: Vec<P2PAddress> = request
        .participants_addresses
        .iter()
        .map(|p2p| P2PAddress {
            address: p2p.address.clone(),
            peer_id: PeerId(p2p.peer_id.clone()),
        })
        .collect();

    let service = app_state.bitvmx_service.read().await;
    let aggregated_key = service.create_agregated_key(agregated_id, participants, Some(participants_keys), 0).await.map_err(|e| http_errors::internal_server_error(&format!("Failed to create aggregated key: {e:?}")))?;

    let program_id = Uuid::parse_str(&request.program_id)
        .map_err(|_| http_errors::bad_request("Invalid program ID"))?;
    let mut service = app_state.add_numbers_service.write().await;
    let game = service.setup_game(program_id, agregated_id, request.participants_addresses, request.participants_keys, aggregated_key);

    Ok(Json(()))
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
        http_errors::error_response(StatusCode::BAD_REQUEST, "INVALID_OPERATION", &error)
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

    if let Some(game) = game {
        Ok(Json(Some(game)))
    } else {
        Ok(Json(None))
    }
}
