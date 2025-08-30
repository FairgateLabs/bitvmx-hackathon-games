use std::sync::Arc;
use tokio::sync::RwLock;
use crate::types::P2PAddress;
use tracing::{trace, debug};
use uuid::Uuid;
use bitvmx_client::types::{IncomingBitVMXApiMessages, OutgoingBitVMXApiMessages};
use crate::rpc::bitvmx_rpc::BitVMXRpcClient;

#[derive(Debug, Clone)]
pub struct BitVMXStore {
    pub p2p_address: Option<P2PAddress>,
    pub pub_key: Option<String>,
    pub funding_key: Option<String>,
}

impl Default for BitVMXStore {
    fn default() -> Self {
        Self {
            p2p_address: None,
            pub_key: None,
            funding_key: None,
        }
    }
}

impl BitVMXStore {
    pub fn new() -> Self {
        Self::default()
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
    pub async fn setup(&mut self, rpc_client: &Arc<RwLock<BitVMXRpcClient>>) -> Result<(), anyhow::Error> {
        debug!("Get comm info from BitVMX");
        let client_guard = rpc_client.write().await;
        client_guard.send(IncomingBitVMXApiMessages::GetCommInfo()).await?;
        // If keys do not exist, setup keys
        if self.get_pub_key().is_none() {
            trace!("No keys found, creating them");
            self.setup_keys(rpc_client).await?;
        }
        Ok(())
    }

    /// Setup operator and funding keys
    async fn setup_keys(&mut self, rpc_client: &Arc<RwLock<BitVMXRpcClient>>) -> Result<(), anyhow::Error> {
        debug!("Create operator key from BitVMX");
        let client_guard = rpc_client.write().await;
        let pub_key_id = Uuid::new_v4();
        let pub_key_response = client_guard.request(IncomingBitVMXApiMessages::GetPubKey(pub_key_id, true)).await?;
        
        if let OutgoingBitVMXApiMessages::PubKey(_, pub_key) = pub_key_response {
            self.set_pub_key(pub_key.to_string());
        } else {
            return Err(anyhow::anyhow!("Expected Operator PubKey response, got: {:?}", pub_key_response));
        }

        debug!("Create funding key for speedups from BitVMX");
        let speedup_key_id = Uuid::new_v4();
        let funding_key_response = client_guard.request(IncomingBitVMXApiMessages::GetPubKey(speedup_key_id, true)).await?;
        
        if let OutgoingBitVMXApiMessages::PubKey(_, funding_key) = funding_key_response {
            self.set_funding_key(funding_key.to_string());
        } else {
            return Err(anyhow::anyhow!("Expected Funding PubKey response, got: {:?}", funding_key_response));
        }

        Ok(())
    }


}



