use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub enum AddNumbersGameStatus {
    SetupParticipants,
    SetupFunding,
    CreateProgram,        // (Player1) Create the program
    BindNumbersToProgram, // (Player1) Here we send the numbers to sum
    SubmitSum, // Participant 2 (Here we send the sum, whenever detect the news then we move to ComputeProgram)
    WaitForSum, // Participant 1 (Here we wait for the sum)
    ProgramDecision, // This should change
    Challenge,
    GameComplete {
        outcome: GameOutcome,
        reason: GameReason,
    },
    TransferBetFunds,
    Finished,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema, PartialEq)]
#[ts(export)]
pub enum GameOutcome {
    Win,
    Lose,
    Draw,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema, PartialEq)]
#[ts(export)]
pub enum GameReason {
    Challenge,
    Timeout,
    Accept,
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
