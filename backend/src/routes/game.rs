use axum::{Router, routing::{get, post}, extract::{Path, State}, http::StatusCode, Json};
use crate::handlers::game;
use crate::types::{CreateGameRequest, CreateGameResponse, GameResponse, GameStatusResponse, MakeMoveRequest, MakeMoveResponse, ErrorResponse};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::stores::GameStore;
use uuid::Uuid;

pub fn router() -> Router {
    // Initialize shared state
            let game_store = Arc::new(Mutex::new(GameStore::new()));

    Router::new()
        .route("/", post(create_game))
        .route("/{id}", get(get_game))
        .route("/{id}/move", post(make_move))
        .route("/{id}/status", get(get_game_status))
        .with_state(game_store)
}

/// Create a new tic-tac-toe game
#[utoipa::path(
    post,
    path = "/game/",
    request_body = CreateGameRequest,
    responses(
        (status = 201, description = "Game created successfully"),
        (status = 400, description = "Invalid request", body = ErrorResponse)
    ),
    tag = "Game"
)]
pub async fn create_game(
    State(store): State<Arc<Mutex<GameStore>>>,
    Json(request): Json<CreateGameRequest>,
) -> Result<Json<CreateGameResponse>, (StatusCode, Json<ErrorResponse>)> {
    let response = game::create_game(State(store), request).await?;
    Ok(Json(response))
}

/// Get a specific game by ID
#[utoipa::path(
    get,
    path = "/game/{id}",
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
    let response = game::get_game(State(store), id).await?;
    Ok(Json(response))
}

/// Make a move in the game
#[utoipa::path(
    post,
    path = "/game/{id}/move",
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
    let response = game::make_move(State(store), id, request).await?;
    Ok(Json(response))
}

/// Get the current status of a game
#[utoipa::path(
    get,
    path = "/game/{id}/status",
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
    State(store): State<Arc<Mutex<GameStore>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<GameStatusResponse>, (StatusCode, Json<ErrorResponse>)> {
    let response = game::get_game_status(State(store), id).await?;
    Ok(Json(response))
}