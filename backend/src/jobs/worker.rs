use std::sync::Arc;
// use std::time::Duration;
use tokio::sync::broadcast::Sender;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
// use tokio::time::sleep;
use tracing::{debug, info, warn, Instrument};

use crate::jobs::Job;

//const CHECK_SHUTDOWN_INTERVAL: u64 = 100; // 100 milliseconds

#[derive(Clone, Debug)]
pub struct JobWorker {
    job_queue_sender: mpsc::UnboundedSender<Box<dyn Job>>,
}

impl JobWorker {
    /// Start the job worker
    /// It will create a new job queue and spawn the worker in background
    pub fn start(shutdown_tx: &Sender<()>) -> (Arc<Self>, JoinHandle<Result<(), anyhow::Error>>) {
        let (tx, rx) = mpsc::unbounded_channel::<Box<dyn Job>>();

        let job_worker = Arc::new(Self {
            job_queue_sender: tx,
        });
        let worker_task = Self::spawn_worker(rx, shutdown_tx);
        (job_worker, worker_task)
    }

    /// Enqueue a job
    pub fn enqueue<J: Job>(&self, job: J) -> Result<(), anyhow::Error> {
        debug!("Enqueuing job: {}", std::any::type_name::<J>());
        self.job_queue_sender
            .send(Box::new(job))
            .map_err(|_| anyhow::anyhow!("Failed to enqueue job - channel closed"))?;
        Ok(())
    }

    /// Spawn the worker in background
    /// It will listen for jobs and spawn them in a separate task for parallel processing
    fn spawn_worker(
        mut queue_receiver: mpsc::UnboundedReceiver<Box<dyn Job>>,
        shutdown_tx: &Sender<()>,
    ) -> JoinHandle<Result<(), anyhow::Error>> {
        let mut shutdown_rx = shutdown_tx.subscribe();
        tokio::spawn(async move {
            info!("Job worker started");
            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        info!("Job worker received shutdown signal");
                        break;
                    }
                    job_result = queue_receiver.recv() => {
                        match job_result {
                            Some(job) => {
                                debug!("Received job, spawning execution");
                                // Spawn job execution in a separate task for parallel processing
                                tokio::spawn(async move {
                                    job.run().await.map_err(|e| anyhow::anyhow!("Job run failed: {}", e))
                                }.instrument(tracing::info_span!("job_worker")));
                            }
                            None => {
                                warn!("Job receiver channel closed");
                                break;
                            }
                        }
                    }
                    // _ = sleep(Duration::from_millis(CHECK_SHUTDOWN_INTERVAL)) => {
                    //     // Periodic check for shutdown signal when no jobs are queued
                    //     // This ensures we can respond to shutdown even when the queue is empty
                    // }
                }
            }
            info!("Job worker shutdown complete");
            Ok::<_, anyhow::Error>(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use tokio::sync::broadcast;

    // Implementation example of Job
    #[derive(Debug)]
    struct CounterJob {
        counter: Arc<AtomicU32>,
        id: u32,
    }

    #[async_trait]
    impl Job for CounterJob {
        async fn run(self: Box<Self>) -> Result<(), anyhow::Error> {
            let current = self.counter.fetch_add(1, Ordering::SeqCst);
            println!("Job {} executed, counter: {}", self.id, current + 1);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_job_worker_thread_safety() {
        let (shutdown_tx, _) = broadcast::channel(1);
        let (worker, worker_task) = JobWorker::start(&shutdown_tx);

        let counter = Arc::new(AtomicU32::new(0));
        let num_jobs = 100;

        // Spawn multiple tasks that enqueue jobs concurrently
        let mut handles = Vec::new();
        for i in 0..num_jobs {
            let worker_clone = worker.clone();
            let counter_clone = counter.clone();

            let handle = tokio::spawn(async move {
                let job = CounterJob {
                    counter: counter_clone,
                    id: i,
                };
                worker_clone.enqueue(job).unwrap();
            });
            handles.push(handle);
        }

        // Wait for all enqueue operations to complete
        for handle in handles {
            handle.await.unwrap();
        }

        // Give some time for jobs to execute
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Shutdown the worker
        let _ = shutdown_tx.send(());
        let _ = worker_task.await;

        // Verify all jobs were executed
        assert_eq!(counter.load(Ordering::SeqCst), num_jobs);
    }

    #[tokio::test]
    async fn test_job_worker_shutdown() {
        let (shutdown_tx, _) = broadcast::channel(1);
        let (worker, worker_task) = JobWorker::start(&shutdown_tx);

        // Enqueue a job
        let job = CounterJob {
            counter: Arc::new(AtomicU32::new(0)),
            id: 0,
        };
        worker.enqueue(job).unwrap();

        // Shutdown immediately
        let _ = shutdown_tx.send(());

        // Worker should shutdown gracefully
        let result = worker_task.await;
        assert!(result.is_ok());
    }
}
