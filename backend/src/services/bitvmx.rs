use std::sync::Arc;
use crate::models::{AggregatedKey, P2PAddress};
use bitcoin::PublicKey;
use tracing::{trace, debug};
use uuid::Uuid;
use bitvmx_client::types::{IncomingBitVMXApiMessages, OutgoingBitVMXApiMessages};
use crate::rpc::rpc_client::RpcClient;

#[derive(Debug, Clone)]
pub struct BitVMXService {
    pub p2p_address: Option<P2PAddress>,
    pub pub_key: Option<String>,
    pub funding_key: Option<String>,
    /// BitVMX RPC client
    pub rpc_client: Arc<RpcClient>,
}

impl BitVMXService {
    pub fn new(rpc_client: Arc<RpcClient>) -> Self {
        Self {
            p2p_address: None,
            pub_key: None,
            funding_key: None,
            rpc_client: rpc_client,
        }
    }

    /// Get pub key
    pub fn get_pub_key(&self) -> Option<String> {
        self.pub_key.clone()
    }

    /// Get funding key
    pub fn get_funding_key(&self) -> Option<String> {
        self.funding_key.clone()
    }

    /// Get P2P address
    pub fn get_p2p_address(&self) -> Option<P2PAddress> {
        self.p2p_address.clone()
    }

    /// Create aggregated key
    pub async fn create_agregated_key(&self, uuid: Uuid, p2p_addresses: Vec<P2PAddress>, operator_keys: Option<Vec<PublicKey>>, leader_idx: u16) -> Result<AggregatedKey, anyhow::Error> {
        trace!("Create aggregated key from BitVMX");
        let addresses = p2p_addresses.iter().map(|p2p| bitvmx_client::types::P2PAddress {
            address: p2p.address.clone(),
            peer_id: bitvmx_client::types::PeerId(p2p.peer_id.clone()),
        }).collect();
        let message = IncomingBitVMXApiMessages::SetupKey(uuid, addresses, operator_keys, leader_idx);
    
        let response = self.rpc_client.send_request(message).await?;

        if let OutgoingBitVMXApiMessages::AggregatedPubkey(uuid, aggregated_pubkey) = response {
            trace!("Obtained aggregated key: {:?}", aggregated_pubkey);
            Ok(AggregatedKey {
                uuid: uuid.to_string(),
                aggregated_key: aggregated_pubkey.to_string(),
            })
        } else {
            Err(anyhow::anyhow!("Expected AggregatedPubkey response, got: {:?}", response))
        }
    }

    /// Get aggregated key
    pub async fn get_aggregated_key(&self, uuid: Uuid) -> Result<AggregatedKey, anyhow::Error> {
        trace!("Get aggregated key from BitVMX");
        let response = self.rpc_client.send_request(IncomingBitVMXApiMessages::GetAggregatedPubkey(uuid)).await?;
        if let OutgoingBitVMXApiMessages::AggregatedPubkey(uuid, aggregated_pubkey) = response {
            trace!("Obtained aggregated key: {:?}", aggregated_pubkey);
            Ok(AggregatedKey {
                uuid: uuid.to_string(),
                aggregated_key: aggregated_pubkey.to_string(),
            })
        } else if let OutgoingBitVMXApiMessages::AggregatedPubkeyNotReady(uuid) = response {
            Err(anyhow::anyhow!("Aggregated key not ready: {:?}", uuid))
        } else {
            Err(anyhow::anyhow!("Expected AggregatedPubkey response, got: {:?}", response))
        }
    }



    /// Update P2P address
    fn set_p2p_address(&mut self, address: P2PAddress) {
        self.p2p_address = Some(address);
        trace!("Updated P2P address in store");
    }

    /// Update pub key
    fn set_pub_key(&mut self, pub_key: String) {
        self.pub_key = Some(pub_key);
        trace!("Updated pub key in store");
    }

    /// Update funding key
    fn set_funding_key(&mut self, funding_key: String) {
        self.funding_key = Some(funding_key);
        trace!("Updated funding key in store");
    }

    /// Setup BitVMX
    pub async fn initial_setup(&mut self) -> Result<(), anyhow::Error> {
        debug!("Get comm info from BitVMX");
        let comm_info_response = self.rpc_client.send_request(IncomingBitVMXApiMessages::GetCommInfo()).await?;
        // Set P2P address
        if let OutgoingBitVMXApiMessages::CommInfo(comm_info) = comm_info_response {
            self.set_p2p_address(P2PAddress {
                address: comm_info.address.clone(),
                peer_id: comm_info.peer_id.to_string(),
            });
        } else {
            return Err(anyhow::anyhow!("Expected Comm Info response, got: {:?}", comm_info_response));
        }

        // If keys do not exist, setup keys
        if self.get_pub_key().is_none() {
            debug!("No keys found, creating them");
            self.initial_setup_keys().await?;
        }
        Ok(())
    }

    /// Setup operator and funding keys
    async fn initial_setup_keys(&mut self) -> Result<(), anyhow::Error> {
        debug!("Create operator key from BitVMX");
        let pub_key_id = Uuid::new_v4();
        let pub_key_response = self.rpc_client.send_request(IncomingBitVMXApiMessages::GetPubKey(pub_key_id, true)).await?;
        
        if let OutgoingBitVMXApiMessages::PubKey(_, pub_key) = pub_key_response {
            self.set_pub_key(pub_key.to_string());
        } else {
            return Err(anyhow::anyhow!("Expected Operator PubKey response, got: {:?}", pub_key_response));
        }

        debug!("Create funding key for speedups from BitVMX");
        let speedup_key_id = Uuid::new_v4();
        let funding_key_response = self.rpc_client.send_request(IncomingBitVMXApiMessages::GetPubKey(speedup_key_id, true)).await?;
        
        if let OutgoingBitVMXApiMessages::PubKey(_, funding_key) = funding_key_response {
            self.set_funding_key(funding_key.to_string());
        } else {
            return Err(anyhow::anyhow!("Expected Funding PubKey response, got: {:?}", funding_key_response));
        }

        Ok(())
    }


}



