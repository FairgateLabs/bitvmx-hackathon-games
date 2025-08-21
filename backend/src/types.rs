use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub enum Player {
    X,
    O,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub enum GameStatus {
    Waiting,
    InProgress,
    Won { winner: Player },
    Draw,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct Position {
    pub row: u8,
    pub col: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct Move {
    pub player: Player,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Game {
    #[ts(type = "string")]
    pub id: Uuid,
    pub board: [[Option<Player>; 3]; 3],
    pub current_player: Player,
    pub status: GameStatus,
    pub moves: Vec<Move>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct CreateGameRequest {
    pub player_name: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CreateGameResponse {
    pub game: Game,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct MakeMoveRequest {
    pub player: Player,
    pub position: Position,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MakeMoveResponse {
    pub game: Game,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GameResponse {
    pub game: Game,
}

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct GameStatusResponse {
    pub status: GameStatus,
    pub current_player: Option<Player>,
}

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: u64,
}

// Add Numbers Game Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub enum AddNumbersGameStatus {
    WaitingForNumbers,
    WaitingForGuess,
    Won { winner: String },
    Lost { correct_answer: i32 },
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AddNumbersGame {
    #[ts(type = "string")]
    pub id: Uuid,
    pub player1: String,
    pub player2: String,
    pub number1: Option<i32>,
    pub number2: Option<i32>,
    pub guess: Option<i32>,
    pub status: AddNumbersGameStatus,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct CreateAddNumbersGameRequest {
    pub player1: String,
    pub player2: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CreateAddNumbersGameResponse {
    pub game: AddNumbersGame,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct AddNumbersRequest {
    pub player: String,
    pub number1: i32,
    pub number2: i32,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AddNumbersResponse {
    pub game: AddNumbersGame,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct MakeGuessRequest {
    pub player: String,
    pub guess: i32,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MakeGuessResponse {
    pub game: AddNumbersGame,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AddNumbersGameResponse {
    pub game: AddNumbersGame,
}
