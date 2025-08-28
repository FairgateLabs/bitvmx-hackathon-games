use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use crate::config::Config;
use crate::stores::{GameStore, AddNumbersStore};
use crate::stores::bitvmx::BITVMX_STORE;

/// Shared application state that can be accessed by both Axum routes and BitVMX RPC
#[derive(Clone, Debug)]
pub struct AppState {
    /// Configuration
    pub config: Arc<RwLock<Config>>,
    
    /// Game stores
    pub game_store: Arc<Mutex<GameStore>>,
    pub add_numbers_store: Arc<Mutex<AddNumbersStore>>,
    
    /// BitVMX store (already a global singleton, but included for completeness)
    pub bitvmx_store: Arc<crate::stores::bitvmx::BitVMXStore>,
}

impl AppState {
    /// Create a new application state
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            game_store: Arc::new(Mutex::new(GameStore::new())),
            add_numbers_store: Arc::new(Mutex::new(AddNumbersStore::new())),
            bitvmx_store: BITVMX_STORE.clone(),
        }
    }
    
    /// Get a reference to the configuration
    pub async fn get_config(&self) -> Config {
        self.config.read().await.clone()
    }
    
}

/// Global application state instance
pub static APP_STATE: once_cell::sync::Lazy<Arc<RwLock<Option<AppState>>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(None)));

/// Initialize the global application state
pub async fn init_app_state(config: Config) {
    let app_state = AppState::new(config);
    let mut state_guard = APP_STATE.write().await;
    *state_guard = Some(app_state);
}

/// Get the global application state
pub async fn get_app_state() -> Option<AppState> {
    let state_guard = APP_STATE.read().await;
    state_guard.clone()
}

/// Get the global application state or panic if not initialized
pub async fn get_app_state_or_panic() -> AppState {
    get_app_state().await.expect("AppState not initialized")
}
