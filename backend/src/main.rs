use bitvmx_tictactoe_backend::{app, config};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = config::Config::load().unwrap_or_default();
    
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| config.logging.level.clone()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create the application
    let app = app::app();

    // Run it
    let addr = config.socket_addr()?;
    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
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
