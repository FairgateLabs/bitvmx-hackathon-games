

use bitvmx_tictactoe_backend::{api, app_state::AppState, config, rpc::bitvmx_rpc::{RpcService}};
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
    // 1. Load configuration
    let config_file = std::env::var("CONFIG_FILE").unwrap_or_else(|_| "player_1".to_string());
    println!("🔄 Loading configuration from: {config_file}");
    let config = config::Config::load(&config_file).unwrap_or_default();
    
    // 2. Initialize logging
    init_tracing(std::env::var("RUST_LOG").unwrap_or_else(|_| config.logging.level.clone()));

    // 3. Create shutdown signals
    let (shutdown_tx, _) = broadcast::channel::<()>(1);
    
    // 4. Connect to BitVMX RPC, spawn sender and listener tasks
    let (rpc_service, rpc_sender_task, rpc_listener_task) = RpcService::connect(config.bitvmx.broker_port, None, &shutdown_tx);

    // 5. Initialize app state
    let app_state = AppState::new(config.clone(), rpc_service.clone());

    // 6. Spawn setup task that waits for RPC to be ready
    let app_state_setup = app_state.clone();
    let setup_task = tokio::task::spawn(async move {
        // Wait for the RPC client to be ready
        app_state_setup.bitvmx_rpc.wait_for_ready().await;
        
        // Now perform the setup
        {
            let mut store_guard = app_state_setup.bitvmx_store.write().await;
            store_guard.setup(&app_state_setup.bitvmx_rpc).await?;
        }
        info!("BitVMX RPC setup successful");
        
        Ok::<_, anyhow::Error>(()) // coercion to Result
    });

    // 7. Spawn Axum server task
    let app_state_axum = app_state.clone();
    let mut shutdown_rx_axum = shutdown_tx.subscribe();
    let axum_task = tokio::task::spawn(async move {
        // Create the application
        let app = api::app(app_state_axum).await;
        
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


    // 8. Run tasks in parallel with tokio::select!
    tokio::select! {
        res = rpc_sender_task => match res {
            Ok(Ok(())) => warn!("[rpc_sender] finished without errors"),
            Ok(Err(e)) => error!("❌ [rpc_sender] Error at BitVMX RPC: {}", e),
            Err(e) => error!("💥 [rpc_sender] Panic at BitVMX RPC: {}", e),
        },
        res = rpc_listener_task => match res {
            Ok(Ok(())) => warn!("[rpc_listener] finished without errors"),
            Ok(Err(e)) => error!("❌ [rpc_listener] Error at BitVMX RPC: {}", e),
            Err(e) => error!("💥 [rpc_listener] Panic at BitVMX RPC: {}", e),
        },
        res = axum_task => match res {
            Ok(Ok(())) => warn!("API finished without errors"),
            Ok(Err(e)) => error!("❌ Error at API: {}", e),
            Err(e) => error!("💥 Panic at API: {}", e),
        },
        _ = tokio::signal::ctrl_c() => {
            info!("Ctrl-C received, shutting down...");
            // Send shutdown signal to all tasks
            let _ = shutdown_tx.send(());
        },
    }

    // 9. Check if setup task finished correctly
    if let Err(e) = setup_task.await? {
        error!("❌ Error at setup: {e}");
        // Send shutdown signal to all tasks and exit
        let _ = shutdown_tx.send(());
    }

    Ok(())
}
