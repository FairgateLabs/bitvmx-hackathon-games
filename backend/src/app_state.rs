use std::sync::Arc;
use tokio::sync::RwLock;
use crate::config::Config;
use crate::services::{GameService, AddNumbersService, bitvmx::BitVMXService};
use crate::rpc::rpc_client::RpcClient;


/// Shared application state that can be accessed by both Axum routes and BitVMX RPC
#[derive(Clone, Debug)]
pub struct AppState {
    /// Configuration
    pub config: Arc<Config>,
    
    /// Game services
    pub game_service: Arc<RwLock<GameService>>,
    pub add_numbers_service: Arc<RwLock<AddNumbersService>>,
    
    /// BitVMX service
    pub bitvmx_service: Arc<RwLock<BitVMXService>>,
    
    /// BitVMX RPC client
    pub rpc_client: Arc<RpcClient>,
}

impl AppState {
    /// Create a new application state
    pub fn new(config: Config, rpc_client: Arc<RpcClient>) -> Self {
        Self {
            config: Arc::new(config),
            game_service: Arc::new(RwLock::new(GameService::new())),
            add_numbers_service: Arc::new(RwLock::new(AddNumbersService::new())),
            bitvmx_service: Arc::new(RwLock::new(BitVMXService::new(rpc_client.clone()))),
            rpc_client: rpc_client,
        }
    }
    
}



