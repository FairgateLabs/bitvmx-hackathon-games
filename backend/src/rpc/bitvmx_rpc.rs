use std::sync::Arc;
use std::collections::HashMap;
use bitvmx_broker::rpc::{BrokerConfig, async_client::AsyncClient};
use bitvmx_client::{
    types::{IncomingBitVMXApiMessages, OutgoingBitVMXApiMessages::{self}, L2_ID, BITVMX_ID},
};
use tracing::{debug, info, trace};

use uuid::Uuid;
use tokio::sync::{Mutex, mpsc, oneshot};
use std::time::Duration;
use crate::rpc::message_queue::MessageQueue;



/// BitVMX RPC Client with async message queue
#[derive(Debug, Clone)]
pub struct BitVMXRpcClient {
    /// Internal Broker RPC client
    client: Option<AsyncClient>,
    /// Message queue for handling communication
    queue: MessageQueue,
}


impl BitVMXRpcClient {
    /// Create a new BitVMX RPC client
    pub fn new() -> Self {
        let queue = MessageQueue::new();
        
        Self {
            client: None,
            queue,
        }
    }

    /// Initialize the Broker RPC client with the specified port
    pub fn init_client(&mut self, broker_port: u16) {
        let config = BrokerConfig::new(broker_port, None);
        let client = AsyncClient::new(&config);
        self.client = Some(client);
    }

    /// Get the message queue for spawning the processor
    pub fn get_queue(&self) -> MessageQueue {
        self.queue.clone()
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
    pub fn inner_client(&self) -> Result<&AsyncClient, anyhow::Error> {
        self.client.as_ref().ok_or_else(|| anyhow::anyhow!("Broker RPC client not initialized"))
    }
}




/// Serve the BitVMX RPC client with message processing
/// Similar to axum::serve, this function runs the RPC client until shutdown
pub async fn serve(
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>
) -> Result<(), anyhow::Error> {
    // Get the client for message processing
    let app_state = crate::app_state::get_app_state_or_panic().await;
    let rpc_client_guard = app_state.get_bitvmx_rpc().await;
    let client = rpc_client_guard.inner_client()?;
    
    // Create a channel for message processing
    let (outgoing_tx, mut outgoing_rx) = mpsc::channel(100);
    
    // Get the global queue and update its sender to use our channel
    let mut global_queue = rpc_client_guard.get_queue();
    global_queue.update_sender(outgoing_tx);

    // Setup BitVMX after message processing is initialized
    let app_state = crate::app_state::get_app_state_or_panic().await;
    let mut store_guard = app_state.bitvmx_store.write().await;
    store_guard.setup(&app_state.bitvmx_rpc).await?;
    info!("BitVMX RPC setup successfull");

    let my_id = L2_ID;
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
                    // BitVMX instance uses ID 1 by convention
                    let serialized = serde_json::to_string(&message)?;
                    trace!("Sending message to BitVMX with correlation_id {correlation_id}: {:?}", serialized);
                    client.send_msg(my_id, BITVMX_ID, serialized).await?;
                }
            }
            // Process incoming messages
            _ = tokio::time::sleep(Duration::from_millis(10)) => {
                match client.get_msg(my_id).await {
                    Ok(Some(msg)) => {
                        trace!("Received message from BitVMX: {:?}", msg);
                        let message = serde_json::from_str(&msg.msg)?;
                        global_queue.handle_response(message).await?;
                        client.ack(my_id, msg.uid).await?;
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


