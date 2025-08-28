use axum::{Router, routing::{get, post}, extract::{Path, State}, http::StatusCode, Json};
use crate::types::{CreateGameRequest, CreateGameResponse, GameResponse, GameStatusResponse, MakeMoveRequest, MakeMoveResponse, ErrorResponse};
use crate::app_state::AppState;
use crate::http_errors;
use uuid::Uuid;
use tracing::instrument;

pub fn router() -> Router<AppState> {
    // Base path is /api/game/
    Router::new()
        .route("/", post(create_game))
        .route("/{id}", get(get_game))
        .route("/{id}/move", post(make_move))
        .route("/{id}/status", get(get_game_status))
}

/// Create a new tic-tac-toe game
#[utoipa::path(
    post,
    path = "/api/game/",
    request_body = CreateGameRequest,
    responses(
        (status = 201, description = "Game created successfully"),
        (status = 400, description = "Invalid request", body = ErrorResponse)
    ),
    tag = "Game"
)]
#[instrument(skip(app_state))]
pub async fn create_game(
    State(app_state): State<AppState>,
    Json(request): Json<CreateGameRequest>,
) -> Result<Json<CreateGameResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut store = app_state.game_store.lock().await;
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
#[instrument(skip(app_state), fields(game_id = %id))]
pub async fn get_game(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<GameResponse>, (StatusCode, Json<ErrorResponse>)> {
    let store = app_state.game_store.lock().await;
    
    let game = store.get_game(id).ok_or(http_errors::not_found("Game not found"))?;

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
#[instrument(skip(app_state), fields(game_id = %id))]
pub async fn make_move(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<MakeMoveRequest>,
) -> Result<Json<MakeMoveResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut store = app_state.game_store.lock().await;
    
    let game = store
        .make_move(id, request.player, request.position)
        .map_err(|error| {
            tracing::warn!("Invalid move: {}", error);
            http_errors::error_response(StatusCode::BAD_REQUEST, "INVALID_MOVE", &error)
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
        (status = 200, description = "Game status retrieved", body = GameStatusResponse),
        (status = 404, description = "Game not found", body = ErrorResponse)
    ),
    tag = "Game"
)]
pub async fn get_game_status(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<GameStatusResponse>, (StatusCode, Json<ErrorResponse>)> {
    let store = app_state.game_store.lock().await;
    
    let game = store.get_game(id).ok_or(http_errors::not_found("Game not found"))?;

    let current_player = match game.status {
        crate::types::GameStatus::InProgress => Some(game.current_player.clone()),
        _ => None,
    };

    Ok(Json(GameStatusResponse {
        status: game.status.clone(),
        current_player,
    }))
}