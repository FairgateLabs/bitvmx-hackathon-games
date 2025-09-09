use axum::Router;
use http::{HeaderName, HeaderValue, Request};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

use crate::config;
use crate::middleware::logging::LoggingLayer;
use crate::routes;
use crate::state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::health::health_check,
        routes::add_numbers::create_game,
        routes::add_numbers::get_game,
        routes::add_numbers::make_guess,
        routes::bitvmx::comm_info,
        routes::bitvmx::operator_keys,
        routes::bitvmx::submit_aggregated_key,
        routes::bitvmx::get_aggregated_key,
        routes::bitvmx::wallet_balance,
        routes::bitvmx::send_funds,
        routes::bitvmx::get_transaction,
        routes::bitvmx::program_setup,
        routes::bitvmx::get_protocol_cost,
    ),
    components(
        schemas(
            crate::models::ErrorResponse,
            crate::models::HealthResponse,
            crate::models::AddNumbersGameStatus,
            crate::models::AddNumbersRequest,
            crate::models::MakeGuessRequest,
            crate::models::P2PAddress,
            crate::models::OperatorKeys,
            crate::models::AggregatedKeyRequest,
            crate::models::AggregatedKeyResponse,
            crate::models::WalletBalance,
            crate::models::SendFundsRequest,
            crate::models::Utxo,
            crate::models::TransactionResponse,
            crate::models::ProgramSetupRequest,
            crate::models::ProgramSetupResponse,
            crate::models::ProtocolCostResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "AddNumbers", description = "Add numbers game management endpoints"),
        (name = "BitVMX", description = "BitVMX communication endpoints")
    ),
    info(
        title = "BitVMX API",
        version = "1.0.0",
        description = "A REST API for BitVMX operations and add numbers games",
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

/// Create the application router with all routes and middleware
///
/// Error handling is implemented at the endpoint level:
/// - Each endpoint returns Result<T, (StatusCode, Json<ErrorResponse>)>
/// - Proper HTTP status codes for different error scenarios
/// - Structured error responses with meaningful messages
/// - Game logic validation (invalid operations, game not found, etc.)
pub async fn app(app_state: AppState) -> Router {
    // Configure CORS
    let cors = create_cors_layer(&app_state.config);

    // Configure trace layer with custom span names
    let trace_layer = TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
        let request_id = Uuid::new_v4();
        tracing::info_span!(
            "request",
            method = %request.method(),
            uri = %request.uri(),
            id = %request_id,
        )
    });

    // Build our application with routes and middleware
    Router::new()
        .nest("/api/health", routes::health::router())
        .nest("/api/add-numbers", routes::add_numbers::router())
        .nest("/api/bitvmx", routes::bitvmx::router())
        .merge(SwaggerUi::new("/").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(LoggingLayer::new(1024)) // Limit the body log to 1024 bytes
        .layer(trace_layer)
        .layer(cors)
        .with_state(app_state)
}

/// Create CORS layer based on configuration
fn create_cors_layer(config: &config::Config) -> CorsLayer {
    let mut cors_layer = CorsLayer::new().allow_methods([http::Method::GET, http::Method::POST]);

    // Configure origins
    if config.cors.allowed_origins.contains(&"*".to_string()) {
        // If wildcard is specified, allow all origins
        cors_layer = cors_layer.allow_origin(Any);
    } else {
        // Use specific origins from config
        for origin in &config.cors.allowed_origins {
            if let Ok(origin_header) = origin.parse::<HeaderValue>() {
                cors_layer = cors_layer.allow_origin(origin_header);
            }
        }
    }

    // Configure headers
    if config.cors.allowed_headers.contains(&"*".to_string()) {
        cors_layer = cors_layer.allow_headers(Any);
    } else {
        let headers: Vec<HeaderName> = config
            .cors
            .allowed_headers
            .iter()
            .filter_map(|header| header.parse().ok())
            .collect();
        if !headers.is_empty() {
            cors_layer = cors_layer.allow_headers(headers);
        }
    }

    cors_layer
}
