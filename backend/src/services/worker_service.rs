use uuid::Uuid;

use crate::{
    jobs::{JobWorker, WaitPlayer2WinsGameOutcomeJob, WaitStartGameJob},
    services::AddNumbersService,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct WorkerService {
    job_worker: Arc<JobWorker>,
    add_numbers_service: Arc<AddNumbersService>,
}

impl WorkerService {
    pub fn new(job_worker: Arc<JobWorker>, add_numbers_service: Arc<AddNumbersService>) -> Self {
        Self {
            job_worker: job_worker.clone(),
            add_numbers_service: add_numbers_service.clone(),
        }
    }

    pub fn handle_start_game_tx(&self, program_id: Uuid) -> Result<(), anyhow::Error> {
        self.job_worker.enqueue(WaitStartGameJob {
            program_id,
            add_numbers_service: self.add_numbers_service.clone(),
        })
    }

    pub fn handle_player2_wins_game_outcome_tx(
        &self,
        program_id: Uuid,
    ) -> Result<(), anyhow::Error> {
        self.job_worker.enqueue(WaitPlayer2WinsGameOutcomeJob {
            program_id,
            add_numbers_service: self.add_numbers_service.clone(),
        })
    }
}
