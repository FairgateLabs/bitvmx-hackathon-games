use std::sync::Arc;
use tokio::sync::RwLock;
use crate::config::Config;
use crate::stores::{GameStore, AddNumbersStore, bitvmx::BitVMXStore};
use crate::rpc::bitvmx_rpc::RpcService;


/// Shared application state that can be accessed by both Axum routes and BitVMX RPC
#[derive(Clone, Debug)]
pub struct AppState {
    /// Configuration
    pub config: Arc<Config>,
    
    /// Game stores
    pub game_store: Arc<RwLock<GameStore>>,
    pub add_numbers_store: Arc<RwLock<AddNumbersStore>>,
    
    /// BitVMX store
    pub bitvmx_store: Arc<RwLock<BitVMXStore>>,
    
    /// BitVMX RPC client
    pub bitvmx_rpc: Arc<RpcService>,
}

impl AppState {
    /// Create a new application state
    pub fn new(config: Config, bitvmx_rpc: Arc<RpcService>) -> Self {
        Self {
            config: Arc::new(config),
            game_store: Arc::new(RwLock::new(GameStore::new())),
            add_numbers_store: Arc::new(RwLock::new(AddNumbersStore::new())),
            bitvmx_store: Arc::new(RwLock::new(BitVMXStore::new())),
            bitvmx_rpc: bitvmx_rpc,
        }
    }
    
}



