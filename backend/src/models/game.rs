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
pub struct TicTacToe {
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
    pub game: TicTacToe,
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
    pub game: TicTacToe,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GameResponse {
    pub game: TicTacToe,
}

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct GameStatusResponse {
    pub status: GameStatus,
    pub current_player: Option<Player>,
}
