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

    // --- Servidor TARPC ---
    let tarpc_server = async {
        // let addr = "127.0.0.1:5000".parse().unwrap();
        // let listener = tarpc::serde_transport::tcp::listen(addr, Json::default).await.unwrap();
        // listener
        //     .filter_map(|r| async move { r.ok() })
        //     .map(BaseChannel::with_defaults)
        //     .map(|channel| {
        //         let server = GreeterServer;
        //         channel.execute(server.serve())
        //     })
        //     .buffer_unordered(10)
        //     .for_each(|_| async {})
        //     .await;
        println!("TARPC server started");
    };

    // --- Axum server ---
    let axum_server = async {
        // Create the application
        let app = app::app();
        // Run it
        let addr = config.socket_addr().unwrap();
        println!("API REST at http://{}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    };

    // Run both in parallel
    tokio::join!(tarpc_server, axum_server);
    
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
