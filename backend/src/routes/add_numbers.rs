use axum::{Router, routing::{get, post}, extract::{Path, State}, http::StatusCode, Json};
use crate::handlers::add_numbers;
use crate::types::{
    CreateAddNumbersGameRequest, CreateAddNumbersGameResponse, AddNumbersGameResponse,
    AddNumbersRequest, AddNumbersResponse, MakeGuessRequest, MakeGuessResponse, ErrorResponse
};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::stores::AddNumbersStore;
use uuid::Uuid;

pub fn router() -> Router {
    // Initialize shared state
    let add_numbers_store = Arc::new(Mutex::new(AddNumbersStore::new()));

    Router::new()
        .route("/", post(create_game))
        .route("/{id}", get(get_game))
        .route("/{id}/add", post(add_numbers))
        .route("/{id}/guess", post(make_guess))
        .with_state(add_numbers_store)
}

/// Create a new add numbers game
#[utoipa::path(
    post,
    path = "/add-numbers/",
    request_body = CreateAddNumbersGameRequest,
    responses(
        (status = 201, description = "Game created successfully"),
        (status = 400, description = "Invalid request", body = ErrorResponse)
    ),
    tag = "AddNumbers"
)]
pub async fn create_game(
    State(store): State<Arc<Mutex<AddNumbersStore>>>,
    Json(request): Json<CreateAddNumbersGameRequest>,
) -> Result<Json<CreateAddNumbersGameResponse>, (StatusCode, Json<ErrorResponse>)> {
    let response = add_numbers::create_game(State(store), request).await?;
    Ok(Json(response))
}

/// Get a specific add numbers game by ID
#[utoipa::path(
    get,
    path = "/add-numbers/{id}",
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
    State(store): State<Arc<Mutex<AddNumbersStore>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<AddNumbersGameResponse>, (StatusCode, Json<ErrorResponse>)> {
    let response = add_numbers::get_game(State(store), id).await?;
    Ok(Json(response))
}

/// Add two numbers to the game
#[utoipa::path(
    post,
    path = "/add-numbers/{id}/add",
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
    State(store): State<Arc<Mutex<AddNumbersStore>>>,
    Path(id): Path<Uuid>,
    Json(request): Json<AddNumbersRequest>,
) -> Result<Json<AddNumbersResponse>, (StatusCode, Json<ErrorResponse>)> {
    let response = add_numbers::add_numbers(State(store), id, request).await?;
    Ok(Json(response))
}

/// Make a guess for the sum
#[utoipa::path(
    post,
    path = "/add-numbers/{id}/guess",
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
    State(store): State<Arc<Mutex<AddNumbersStore>>>,
    Path(id): Path<Uuid>,
    Json(request): Json<MakeGuessRequest>,
) -> Result<Json<MakeGuessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let response = add_numbers::make_guess(State(store), id, request).await?;
    Ok(Json(response))
}
