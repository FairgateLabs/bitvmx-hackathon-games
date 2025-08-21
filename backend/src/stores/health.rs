use std::time::{SystemTime, UNIX_EPOCH};

pub struct HealthStore {
    start_time: u64,
}

impl HealthStore {
    pub fn new() -> Self {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self { start_time }
    }

    pub fn get_uptime(&self) -> u64 {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        current_time - self.start_time
    }

    pub fn get_current_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

impl Default for HealthStore {
    fn default() -> Self {
        Self::new()
    }
}
