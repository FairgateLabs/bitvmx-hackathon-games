use crate::config::Config;
use crate::jobs::JobWorker;
use crate::rpc::rpc_client::RpcClient;
use crate::services::BitcoinService;
use crate::services::{bitvmx::BitvmxService, AddNumbersService};
use std::sync::Arc;

/// Shared application state that can be accessed by both Axum routes and BitVMX RPC
#[derive(Clone, Debug)]
pub struct AppState {
    /// Configuration
    pub config: Arc<Config>,

    /// Game services
    pub add_numbers_service: Arc<AddNumbersService>,

    /// BitVMX service
    pub bitvmx_service: Arc<BitvmxService>,

    /// Bitcoin service
    pub bitcoin_service: Arc<BitcoinService>,

    /// BitVMX RPC client
    pub rpc_client: Arc<RpcClient>,

    /// Job worker
    pub job_worker: Arc<JobWorker>,
}

impl AppState {
    /// Create a new application state
    pub fn new(config: Config, rpc_client: Arc<RpcClient>, job_worker: Arc<JobWorker>) -> Self {
        let bitcoin_service = Arc::new(BitcoinService::new(config.bitcoin.clone()));
        let bitvmx_service = Arc::new(BitvmxService::new(
            rpc_client.clone(),
            bitcoin_service.clone(),
        ));
        let add_numbers_service = Arc::new(AddNumbersService::new(bitvmx_service.clone()));
        Self {
            config: Arc::new(config.clone()),
            add_numbers_service: add_numbers_service.clone(),
            bitcoin_service: bitcoin_service.clone(),
            bitvmx_service: bitvmx_service.clone(),
            rpc_client,
            job_worker,
        }
    }
}
