use axum::{Json, Router, routing::{get, post}};
use http::StatusCode;
use crate::handlers::bitvmx;
use crate::types::{ErrorResponse, P2PAddress, SetupKey};

pub fn router() -> Router {
    Router::new()
        .route("/comm-info", get(comm_info))
        .route("/setup-aggregated-key", post(setup_aggregated_key))
}

/// Get BitVMX P2P address information
#[utoipa::path(
    get,
    path = "/bitvmx/comm-info",
    responses(
        (status = 200, description = "BitVMX P2P address information", body = P2PAddress),
        (status = 404, description = "P2P address not found", body = ErrorResponse)
    ),
    tag = "BitVMX"
)]
pub async fn comm_info() -> Result<Json<P2PAddress>, (StatusCode, Json<ErrorResponse>)> {
    let response = bitvmx::get_comm_info().await?;
    Ok(Json(response))
}

/// Submit BitVMX setup aggregated key
#[utoipa::path(
    post,
    path = "/bitvmx/setup-aggregated-key",
    request_body = SetupKey,
    responses(
        (status = 200, description = "Setup aggregated key submitted successfully"),
        (status = 400, description = "Invalid setup aggregated key", body = ErrorResponse)
    ),
    tag = "BitVMX"
)]
pub async fn setup_aggregated_key(
    Json(setup_key): Json<SetupKey>
) -> Result<Json<()>, (StatusCode, Json<ErrorResponse>)> {
    bitvmx::submit_aggregated_key(setup_key).await?;
    Ok(Json(()))
}