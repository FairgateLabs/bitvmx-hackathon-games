use bitvmx_client::bitcoin::Txid;
use bitvmx_client::program::participant::P2PAddress as BitVMXP2PAddress;
use bitvmx_client::p2p_handler::PeerId;
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

impl From<P2PAddress> for BitVMXP2PAddress {
    fn from(p2p: P2PAddress) -> Self {
        BitVMXP2PAddress {
            address: p2p.address.clone(),
            peer_id: PeerId(p2p.peer_id.clone()),
        }
    }
}

impl From<BitVMXP2PAddress> for P2PAddress {
    fn from(p2p: BitVMXP2PAddress) -> Self {
        P2PAddress {
            address: p2p.address.clone(),
            peer_id: p2p.peer_id.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct OperatorKeys {
    /// The public key in hex format
    pub pub_key: String,
    /// The funding key in hex format
    pub funding_key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct AggregatedKeyResponse {
    /// The UUID of the aggregated key
    pub uuid: String,
    /// The aggregated public key in hex format
    pub aggregated_key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct WalletBalance {
    /// The address of the wallet
    pub address: String,
    /// The balance of the wallet in satoshis
    pub balance: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
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

impl From<PartialUtxo> for Utxo {
    fn from(partial_utxo: PartialUtxo) -> Self {
        let txid = partial_utxo.0.to_string();
        let vout = partial_utxo.1;
        let amount = partial_utxo.2.unwrap_or_default();
        // Convert the output_type to serde_json::Value
        let output_type = serde_json::to_value(partial_utxo.3).expect("Invalid output type format");
        Self {
            txid,
            vout,
            amount,
            output_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
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


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct ProtocolCostResponse {
    /// The program ID
    pub protocol_cost: u64,
}
