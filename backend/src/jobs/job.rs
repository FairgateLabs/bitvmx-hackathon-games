use crate::services::AddNumbersService;
use async_trait::async_trait;
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

// ---------- Definition of a generic Job ----------
#[async_trait]
pub trait Job: Send + Sync + Debug + 'static {
    async fn run(self: Box<Self>) -> Result<(), anyhow::Error>;
}

// Implementation of Jobs
#[derive(Debug)]
pub struct WaitStartGameJob {
    pub program_id: Uuid,
    pub add_numbers_service: Arc<AddNumbersService>,
}

#[async_trait]
impl Job for WaitStartGameJob {
    async fn run(self: Box<Self>) -> Result<(), anyhow::Error> {
        self.add_numbers_service
            .wait_start_game_tx(self.program_id)
            .await
    }
}

#[derive(Debug)]
pub struct WaitPlayer2WinsGameOutcomeJob {
    pub program_id: Uuid,
    pub add_numbers_service: Arc<AddNumbersService>,
}

#[async_trait]
impl Job for WaitPlayer2WinsGameOutcomeJob {
    async fn run(self: Box<Self>) -> Result<(), anyhow::Error> {
        self.add_numbers_service
            .wait_player2_wins_game_outcome_tx(self.program_id)
            .await
    }
}
