use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::types::P2PAddress;
use bitvmx_client::types::OutgoingBitVMXApiMessages;
use tracing::{debug, info};

// Global singleton instance of BitVMXStore
pub static BITVMX_STORE: Lazy<Arc<BitVMXStore>> = Lazy::new(|| {
    Arc::new(BitVMXStore::new())
});

#[derive(Debug, Clone)]
pub struct BitVMXState {
    pub is_connected: bool,
    pub p2p_address: Option<P2PAddress>,
}

impl Default for BitVMXState {
    fn default() -> Self {
        Self {
            is_connected: false,
            p2p_address: None,
        }
    }
}

pub struct BitVMXStore {
    state: Arc<Mutex<BitVMXState>>,
    message_handlers: Arc<Mutex<HashMap<String, Box<dyn Fn(OutgoingBitVMXApiMessages) + Send + Sync>>>>,
}

impl BitVMXStore {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(BitVMXState::default())),
            message_handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get a clone of the current state
    pub fn get_state(&self) -> BitVMXState {
        let state_guard = self.state.lock().unwrap();
        state_guard.clone()
    }

    /// Update connection status
    pub fn set_connected(&self, connected: bool) {
        let mut state_guard = self.state.lock().unwrap();
        state_guard.is_connected = connected;
        info!("BitVMX connection status: {}", connected);
    }

    /// Update P2P address
    pub fn set_p2p_address(&self, address: P2PAddress) {
        let mut state_guard = self.state.lock().unwrap();
        state_guard.p2p_address = Some(address);
        debug!("Updated P2P address in store");
    }

    /// Register a message handler
    pub fn register_handler<F>(&self, name: String, handler: F)
    where
        F: Fn(OutgoingBitVMXApiMessages) + Send + Sync + 'static,
    {
        let mut handlers_guard = self.message_handlers.lock().unwrap();
        handlers_guard.insert(name.clone(), Box::new(handler));
        debug!("Registered message handler: {}", name);
    }

    /// Get all registered handlers
    pub fn get_handlers(&self) -> Arc<Mutex<HashMap<String, Box<dyn Fn(OutgoingBitVMXApiMessages) + Send + Sync>>>> {
        Arc::clone(&self.message_handlers)
    }

    /// Notify all handlers of a message
    pub fn notify_handlers(&self, message: OutgoingBitVMXApiMessages) {
        let handlers_guard = self.message_handlers.lock().unwrap();
        for (name, handler) in handlers_guard.iter() {
            debug!("Notifying handler: {}", name);
            handler(message.clone());
        }
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        let state_guard = self.state.lock().unwrap();
        state_guard.is_connected
    }

    /// Get P2P address
    pub fn get_p2p_address(&self) -> Option<P2PAddress> {
        let state_guard = self.state.lock().unwrap();
        state_guard.p2p_address.clone()
    }


}

impl Default for BitVMXStore {
    fn default() -> Self {
        Self::new()
    }
}
