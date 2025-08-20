use axum::{
    routing::{get, post},
    response::Html,
    Json,
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;

mod api;
mod config;
mod models;
mod types;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::health::health_check,
        api::game::create_game,
        api::game::get_game,
        api::game::make_move,
        api::game::get_game_status
    ),
    components(
        schemas(
            crate::types::Player,
            crate::types::GameStatus,
            crate::types::Position,
            crate::types::Move,
            crate::types::CreateGameRequest,
            crate::types::MakeMoveRequest,
            crate::types::ErrorResponse,
            crate::types::HealthResponse
        )
    ),
    tags(
        (name = "Game", description = "Tic-tac-toe game management endpoints"),
        (name = "Health", description = "Health check endpoints")
    ),
    info(
        title = "Tic-Tac-Toe API",
        version = "1.0.0",
        description = "A REST API for playing tic-tac-toe games",
        contact(
            name = "BitVMX Hackathon",
            url = "https://github.com/bitvmx-hackathon"
        ),
        license(
            name = "MIT"
        )
    )
)]
struct ApiDoc;

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

    // Initialize shared state
    let game_store = Arc::new(Mutex::new(models::GameStore::new()));

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

            // Build our application with a route
    let app = Router::new()
        .route("/", get(api::health::health_check))
        .route("/api/game", post(api::game::create_game))
        .route("/api/game/:id", get(api::game::get_game))
        .route("/api/game/:id/move", post(api::game::make_move))
        .route("/api/game/:id/status", get(api::game::get_game_status))
        .route("/swagger-ui", get(|| async { Html(include_str!("../swagger-ui.html")) }))
        .route("/api-docs/openapi.json", get(|| async { Json(ApiDoc::openapi()) }))
        .layer(cors)
        .with_state(game_store);

    // Run it
    let addr = config.socket_addr()?;
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service())
        .await?;
    
    Ok(())
}
