use bitvmx_client::types::OutgoingBitVMXApiMessages;
use tracing::info;

pub fn outgoing_message(message: OutgoingBitVMXApiMessages) -> Result<(), anyhow::Error> {
    match message {
        OutgoingBitVMXApiMessages::Pong() => {
            info!("Pong received from BitVMX");
        }
        _ => {
            info!("Message received from BitVMX: {:?}", message);
        }
    }
    Ok(())
}