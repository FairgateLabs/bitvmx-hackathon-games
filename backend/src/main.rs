use std::{thread::sleep, time::Duration};

use bitvmx_tictactoe_backend::{app, config, bitvmx};
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use bitvmx_client::{
    client::BitVMXClient,
    types::L2_ID,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration
    let config = config::Config::load("player_1").unwrap_or_default();
    
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| config.logging.level.clone()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // --- BITVMX RPC connection ---
    let bitvmx_rpc = tokio::task::spawn_blocking(move || {
        // Create the client to connect to BitVMX as a L2
        let client = BitVMXClient::new(config.bitvmx.broker_port, L2_ID);
        // Send a ping bitvmx to check if it is alive
        client.ping()?;
        info!("Connected to BitVMX RPC at port {}", config.bitvmx.broker_port);
        loop {
            let result = client.get_message();
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            if let Some((message, _from)) = result.unwrap() {
                // Send the message to the handler
                bitvmx::handler::outgoing_message(message)?;
            }
            // Wait before checking for new messages
            sleep(Duration::from_millis(100));
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(()) // coercion to Result
    });

    // --- Axum server ---
    let axum_server = tokio::spawn(async move {
        // Create the application
        let app = app::app();
        // Run it
        let addr = config.server_addr()?;
        info!("API REST at http://{}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok::<_, anyhow::Error>(()) // coercion to Result
    });

    // Run both in parallel
    tokio::select! {
        res = bitvmx_rpc => match res {
            Ok(Ok(())) => warn!("BitVMX RPC finished without errors"),
            Ok(Err(e)) => error!("❌ Error at BitVMX RPC: {}", e),
            Err(e) => error!("💥 Panic at BitVMX RPC: {}", e),
        },
        res = axum_server => match res {
            Ok(Ok(())) => warn!("API finished without errors"),
            Ok(Err(e)) => error!("❌ Error at API: {}", e),
            Err(e) => error!("💥 Panic at API: {}", e),
        },
        _ = tokio::signal::ctrl_c() => info!("Ctrl-C received, shutting down..."),
    }

    Ok(())
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation_with_config() {
        let _app = app::app();
        // The app should be created successfully
        assert!(true, "App created successfully with configuration");
    }
}
