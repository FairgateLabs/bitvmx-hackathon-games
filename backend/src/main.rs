use std::time::Duration;

use bitvmx_tictactoe_backend::{api, config, bitvmx_rpc, app_state};
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::sync::broadcast;


/// Initialize logs with the given log level
/// It disables the tarpc and broker layers to avoid logging too much information
fn init_tracing(log_level: String) {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            format!("{log_level},tarpc=off,broker=off"),
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
        )
        .init();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration
    let config_file = std::env::var("CONFIG_FILE").unwrap_or_else(|_| "player_1".to_string());
    println!("--- Loading configuration from {config_file} ---");
    let config = config::Config::load(&config_file).unwrap_or_default();
    
    // Initialize logs
    init_tracing(std::env::var("RUST_LOG").unwrap_or_else(|_| config.logging.level.clone()));

    // Create shutdown signal
    let (shutdown_tx, _) = broadcast::channel::<()>(1);
    let mut shutdown_rx_bitvmx = shutdown_tx.subscribe();
    
    // Initialize shared app state
    app_state::init_app_state(config.clone()).await;
    
    // --- BITVMX RPC connection ---
    let bitvmx_rpc = tokio::task::spawn(async move {
        // Get the shared app state
        let app_state = app_state::get_app_state_or_panic().await;
        
        // Initialize the singleton BitVMXClient
        bitvmx_rpc::handler::init_client(&app_state).await?;
        info!("BitVMX RPC client initialized successfully");
        
        // Check for shutdown signal every 100ms
        loop {
            // Check if shutdown signal was received
            if shutdown_rx_bitvmx.try_recv().is_ok() {
                info!("BitVMX RPC shutting down...");
                break;
            }
            
            // Receive and process messages from BitVMX
            bitvmx_rpc::handler::receive_message().await?;
            
            // Wait before checking for new messages
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        Ok::<_, anyhow::Error>(()) // coercion to Result
    });

    // --- Axum server ---
    let mut shutdown_rx_axum = shutdown_tx.subscribe();
    let axum_server = tokio::task::spawn(async move {
        // Get the shared app state
        let app_state = app_state::get_app_state_or_panic().await;
        
        // Create the application
        let app = api::app(app_state).await;
        
        // Run it
        let addr = config.server_addr()?;
        info!("API REST at http://{}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        
        // Use graceful shutdown
        axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                let _ = shutdown_rx_axum.recv().await;
                info!("Axum server shutting down...");
            })
            .await?;
            
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
        _ = tokio::signal::ctrl_c() => {
            info!("Ctrl-C received, shutting down...");
            // Send shutdown signal to both tasks
            let _ = shutdown_tx.send(());
        },
    }

    Ok(())
}
