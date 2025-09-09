use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub enum AddNumbersGameStatus {
    WaitingForNumbers,
    WaitingForGuess,
    Guessed,
    Finished,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct AddNumbersGame {
    #[ts(type = "string")]
    #[schema(value_type = String)]
    pub id: Uuid,
    pub number1: i32,
    pub number2: i32,
    pub guess: Option<i32>,
    pub status: AddNumbersGameStatus,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddNumbersGameSchema {
    pub id: String,
    pub number1: i32,
    pub number2: i32,
    pub guess: Option<i32>,
    pub status: AddNumbersGameStatus,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct AddNumbersRequest {
    #[ts(type = "string")]
    #[schema(value_type = String)]
    pub id: Uuid,
    pub number1: i32,
    pub number2: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddNumbersRequestSchema {
    pub id: String,
    pub number1: i32,
    pub number2: i32,
}

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct AddNumbersResponse {
    pub game: AddNumbersGame,
}

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct MakeGuessRequest {
    pub id: String,
    pub guess: i32,
}
