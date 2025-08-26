use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use bitvmx_client::{
    client::BitVMXClient,
    types::{IncomingBitVMXApiMessages, OutgoingBitVMXApiMessages::{self, *}, L2_ID},
};
use tracing::{debug, info, trace, warn};
use crate::config;
use crate::types::P2PAddress;
use crate::stores::bitvmx::BITVMX_STORE;

// Thread-safe singleton BitVMXClient
static BITVMX_CLIENT: Lazy<Arc<Mutex<Option<BitVMXClient>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});

/// Initialize the BitVMXClient singleton with custom config
pub fn init_client(config: &config::Config) -> Result<(), anyhow::Error> {
    let client = BitVMXClient::new(config.bitvmx.broker_port, L2_ID);
    info!("Connected to BitVMX RPC at port {}", config.bitvmx.broker_port);

    // Requests one time to get bitvmx information
    // Get the connection info and test the connection
    client.get_comm_info()?;
    trace!("Get comm info from BitVMX");
    
    // Store the client in the singleton
    let mut client_guard = BITVMX_CLIENT.lock().unwrap();
    *client_guard = Some(client);
    
    // Update the BitVMX store to indicate we're connected
    BITVMX_STORE.set_connected(true);
    
    Ok(())
}

/// Get a reference to the BitVMXClient singleton
pub fn get_client() -> Result<Arc<Mutex<Option<BitVMXClient>>>, anyhow::Error> {
    Ok(Arc::clone(&BITVMX_CLIENT))
}

/// Check if the BitVMXClient is initialized
pub fn is_initialized() -> bool {
    let client_guard = BITVMX_CLIENT.lock().unwrap();
    client_guard.is_some()
}

/// Send a message using the singleton client
pub fn send_message(message: IncomingBitVMXApiMessages) -> Result<(), anyhow::Error> {
    let client_guard = BITVMX_CLIENT.lock().unwrap();
    if let Some(client) = &*client_guard {
        // Here you would use the client to send messages
        // For now, just log that we have access to the client
        trace!("Sending message to BitVMX: {:?}", message);
        client.send_message(message)?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("BitVMXClient not initialized"))
    }
}

/// Receive and process messages from BitVMX
pub fn receive_message() -> Result<(), anyhow::Error> {
    let client_guard = BITVMX_CLIENT.lock().unwrap();
    if let Some(client) = &*client_guard {
        let result = client.get_message();
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        if let Some((message, _from)) = result.unwrap() {
            // Send the message to the handler
            response_message(message)?;
        }
    } else {
        return Err(anyhow::anyhow!("BitVMXClient not initialized"));
    }
    Ok(())
}

pub fn response_message(message: OutgoingBitVMXApiMessages) -> Result<(), anyhow::Error> {
    match message {
        Pong() => {
            debug!("Pong received from BitVMX");
        }
        CommInfo(p2p_address) => {
            debug!("CommInfo received from BitVMX: {:?}", p2p_address);
            BITVMX_STORE.set_p2p_address(P2PAddress {
                peer_id: p2p_address.peer_id.0,
                address: p2p_address.address,
            });
        }
        _ => {
            warn!("Unhandled message received from BitVMX: {:?}", message);
        }
    }
    Ok(())
}

