use bitvmx_client::bitcoin::secp256k1::PublicKey;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::{P2PAddress, Utxo};

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
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
    pub bitvmx_program_properties: BitVMXProgramProperties,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct BitVMXProgramProperties {
    #[ts(type = "string")]
    #[schema(value_type = String)]
    pub aggregated_key: Option<PublicKey>,
    #[ts(type = "string")]
    #[schema(value_type = String)]
    pub aggregated_key_uuid: Uuid,
    pub participants: Vec<P2PAddress>,
    #[ts(type = "array")]
    #[schema(value_type = Vec<String>)]
    pub participants_keys: Vec<PublicKey>,
    pub my_idx: u16,
    pub leader_idx: u16,
    pub initial_utxo: Option<Utxo>,
    pub player1_bet_utxo: Option<Utxo>,
    pub player2_bet_utxo: Option<Utxo>,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct AddNumbersRequest {
    #[ts(type = "string")]
    #[schema(value_type = String)]
    pub id: Uuid,
    pub number1: i32,
    pub number2: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct AddNumbersResponse {
    pub game: AddNumbersGame,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct MakeGuessRequest {
    pub id: String,
    pub guess: i32,
}
