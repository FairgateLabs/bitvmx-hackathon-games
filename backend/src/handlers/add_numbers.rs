use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    stores::AddNumbersStore,
    types::{
        CreateAddNumbersGameRequest, CreateAddNumbersGameResponse, AddNumbersGameResponse,
        AddNumbersRequest, AddNumbersResponse, MakeGuessRequest, MakeGuessResponse, ErrorResponse,
    },
};

/// Create a new add numbers game
pub async fn create_game(
    State(store): State<Arc<Mutex<AddNumbersStore>>>,
    request: CreateAddNumbersGameRequest,
) -> Result<CreateAddNumbersGameResponse, (StatusCode, Json<ErrorResponse>)> {
    let mut store = store.lock().await;
    let game = store.create_game(request.player1, request.player2);

    Ok(CreateAddNumbersGameResponse {
        game,
        message: "Add numbers game created successfully".to_string(),
    })
}

/// Get a specific add numbers game by ID
pub async fn get_game(
    State(store): State<Arc<Mutex<AddNumbersStore>>>,
    id: Uuid,
) -> Result<AddNumbersGameResponse, (StatusCode, Json<ErrorResponse>)> {
    let store = store.lock().await;
    
    let game = store.get_game(id).ok_or((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "NOT_FOUND".to_string(),
            message: "Game not found".to_string(),
        }),
    ))?;

    Ok(AddNumbersGameResponse {
        game: game.clone(),
    })
}

/// Add two numbers to the game
pub async fn add_numbers(
    State(store): State<Arc<Mutex<AddNumbersStore>>>,
    id: Uuid,
    request: AddNumbersRequest,
) -> Result<AddNumbersResponse, (StatusCode, Json<ErrorResponse>)> {
    let mut store = store.lock().await;
    
    let game = store
        .add_numbers(id, request.player, request.number1, request.number2)
        .map_err(|error| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "INVALID_OPERATION".to_string(),
                    message: error,
                }),
            )
        })?;

    Ok(AddNumbersResponse {
        game,
        message: "Numbers added successfully".to_string(),
    })
}

/// Make a guess for the sum
pub async fn make_guess(
    State(store): State<Arc<Mutex<AddNumbersStore>>>,
    id: Uuid,
    request: MakeGuessRequest,
) -> Result<MakeGuessResponse, (StatusCode, Json<ErrorResponse>)> {
    let mut store = store.lock().await;
    
    let game = store
        .make_guess(id, request.player, request.guess)
        .map_err(|error| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "INVALID_OPERATION".to_string(),
                    message: error,
                }),
            )
        })?;

    Ok(MakeGuessResponse {
        game,
        message: "Guess made successfully".to_string(),
    })
}
