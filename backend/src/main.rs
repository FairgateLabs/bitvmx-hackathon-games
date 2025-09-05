use bitvmx_client::types::{BITVMX_ID, L2_ID};
use bitvmx_tictactoe_backend::{api, config, rpc::rpc_client::RpcClient, state::AppState};
use tokio::{signal, sync::broadcast};
use tracing::{error, info, trace, warn, Instrument};
use tracing_appender::rolling;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize logs with the given log level
/// It disables the tarpc and broker layers to avoid logging too much information
fn init_tracing(log_level: String, name: String) -> tracing_appender::non_blocking::WorkerGuard {
    // Ensure logs directory exists
    let logs_dir = "logs";
    if !std::path::Path::new(logs_dir).exists() {
        std::fs::create_dir_all(logs_dir).expect("Failed to create logs directory");
    }
    // Log file name
    let log_file = format!("{logs_dir}/{name}.log");
    println!(
        "📝 Logging to: {}",
        std::fs::canonicalize(&log_file)
            .unwrap_or_else(|_| log_file.clone().into())
            .display()
    );

    // Log appender configuration
    let file_appender = rolling::never(log_file, "backend");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // File log format
    let file_log = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_ansi(false) // No colors for file
        .with_writer(non_blocking);

    // Console log format
    let console_log = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_ansi(true); // Keep colors for console

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(format!(
            "{log_level},tarpc=off,broker=off"
        )))
        .with(file_log) // File log first otherwise it will use ansi for file
        .with(console_log)
        .init();
    guard
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load configuration
    let config_file = std::env::var("CONFIG_FILE").unwrap_or_else(|_| "player_1".to_string());
    println!("🔄 Loading configuration from: {config_file}");
    let config = config::Config::load(&config_file).unwrap_or_default();

    // 2. Initialize logging
    let _log_guard = init_tracing(
        std::env::var("RUST_LOG").unwrap_or_else(|_| config.logging.level.clone()),
        config_file.to_string(),
    );

    // Create a span for the main application
    let _main_span = tracing::info_span!("", config = %config_file).entered();

    // 3. Create shutdown signals
    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    // 4. Connect to BitVMX RPC, spawn sender and listener tasks
    let (rpc_client, rpc_sender_task, rpc_listener_task) = RpcClient::connect(
        L2_ID,
        BITVMX_ID,
        config.bitvmx.broker_port,
        None,
        &shutdown_tx,
    );

    // 5. Initialize app state
    let app_state = AppState::new(config.clone(), rpc_client.clone());

    // 6. Spawn setup task that waits for RPC to be ready
    let app_state_setup = app_state.clone();
    let shutdown_rx_setup = shutdown_tx.subscribe();
    let setup_task = tokio::task::spawn(
        async move {
            // Wait for the RPC client to be ready
            app_state_setup.rpc_client.wait_for_ready(shutdown_rx_setup).await;

            // Now perform the setup
            {
                // Setup does multiple things so we should not lock the service,
                // but since this is just a one time task at the beginning, we can do it here
                let mut service_guard = app_state_setup.bitvmx_service.write().await;
                service_guard.initial_setup().await?;
            }
            info!("BitVMX RPC setup successful");

            Ok::<_, anyhow::Error>(()) // coercion to Result
        }
        .instrument(tracing::info_span!("setup")),
    );

    // 7. Spawn Axum server task
    let app_state_axum = app_state.clone();
    let mut shutdown_rx_axum = shutdown_tx.subscribe();
    let axum_task = tokio::task::spawn(
        async move {
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
                    trace!("Axum server shutting down...");
                })
                .await?;

            Ok::<_, anyhow::Error>(()) // coercion to Result
        }
        .instrument(tracing::info_span!("axum_server")),
    );

    // 8. Run tasks in parallel with tokio::select!
    tokio::select! {
        res = rpc_sender_task => match res {
            Ok(Ok(())) => warn!("rpc_sender: Finished without errors"),
            Ok(Err(e)) => {
                error!("❌ rpc_sender: Error: {}", e);
                // Send shutdown signal to all tasks and exit
                let _ = shutdown_tx.send(());
            },
            Err(e) => {
                error!("💥 rpc_sender: Panic: {}", e);
                // Send shutdown signal to all tasks and exit
                let _ = shutdown_tx.send(());
            },
        },
        res = rpc_listener_task => match res {
            Ok(Ok(())) => warn!("rpc_listener: Finished without errors"),
            Ok(Err(e)) => {
                error!("❌ rpc_listener: Error: {}", e);
                // Send shutdown signal to all tasks and exit
                let _ = shutdown_tx.send(());
            },
            Err(e) => {
                error!("💥 rpc_listener: Panic: {}", e);
                // Send shutdown signal to all tasks and exit
                let _ = shutdown_tx.send(());
            },
        },
        res = axum_task => match res {
            Ok(Ok(())) => warn!("axum_server: API Finished without errors"),
            Ok(Err(e)) => {
                error!("❌ axum_server: Error: {}", e);
                // Send shutdown signal to all tasks and exit
                let _ = shutdown_tx.send(());
            },
            Err(e) => {
                error!("💥 axum_server: Panic: {}", e);
                // Send shutdown signal to all tasks and exit
                let _ = shutdown_tx.send(());
            },
        },
        _ = signal::ctrl_c() => {
            info!("Ctrl-C received, shutting down...");
            // Send shutdown signal to all tasks
            let _ = shutdown_tx.send(());
        },
    }

    // 9. Check if setup task finished correctly
    if let Err(e) = setup_task.await? {
        error!("❌ setup: Error: {e}");
        // Send shutdown signal to all tasks and exit
        let _ = shutdown_tx.send(());
    }

    Ok(())
}
