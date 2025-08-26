use axum::Json;
use bitvmx_client::types::IncomingBitVMXApiMessages;
use http::StatusCode;
use uuid::Uuid;
use crate::bitvmx_rpc;
use crate::stores::bitvmx::BITVMX_STORE;
use crate::types::{ErrorResponse, P2PAddress, SetupKey};
use crate::http_errors;
use tracing::instrument;

/// Get BitVMX P2P address information
#[instrument]
pub async fn get_comm_info() -> Result<P2PAddress, (StatusCode, Json<ErrorResponse>)> {
    let p2p_address = BITVMX_STORE.get_p2p_address().ok_or(http_errors::not_found("P2P address not found"))?;
    Ok(p2p_address)
}

/// Submit BitVMX setup aggregated key to the client
#[instrument]
pub async fn submit_aggregated_key(setup_key: SetupKey) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    // Validate the setup key
    if setup_key.id.is_empty() {
        return Err(http_errors::bad_request("Setup key ID cannot be empty"));
    }
    // let id = Uuid::parse_str(&setup_key.id).map_err(|_| http_errors::bad_request("Invalid setup key ID"))?;

    if setup_key.addresses.is_empty() {
        return Err(http_errors::bad_request("At least one P2P address is required"));
    }

    // let p2p_addresses = setup_key.addresses.iter().map(|addr| BitVMXP2PAddress {
    //     address: addr.address.clone(),
    //     peer_id: PeerId::from_str(&addr.peer_id).map_err(|_| http_errors::bad_request("Invalid peer ID"))?,
    // }).collect::<Vec<_>>();

    // bitvmx_rpc::handler::send_message(IncomingBitVMXApiMessages::SetupKey(id, p2p_addresses, None, 0))
    //     .map_err(|_| http_errors::internal_server_error("Failed to submit setup key"))?;
    
    // For now, just log the submission
    tracing::info!("Submitting setup aggregated key: {:?}", setup_key);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_comm_info() {
        // Set up the store with a P2P address
        let test_address = P2PAddress {
            address: "127.0.0.1:8080".to_string(),
            peer_id: "L2_ID".to_string(),
        };
        BITVMX_STORE.set_p2p_address(test_address.clone());
        
        let response = get_comm_info().await.unwrap();
        let p2p_address = response;

        assert_eq!(p2p_address.peer_id, "L2_ID");
        assert!(p2p_address.address.contains("127.0.0.1:"));
        assert!(p2p_address.address.contains(":"));
    }

    #[tokio::test]
    async fn test_submit_aggregated_key() {
        let setup_key = SetupKey {
            id: "test-id-123".to_string(),
            addresses: vec![
                P2PAddress {
                    address: "127.0.0.1:8080".to_string(),
                    peer_id: "L2_ID".to_string(),
                }
            ],
        };
        
        let result = submit_aggregated_key(setup_key).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_submit_aggregated_key_empty_key() {
        let setup_key = SetupKey {
            id: "".to_string(),
            addresses: vec![
                P2PAddress {
                    address: "127.0.0.1:8080".to_string(),
                    peer_id: "L2_ID".to_string(),
                }
            ],
        };
        
        let result = submit_aggregated_key(setup_key).await;
        assert!(result.is_err());
        
        if let Err((status, _)) = result {
            assert_eq!(status, StatusCode::BAD_REQUEST);
        }
    }

    #[tokio::test]
    async fn test_submit_aggregated_key_empty_addresses() {
        let setup_key = SetupKey {
            id: "test-id-123".to_string(),
            addresses: vec![],
        };
        
        let result = submit_aggregated_key(setup_key).await;
        assert!(result.is_err());
        
        if let Err((status, _)) = result {
            assert_eq!(status, StatusCode::BAD_REQUEST);
        }
    }


}
