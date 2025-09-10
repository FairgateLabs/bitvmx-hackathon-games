use bitvmx_client::bitcoin::Txid;
use bitvmx_client::program::variables::PartialUtxo;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct P2PAddress {
    /// The address of the P2P node
    pub address: String,
    /// The peer ID of the P2P node
    pub peer_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct OperatorKeys {
    /// The public key in hex format
    pub pub_key: String,
    /// The funding key in hex format
    pub funding_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct AggregatedKeyRequest {
    /// The UUID of the aggregated key
    pub uuid: String,
    /// The P2P addresses of the bitvmx nodes in the aggregated key
    pub p2p_addresses: Vec<P2PAddress>,
    /// The operator keys in hex format
    pub operator_keys: Option<Vec<String>>,
    /// The leader index of the aggregated key
    pub leader_idx: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct AggregatedKeyResponse {
    /// The UUID of the aggregated key
    pub uuid: String,
    /// The aggregated public key in hex format
    pub aggregated_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct WalletBalance {
    /// The address of the wallet
    pub address: String,
    /// The balance of the wallet in satoshis
    pub balance: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct SendFundsRequest {
    /// The destination hex public key or address to send funds to
    pub destination: String,
    /// The scripts in hex format if its a P2TR x only public key destination
    pub scripts: Option<Vec<String>>,
    /// The amount to send in satoshis
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct Utxo {
    /// The transaction ID of the sent funds
    pub txid: String,
    /// The output index
    pub vout: u32,
    /// The amount in satoshis
    pub amount: u64,
    /// The output type
    pub output_type: serde_json::Value,
}

impl From<Utxo> for PartialUtxo {
    fn from(utxo: Utxo) -> Self {
        // Parse the txid string into a Txid
        let txid = Txid::from_str(&utxo.txid).expect("Invalid txid format");

        // Convert the output_type from serde_json::Value to OutputType
        let output_type =
            serde_json::from_value(utxo.output_type).expect("Invalid output type format");

        (txid, utxo.vout, Some(utxo.amount), output_type)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct TransactionResponse {
    /// The transaction ID
    pub txid: String,
    /// The transaction status
    pub status: String,
    /// The number of confirmations
    pub confirmations: u32,
    /// The block height
    pub block_height: u32,
    /// The block hash
    pub block_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct ProgramSetupRequest {
    /// The program ID
    pub program_id: String,
    /// The participants
    pub participants: Vec<P2PAddress>,
    /// The aggregated key
    pub aggregated_key: String,
    /// The initial utxo
    pub initial_utxo: Utxo,
    /// The prover win utxo
    pub prover_win_utxo: Utxo,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct ProgramSetupResponse {
    /// The program ID
    pub program_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct ProtocolCostResponse {
    /// The program ID
    pub protocol_cost: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct MyFundingUtxoResponse {
    /// The funding UTXO for the current participant
    pub utxo: Utxo,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct OtherParticipantFundingUtxoRequest {
    /// The other participant's funding UTXO
    pub utxo: Utxo,
}
