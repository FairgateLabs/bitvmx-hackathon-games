use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    models::GameStore,
    types::{
        CreateGameRequest, CreateGameResponse, GameResponse, GameStatusResponse, MakeMoveRequest,
        MakeMoveResponse, ErrorResponse,
    },
};

/// Create a new tic-tac-toe game
#[utoipa::path(
    post,
    path = "/api/game",
    request_body = CreateGameRequest,
    responses(
        (status = 201, description = "Game created successfully"),
        (status = 400, description = "Invalid request", body = ErrorResponse)
    ),
    tag = "Game"
)]
pub async fn create_game(
    State(store): State<Arc<Mutex<GameStore>>>,
    Json(_request): Json<CreateGameRequest>,
) -> Result<Json<CreateGameResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut store = store.lock().await;
    let game = store.create_game();

    Ok(Json(CreateGameResponse {
        game,
        message: "Game created successfully".to_string(),
    }))
}

/// Get a specific game by ID
#[utoipa::path(
    get,
    path = "/api/game/{id}",
    params(
        ("id" = String, Path, description = "Game ID")
    ),
    responses(
        (status = 200, description = "Game found"),
        (status = 404, description = "Game not found", body = ErrorResponse)
    ),
    tag = "Game"
)]
pub async fn get_game(
    State(store): State<Arc<Mutex<GameStore>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<GameResponse>, (StatusCode, Json<ErrorResponse>)> {
    let store = store.lock().await;
    
    let game = store.get_game(id).ok_or((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "NOT_FOUND".to_string(),
            message: "Game not found".to_string(),
        }),
    ))?;

    Ok(Json(GameResponse {
        game: game.clone(),
    }))
}

/// Make a move in the game
#[utoipa::path(
    post,
    path = "/api/game/{id}/move",
    params(
        ("id" = String, Path, description = "Game ID")
    ),
    request_body = MakeMoveRequest,
    responses(
        (status = 200, description = "Move made successfully"),
        (status = 400, description = "Invalid move", body = ErrorResponse),
        (status = 404, description = "Game not found", body = ErrorResponse)
    ),
    tag = "Game"
)]
pub async fn make_move(
    State(store): State<Arc<Mutex<GameStore>>>,
    Path(id): Path<Uuid>,
    Json(request): Json<MakeMoveRequest>,
) -> Result<Json<MakeMoveResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut store = store.lock().await;
    
    let game = store
        .make_move(id, request.player, request.position)
        .map_err(|error| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "INVALID_MOVE".to_string(),
                    message: error,
                }),
            )
        })?;

    Ok(Json(MakeMoveResponse {
        game,
        message: "Move made successfully".to_string(),
    }))
}

/// Get the current status of a game
#[utoipa::path(
    get,
    path = "/api/game/{id}/status",
    params(
        ("id" = String, Path, description = "Game ID")
    ),
    responses(
        (status = 200, description = "Game status retrieved"),
        (status = 404, description = "Game not found", body = ErrorResponse)
    ),
    tag = "Game"
)]
pub async fn get_game_status(
    State(store): State<Arc<Mutex<GameStore>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<GameStatusResponse>, (StatusCode, Json<ErrorResponse>)> {
    let store = store.lock().await;
    
    let game = store.get_game(id).ok_or((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "NOT_FOUND".to_string(),
            message: "Game not found".to_string(),
        }),
    ))?;

    let current_player = match game.status {
        crate::types::GameStatus::InProgress => Some(game.current_player.clone()),
        _ => None,
    };

    Ok(Json(GameStatusResponse {
        status: game.status.clone(),
        current_player,
    }))
}
