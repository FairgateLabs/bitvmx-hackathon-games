use std::collections::HashMap;

use bitvmx_client::bitcoin::PublicKey;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::{P2PAddress, Utxo};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub enum AddNumbersGameStatus {
    SetupParticipants, // it stores program id and creates the aggregated key and stores participants
    PlaceBet,          // It sends funds to the agregated address and returns the utxo
    SetupFunding,      // Add other participants utxos
    SetupGame,         // Create the program sending the numbers to sum.
    StartGame, // Player 1 will send the challenge transaction to start the game. Player 2 will wait until see the first challenge transaction.
    SubmitGameData, // After see the first challenge transaction, Player 2 will send the sum to answer the challenge.
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
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema, PartialEq)]
#[ts(export)]
pub enum GameReason {
    Challenge,
    Timeout,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct AddNumbersGame {
    #[ts(type = "string")]
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
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
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub program_id: Uuid,
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
pub struct PlaceBetResponse {
    pub game: AddNumbersGame,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct BitVMXProgramProperties {
    #[ts(type = "string")]
    #[schema(value_type = String, example = "020202020202020202020202020202020202020202020202020202020202020202")]
    pub aggregated_key: PublicKey,
    #[ts(type = "string")]
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub aggregated_id: Uuid,
    pub protocol_address: String,
    pub participants_addresses: Vec<P2PAddress>,
    pub participants_keys: Vec<String>,
    pub funding_protocol_utxo: Option<Utxo>,
    pub funding_bet_utxo: Option<Utxo>,
    pub txs: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct AddNumbersResponse {
    pub game: AddNumbersGame,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct SubmitSumRequest {
    #[ts(type = "string")]
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    pub guess: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct SubmitSumResponse {
    #[ts(type = "string")]
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub program_id: Uuid,
    pub game: AddNumbersGame,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct FundingUtxoRequest {
    #[ts(type = "string")]
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub program_id: Uuid,
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
    #[ts(type = "string")]
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub aggregated_id: Uuid,
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
    /// The generated program ID, this is the program id that will be used to identify the game
    #[ts(type = "string")]
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub program_id: Uuid,
    /// The aggregated key
    #[ts(type = "string")]
    #[schema(value_type = String, example = "020202020202020202020202020202020202020202020202020202020202020202")]
    pub aggregated_key: PublicKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub enum PlayerRole {
    Player1,
    Player2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct SetupGameRequest {
    #[ts(type = "string")]
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub program_id: Uuid,
    #[schema(example = 42)]
    pub number1: u32,
    #[schema(example = 58)]
    pub number2: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct SetupGameResponse {
    #[ts(type = "string")]
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub program_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct StartGameRequest {
    #[ts(type = "string")]
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub program_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct StartGameResponse {
    #[ts(type = "string")]
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub program_id: Uuid,
    pub challenge_tx: serde_json::Value,
}
