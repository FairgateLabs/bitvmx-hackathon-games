use std::sync::Arc;
use crate::models::P2PAddress;
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

    /// Update P2P address
    pub fn set_p2p_address(&mut self, address: P2PAddress) {
        self.p2p_address = Some(address);
        trace!("Updated P2P address in store");
    }

    /// Update pub key
    pub fn set_pub_key(&mut self, pub_key: String) {
        self.pub_key = Some(pub_key);
        trace!("Updated pub key in store");
    }

    /// Update funding key
    pub fn set_funding_key(&mut self, funding_key: String) {
        self.funding_key = Some(funding_key);
        trace!("Updated funding key in store");
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

    /// Setup BitVMX
    pub async fn setup(&mut self) -> Result<(), anyhow::Error> {
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
            self.setup_keys().await?;
        }
        Ok(())
    }

    /// Setup operator and funding keys
    async fn setup_keys(&mut self) -> Result<(), anyhow::Error> {
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



