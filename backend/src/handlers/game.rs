use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    stores::GameStore,
    types::{
        CreateGameRequest, CreateGameResponse, GameResponse, GameStatusResponse, MakeMoveRequest,
        MakeMoveResponse, ErrorResponse,
    },
};

/// Create a new tic-tac-toe game
pub async fn create_game(
    State(store): State<Arc<Mutex<GameStore>>>,
    _request: CreateGameRequest,
) -> Result<CreateGameResponse, (StatusCode, Json<ErrorResponse>)> {
    let mut store = store.lock().await;
    let game = store.create_game();

    Ok(CreateGameResponse {
        game,
        message: "Game created successfully".to_string(),
    })
}

/// Get a specific game by ID
pub async fn get_game(
    State(store): State<Arc<Mutex<GameStore>>>,
    id: Uuid,
) -> Result<GameResponse, (StatusCode, Json<ErrorResponse>)> {
    let store = store.lock().await;
    
    let game = store.get_game(id).ok_or((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "NOT_FOUND".to_string(),
            message: "Game not found".to_string(),
        }),
    ))?;

    Ok(GameResponse {
        game: game.clone(),
    })
}

/// Make a move in the game
pub async fn make_move(
    State(store): State<Arc<Mutex<GameStore>>>,
    id: Uuid,
    request: MakeMoveRequest,
) -> Result<MakeMoveResponse, (StatusCode, Json<ErrorResponse>)> {
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

    Ok(MakeMoveResponse {
        game,
        message: "Move made successfully".to_string(),
    })
}

/// Get the current status of a game
pub async fn get_game_status(
    State(store): State<Arc<Mutex<GameStore>>>,
    id: Uuid,
) -> Result<GameStatusResponse, (StatusCode, Json<ErrorResponse>)> {
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

    Ok(GameStatusResponse {
        status: game.status.clone(),
        current_player,
    })
}
