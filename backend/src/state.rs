use crate::config::Config;
use crate::rpc::rpc_client::RpcClient;
use crate::services::BitcoinService;
use crate::services::{bitvmx::BitVMXService, AddNumbersService};
use std::sync::Arc;

/// Shared application state that can be accessed by both Axum routes and BitVMX RPC
#[derive(Clone, Debug)]
pub struct AppState {
    /// Configuration
    pub config: Arc<Config>,

    /// Game services
    pub add_numbers_service: Arc<AddNumbersService>,

    /// BitVMX service
    pub bitvmx_service: Arc<BitVMXService>,

    /// Bitcoin service
    pub bitcoin_service: Arc<BitcoinService>,

    /// BitVMX RPC client
    pub rpc_client: Arc<RpcClient>,
}

impl AppState {
    /// Create a new application state
    pub fn new(config: Config, rpc_client: Arc<RpcClient>) -> Self {
        let bitcoin_service = Arc::new(BitcoinService::new(config.bitcoin.clone()));
        Self {
            config: Arc::new(config.clone()),
            add_numbers_service: Arc::new(AddNumbersService::new()),
            bitcoin_service: bitcoin_service.clone(),
            bitvmx_service: Arc::new(BitVMXService::new(rpc_client.clone(), bitcoin_service)),
            rpc_client,
        }
    }
}
