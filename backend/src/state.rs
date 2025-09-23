use crate::config::Config;
use crate::jobs::JobWorker;
use crate::rpc::rpc_client::RpcClient;
use crate::services::{bitvmx::BitvmxService, AddNumbersService};
use crate::services::{BitcoinService, WorkerService};
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
    /// Worker service
    pub worker_service: Arc<WorkerService>,
}

impl AppState {
    /// Create a new application state
    pub fn new(config: Config, rpc_client: Arc<RpcClient>, job_worker: Arc<JobWorker>) -> Self {
        let bitcoin_service = Arc::new(BitcoinService::new(config.bitcoin.clone()));
        let bitvmx_service = Arc::new(BitvmxService::new(
            rpc_client.clone(),
            bitcoin_service.clone(),
        ));

        // Create AddNumbersService without WorkerService
        let add_numbers_service = Arc::new(AddNumbersService::new(bitvmx_service.clone()));

        // Create WorkerService with the AddNumbersService
        let worker_service = Arc::new(WorkerService::new(
            job_worker.clone(),
            add_numbers_service.clone(),
        ));

        Self {
            config: Arc::new(config.clone()),
            add_numbers_service,
            bitcoin_service,
            bitvmx_service,
            rpc_client,
            worker_service,
        }
    }
}
