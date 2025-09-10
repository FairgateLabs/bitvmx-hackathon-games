use std::str::FromStr;

use crate::models::{
    AddNumbersGame, ErrorResponse, FundingUtxoRequest, FundingUtxosResponse, MakeGuessRequest,
    PlaceBetRequest, SetupParticipantsRequest, SetupParticipantsResponse, Utxo,
};
use crate::state::AppState;
use crate::utils::http_errors;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use bitvmx_client::bitcoin::{Amount, PublicKey};
use bitvmx_client::p2p_handler::PeerId;
use bitvmx_client::program::participant::P2PAddress;
use bitvmx_client::program::variables::PartialUtxo;
use tracing::debug;
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
    // Validate the aggregated ID
    if request.agregated_id.is_empty() {
        return Err(http_errors::bad_request(
            "Aggregated key ID cannot be empty",
        ));
    }
    let agregated_id = Uuid::parse_str(&request.agregated_id)
    .map_err(|_| http_errors::bad_request("Invalid Aggregated ID"))?;


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
        PublicKey::from_str(key).map_err(|_| http_errors::bad_request("Invalid participants key"))
    })
    .collect::<Result<Vec<PublicKey>, (StatusCode, Json<ErrorResponse>)>>()?;

    // Create the aggregated key
    let aggregated_key: PublicKey;
    {
        let service = app_state.bitvmx_service.read().await;
        aggregated_key = service
            .create_agregated_key(agregated_id, participants_addresses, Some(participants_keys), 0)
            .await
            .map_err(|e| {
                http_errors::internal_server_error(&format!("Failed to create aggregated key: {e:?}"))
            })?;
    }

    // Create the program id
    let program_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, request.aggregated_id.as_bytes());

    // Setup the game
    {
        let mut service = app_state.add_numbers_service.write().await;
        debug!("🎉 Setting up game with program id: {:?} 🎉", program_id);
        service.setup_game(
            program_id,
            agregated_id,
            request.participants_addresses,
            request.participants_keys,
            aggregated_key,
        ).map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to setup game: {e:?}"))
        })?;
    }

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

#[utoipa::path(
    post,
    path = "/api/add-numbers/place-bet",
    request_body = PlaceBetRequest,
    responses(
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 404, description = "Game not found", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn place_bet(
    State(app_state): State<AppState>,
    Json(request): Json<PlaceBetRequest>,
) -> Result<Json<()>, (StatusCode, Json<ErrorResponse>)> {
    let program_id = Uuid::parse_str(&request.program_id)
        .map_err(|_| http_errors::bad_request("Invalid program ID"))?;

    // Validate the amount
    if request.amount == 0 {
        return Err(http_errors::bad_request("Amount cannot be 0"));
    }

    let aggregated_key: PublicKey;
    {
        let add_numbers_service = app_state.add_numbers_service.read().await;
        let game = add_numbers_service
            .get_game(program_id)
            .ok_or(http_errors::not_found("Game not found"))?;

        aggregated_key = game.bitvmx_program_properties.aggregated_key;
    }

    // Send funds to cover protocol fees to the aggregated key
    let initial_utxo: PartialUtxo;
    {
        let bitvmx_service = app_state.bitvmx_service.read().await;
        let protocol_amount = bitvmx_service.protocol_cost();
        initial_utxo = bitvmx_service
            .send_funds(aggregated_key.to_string(), protocol_amount, None)
            .await
            .map_err(|e| http_errors::internal_server_error(&format!("Failed to send funds: {e:?}")))?;
   
        debug!(
            "Funds {protocol_amount} satoshis sent to cover protocol fees to the aggregated key txid: {:?}",
            initial_utxo.0
        );
   }

    // Send the amount that the players will bet to the aggregated key
    let amount = Amount::from_btc(request.amount as f64)
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to convert amount: {e:?}"))
        })?
        .to_sat();

    // TODO PEDRO: Add taproot address in aggregated_key
    {
        let bitvmx_service = app_state.bitvmx_service.read().await;
        let prover_win_utxo = bitvmx_service
            .send_funds(aggregated_key.to_string(), amount, None)
            .await
            .map_err(|e| http_errors::internal_server_error(&format!("Failed to send funds: {e:?}")))?;
        debug!(
            "Funds {amount} satoshis sent to the aggregated key to cover the players bet txid: {:?}",
            prover_win_utxo.0
        );
    }

    Ok(Json(()))
}

#[utoipa::path(
    get,
    path = "/api/add-numbers/fundings_utxos/{id}",
    responses(
        (status = 200, description = "My participant UTXO", body = FundingUtxosResponse),
        (status = 404, description = "My participant UTXO not found", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn get_fundings_utxos(
) -> Result<Json<FundingUtxosResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO PEDRO: Get the my funding utxo
    // Hit the BitVMX client to check if the funding UTXO is mined.
    // TODO PEDRO: Save funding_protocol_utxo, and funding_bet_utxo

    // For now, return a hardcoded UTXO
    let funding_protocol_utxo = Utxo {
        txid: "hardcoded_txid".to_string(),
        vout: 0,
        amount: 100_000, // Amount in satoshis
        output_type: serde_json::json!({
            "type": "P2PKH",
            "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
        }),
    };

    let funding_bet_utxo = Utxo {
        txid: "hardcoded_txid".to_string(),
        vout: 0,
        amount: 100_000, // Amount in satoshis
        output_type: serde_json::json!({
            "type": "P2PKH",
            "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
        }),
    };

    let funding_utxo_response = FundingUtxosResponse {
        funding_protocol_utxo,
        funding_bet_utxo,
    };

    Ok(Json(funding_utxo_response))
}

#[utoipa::path(
    post,
    path = "/api/add-numbers/setup-funding-utxo",
    request_body = FundingUtxoRequest,
    responses(
        (status = 200, description = "Funding UTXO setup successfully"),
        (status = 400, description = "Invalid UTXO", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn setup_funding_utxo(
    State(app_state): State<AppState>,
    Json(request): Json<FundingUtxoRequest>,
) -> Result<Json<()>, (StatusCode, Json<ErrorResponse>)> {
    // Validate the UTXO
    if request.funding_protocol_utxo.txid.is_empty() || request.funding_protocol_utxo.amount == 0 {
        return Err(http_errors::bad_request("Invalid UTXO"));
    }

    let program_id = Uuid::parse_str(&request.program_id)
        .map_err(|_| http_errors::bad_request("Invalid program ID"))?;
    let mut service = app_state.add_numbers_service.write().await;

    service
        .save_fundings_utxos(
            program_id,
            request.funding_protocol_utxo,
            request.funding_bet_utxo,
        )
        .map_err(|e| {
            http_errors::internal_server_error(&format!("Failed to add funding UTXO: {e:?}"))
        })?;

    Ok(Json(()))
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
