# Job Worker System

This module provides an asynchronous job processing system that allows background execution of long-running tasks. It's designed to handle operations that need to run independently of the main request-response cycle, such as waiting for blockchain transactions or game state changes.

## Overview

The job worker system consists of three main components:

- **JobWorker**: Manages the job queue and spawns background tasks
- **Job Trait**: Defines the interface for executable jobs
- **WorkerService**: Provides a high-level API for enqueueing specific job types

## Key Components

### JobWorker

The main worker that manages job execution:

- **job_queue_sender**: Unbounded channel sender for enqueueing jobs
- **Background Processing**: Spawns each job in a separate task for parallel execution
- **Graceful Shutdown**: Responds to shutdown signals and completes pending jobs

### Job Trait

Defines the interface that all jobs must implement:

```rust
#[async_trait]
pub trait Job: Send + Sync + 'static {
    async fn run(self: Box<Self>) -> Result<(), anyhow::Error>;
}
```

### WorkerService

High-level service that provides convenient methods for enqueueing specific job types:

- **handle_start_game_tx()**: Enqueues a job to wait for game start transaction
- **handle_player2_wins_game_outcome_tx()**: Enqueues a job to wait for game outcome

## Core Functions

### `JobWorker::start(shutdown_tx)`

**Purpose**: Initialize and start the job worker system.

**Parameters**:

- `shutdown_tx`: Shutdown signal sender for graceful termination

**Returns**: A tuple containing:

- `Arc<JobWorker>`: The worker instance for enqueueing jobs
- `JoinHandle<Result<(), anyhow::Error>>`: Background task handle for the worker

**Example**:

```rust
let (job_worker, job_worker_task) = JobWorker::start(&shutdown_tx);
```

### `enqueue<J: Job>(job)`

**Purpose**: Add a job to the processing queue.

**Parameters**:

- `job`: Any type that implements the `Job` trait

**Returns**: `Result<(), anyhow::Error>`

**Example**:

```rust
let job = WaitStartGameJob {
    program_id,
    add_numbers_service: add_numbers_service.clone(),
};
job_worker.enqueue(job)?;
```

### `WorkerService::handle_start_game_tx(program_id)`

**Purpose**: Enqueue a job to wait for game start transaction confirmation.

**Parameters**:

- `program_id`: UUID of the game program

**Returns**: `Result<(), anyhow::Error>`

**Example**:

```rust
worker_service.handle_start_game_tx(program_id)?;
```

### `WorkerService::handle_player2_wins_game_outcome_tx(program_id)`

**Purpose**: Enqueue a job to wait for player 2's game outcome transaction.

**Parameters**:

- `program_id`: UUID of the game program

**Returns**: `Result<(), anyhow::Error>`

**Example**:

```rust
worker_service.handle_player2_wins_game_outcome_tx(program_id)?;
```

## Integration with AppState

The job worker is integrated into the application through the `AppState`:

```rust
// 1. Start job worker
let (job_worker, job_worker_task) = JobWorker::start(&shutdown_tx);

// 2. Create services
let add_numbers_service = Arc::new(AddNumbersService::new(bitvmx_service.clone()));
let worker_service = Arc::new(WorkerService::new(job_worker.clone(), add_numbers_service.clone()));

// 3. Initialize app state
let app_state = AppState::new(config, rpc_client, job_worker);
```

The `AppState` provides access to:

- **worker_service**: For enqueueing jobs from HTTP routes
- **add_numbers_service**: For direct service calls
- **job_worker**: For low-level job management

## Job Types

The system includes several predefined job types for common operations. See [Creating Custom Jobs](#creating-custom-jobs) for how to implement new job types.

## Job Execution Flow

1. Job is enqueued via WorkerService or JobWorker
2. JobWorker receives the job from the queue
3. Job is spawned in a separate async task for parallel execution
4. Job runs independently and can perform long-running operations
5. Job completes and logs success/failure

## Error Handling

- **Channel Errors**: Handles closed channels gracefully
- **Job Failures**: Individual job failures don't affect other jobs
- **Shutdown**: Graceful shutdown allows pending jobs to complete
- **Logging**: All job execution is traced with structured logging

## Thread Safety

The job worker is designed for concurrent use:

- Uses `Arc<JobWorker>` for shared ownership
- Unbounded channel allows unlimited job queuing
- Each job runs in its own task for true parallelism
- Thread-safe job enqueueing from multiple sources

## Usage Example

```rust
// Start the job worker
let (job_worker, job_worker_task) = JobWorker::start(&shutdown_tx);

// Create worker service
let worker_service = Arc::new(WorkerService::new(
    job_worker.clone(),
    add_numbers_service.clone(),
));

// Enqueue a job to wait for game start
worker_service.handle_start_game_tx(program_id)?;

// Enqueue a job to wait for game outcome
worker_service.handle_player2_wins_game_outcome_tx(program_id)?;

// Jobs will execute in background tasks
// The main application continues processing other requests
```

## Creating Custom Jobs

To create a new job type:

```rust
pub struct MyCustomJob {
    pub data: String,
    pub service: Arc<SomeService>,
}

#[async_trait]
impl Job for MyCustomJob {
    async fn run(self: Box<Self>) -> Result<(), anyhow::Error> {
        // Perform long-running operation
        self.service.do_something(&self.data).await?;
        Ok(())
    }
}

// Enqueue the job
let job = MyCustomJob {
    data: "example".to_string(),
    service: some_service.clone(),
};
job_worker.enqueue(job)?;
```
