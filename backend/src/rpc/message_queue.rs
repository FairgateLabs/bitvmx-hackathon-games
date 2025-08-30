use std::sync::Arc;
use std::collections::HashMap;
use bitvmx_client::{
    types::{IncomingBitVMXApiMessages, OutgoingBitVMXApiMessages::{self, *}},
};
use tracing::{debug, trace, warn};
use crate::types::P2PAddress;

use uuid::Uuid;
use tokio::sync::{Mutex, mpsc, oneshot};

/// Message queue for handling BitVMX RPC communication
/// Similar to RabbitMQ pattern with correlation ID tracking
#[derive(Clone, Debug)]
pub struct MessageQueue {
    /// Channel for sending outgoing messages
    pub outgoing_tx: mpsc::Sender<(Uuid, IncomingBitVMXApiMessages)>,
    /// Response handlers for correlation IDs
    pub response_handlers: Arc<Mutex<HashMap<String, oneshot::Sender<OutgoingBitVMXApiMessages>>>>,
}

impl MessageQueue {
    pub fn new() -> Self {
        let (outgoing_tx, _) = mpsc::channel(100);
        Self {
            outgoing_tx,
            response_handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn with_sender(outgoing_tx: mpsc::Sender<(Uuid, IncomingBitVMXApiMessages)>) -> Self {
        Self {
            outgoing_tx,
            response_handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn update_sender(&mut self, outgoing_tx: mpsc::Sender<(Uuid, IncomingBitVMXApiMessages)>) {
        self.outgoing_tx = outgoing_tx;
    }

    /// Register a response handler for a correlation ID
    pub async fn register_response(&self, correlation_id: Uuid, tx: oneshot::Sender<OutgoingBitVMXApiMessages>) {
        let mut handlers = self.response_handlers.lock().await;
        handlers.insert(correlation_id.to_string(), tx);
        trace!("Registered response handler for correlation ID: {}", correlation_id);
    }

    /// Send a message to the queue
    pub async fn send_message(&self, correlation_id: Uuid, message: IncomingBitVMXApiMessages) -> Result<(), anyhow::Error> {
        self.outgoing_tx.send((correlation_id, message)).await
            .map_err(|e| anyhow::anyhow!("Failed to send message to queue: {}", e))
    }

    /// Handle an incoming response
    pub async fn handle_response(&self, response: OutgoingBitVMXApiMessages) -> Result<(), anyhow::Error> {
        match response {
            PubKey(correlation_id, _pub_key) => {
                let key = correlation_id.to_string();
                let mut handlers = self.response_handlers.lock().await;
                
                if let Some(tx) = handlers.remove(&key) {
                    debug!("Sending PubKey response for correlation ID: {}", correlation_id);
                    if let Err(_) = tx.send(response) {
                        warn!("Failed to send response for correlation ID: {}", correlation_id);
                    }
                } else {
                    warn!("No response handler found for correlation ID: {}", correlation_id);
                }
            }
            CommInfo(p2p_address) => {
                debug!("CommInfo received from BitVMX: {:?}", p2p_address);
                let app_state = crate::app_state::get_app_state_or_panic().await;
                let mut store_guard = app_state.bitvmx_store.write().await;
                store_guard.set_p2p_address(P2PAddress {
                    peer_id: p2p_address.peer_id.0,
                    address: p2p_address.address,
                });
            }
            _ => {
                let message_type = response.as_ref();
                warn!("Unhandled message type: {:?}", message_type);
            }
        }
        Ok(())
    }
}
