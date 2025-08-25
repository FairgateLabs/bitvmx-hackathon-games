use bitvmx_client::types::OutgoingBitVMXApiMessages;
use tracing::{info, warn};

pub fn outgoing_message(message: OutgoingBitVMXApiMessages) -> Result<(), anyhow::Error> {
    match message {
        OutgoingBitVMXApiMessages::Pong() => {
            info!("Pong received from BitVMX");
        }
        _ => {
            warn!("Unhandled message received from BitVMX: {:?}", message);
        }
    }
    Ok(())
}