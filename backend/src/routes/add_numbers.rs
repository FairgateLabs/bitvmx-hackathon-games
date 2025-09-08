use crate::utils::http_errors;
use crate::models::{
    AddNumbersGameResponse, AddNumbersRequest, AddNumbersResponse, CreateAddNumbersGameRequest,
    CreateAddNumbersGameResponse, ErrorResponse, MakeGuessRequest, MakeGuessResponse,
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
        .route("/{id}/add", post(add_numbers))
        .route("/{id}/guess", post(make_guess))
}

/// Create a new add numbers game
#[utoipa::path(
    post,
    path = "/api/add-numbers/",
    request_body = CreateAddNumbersGameRequest,
    responses(
        (status = 201, description = "Game created successfully"),
        (status = 400, description = "Invalid request", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn create_game(
    State(app_state): State<AppState>,
    Json(request): Json<CreateAddNumbersGameRequest>,
) -> Result<Json<CreateAddNumbersGameResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut service = app_state.add_numbers_service.write().await;
    let game = service.create_game(request.player1, request.player2);

    Ok(Json(CreateAddNumbersGameResponse {
        game,
        message: "Add numbers game created successfully".to_string(),
    }))
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
) -> Result<Json<AddNumbersGameResponse>, (StatusCode, Json<ErrorResponse>)> {
    let service = app_state.add_numbers_service.read().await;

    let game = service
        .get_game(id)
        .ok_or(http_errors::not_found("Game not found"))?;

    Ok(Json(AddNumbersGameResponse { game: game.clone() }))
}

/// Add two numbers to the game
#[utoipa::path(
    post,
    path = "/api/add-numbers/{id}/add",
    params(
        ("id" = String, Path, description = "Game ID")
    ),
    request_body = AddNumbersRequest,
    responses(
        (status = 200, description = "Numbers added successfully"),
        (status = 400, description = "Invalid operation", body = ErrorResponse),
        (status = 404, description = "Game not found", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn add_numbers(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<AddNumbersRequest>,
) -> Result<Json<AddNumbersResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut service = app_state.add_numbers_service.write().await;

    let game = service
        .add_numbers(id, request.player, request.number1, request.number2)
        .map_err(|error| {
            http_errors::error_response(StatusCode::BAD_REQUEST, "INVALID_OPERATION", &error)
        })?;

    Ok(Json(AddNumbersResponse {
        game,
        message: "Numbers added successfully".to_string(),
    }))
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
) -> Result<Json<MakeGuessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut service = app_state.add_numbers_service.write().await;

    let game = service
        .make_guess(id, request.player, request.guess)
        .map_err(|error| {
            http_errors::error_response(StatusCode::BAD_REQUEST, "INVALID_OPERATION", &error)
        })?;

    Ok(Json(MakeGuessResponse {
        game,
        message: "Guess made successfully".to_string(),
    }))
}
