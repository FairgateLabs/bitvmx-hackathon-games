use crate::utils::http_errors;
use crate::models::{
    AddNumbersGame, AddNumbersRequest, AddNumbersResponse, ErrorResponse, MakeGuessRequest,
};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    // Base path is /api/add-numbers/
    Router::new()
        .route("/", post(create_game))
        .route("/{id}", get(get_game))
        .route("/{id}/guess", post(make_guess))
        .route("/current-game-id", get(get_current_game_id))
}

/// Create a new add numbers game
#[utoipa::path(
    post,
    path = "/api/add-numbers/",
    request_body = AddNumbersResponse,
    responses(
        (status = 201, description = "Game created successfully", body = AddNumbersResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn create_game(
    State(app_state): State<AppState>,
    Json(request): Json<AddNumbersRequest>,
) -> Result<Json<AddNumbersGame>, (StatusCode, Json<ErrorResponse>)> {
    let mut service = app_state.add_numbers_service.write().await;
    let game = service.create_game(request.id, request.number1, request.number2);

    Ok(Json(game))
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
