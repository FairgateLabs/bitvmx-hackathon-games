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
    CreateProgram, // Create the program, uses the aggregated key and participants to send the 2 numbers to sum
    SubmitSum, // Participant 2 (Here we send the sum, whenever detect the news then we move to ComputeProgram)
    GameComplete {
        outcome: GameOutcome,
        reason: GameReason,
    },
    // TransferBetFunds,
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
    pub role: PlayerRole,
    pub number1: Option<u32>,
    pub number2: Option<u32>,
    pub guess: Option<u32>,
    pub status: AddNumbersGameStatus,
    pub created_at: u64,
    pub updated_at: u64,
    pub bitvmx_program_properties: BitVMXProgramProperties,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
pub struct PlaceBetRequest {
    #[ts(type = "string")]
    #[schema(value_type = String)]
    pub program_id: Uuid,
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
    pub participants_keys: Vec<String>,
    pub funding_protocol_utxo: Option<Utxo>,
    pub funding_bet_utxo: Option<Utxo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct AddNumbersResponse {
    pub game: AddNumbersGame,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct MakeGuessRequest {
    #[ts(type = "string")]
    #[schema(value_type = String)]
    pub id: Uuid,
    pub guess: u32,
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
    pub funding_protocol_utxo: Option<Utxo>,
    pub funding_bet_utxo: Option<Utxo>,
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
    /// The role of the player. This is used to determine the role in bitvmx program.
    /// Player 1 is the player that creates the game and player 2 is the player that joins the game.
    pub role: PlayerRole,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct SetupParticipantsResponse {
    /// The generated program ID
    pub program_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub enum PlayerRole {
    Player1,
    Player2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct StartGameRequest {
    #[ts(type = "string")]
    #[schema(value_type = String)]
    pub program_id: Uuid,
    pub number1: u32,
    pub number2: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct StartGameResponse {
    #[ts(type = "string")]
    #[schema(value_type = String)]
    pub program_id: Uuid,
}