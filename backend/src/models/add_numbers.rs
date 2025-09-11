use bitvmx_client::bitcoin::PublicKey;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::{P2PAddress, Utxo};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub enum AddNumbersGameStatus {
    // Crear Ids del programa y compartirlos al otro jugador
    SetupParticipants, // it stores program id and creates the aggregated key and stores participants
    PlaceBet,          // It sends funds to the agregated address and returns the utxo
    SetupFunding,      // Add other participants utxos
    CreateProgram,     // Create the program, uses the aggregated key and participants
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
    pub program_id: Uuid,
    pub number1: Option<i32>,
    pub number2: Option<i32>,
    pub guess: Option<i32>,
    pub status: AddNumbersGameStatus,
    pub created_at: u64,
    pub updated_at: u64,
    pub bitvmx_program_properties: BitVMXProgramProperties,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
pub struct PlaceBetRequest {
    pub program_id: String,
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
pub struct PlaceBetResponse {
    pub funding_protocol_utxo: Utxo,
    pub funding_bet_utxo: Utxo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct BitVMXProgramProperties {
    #[ts(type = "string")]
    #[schema(value_type = String)]
    pub aggregated_key: PublicKey,
    #[ts(type = "string")]
    #[schema(value_type = String)]
    pub aggregated_id: Uuid,
    pub protocol_address: String,
    pub participants_addresses: Vec<P2PAddress>,
    #[ts(type = "array")]
    #[schema(value_type = Vec<String>)]
    pub participants_keys: Vec<String>,
    pub funding_protocol_utxo: Option<Utxo>,
    pub funding_bet_utxo: Option<Utxo>,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct FundingUtxoRequest {
    pub program_id: String,
    pub funding_protocol_utxo: Utxo,
    pub funding_bet_utxo: Utxo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct FundingUtxosResponse {
    pub funding_protocol_utxo: Utxo,
    pub funding_bet_utxo: Utxo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct SetupParticipantsRequest {
    /// The UUID of the aggregated key, must be the same for all participants
    pub aggregated_id: String,
    /// The P2P addresses of the bitvmx nodes in the aggregated key
    pub participants_addresses: Vec<P2PAddress>,
    /// The operator keys in hex format
    pub participants_keys: Vec<String>,
    /// The leader index of the aggregated key
    pub leader_idx: u16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct SetupParticipantsResponse {
    /// The generated program ID
    pub program_id: String,
}
