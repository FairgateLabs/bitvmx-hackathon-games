use std::sync::Arc;
use once_cell::sync::Lazy;
use bitvmx_client::{
    client::BitVMXClient,
    types::{IncomingBitVMXApiMessages, OutgoingBitVMXApiMessages::{self, *}, L2_ID},
};
use tracing::{debug, info, trace, warn};
use crate::app_state::AppState;
use crate::types::P2PAddress;
use crate::stores::bitvmx::BITVMX_STORE;
use uuid::Uuid;
use tokio::sync::Mutex;

// Thread-safe singleton BitVMXClient
static BITVMX_CLIENT: Lazy<Arc<Mutex<Option<BitVMXClient>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});

/// Initialize the BitVMXClient singleton with app state
pub async fn init_client(app_state: &AppState) -> Result<(), anyhow::Error> {
    let config = app_state.get_config().await;
    let client = BitVMXClient::new(config.bitvmx.broker_port, L2_ID);
    info!("Connected to BitVMX RPC at port {}", config.bitvmx.broker_port);

    // Send CommInfo request
    send(IncomingBitVMXApiMessages::GetCommInfo()).await?;
    debug!("Get comm info from BitVMX");

    // Store the client in the singleton
    {
        let mut client_guard = BITVMX_CLIENT.lock().await;
        *client_guard = Some(client);
    } // Lock is automatically released here

    // Update the BitVMX store to indicate we're connected
    BITVMX_STORE.set_connected(true).await;

    Ok(())
}

pub async fn setup_keys() -> Result<(), anyhow::Error> {
    debug!("Create pub key from BitVMX");
    let pub_key_id = Uuid::new_v4();
    let pub_key_response = send_and_wait(IncomingBitVMXApiMessages::GetPubKey(pub_key_id, true), &pub_key_id).await?;
    if let OutgoingBitVMXApiMessages::PubKey(_, pub_key) = pub_key_response {
        BITVMX_STORE.set_pub_key(pub_key.to_string()).await;
    } else {
        return Err(anyhow::anyhow!("Expected PubKey response, got: {:?}", pub_key_response));
    }


    debug!("Create funding key for speedups from BitVMX");
    let speedup_key_id = Uuid::new_v4();
    let funding_key_response = send_and_wait(IncomingBitVMXApiMessages::GetPubKey(speedup_key_id, true), &speedup_key_id).await?;
    if let OutgoingBitVMXApiMessages::PubKey(_, funding_key) = funding_key_response {
        BITVMX_STORE.set_funding_key(funding_key.to_string()).await;
    } else {
        return Err(anyhow::anyhow!("Expected PubKey response, got: {:?}", funding_key_response));
    }

    Ok(())
}

/// Check if the BitVMXClient is initialized
pub async fn is_initialized() -> bool {
    let client_guard = BITVMX_CLIENT.lock().await;
    client_guard.is_some()
}

/// Send a message using the singleton client
pub async fn send(message: IncomingBitVMXApiMessages) -> Result<(), anyhow::Error> {
    let client_guard = BITVMX_CLIENT.lock().await;
    let client = match &*client_guard {
        Some(client) => client,
        None => return Err(anyhow::anyhow!("BitVMXClient not initialized")),
    };
    
    // Here you would use the client to send messages
    // For now, just log that we have access to the client
    trace!("Sending message to BitVMX: {:?}", message);
    client.send_message(message)?;
    Ok(())
}

/// Send a message and wait for a PubKey response
pub async fn send_and_wait(
    message: IncomingBitVMXApiMessages,
    correlation_id: &Uuid,
) -> Result<OutgoingBitVMXApiMessages, anyhow::Error> {
    debug!("Sending message with correlation ID: {}", correlation_id);
    
    // Send the message
    send(message).await?;
    
    // Wait for the response
    match BITVMX_STORE.wait_for_response(&correlation_id).await {
        Ok(response) => {
            debug!("Received response for correlation ID {}: {:?}", correlation_id, response);
            Ok(response)
        }
        Err(e) => {
            Err(anyhow::anyhow!("Error waiting for response: {:?}", e))
        }
    }
}

/// Receive and process messages from BitVMX
pub async fn receive_messages() -> Result<(), anyhow::Error> {
    loop {
        if !receive_message().await? {
            break;
        }
    }
    Ok(())
}

/// Receive and process a single message from BitVMX
/// Returns true if a message was received, false if no message was received
pub async fn receive_message() -> Result<bool, anyhow::Error> {
    let result = {
        let client_guard = BITVMX_CLIENT.lock().await;
        let client = match &*client_guard {
            Some(client) => client,
            None => return Err(anyhow::anyhow!("BitVMXClient not initialized")),
        };
        
        client.get_message()
    }; // Lock is released here
    
    if result.is_err() {
        return Err(result.err().unwrap());
    }
    
    if let Some((message, _from)) = result.unwrap() {
        // Send the message to the handler
        response_message(message).await?;
        return Ok(true);
    }
    
    Ok(false)
}

pub async fn response_message(message: OutgoingBitVMXApiMessages) -> Result<(), anyhow::Error> {
    // Handle specific message types that need direct processing
    match message {
        CommInfo(p2p_address) => {
            debug!("CommInfo received from BitVMX: {:?}", p2p_address);
            BITVMX_STORE.set_p2p_address(P2PAddress {
                peer_id: p2p_address.peer_id.0,
                address: p2p_address.address,
            }).await;
        }
        PubKey(correlation_id, pub_key) => {    
            // Try to send response to any waiting requests
            BITVMX_STORE.send_response(&correlation_id, message.clone()).await?;
            debug!("PubKey received from BitVMX: for message id: {} pub_key: {:?}", correlation_id, pub_key);
        },
        _ => {
            let message_type = message.as_ref();
            warn!("Unhandled message type: {:?}", message_type);
        }
    }
    Ok(())
}

