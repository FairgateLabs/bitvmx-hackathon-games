use std::sync::Arc;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use uuid::Uuid;
use crate::types::P2PAddress;
use bitvmx_client::types::OutgoingBitVMXApiMessages;
use tracing::{debug, error, trace};
use tokio::sync::{oneshot, Mutex};

// Global singleton instance of BitVMXStore
pub static BITVMX_STORE: Lazy<Arc<BitVMXStore>> = Lazy::new(|| {
    Arc::new(BitVMXStore::new())
});

#[derive(Debug, Clone)]
pub struct BitVMXState {
    pub is_connected: bool,
    pub p2p_address: Option<P2PAddress>,
    pub pub_key: Option<String>,
    pub funding_key: Option<String>,
}

impl Default for BitVMXState {
    fn default() -> Self {
        Self {
            is_connected: false,
            p2p_address: None,
            pub_key: None,
            funding_key: None,
        }
    }
}

#[derive(Debug)]
pub struct BitVMXStore {
    state: Arc<Mutex<BitVMXState>>,
    pending_responses: Arc<Mutex<HashMap<String, oneshot::Sender<OutgoingBitVMXApiMessages>>>>,
}

impl BitVMXStore {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(BitVMXState::default())),
            pending_responses: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get a clone of the current state
    pub async fn get_state(&self) -> BitVMXState {
        let state_guard = self.state.lock().await;
        state_guard.clone()
    }

    /// Update connection status
    pub async fn set_connected(&self, connected: bool) {
        let mut state_guard = self.state.lock().await;
        state_guard.is_connected = connected;
        trace!("BitVMX connection status: {}", connected);
    }

    /// Update P2P address
    pub async fn set_p2p_address(&self, address: P2PAddress) {
        let mut state_guard = self.state.lock().await;
        state_guard.p2p_address = Some(address);
        trace!("Updated P2P address in store");
    }

    /// Update pub key
    pub async fn set_pub_key(&self, pub_key: String) {
        let mut state_guard = self.state.lock().await;
        state_guard.pub_key = Some(pub_key);
        trace!("Updated pub key in store");
    }

    /// Update funding key
    pub async fn set_funding_key(&self, funding_key: String) {
        let mut state_guard = self.state.lock().await;
        state_guard.funding_key = Some(funding_key);
        trace!("Updated funding key in store");
    }

    /// Wait for a response of a specific message type with correlation ID
    pub async fn wait_for_response(&self, correlation_id: &Uuid) -> Result<OutgoingBitVMXApiMessages, anyhow::Error> {
        let (tx, rx) = oneshot::channel();
        let key = correlation_id.to_string();
        
        {
            let mut pending_guard = self.pending_responses.lock().await;
            pending_guard.insert(key.clone(), tx);
            trace!("Waiting for response: {}", key);
        }
        
        // Wait for the response with a timeout
        match tokio::time::timeout(std::time::Duration::from_secs(30), rx).await {
            Ok(Ok(response)) => {
                trace!("Received response for: {}", key);
                Ok(response)
            }
            Ok(Err(e)) => {
                error!("Received response error for: {}, error: {:?}", key, e);
                Err(e.into())
            }
            Err(e) => {
                error!("Timeout waiting for response: {}", key);
                // Clean up the pending response
                let mut pending_guard = self.pending_responses.lock().await;
                pending_guard.remove(&key);
                Err(e.into())
            }
        }
    }

    /// Send a response to a waiting request
    pub async fn send_response(&self, correlation_id: &Uuid, response: OutgoingBitVMXApiMessages) -> Result<(), anyhow::Error> {
        let key = correlation_id.to_string();
        let mut pending_guard = self.pending_responses.lock().await;
        
        let tx = match pending_guard.remove(&key) {
            Some(tx) => tx,
            None => {
                return Err(anyhow::anyhow!("No pending request found for: {}", key));
            }
        };
        
        debug!("Sending response for: {}", key);
        if let Err(message) = tx.send(response) {
            return Err(anyhow::anyhow!("Failed to send response key: {}, message: {:?}", key, message));
        }
        Ok(())
    }

    /// Get pub key
    pub async fn get_pub_key(&self) -> Option<String> {
        let state_guard = self.state.lock().await;
        state_guard.pub_key.clone()
    }

    /// Get funding key

    /// Get funding key
    pub async fn get_funding_key(&self) -> Option<String> {
        let state_guard = self.state.lock().await;
        state_guard.funding_key.clone()
    }

    /// Check if connected
    pub async fn is_connected(&self) -> bool {
        let state_guard = self.state.lock().await;
        state_guard.is_connected
    }

    /// Get P2P address
    pub async fn get_p2p_address(&self) -> Option<P2PAddress> {
        let state_guard = self.state.lock().await;
        state_guard.p2p_address.clone()
    }


}

impl Default for BitVMXStore {
    fn default() -> Self {
        Self::new()
    }
}

