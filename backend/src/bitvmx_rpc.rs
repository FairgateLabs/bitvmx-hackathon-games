use std::sync::Arc;
use std::collections::HashMap;
use bitvmx_client::{
    client::BitVMXClient,
    types::{IncomingBitVMXApiMessages, OutgoingBitVMXApiMessages::{self, *}, L2_ID},
};
use tracing::{debug, info, trace, warn};
use crate::app_state::AppState;
use crate::types::P2PAddress;

use uuid::Uuid;
use tokio::sync::{Mutex, mpsc, oneshot};
use std::time::Duration;

/// Message queue for handling BitVMX RPC communication
/// Similar to RabbitMQ pattern with correlation ID tracking
#[derive(Clone, Debug)]
pub struct BitVMXMessageQueue {
    /// Channel for sending outgoing messages
    pub outgoing_tx: mpsc::Sender<(Uuid, IncomingBitVMXApiMessages)>,
    /// Response handlers for correlation IDs
    pub response_handlers: Arc<Mutex<HashMap<String, oneshot::Sender<OutgoingBitVMXApiMessages>>>>,
}

impl BitVMXMessageQueue {
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

/// BitVMX RPC Client with async message queue
#[derive(Clone)]
pub struct BitVMXRpcClient {
    /// Internal BitVMX client
    client: Option<BitVMXClient>,
    /// Message queue for handling communication
    queue: BitVMXMessageQueue,
}

impl std::fmt::Debug for BitVMXRpcClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitVMXRpcClient")
            .field("client", &"<BitVMXClient>")
            .field("queue", &self.queue)
            .finish()
    }
}

impl BitVMXRpcClient {
    /// Create a new BitVMX RPC client
    pub fn new() -> Self {
        let queue = BitVMXMessageQueue::new();
        
        Self {
            client: None,
            queue,
        }
    }



    /// Get the message queue for spawning the processor
    pub fn get_queue(&self) -> BitVMXMessageQueue {
        self.queue.clone()
    }

    /// Get the client for spawning the processor
    pub fn get_client(&self) -> Option<BitVMXClient> {
        self.client.clone()
    }

    /// Get the response handlers for spawning the processor
    pub fn get_response_handlers(&self) -> Arc<Mutex<HashMap<String, oneshot::Sender<OutgoingBitVMXApiMessages>>>> {
        self.queue.response_handlers.clone()
    }



    /// Send a message and wait for response
    pub async fn request(&self, message: IncomingBitVMXApiMessages) -> Result<OutgoingBitVMXApiMessages, anyhow::Error> {
        let correlation_id = Uuid::new_v4();
        let (tx, rx) = oneshot::channel();
        
        // Register response handler
        self.queue.register_response(correlation_id, tx).await;
        
        // Send message
        self.queue.send_message(correlation_id, message).await?;
        
        // Wait for response with timeout
        match tokio::time::timeout(
            std::time::Duration::from_secs(30),
            rx
        ).await {
            Ok(Ok(response)) => {
                debug!("Received response for correlation ID: {}", correlation_id);
                Ok(response)
            }
            Ok(Err(_)) => {
                Err(anyhow::anyhow!("Response channel closed for correlation ID: {}", correlation_id))
            }
            Err(_) => {
                Err(anyhow::anyhow!("Timeout waiting for response with correlation ID: {}", correlation_id))
            }
        }
    }

    /// Send a message without waiting for response
    pub async fn send(&self, message: IncomingBitVMXApiMessages) -> Result<(), anyhow::Error> {
        let correlation_id = Uuid::new_v4();
        self.queue.send_message(correlation_id, message).await
    }

    /// Get the underlying client for direct access if needed
    pub fn inner_client(&self) -> Result<&BitVMXClient, anyhow::Error> {
        self.client.as_ref().ok_or_else(|| anyhow::anyhow!("BitVMX client not initialized"))
    }
}


/// Initialize the BitVMX RPC client
pub async fn init_client(app_state: &AppState) -> Result<(), anyhow::Error> {
    let config = app_state.get_config().await;
    let mut client = BitVMXRpcClient::new();
    
    // Initialize the actual BitVMX client
    let bitvmx_client = BitVMXClient::new(config.bitvmx.broker_port, L2_ID);
    client.client = Some(bitvmx_client);
    
    info!("Connected to BitVMX RPC at port {}", config.bitvmx.broker_port);

    // Store the client in the AppState
    app_state.set_bitvmx_rpc(client).await;

    // Update the BitVMX store to indicate we're connected
    let mut store_guard = app_state.bitvmx_store.write().await;
    store_guard.set_connected(true);

    Ok(())
}


/// Check if the BitVMX client is initialized
pub async fn is_initialized() -> bool {
    let app_state = crate::app_state::get_app_state_or_panic().await;
    app_state.get_bitvmx_rpc().await.inner_client().is_ok()
}

/// Get the BitVMX client from AppState
pub async fn get_client() -> BitVMXRpcClient {
    let app_state = crate::app_state::get_app_state_or_panic().await;
    app_state.get_bitvmx_rpc().await
}



/// Send a message using the client from AppState
pub async fn send(message: IncomingBitVMXApiMessages) -> Result<(), anyhow::Error> {
    let client = get_client().await;
    client.send(message).await
}

/// Send a message and wait for response
pub async fn request(message: IncomingBitVMXApiMessages) -> Result<OutgoingBitVMXApiMessages, anyhow::Error> {
    let client = get_client().await;
    client.request(message).await
}

/// Serve the BitVMX RPC client with message processing
/// Similar to axum::serve, this function runs the RPC client until shutdown
pub async fn serve(
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>
) -> Result<(), anyhow::Error> {
    // Get the client for message processing
    let client_guard = get_client().await;
    let client = client_guard.inner_client()?;
    
    // Create a channel for message processing
    let (outgoing_tx, mut outgoing_rx) = mpsc::channel(100);
    
    // Get the global queue and update its sender to use our channel
    let mut global_queue = client_guard.get_queue();
    global_queue.update_sender(outgoing_tx);

    // Message processing loop with shutdown handling
    loop {
        // Check if shutdown signal was received
        if shutdown_rx.try_recv().is_ok() {
            info!("BitVMX RPC shutting down...");
            break;
        }
        
        // Process messages with timeout to allow shutdown check
        tokio::select! {
            // Process outgoing messages
            msg = outgoing_rx.recv() => {
                if let Some((correlation_id, message)) = msg {
                    trace!("Sending message with correlation ID: {}", correlation_id);
                    client.async_send_message(message).await?;
                }
            }
            // Process incoming messages
            _ = tokio::time::sleep(Duration::from_millis(10)) => {
                match client.async_get_message().await {
                    Ok(Some((message, _from))) => {
                        trace!("Received message from BitVMX: {:?}", message);
                        global_queue.handle_response(message).await?;
                    }
                    Ok(None) => {
                        // No message received, continue
                    }
                    Err(e) => {
                        return Err(anyhow::anyhow!("Error getting message: {}", e));
                    }
                }
            }
        }
    }
    
    Ok(())
}

/// Legacy function for compatibility - now just sends without waiting
pub async fn send_and_wait(
    message: IncomingBitVMXApiMessages,
    _correlation_id: &Uuid,
) -> Result<OutgoingBitVMXApiMessages, anyhow::Error> {
    // Use the new request method
    request(message).await
}


